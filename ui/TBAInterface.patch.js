// This file overwrites the `TBAInterface.js` file. It overrides the functions there to cache data instead of getting it live every time.

var teams = null;
var schedule = null;
const INVOKE = window.__TAURI__.core.invoke;

function getTeams(eventCode) {
    INVOKE("get_tba_data", { eventCode: eventCode, teamsOrMatches: "teams" })
        .then((data) => { teams = JSON.parse(data); console.log(data); })
        .catch((err) => console.log("getTeams: Error getting TBA data: " + err))
}

function getSchedule(eventCode) {
    INVOKE("get_tba_data", { eventCode: eventCode, teamsOrMatches: "matches" })
        .then((data) => schedule = JSON.parse(data))
        .catch((err) => console.log("getSchedule: Error getting TBA data: " + err))
}
