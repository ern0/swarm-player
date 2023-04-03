function init_options()
{
	init_const_options();

	var url = new URL(document.location.href);
	init_url_connection(url);
	init_url_admin(url);
}

function init_const_options()
{
	app.opt_skew = CLOCK_SKEW;
	if (app.opt_autoconnect) {
		log("skew value is " + app.opt_skew + " ms");
	}

	app.opt_autoconnect = AUTOCONNECT;
	if (app.opt_autoconnect) {
		log("autoconnect is on");
	}
}

function init_url_connection(url)
{
	var protocol = url.protocol.replace("http", "ws");
	app.server_url = protocol + "//";
	app.server_url += url.hostname;
	if (!protocol.startsWith("wss")) {
		app.server_url += ":" + WEBSOCKET_PORT;
	}
	app.server_url += "/api";
}

function init_url_admin(url)
{
	app.is_admin = false;

	//if (!is_dev_machine()) return false;
	if (!url.protocol.startsWith("https")) return false;
	if (!document.referrer.endsWith("/admin")) return false;
	
	app.is_admin = true;
	report_is_admin();
}

function is_dev_machine()
{
	var v = navigator.appVersion;  //TODO: it's deprecated

	if (v == "5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36") return true;
	if (v.indexOf("Intel Mac OS X 10_15_7") == -1) return false;
	if (v.indexOf("Chrome/") == -1) return false;

	return true;
}
