function init_websocket()
{
	app.websocket = null;
	app.client_id = -1;
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

	flush_log();
	reset_stat();
	schedule_heartbeat(HEARTBEAT_TIMING_S[0]);
	clock_sync_reset();
	clock_sync_start();
}

function handle_socket_message(event)
{
	var packet = JSON.parse(event.data);
	process_packet(packet);
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
	if (!app.websocket) return false;
	if (app.websocket.readyState != app.websocket.OPEN) return false;

	if ((typeof args) != "object") {
		log("INTERNAL ERROR: invalid send format");
		return false;
	}

	packet = { "type": signature, "data": args };
	data = JSON.stringify(packet);
	app.websocket.send(data);

	schedule_heartbeat(HEARTBEAT_TIMING_S[1]);

	return true;
}
