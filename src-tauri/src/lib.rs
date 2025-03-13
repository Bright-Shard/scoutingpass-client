use {
	bytes::Bytes,
	scraper::{Html, Selector},
	std::{
		fs::{self, File, OpenOptions, Permissions},
		io::{Read, Write},
		os::unix::fs::PermissionsExt,
		path::PathBuf,
	},
	tauri::{AppHandle, Manager},
	tauri_plugin_http::reqwest::{Client, Url},
};

/// The URL this app downloads and caches
// const URL: &str = "https://team900.org/ScoutingPASS/";
const URL: &str = "https://smritikalidindi.github.io/ScoutingPASS-SSSSS/";
/// Our API key for The Blue Alliance
const TBA_API_KEY: &str = "uTHeEfPigDp9huQCpLNkWK7FBQIb01Qrzvt4MAjh9z2WQDkrsvNE77ch6bOPvPb6";

const TBA_INTERFACE_PATCH: &[u8] = include_bytes!("../../ui/TBAInterface.patch.js");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_http::init())
		.invoke_handler(tauri::generate_handler![
			launch,
			download,
			clean_cache,
			get_tba_data,
			save_qr,
			get_qr_code_events,
			get_qr_codes
		])
		.setup(|app| {
			let webview = app.get_webview_window("main").unwrap();
			webview
				.with_webview(|webview| {
					#[cfg(target_os = "android")]
					{
						// Android's webview disallows loading local files by default. Have to use the
						// `setAllowFileAccess` method to enable it. That method is on the `WebSettings`
						// class, which you can get with `getSettings`.
						use jni::objects::{JValue, JValueOwned};
						webview.jni_handle().exec(|env, _, webview| {
							let settings = env
								.call_method(
									webview,
									"getSettings",
									"()Landroid/webkit/WebSettings;",
									&[],
								)
								.unwrap();
							let JValueOwned::Object(settings) = settings else {
								unreachable!();
							};
							env.call_method(
								settings,
								"setAllowFileAccess",
								"(Z)V",
								&[JValue::Bool(jni::sys::JNI_TRUE)],
							)
							.unwrap();
						})
					}
				})
				.unwrap();

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("scoutn't");
}

#[tauri::command]
async fn download(app: AppHandle, page: String) -> Result<(), String> {
	// (url, page_contents)
	let mut pages: Vec<(String, Bytes)> = Vec::new();
	let client = Client::new();
	let cache_dir = cache_folder_path(&app);
	let base_url = URL.parse::<Url>().map_err(|e| e.to_string())?;

	if !cache_dir.exists() {
		fs::create_dir(&cache_dir).map_err(|e| e.to_string())?;
		fs::set_permissions(&cache_dir, Permissions::from_mode(0o777)).unwrap();
	}

	let index_page = client
		.get(base_url.join(&page).map_err(|e| e.to_string())?)
		.send()
		.await
		.map_err(|e| e.to_string())?
		.text()
		.await
		.map_err(|e| e.to_string())?;

	let mut resources = Vec::new();
	let mut config_url = None;
	// Scripts imported by the page
	Html::parse_document(&index_page)
		.select(&Selector::parse("script").map_err(|e| e.to_string())?)
		.filter_map(|sel| sel.attr("src"))
		.for_each(|src| {
			if src.ends_with("config.js") {
				if let Some(ref old_config) = config_url {
					panic!("Found two config paths: `{old_config}` and `{src}`.");
				}
				config_url = Some(src.to_string());
			}
			resources.push(src.to_string())
		});
	Html::parse_document(&index_page)
		.select(&Selector::parse("link").map_err(|e| e.to_string())?)
		.filter_map(|sel| sel.attr("href"))
		.for_each(|href| resources.push(href.to_string()));
	if let Some(cfg_url) = config_url {
		let cfg = client
			.get(base_url.join(&cfg_url).map_err(|e| e.to_string())?)
			.send()
			.await
			.map_err(|e| e.to_string())?
			.text()
			.await
			.map_err(|e| e.to_string())?;

		cfg.lines()
			.filter(|line| line.contains("\"filename\":"))
			.for_each(|filename_cfg| {
				let mut key_val = filename_cfg.split(':');
				key_val.next().unwrap();
				let url = key_val
					.next()
					.expect("Failed to find file in `filename` config");
				let opening_quote = url
					.find('"')
					.expect("Failed to find file in `filename` config");
				let url = &url[opening_quote + 1..];
				let closing_quote = url
					.find('"')
					.expect("Failed to find file in `filename` config");
				resources.push(url[..closing_quote].to_string());
			})
	} else {
		panic!("Failed to find config path (usually `<game>_config.js`).");
	}
	for src in resources {
		println!("Downloading `{}`", base_url.join(&src).unwrap());
		let page = client
			.get(base_url.join(&src).map_err(|e| e.to_string())?)
			.send()
			.await
			.map_err(|e| e.to_string())?
			.bytes()
			.await
			.map_err(|e| e.to_string())?;
		pages.push((src, page));
	}

	pages.push((page, index_page.into_bytes().into()));
	pages.push((
		String::from("resources/js/TBAInterface.js"),
		TBA_INTERFACE_PATCH.into(),
	));
	for (path, page) in pages {
		let path = cache_dir.join(path);
		println!("Writing to `{}`", path.display());

		let parent = path.parent().unwrap();
		if !parent.exists() {
			fs::create_dir_all(parent).map_err(|e| e.to_string())?;
		}

		let mut out_file = OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(path)
			.map_err(|e| e.to_string())?;
		out_file.write_all(&page).map_err(|e| e.to_string())?;
	}

	Ok(())
}

#[tauri::command]
async fn launch(app: AppHandle, page: String) -> Result<(), String> {
	let cache_file_path = cache_folder_path(&app).join(&page);
	if !cache_file_path.exists() {
		download(app.clone(), page).await?;
	}
	app.get_webview_window("main")
		.unwrap()
		.navigate(Url::from_file_path(cache_file_path).unwrap())
		.unwrap();

	Ok(())
}

#[tauri::command]
fn clean_cache(app: AppHandle) -> Result<(), String> {
	fs::remove_dir_all(cache_folder_path(&app)).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_tba_data(
	app: AppHandle,
	event_code: String,
	teams_or_matches: String,
) -> Result<String, String> {
	let cache_path = cache_folder_path(&app)
		.join("tba-cache")
		.join(&format!("{event_code}-{teams_or_matches}.json"));
	if !cache_path.parent().unwrap().exists() {
		fs::create_dir_all(&cache_path.parent().unwrap()).map_err(|e| e.to_string())?;
	}

	if cache_path.exists() {
		fs::read_to_string(cache_path).map_err(|e| e.to_string())
	} else {
		let client = Client::new();
		let data = client
			.get(format!(
			"https://www.thebluealliance.com/api/v3/event/{event_code}/{teams_or_matches}/simple"
		))
			.header("X-TBA-AUTH-KEY", TBA_API_KEY)
			.send()
			.await
			.map_err(|e| e.to_string())?
			.text()
			.await
			.map_err(|e| e.to_string())?;

		let mut out_file = OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(cache_path)
			.map_err(|e| e.to_string())?;
		out_file
			.write_all(data.as_bytes())
			.map_err(|e| e.to_string())?;

		Ok(data)
	}
}

#[tauri::command]
async fn save_qr(app: AppHandle, event: String, data_url: String) {
	let cache_folder = cache_folder_path(&app).join("qrcodes");
	if !cache_folder.exists() {
		fs::create_dir_all(&cache_folder).unwrap();
	}

	let qr_cache_file = cache_folder.join(event);
	let mut file = File::options()
		.create(true)
		.append(true)
		.open(&qr_cache_file)
		.unwrap();

	file.write_all(format!("{data_url}\n").as_bytes()).unwrap();
}
#[tauri::command]
async fn get_qr_code_events(app: AppHandle) -> Vec<String> {
	let cache_folder = cache_folder_path(&app).join("qrcodes");
	if !cache_folder.exists() {
		fs::create_dir_all(&cache_folder).unwrap();
	}

	cache_folder
		.read_dir()
		.unwrap()
		.into_iter()
		.map(|entry| entry.unwrap().file_name().into_string().unwrap())
		.collect()
}
#[tauri::command]
async fn get_qr_codes(app: AppHandle, event: String) -> Vec<String> {
	let cache = cache_folder_path(&app).join("qrcodes").join(event);
	let mut file_contents = String::new();
	File::options()
		.read(true)
		.open(cache)
		.unwrap()
		.read_to_string(&mut file_contents)
		.unwrap();
	file_contents
		.lines()
		.into_iter()
		.map(String::from)
		.collect()
}

fn cache_folder_path(app: &AppHandle) -> PathBuf {
	app.path().download_dir().unwrap().join("scout-app-cache")
}
