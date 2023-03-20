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
