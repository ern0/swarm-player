function init_log() 
{
	app.log_lock = false;
	app.log_queue = [];
}

function log(message) 
{
	console.log(message);
	if (app.log_lock) return;

	app.log_lock = true;

	var success = send("LOG", [message, 0]);	
	if (success) {
		flush_log();
	} else {
		app.log_queue.push(message);
	}

	app.log_lock = false;
}

function flush_log()
{
	while (app.log_queue.length > 0) {

		var message = app.log_queue[0];
		var success = send("LOG", [message, 1]);
		if (!success) break;
		
		app.log_queue.shift();
	}
}