REPORT_UNSET = 0;
REPORT_TO_SEND = 1;
REPORT_SENT = 2;

function init_report() 
{
	app.report_retry_timeout = null;
	app.report_control_station = REPORT_UNSET;
	app.report_audio_lag = REPORT_UNSET;
}

function reset_report() 
{
	if (app.report_control_station != REPORT_UNSET) {
		app.report_control_station = REPORT_TO_SEND;
	}

	if (app.report_audio_lag != REPORT_UNSET) {
		app.report_audio_lag = REPORT_TO_SEND;
	}

	if (app.report_retry_timeout != null) {
		clearTimeout(app.report_retry_timeout);
		app.report_retry_timeout = null;
	}

	report_all();
}

function report_all() 
{
	send_control_station();
	send_audio_lag();
	retry_report_if_needed();
}

function retry_report_if_needed()
{
	var retry = false;
	if (app.report_control_station == REPORT_TO_SEND) retry = true;
	if (app.report_audio_lag == REPORT_TO_SEND) retry = true;
	
	if (retry) retry_report();
}

function retry_report()
{
	if (app.report_retry_timeout != null) return;

	app.report_retry_timeout = setTimeout(function() {
		app.report_retry_timeout = null;
		report_all();
	}, 100);

}

function report_control_station() 
{
	app.report_control_station = REPORT_TO_SEND;
	send_control_station();
	retry_report_if_needed();
}

function send_control_station() 
{
	if (app.report_control_station != REPORT_TO_SEND) return;

	var success = true;
	if (app.opt_control_station) {
		success = send("CTRL", []);
	}

	if (success) {
		app.report_control_station = REPORT_SENT;		
	}

}

function report_audio_lag()
{
	app.report_audio_lag = REPORT_TO_SEND;
	send_audio_lag();
	retry_report_if_needed();
}

function send_audio_lag()
{
	if (app.report_audio_lag != REPORT_TO_SEND) return;

	var success = send("AUDIO", [app.audio_lag]);

	if (success) {
		app.report_audio_lag = REPORT_SENT;
	}
}
