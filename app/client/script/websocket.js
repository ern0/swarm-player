function init_websocket()
{
	app.websocket = null;
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
		app.stat_max_delay = Math.round(delay);
	}

	if (delay < 1) delay = 1;

	setTimeout(function() {
		process_packet(packet);
	}, delay);
}

function handle_socket_close(event)
{
	stop_heartbeat();
	discard_websocket();

	if (app.intent == "offline") return;

	page("join");
	setTimeout(create_websocket, 400);
}

function send(signature, args)
{
	if (!app.websocket) return;
	if (app.websocket.readyState != app.websocket.OPEN) return;

	if ((typeof args) != "object") {
		local_log("INTERNAL ERROR: invalid send format");
		return;
	}

	packet = { "type": signature, "data": args };
	data = JSON.stringify(packet);
	app.websocket.send(data);

	schedule_heartbeat(HEARTBEAT_TIMING_S[1]);
}
