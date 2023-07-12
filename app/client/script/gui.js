function init_gui() {

	app.color_timeout = null;

	elm("connect").onclick = handle_button_connect;
	elm("abort").onclick = handle_button_abort_or_disconnect;
	elm("disconnect").onclick = handle_button_abort_or_disconnect;

	elm("cmd_red").onclick = handle_button_cmd;
	elm("cmd_green").onclick = handle_button_cmd;
	elm("cmd_blue").onclick = handle_button_cmd;
	elm("cmd_yellow").onclick = handle_button_cmd;
	elm("cmd_gray").onclick = handle_button_cmd;

	elm("local").onclick = beep;
}

function display(content)
{
	elm("kontent").innerHTML = content;
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
		display("...");
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
	var color = this.id.split("_")[1];
	send("COLOR", [color]);
}
