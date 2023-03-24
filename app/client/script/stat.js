function reset_stat()
{
	app.stat_count = 0;
	app.stat_min_delay = -99999;
	app.stat_max_delay = -99999;
}

function feed_stat(delay)
{
	app.stat_count += 1;

	if (app.stat_min_delay == -99999) app.stat_min_delay = delay;
	if (app.stat_min_delay > delay) app.stat_min_delay = delay;

	if (app.stat_max_delay == -99999) app.stat_max_delay = delay;
	if (app.stat_max_delay < delay) app.stat_max_delay = delay;
}
