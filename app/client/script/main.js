WEBSOCKET_PORT = 8080;
AUTOCONNECT = 1;
CLOCK_SKEW = 100;

MAGIC_AUDIO_LAG_FACTOR = 2.5;

HEARTBEAT_TIMING_S = [0.3, 10];
CLOCK_SYNC_TIMING_S = [0, 0.5, 2, 5, 30];

document.addEventListener("DOMContentLoaded", main);

function main()
{
	app = {};

	init_log();
	clock_sync_reset();
	init_awake();
	init_websocket();
	init_audio();
	init_options();
	init_heartbeat();
	init_gui();
	reset_stat();

	startup();
}

function startup()
{
	app.intent = "offline";
	page("welcome");

	if (app.opt_autoconnect) {
		handle_button_connect();
	}
}

function process_packet(packet)
{
	if (packet.stamp == 0) {
		process_packet_now(packet);
	} else {
		var delay = calc_packet_delay(packet);
		feed_stat(delay);
		process_packet_later(packet, delay);
	}
}

function calc_packet_delay(packet)
{
	var now = get_clock();
	var action = packet.stamp;
	var delay = action - now;
	if (delay < 1) delay = 1;

	return delay
}

function process_packet_later(packet, delay)
{
	if (packet.type == "COLOR") {
		beep(delay);
	}

	setTimeout(function() {
		process_packet_now(packet);
	}, delay);
}

function process_packet_now(packet)
{
	if (packet.type == "ID") {
		app.client_id = +packet.data[0];
		display("id", app.client_id);
	}

	if (packet.type == "CLK_REF") {
		clock_sync_eval(packet.data[0]);
	}

	if (packet.type == "RELOAD") {
		document.location.reload();
	}

	if (packet.type == "DISPLAY") {
		display(packet.data[0], packet.data[1]);
	}

	if (packet.type == "COLOR") {
		flash_color(packet.data[0]);
	}

}
