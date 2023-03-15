WEBSOCKET_PORT = 8080;
AUTOCONNECT = 1
CLOCK_SKEW = 300

HEARTBEAT_TIMING_S = [0.3, 10];
CLOCK_SYNC_TIMING_S = [0, 0.5, 2, 5, 30];

document.addEventListener("DOMContentLoaded", main);

function main()
{

	app = {};
	init_awake();
	init_options();

	if (app.opt_autoconnect) {
		check_update_retry(first_update_proc);
	} else {
		startup();
	}

}

function init_awake()
{

	if (init_awake_wakelock()) {
		elm("nosleep").innerHTML = "wakelock";
		return;
	}

	init_awake_video();
	elm("nosleep").innerHTML = "awake-video";

}

function init_awake_wakelock()
{

	if ((typeof navigator.wakeLock) == "undefined") {
		return false;
	}

	navigator.wakeLock.request("screen");

	return true;
}

function set_video_data(elm, fmt, data)
{

	var source = document.createElement("source");
	source.src = data;
	source.type = 'video/' + fmt;
	elm.appendChild(source);

}

function init_awake_video()
{

	const webm_data = "data:video/webm;base64,GkXfo0AgQoaBAUL3gQFC8oEEQvOBCEKCQAR3ZWJtQoeBAkKFgQIYU4BnQI0VSalmQCgq17FAAw9CQE2AQAZ3aGFtbXlXQUAGd2hhbW15RIlACECPQAAAAAAAFlSua0AxrkAu14EBY8WBAZyBACK1nEADdW5khkAFVl9WUDglhohAA1ZQOIOBAeBABrCBCLqBCB9DtnVAIueBAKNAHIEAAIAwAQCdASoIAAgAAUAmJaQAA3AA/vz0AAA=";
	const mp4_data = "data:video/mp4;base64,AAAAIGZ0eXBtcDQyAAACAGlzb21pc28yYXZjMW1wNDEAAAAIZnJlZQAACKBtZGF0AAAC8wYF///v3EXpvebZSLeWLNgg2SPu73gyNjQgLSBjb3JlIDE0MiByMjQ3OSBkZDc5YTYxIC0gSC4yNjQvTVBFRy00IEFWQyBjb2RlYyAtIENvcHlsZWZ0IDIwMDMtMjAxNCAtIGh0dHA6Ly93d3cudmlkZW9sYW4ub3JnL3gyNjQuaHRtbCAtIG9wdGlvbnM6IGNhYmFjPTEgcmVmPTEgZGVibG9jaz0xOjA6MCBhbmFseXNlPTB4MToweDExMSBtZT1oZXggc3VibWU9MiBwc3k9MSBwc3lfcmQ9MS4wMDowLjAwIG1peGVkX3JlZj0wIG1lX3JhbmdlPTE2IGNocm9tYV9tZT0xIHRyZWxsaXM9MCA4eDhkY3Q9MCBjcW09MCBkZWFkem9uZT0yMSwxMSBmYXN0X3Bza2lwPTEgY2hyb21hX3FwX29mZnNldD0wIHRocmVhZHM9NiBsb29rYWhlYWRfdGhyZWFkcz0xIHNsaWNlZF90aHJlYWRzPTAgbnI9MCBkZWNpbWF0ZT0xIGludGVybGFjZWQ9MCBibHVyYXlfY29tcGF0PTAgY29uc3RyYWluZWRfaW50cmE9MCBiZnJhbWVzPTMgYl9weXJhbWlkPTIgYl9hZGFwdD0xIGJfYmlhcz0wIGRpcmVjdD0xIHdlaWdodGI9MSBvcGVuX2dvcD0wIHdlaWdodHA9MSBrZXlpbnQ9MzAwIGtleWludF9taW49MzAgc2NlbmVjdXQ9NDAgaW50cmFfcmVmcmVzaD0wIHJjX2xvb2thaGVhZD0xMCByYz1jcmYgbWJ0cmVlPTEgY3JmPTIwLjAgcWNvbXA9MC42MCBxcG1pbj0wIHFwbWF4PTY5IHFwc3RlcD00IHZidl9tYXhyYXRlPTIwMDAwIHZidl9idWZzaXplPTI1MDAwIGNyZl9tYXg9MC4wIG5hbF9ocmQ9bm9uZSBmaWxsZXI9MCBpcF9yYXRpbz0xLjQwIGFxPTE6MS4wMACAAAAAOWWIhAA3//p+C7v8tDDSTjf97w55i3SbRPO4ZY+hkjD5hbkAkL3zpJ6h/LR1CAABzgB1kqqzUorlhQAAAAxBmiQYhn/+qZYADLgAAAAJQZ5CQhX/AAj5IQAD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";

	vid = elm("awake");

	if (false) {
		if ((typeof document.hidden) != "undefined") {
			vid.setAttribute("hidden", "hidden");
			vid.setAttribute("visibilitychange", "visibilitychange");
		} else if ((typeof document.msHidden) != "undefined") {
			vid.setAttribute("hidden", "msHidden");
			vid.setAttribute("visibilitychange", "msvisibilitychange");
		} else if ((typeof document.webkitHidden) != "undefined") {
			vid.setAttribute("hidden", "webkitHidden");
			vid.setAttribute("visibilitychange", "webkitvisibilitychange");
		}
	}

	vid.setAttribute("loop", "");
	vid.setAttribute("muted", true);
	vid.muted = true;
	vid.setAttribute("title", "awake");
	vid.setAttribute("playsinline", "");

	set_video_data(vid, "webm", webm_data);
	set_video_data(vid, "mp4", mp4_data);

	if (false) {
		vid.addEventListener("loadedmetadata", function ()
		{

			if (vid.duration && vid.duration <= 1) {
				vid.setAttribute("loop", "");
			} else {
				vid.addEventListener("timeupdate", function ()
				{
					if (vid.currentTime > 0.5) {
						vid.currentTime = Math.random();
					}
				}); // timeupdate
			}
		}); // loadedmetadata
	}

	vid.play();

}

