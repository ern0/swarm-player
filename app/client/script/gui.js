function init_gui() {
	
	app.color_timeout = null;
	init_buttons();
	init_https_link();

}

function init_buttons()
{
	$("connect").onclick = handle_button_connect;
	$("abort").onclick = handle_button_abort_or_disconnect;
	$("disconnect").onclick = handle_button_abort_or_disconnect;

	$("color_red").onclick = handle_button_cmd;
	$("color_orange").onclick = handle_button_cmd;
	$("color_green").onclick = handle_button_cmd;
	$("color_blue").onclick = handle_button_cmd;

	$("local").onclick = function() { beep(0); };
	$("reload").onclick = handle_button_cmd;
}

function init_https_link() 
{
	var url = new URL(document.location.href);

	if (url.protocol.startsWith("https")) {
		hide("https_div");
		return;
	} 

	var link = "https://" + url.hostname + url.pathname;
	$("https_link").setAttribute("href", link);
}

function display_defaults() 
{
	display("id", "...");
}

function display(key, content)
{
	$(key).innerHTML = content;
}

function page(req)
{
	const pages = ["welcome", "op", "join", "bye"];

	for (var index in pages) {

		var pg = pages[index];

		if (pg == req) {
			show(pg);
		} else {
			hide(pg);
		}

	} // for

	if (req == "op") {
		display_defaults();
	}

	if (req == "bye") {
		setTimeout(function() {
			page("welcome");
		}, 1000);
	}

}

function intent(req)
{
	app.intent = req;
}

function flash_color(color)
{
	document.body.style.background = color;

	if (app.color_timeout != null) {
		clearTimeout(app.color_timeout);
	}

	app.color_timeout = setTimeout(function () {
		document.body.style.background = "#ffffff";
		app.color_timeout = null;
	}, 200);

}

function handle_button_connect()
{
	create_websocket();
	intent("online");
}

function handle_button_abort_or_disconnect()
{
	discard_websocket();
	intent("offline");
	page("bye");
}

function handle_button_cmd()
{
	var a = this.id.split("_");
	var action = a[0];
	var color = a[1];

	if (action == "reload") {
		send("RELOAD", []);
	}
	if (action == "color") {
		send("COLOR", [color]);
	}
}
