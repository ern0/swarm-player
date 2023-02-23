URL = "ws://127.0.0.1:8080/";

document.addEventListener("DOMContentLoaded", main);

function main() {

	app = {};
	app.ws = null;
	init_ui();

	app.intent = "offline";
	page("welcome");

	setInterval(function() {
		if ((app.ws) && (app.ws.readyState == app.ws.OPEN)) {
			app.ws.send("client->server");
		}
	}, 2500);

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

	if (req == "bye") {
		setTimeout(function() {
			page("welcome");
		}, 1000);
	}

}

function intent(req) {
	app.intent = req;
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
}

function handle_socket_message(event) {
	elm("kontent").innerHTML = event.data;
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