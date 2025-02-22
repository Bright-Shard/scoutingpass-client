var cleanCachePresses = 0;
const INVOKE = window.__TAURI__.core.invoke;

window.addEventListener("load", () => {
  let pitLaunchBtn = document.querySelector("#pit-launch");
  let standLaunchBtn = document.querySelector("#stand-launch");
  let cleanCacheBtn = document.querySelector("#clean-cache");
  let downloadBtn = document.querySelector("#download");
  let log = document.querySelector("#log");

  function print(text) {
    log.innerHTML += text;
    log.innerHTML += "<br>";
  }

  pitLaunchBtn.addEventListener("click", (_) => {
    INVOKE("launch", { page: "pit.html" })
      .catch((error) => print(error));
  });
  standLaunchBtn.addEventListener("click", (_) => {
    INVOKE("launch", { page: "index.html" })
      .catch((error) => print(error));
  });

  downloadBtn.addEventListener("click", (_) => {
    print("Note: To get team autocompletion working, you need to launch the stand scouter at least once.");

    print("Downloading stand scouter...");
    INVOKE("download", { page: "index.html" })
      .then(() => print("Finished downloading stand scouter."))
      .catch((error) => print("Error downloading stand scouter: " + error));

    print("Downloading pit scouter...");
    INVOKE("download", { page: "pit.html" })
      .then(() => print("Finished downloading pit scouter."))
      .catch((error) => print("Error downloading pit scouter: " + error));
  })

  cleanCacheBtn.addEventListener("click", (_) => {
    if (cleanCachePresses < 1) {
      cleanCachePresses += 1;
      print("DO NOT RUN THIS IN A COMPETITION, YOU WILL HAVE TO REDOWNLOAD SCOUTER, WHICH REQUIRES WIFI.");
      print("If you really want to clean the cache, hit the clean cache button again.");
    } else {
      INVOKE("clean_cache")
        .then(() => print("Cache cleaned; please redownload scouter to keep using it."))
        .catch((error) => print(error));
      cleanCachePresses = 0;
    }
  });
});
