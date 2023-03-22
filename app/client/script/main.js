WEBSOCKET_PORT = 8080;
AUTOCONNECT = 1
CLOCK_SKEW = 300

HEARTBEAT_TIMING_S = [0.3, 10];
CLOCK_SYNC_TIMING_S = [0, 0.5, 2, 5, 30];

document.addEventListener("DOMContentLoaded", main);

function main()
{
	app = {};

	clock_sync_reset();
	init_log();
	init_awake();
	init_options();
	init_audio();
	init_websocket();
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


function reset_stat()
{
	app.client_id = "";
	app.stat_count = 0;
	app.stat_min_delay = 0;
	app.stat_max_delay = 0;
}

function process_packet(packet)
{
	if (packet.type == "ID") {
		app.client_id = +packet.data[0];
		display("id", app.client_id);
		set_beep_freq();
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
		beep();
	}


	//...
}
