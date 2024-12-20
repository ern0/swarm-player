WEBSOCKET_PORT = 8080;
AUTOCONNECT = 1;
CLOCK_SKEW = 0;

MAGIC_AUDIO_LAG_FACTOR = 2.5;

ADMIN_CELL_MARGIN = 2;
ADMIN_CELL_BORDER = 2;
ADMIN_CELL_AURA = (2 * ADMIN_CELL_MARGIN) + (2 * ADMIN_CELL_BORDER);
ADMIN_CELL_CHANNELS = 4;

ADMIN_SKEW_WARNING = 10;
ADMIN_LAG_WARNING = 50;

HEARTBEAT_TIMING_S = [0.3, 10];
CLOCK_SYNC_TIMING_S = [0, 0.5, 2, 5, 30];

document.addEventListener("DOMContentLoaded", function() {
	main(false);
});

function main()
{
	app = {};

	init_log();
	clock_sync_reset();
	init_awake();
	init_websocket();
	init_report();
	init_audio();
	init_options();
	init_heartbeat();
	init_gui();
	reset_stat();
	init_admin();

	startup();``
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

	return delay;
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
		admin_add_self(packet);
	}

	if (packet.type == "CLK_REF") {
		clock_sync_eval(packet.data[0]);
	}

	if (packet.type == "REPORT") {
		admin_report(packet);
	}

	if (packet.type == "CONNECT") {
		admin_connect(packet);
	}

	if (packet.type == "DISCONNECT") {
		admin_disconnect(packet);
	}

	// unofficial commands

	if (packet.type == "COLOR") {
		flash_color(packet.data[0]);
	}

	if (packet.type == "RELOAD") {
		if (app.is_admin) {
			document.location = document.location.href + "/admin"
		} else {
			document.location.reload();
		}
	}


}
