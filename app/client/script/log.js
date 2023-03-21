function init_log() 
{
	app.log_lock = false;
	app.log_queue = [];
}

function log(message) 
{
	if (app.log_lock) {
		console.log("[can't send]", message);
		return;
	}

	app.log_lock = true;

	var stamp = get_clock();
	var success = send("LOG", [message, 0, stamp]);

	if (success) {
		console.log(message);
		flush_log();
	} else {
		console.log("[later]", message);
		app.log_queue.push([message, stamp]);
	}

	app.log_lock = false;
}

function flush_log()
{
	while (app.log_queue.length > 0) {

		var message = app.log_queue[0][0];
		var stamp = app.log_queue[0][1]
		var success = send("LOG", [message, 1, stamp]);
		if (!success) break;
		
		app.log_queue.shift();
	}
}
