WEBSOCKET_PORT = 8080;
HEARTBEAT_TIMING_S = [0.3, 10];
CLOCK_SYNC_TIMING_S = [ 0, 0.5, 2, 5, 30 ];

document.addEventListener("DOMContentLoaded", main);

function main() {

	init_app_props();
	init_url_options();
	clock_sync_reset();
	init_bind_buttons();
	reset_stat();

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
	init_url_connection(url);
	init_url_option_skew(url);
	init_url_option_autoconnect(url);

}

function init_url_connection(url) {

	var protocol = url.protocol.replace("http", "ws");

	app.server_url = protocol + "//";
	app.server_url += url.hostname;
	app.server_url += ":" + WEBSOCKET_PORT;

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

	elm("cmd_red").onclick = handle_button_cmd;
	elm("cmd_green").onclick = handle_button_cmd;
	elm("cmd_blue").onclick = handle_button_cmd;
	elm("cmd_yellow").onclick = handle_button_cmd;
	elm("cmd_gray").onclick = handle_button_cmd;

}

function reset_stat() {

	app.stat_count = 0;
	app.stat_min_delay = 0;
	app.stat_max_delay = 0;

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
		set_color("white");
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

function set_color(color) {
	elm("control").style.backgroundColor = color;
}

function show(id) {
	elm(id).style.display = "block";
}

function hide(id) {
	elm(id).style.display = "none";
}

function create_websocket() {

	app.websocket = new WebSocket(app.server_url);
	
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

	schedule_heartbeat(HEARTBEAT_TIMING_S[0]);
	clock_sync_reset();
	clock_sync_start();	

}

function handle_socket_message(event) {

	var packet = JSON.parse(event.data);

	if (packet.stamp == 0) {
		process_packet(packet);
		return;
	}

	var now = get_clock();
	var action = packet.stamp;
	var delay = action - now;

	app.stat_count += 1;
	if (app.stat_min_delay == 0 || app.stat_min_delay < delay) {
		app.stat_min_delay = delay;
	}
	if (app.stat_max_delay == 0 || app.stat_max_delay > delay) {
		app.stat_max_delay = delay;
	}

	if (delay < 1) delay = 1;

	setTimeout(function() {
		process_packet(packet);
	}, delay);

}

function process_packet(packet) {
	if (packet.type == "CLK_REF") clock_sync_eval(packet.data[0]);
	if (packet.type == "DISPLAY") display(packet.data[0]);
	if (packet.type == "COLOR") set_color(packet.data[0]);
	//...
}

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

function handle_button_cmd() {

	var color = this.id.split("_")[1];
	send("COLOR", [color]);

}

function send(signature, args) {	

	if (!app.websocket) return;
	if (app.websocket.readyState != app.websocket.OPEN) return;
	
	if ((typeof args) != "object") {
		console.error("INTERNAL: invalid send format");
		return;
	}

	packet = { "type": signature, "data": args };
	data = JSON.stringify(packet);
	app.websocket.send(data);

	schedule_heartbeat(HEARTBEAT_TIMING_S[1]);
}

function send_heartbeat() {

	send("HEARTBEAT", [
		app.clock_skew,
		app.stat_count,
		app.stat_min_delay,
		app.stat_max_delay
	]);

	reset_stat();

}

function schedule_heartbeat(timeout_s) {

	stop_heartbeat();	
	app.heartbeat = setTimeout(send_heartbeat, timeout_s * 1000);

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
	if (change > 30) app.clock_skew = skew;

}

function clock_sync_reschedule() {

	if (app.clock_sync_round >= CLOCK_SYNC_TIMING_S.length) {
		var sleep_duration_s = CLOCK_SYNC_TIMING_S[CLOCK_SYNC_TIMING_S.length - 1];
	} else {
		var sleep_duration_s = CLOCK_SYNC_TIMING_S[app.clock_sync_round];
	}
	app.clock_sync_round += 1;

	var sleep_duration_ms = sleep_duration_s * 1000;
	sleep_duration_ms += Math.random() * (sleep_duration_ms / 10);

	setTimeout(clock_sync_start, sleep_duration_ms);
	
}