function check_update_retry(callback)
{

	app.update_callback = callback;

	var xhr = new XMLHttpRequest();
	xhr.open("GET", "client.js");
	xhr.send(null);

	xhr.onreadystatechange = check_update_ajax_handler;

}

function check_update_ajax_handler()
{

	var DONE = 4;
	var OK = 200;

	if (this.readyState != DONE) return;

	if (this.status == OK) {
		app.update_callback(this.responseText);
		return;
	}

	console.log('AJAX error: ' + this.status);

	setTimeout(function ()
	{
		check_update_retry(app.update_callback);
	}, 200);

}

function first_update_proc(response)
{

	app.update_hash = cyrb53(response);
	startup();

}

function cyrb53(str, seed = 0)
{

	var h1 = 0xdeadbeef ^ seed;
	var h2 = 0x41c6ce57 ^ seed;

	for (var i = 0; i < str.length; i++) {
		var ch = str.charCodeAt(i);
		h1 = Math.imul(h1 ^ ch, 2654435761);
		h2 = Math.imul(h2 ^ ch, 1597334677);
	}

	h1 = Math.imul(h1 ^ (h1 >>> 16), 2246822507) ^ Math.imul(h2 ^ (h2 >>> 13), 3266489909);
	h2 = Math.imul(h2 ^ (h2 >>> 16), 2246822507) ^ Math.imul(h1 ^ (h1 >>> 13), 3266489909);

	return 4294967296 * (2097151 & h2) + (h1 >>> 0);
}

function startup()
{

	init_app_props();
	clock_sync_reset();
	init_bind_buttons();
	init_auto_update();
	reset_stat();

	app.intent = "offline";
	page("welcome");

	if (app.opt_autoconnect) {
		handle_button_connect();
	}

}

function init_app_props()
{

	app.websocket = null;
	app.heartbeat = null;

}

function init_options()
{

	init_const_options();

	var url = new URL(document.location.href);
	init_url_connection(url);
	init_url_control_station(url);

}

function init_const_options()
{

	app.opt_autoconnect = AUTOCONNECT;
	if (app.opt_autoconnect) {
		console.warn("TEST: autoconnect is on");
	}

	app.opt_skew = CLOCK_SKEW;
	if (app.opt_autoconnect) {
		console.warn("TEST: skew is", app.opt_skew, "ms");
	}

}

function init_url_connection(url)
{

	var protocol = url.protocol.replace("http", "ws");

	app.server_url = protocol + "//";
	app.server_url += url.hostname;
	app.server_url += ":" + WEBSOCKET_PORT;

}

function init_url_control_station(url)
{

	if (is_dev_machine()) {
	
		app.opt_control_station = true;
	
	} else {

		app.opt_control_station = url.searchParams.get("control_station");

		if (app.opt_control_station == null) {
			app.opt_control_station = false;
		}
		if (!app.opt_control_station) {
			app.opt_control_station = +app.opt_control_station;
		}
	
	}

	if (app.opt_control_station) {
		console.warn("TEST: control station mode");
	}

}

function is_dev_machine() {

	var v = navigator.appVersion;

	if (v.indexOf("Intel Mac OS X 10_15_7") == -1) return false;
	if (v.indexOf("Chrome/") == -1) return false;

	return true;
}

