URL = "ws://127.0.0.1:8080/";

document.addEventListener("DOMContentLoaded", main);

function main() {

	app = {};
	app.ws = null;
	init_ui();
	app.clock_skew = 0;

	app.intent = "offline";
	page("welcome");

	setTimeout(heartbeat, 500);

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

	if (packet.type == "DISP") display(packet.data);
	if (packet.type == "CLKR") clock_sync_eval(packet.data);
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

}

function heartbeat() {
	send("heartbeat", [get_clock(), app.clock_skew]);
	setTimeout(heartbeat, 2500);
}

function get_clock() {
	return Date.now() + app.clock_skew;
}

function clock_sync_start() {
	app.clock_c0 = Date.now();
	send("CLK0", [app.clock_c0]);
}

function clock_sync_eval(clock_ref) {
	app.clock_c1 = Date.now();
	console.log(app.clock_c0, clock_ref, app_clock_c1);///
}
