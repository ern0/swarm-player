function init_heartbeat() 
{
	app.heartbeat = null;
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
