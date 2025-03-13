// pick or show
let STATE = "pick";
let EVENT = "";
const INVOKE = window.__TAURI__.core.invoke;

let view;
let pick;
let list;
let events;

function render() {
	while (view.firstChild) {
		view.removeChild(view.firstChild);
	}

	if (STATE == "pick") {
		let label = document.createElement("h2");
		label.innerText = "Pick an event to view QR codes from:";
		view.appendChild(label);

		for (var event of events) {
			var btn = document.createElement("button");
			btn.innerText = event;
			btn.addEventListener("click", () => viewEvent(event));
			view.appendChild(btn);
		}
		view.style = "--gap:10px;";
	} else if (STATE == "show") {
		view.innerHTML = "";
		INVOKE("get_qr_codes", { event: EVENT }).then((codes) => {
			for (let code of codes) {
				var img = document.createElement("img");
				img.src = code;
				view.appendChild(img);
			}
			view.style = "--gap:20px;";
		});
	}
}

function viewEvent(event) {
	STATE = "show";
	EVENT = event;
	render();
}

function goBack() {
	if (STATE == "pick") {
		document.location.href = "index.html";
	} else if (STATE == "show") {
		STATE = "pick";
		render();
	}
}

window.addEventListener("load", () => {
	view = document.getElementById("view");
	pick = document.getElementById("pick-events");
	list = document.getElementById("qr-codes");

	document.getElementById("back").addEventListener("click", goBack);

	INVOKE("get_qr_code_events").then((_events) => {
		events = _events;
		render();
	});
});
