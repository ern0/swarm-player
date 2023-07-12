SERVER_URL = "ws://127.0.0.1:8080/";

document.addEventListener("DOMContentLoaded", main);

function main() {

	init_app_props();
	init_url_options();
	clock_sync_reset();
	init_bind_buttons();

	app.intent = "offline";
	page("welcome");

	if (app.opt_autoconnect) {
		handle_button_connect();
	}

}

function init_app_props() {

	app = {};
	app.websocket = null;
	app.heartbeat = null;

}

function init_url_options() {

	var url = new URL(document.location.href);
	init_url_option_skew(url);
	init_url_option_autoconnect(url);

}

function init_url_option_skew(url) {

	app.opt_skew = url.searchParams.get("skew");

	if (app.opt_skew == null) {
		app.opt_skew = 0;
	} else {
		app.opt_skew = Math.round(app.opt_skew);
		console.warn("TEST: skew is set to", app.opt_skew);
	}
	
}

function init_url_option_autoconnect(url) {

	app.opt_autoconnect = url.searchParams.get("autoconnect");
	if (app.opt_autoconnect == null) {
		app.opt_autoconnect = false; 
	}
	if (!app.opt_autoconnect) {
		app.opt_autoconnect = +app.opt_autoconnect;
	}
	if (app.opt_autoconnect) {
		console.warn("TEST: autoconnect is active");
	}

}

function init_bind_buttons() {

	elm("connect").onclick = handle_button_connect;
	elm("abort").onclick = handle_button_abort_or_disconnect;
	elm("disconnect").onclick = handle_button_abort_or_disconnect;

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

	app.websocket = new WebSocket(SERVER_URL);
	
	app.websocket.onopen = handle_socket_open;
	app.websocket.onmessage = handle_socket_message;
	app.websocket.onclose = handle_socket_close;

}

function discard_websocket() {
	if (app.websocket != null) app.websocket.close();
	app.websocket = null;
}

function handle_socket_open(event) {

	page("op");

	schedule_heartbeat(500);
	clock_sync_reset();
	clock_sync_start();	

}

function handle_socket_message(event) {
	packet = JSON.parse(event.data);

	if (packet.type == "DISPLAY") display(packet.data);
	if (packet.type == "CLK_REF") clock_sync_eval(packet.data[0]);
	
	//...

};

function handle_socket_close(event) {

	stop_heartbeat();
	discard_websocket();

	if (app.intent == "offline") return;

	page("join");
	setTimeout(create_websocket, 400);

}

function handle_button_connect() {

	create_websocket();
	intent("online");

}

function handle_button_abort_or_disconnect() {

	discard_websocket();
	intent("offline");
	page("bye");

}

function send(signature, args) {	

	if (!app.websocket) return;
	if (app.websocket.readyState != app.websocket.OPEN) return;
	
	packet = { "type": signature, "data": args };
	data = JSON.stringify(packet);
	app.websocket.send(data);

	schedule_heartbeat(1000);
}

function send_heartbeat() {
	send("HEARTBEAT", [get_clock(), app.clock_skew]);
}

function schedule_heartbeat(timeout) {

	stop_heartbeat();	
	app.heartbeat = setTimeout(send_heartbeat, timeout);

}

function stop_heartbeat() {

	if (app.heartbeat == null) return;

	clearTimeout(app.heartbeat);
	app.heartbeat = null;

}

function get_raw_clock() {
	var now = Date.now() + app.opt_skew;
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

function clock_sync_reset() {
	app.clock_skew = 0;
	app.clock_sync_round = 0;
}

function clock_sync_start() {
	app.clock_c0 = get_raw_clock();
	send("CLK_0", [app.clock_skew]);
}

function clock_sync_eval(clock_ref) {
	
	clock_sync_calc_skew(clock_ref);
	clock_sync_reschedule();
}

function clock_sync_calc_skew(clock_ref) {

	app.clock_c1 = get_raw_clock();

	var turnaround = app.clock_c1 - app.clock_c0;
	var distance = turnaround / 2;
	var estimation = app.clock_c0 + distance;
	var skew = Math.round(estimation - clock_ref);

	var change = Math.abs(app.clock_skew - skew);
	if (change > 50) app.clock_skew = skew;

	//clock_sync_debug(clock_ref, turnaround, distance, estimation, skew, change);
}

function clock_sync_reschedule() {

	var sleep_duration_s = 2;
	if (app.clock_sync_round > 0) sleep_duration_s = 5;
	if (app.clock_sync_round > 2) sleep_duration_s = 15;
	if (app.clock_sync_round > 5) sleep_duration_s = 30;
	app.clock_sync_round += 1;

	var sleep_duration_ms = sleep_duration_s * 1000;
	sleep_duration_ms += Math.random() * 2000;

	setTimeout(clock_sync_start, sleep_duration_ms);
}

function clock_sync_debug(clock_ref, turnaround, distance, estimation, skew) {

	var z = 0; 
	var t0 = app.clock_c0 - z;
	var tref = clock_ref - z;
	var t1 = app.clock_c1 - z;
	var est = estimation - z;

	console.log("--");
	console.log("t_0:", t0, "t_ref:", tref, "t_1:", t1);
	console.log("turnaround:", turnaround, "distance:", distance);	
	console.log("estimation:", est, "meas.skew:", skew, "eff.skew:", app.clock_skew);
}