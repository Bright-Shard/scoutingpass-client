// This file overwrites the `TBAInterface.js` file. It overrides the functions
// there to cache TBA data instead of getting it live every time.
//
// Since this file provides an easy way to hook the ScoutingPASS app, it's
// also used to cache QR codes.

var teams = null;
var schedule = null;
const INVOKE = window.__TAURI__.core.invoke;
let EVENT_CODE;

function getTeams(eventCode) {
	EVENT_CODE = eventCode;
	INVOKE("get_tba_data", { eventCode: eventCode, teamsOrMatches: "teams" })
		.then((data) => {
			teams = JSON.parse(data);
		})
		.catch((err) => console.log("getTeams: Error getting TBA data: " + err));
}

function getSchedule(eventCode) {
	INVOKE("get_tba_data", { eventCode: eventCode, teamsOrMatches: "matches" })
		.then((data) => (schedule = JSON.parse(data)))
		.catch((err) =>
			console.log("getSchedule: Error getting TBA data: " + err),
		);
}

function onQRCodeAppear(event) {
	var event = event[0];
	if (event.isIntersecting) {
		var qrCodeCanvas = document
			.getElementById("qrcode")
			.getElementsByTagName("canvas")[0];
		INVOKE("save_qr", {
			event: EVENT_CODE,
			dataUrl: qrCodeCanvas.toDataURL(),
		});
	}
}

window.addEventListener("load", () => {
	let observer = new IntersectionObserver(onQRCodeAppear, null);
	observer.observe(document.getElementById("qr-code"));
});