function init_url_option_autoconnect(url)
{

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

function init_bind_buttons()
{

	elm("connect").onclick = handle_button_connect;
	elm("abort").onclick = handle_button_abort_or_disconnect;
	elm("disconnect").onclick = handle_button_abort_or_disconnect;

	elm("cmd_red").onclick = handle_button_cmd;
	elm("cmd_green").onclick = handle_button_cmd;
	elm("cmd_blue").onclick = handle_button_cmd;
	elm("cmd_yellow").onclick = handle_button_cmd;
	elm("cmd_gray").onclick = handle_button_cmd;

}

function init_auto_update()
{

	if (!app.opt_autoconnect) return;

	rescheduler_auto_update();
}

function rescheduler_auto_update()
{

	setTimeout(function ()
	{
		check_update_retry(auto_update_proc);
	}, 200);

}


function auto_update_proc(response)
{

	if (app.update_hash == cyrb53(response)) {
		rescheduler_auto_update();
		return;
	}

	console.log("---- update ----");
	location.reload();

}

function reset_stat()
{

	app.stat_count = 0;
	app.stat_min_delay = 0;
	app.stat_max_delay = 0;

}

function elm(id)
{
	return document.getElementById(id);
}

function page(req)
{

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
		setTimeout(function ()
		{
			page("welcome");
		}, 1000);
	}

}

function intent(req)
{
	app.intent = req;
}

function display(content)
{
	elm("kontent").innerHTML = content;
}

function set_color(color)
{
	elm("control").style.backgroundColor = color;
}

function show(id)
{
	elm(id).style.display = "block";
}

function hide(id)
{
	elm(id).style.display = "none";
}

function create_websocket()
{

	app.websocket = new WebSocket(app.server_url);

	app.websocket.onopen = handle_socket_open;
	app.websocket.onmessage = handle_socket_message;
	app.websocket.onclose = handle_socket_close;

}

function discard_websocket()
{
	if (app.websocket != null) app.websocket.close();
	app.websocket = null;
}

function handle_socket_open(event)
{

	page("op");

	schedule_heartbeat(HEARTBEAT_TIMING_S[0]);
	clock_sync_reset();
	clock_sync_start();

}

function handle_socket_message(event)
{

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

	setTimeout(function ()
	{
		process_packet(packet);
	}, delay);

}

function process_packet(packet)
{
	if (packet.type == "CLK_REF") clock_sync_eval(packet.data[0]);
	if (packet.type == "DISPLAY") display(packet.data[0]);
	if (packet.type == "COLOR") set_color(packet.data[0]);
	//...
}

function handle_socket_close(event)
{

	stop_heartbeat();
	discard_websocket();

	if (app.intent == "offline") return;

	page("join");
	setTimeout(create_websocket, 400);

}

function handle_button_connect()
{

	create_websocket();
	intent("online");

}

function handle_button_abort_or_disconnect()
{

	discard_websocket();
	intent("offline");
	page("bye");

}

function handle_button_cmd()
{

	var color = this.id.split("_")[1];
	send("COLOR", [color]);

}

function send(signature, args)
{

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

function send_heartbeat()
{

	send("HEARTBEAT", [
		app.clock_skew,
		app.stat_count,
		app.stat_min_delay,
		app.stat_max_delay
	]);

	reset_stat();

}

function schedule_heartbeat(timeout_s)
{

	stop_heartbeat();
	app.heartbeat = setTimeout(send_heartbeat, timeout_s * 1000);

}

function stop_heartbeat()
{

	if (app.heartbeat == null) return;

	clearTimeout(app.heartbeat);
	app.heartbeat = null;

}

function get_raw_clock()
{
	var now = Date.now() + app.opt_skew;
	return now;
}

function get_clock(parm = undefined)
{

	if (typeof (parm) == "undefined") {
		var now = get_raw_clock();
	} else {
		var now = parm;
	}
	var corrected = now - app.clock_skew;

	return corrected;
}

function clock_sync_reset()
{
	app.clock_skew = 0;
	app.clock_sync_round = 0;
}

function clock_sync_start()
{
	app.clock_c0 = get_raw_clock();
	send("CLK_0", [app.clock_skew]);
}

function clock_sync_eval(clock_ref)
{

	clock_sync_calc_skew(clock_ref);
	clock_sync_reschedule();
}

function clock_sync_calc_skew(clock_ref)
{

	app.clock_c1 = get_raw_clock();

	var turnaround = app.clock_c1 - app.clock_c0;
	var distance = turnaround / 2;
	var estimation = app.clock_c0 + distance;
	var skew = Math.round(estimation - clock_ref);

	var change = Math.abs(app.clock_skew - skew);
	if (change > 30) app.clock_skew = skew;

}

function clock_sync_reschedule()
{

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
