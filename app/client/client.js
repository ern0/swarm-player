URL = "ws://127.0.0.1:8080/";

document.addEventListener("DOMContentLoaded", main);

function main() {

	app = {};
	app.ws = null;
	init_ui();
	app.clock_skew = 0;
	app.heartbeat = null;

	app.intent = "offline";
	page("welcome");

	schedule_heartbeat(500);
	handle_button_connect(); /////////////////TODO
}

function init_ui() {

	elm("connect").onclick = handle_button_connect;
	elm("abort").onclick = handle_button_abort;
	elm("disconnect").onclick = handle_button_disconnect;

}

function elm(id) {
	return document.getElementById(id);
}

function page(req) {

	const pages = ["welcome", "op", "join", "bye"];

	for (var index in pages) {

		var pg = pages[index];

		if (pg == req) {
			show(pg);
		} else {
			hide(pg);
		}

	} // for

	if (req == "op") {
		display("...");
	}

	if (req == "bye") {
		setTimeout(function() {
			page("welcome");
		}, 1000);
	}

}

function intent(req) {
	app.intent = req;
}

function display(content) {
	elm("kontent").innerHTML = content;
}

function show(id) {
	elm(id).style.display = "block";
}

function hide(id) {
	elm(id).style.display = "none";
}

function create_websocket() {

	app.ws = new WebSocket(URL);
	
	app.ws.onopen = handle_socket_open;
	app.ws.onmessage = handle_socket_message;
	app.ws.onclose = handle_socket_close;

}

function discard_websocket() {
	if (app.ws != null) app.ws.close();
	app.ws = null;
}

function handle_socket_open(event) {
	page("op");
	clock_sync_start();
}

function handle_socket_message(event) {
	packet = JSON.parse(event.data);

	if (packet.type == "DISPLAY") display(packet.data);
	if (packet.type == "CLK_REF") clock_sync_eval(packet.data[0]);
};

function handle_socket_close(event) {

	if (app.intent == "offline") return;

	discard_websocket();
	page("join");
	setTimeout(create_websocket, 400);

}

function handle_button_connect() {
	create_websocket();
	intent("online");
}

function handle_button_abort() {
	discard_websocket();
	intent("offline");
	page("bye");
}

function handle_button_disconnect() {
	discard_websocket();
	intent("offline");
	page("bye");
}

function send(signature, args) {	

	if (!app.ws) return;
	if (app.ws.readyState != app.ws.OPEN) return;
	
	packet = { "type": signature, "data": args };
	data = JSON.stringify(packet);
	app.ws.send(data);

	schedule_heartbeat(1000);
}

function send_heartbeat() {
	send("HEARTBEAT", [get_clock(), app.clock_skew]);
}

function schedule_heartbeat(timeout) {

	if (app.heartbeat != null) {
		clearTimeout(app.heartbeat);
	}
	
	app.heartbeat = setTimeout(send_heartbeat, timeout);

}

function get_raw_clock() {

	var forced_error = -888;
	var now = Date.now() + forced_error;
	return now;
}

function get_clock(parm = undefined) {

	if (typeof(parm) == "undefined") {
		var now = get_raw_clock();
	} else {
		var now = parm;
	}
	var corrected = now - app.clock_skew;

	return corrected;
}

function clock_sync_start() {
	app.clock_c0 = get_raw_clock();
	send("CLK_0", [app.clock_c0]);
}

function clock_sync_eval(clock_ref) {
	
	app.clock_c1 = get_raw_clock();

	var turnaround = app.clock_c1 - app.clock_c0;
	var distance = turnaround / 2;
	var estimation = app.clock_c0 + distance;
	app.clock_skew = Math.round(estimation - clock_ref);

	var repeat = 20000 + Math.random() * 5000;
	setTimeout(clock_sync_start, repeat);

	if (true) {
		var z = 1677710000000; 
		var t0 = app.clock_c0 - z;
		var tref = clock_ref - z;
		var t1 = app.clock_c1 - z;
		var est = estimation - z;
		console.log("--");
		console.log("t0:", t0, "tref:", tref, "t1:", t1);
		console.log("turnaround:", turnaround, "distance:", distance);	
		console.log("estimation:", est, "skew:", app.clock_skew);
	}

}

