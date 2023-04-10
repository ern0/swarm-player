function init_admin() 
{
	if (!app.is_admin) return;

	app.admin_cells = {};

	init_admin_ui();
	init_admin_resize_handler();
	first_rethink_repaint();
}

function init_admin_ui()
{
	show("admin");
	$("client").style.width = "30%";
	document.body.style.fontSize = "3vmin";
	var elms = document.getElementsByTagName("button");
	Array.prototype.filter.call(elms, function(elm) {
		elm.style.fontSize = "3vmin";
	});

}

function init_admin_resize_handler()
{
	app.resize_timeout = null;

	window.onresize = function() {		

		if (app.resize_timeout != null) {
			clearTimeout(app.resize_timeout);
		}

		app.resize_timeout = setTimeout(function() {
			app.resize_timeout = null;
			admin_rethink();
			admin_repaint();		
		}, 500);

	};
}

function first_rethink_repaint() 
{
	if ($("admin").getBoundingClientRect().right > 0) {
		admin_rethink();
		admin_repaint();
	} else {
		setTimeout(first_rethink_repaint, 1);
	}
}

function admin_socket_open()
{ 
	if (!app.is_admin) return;

	send("MASTER", []);

	for (var cell in app.admin_cells) {
		admin_remove(cell);
	}

	// TODO: this is only temporary
	if (false) for (var i = 1000; i < 1080; i++) {
		app.admin_cells[i] = { "x": 0, "y": 0, "skew": "-", "lag": "-", "channels": 0 };
	}

}

function admin_socket_close()
{
	//TODO: gray out all cells or something
}

function mk_cell_id(id, token)
{
	return "admin_" + token + "_" + id;
}

function admin_add_self() 
{
	if (!app.is_admin) return;
	admin_add(app.client_id);
}

function admin_elm_self()
{
	var elm_id = mk_cell_id(app.client_id, "cell");
	return $(elm_id);
}

function admin_repaint()
{
	for (var id in app.admin_cells) {
		admin_repaint_cell(id);
	}
}

function admin_repaint_cell(id)
{
	var cell_data = app.admin_cells[id];

	var elm = $(mk_cell_id(id, "cell"));
	if (elm == null) {
		elm = admin_create_cell(id);
	}
	
	admin_repaint_cell_box(cell_data, elm);
	admin_repaint_cell_font(id);
	admin_repaint_cell_transition(elm);
	admin_repaint_cell_field_title(id);
	admin_repaint_cell_field_skew(id, cell_data);
	admin_repaint_cell_field_lag(id, cell_data);
	admin_repaint_cell_field_channels(id, cell_data);
}

function admin_repaint_cell_box(cell_data, elm)
{
	elm.style.width = (app.admin_cell_width - ADMIN_CELL_AURA) + "px";
	elm.style.height = (app.admin_cell_height - ADMIN_CELL_AURA) + "px";

	elm.style.left = cell_data["x"] + "px";
	elm.style.top = cell_data["y"] + "px";
}

function admin_repaint_cell_font(id) 
{
	var title_font_size = app.admin_cell_height * 0.2;
	var title_elm = $(mk_cell_id(id, "title"));
	title_elm.style.fontSize = title_font_size + "px";

	var area_font_size = app.admin_cell_height * 0.15;
	var area_elm = $(mk_cell_id(id, "area"));
	area_elm.style.fontSize = area_font_size + "px";
}

function admin_repaint_cell_transition(elm)
{
	var duration = 0.3 + Math.random() * 0.2;
	var opacity_duration = duration + 0.4;
	var delay = Math.random() * 0.1;
	elm.style.transition = (
		"all " + (duration) + "s ease-out " + (delay) + "s, " +
		"font-size " + (duration) + "s linear " + (delay) + "s, " +
		"opacity " + (opacity_duration) + "s ease-in");
}

function admin_repaint_cell_field_title(id)
{
	var title_elm = $(mk_cell_id(id, "title"));
	title_elm.innerHTML = id;
}

function admin_repaint_cell_field_skew(id, cell_data)
{
	var skew = cell_data["skew"];
	var skew_elm = $(mk_cell_id(id, "skew"));
	skew_elm.innerHTML = "S: " + skew;

	if (skew == "-") return;

	skew_elm.classList.remove("admin-line-undef");

	if (Math.abs(skew) < ADMIN_SKEW_WARNING) {
		skew_elm.classList.remove("admin-line-skew-warning");
	} else {
		skew_elm.classList.add("admin-line-skew-warning");
	}
}

function admin_repaint_cell_field_lag(id, cell_data)
{
	var lag = cell_data["lag"];
	var lag_elm = $(mk_cell_id(id, "lag"));
	lag_elm.innerHTML = "L: " + lag;

	if (lag == "-") return;

	lag_elm.classList.remove("admin-line-undef");

	if (lag < ADMIN_LAG_WARNING) {
		lag_elm.classList.remove("admin-line-lag-warning");
	} else {
		lag_elm.classList.add("admin-line-lag-warning");
	}	
}

function admin_repaint_cell_field_channels(id, cell_data)
{
	//TODO
}

function admin_report(packet)
{
	var id = packet.data[0];

	console.log("report", id, packet);

	// server data reporting is slower than client list reporting
	if (!(id in app.admin_cells)) return;

	app.admin_cells[id]["skew"] = packet.data[1];
	app.admin_cells[id]["lag"] = packet.data[2];
	app.admin_cells[id]["channels"] = packet.data[3];

	admin_repaint_cell(id);
}

function admin_create_cell(id)
{
	var elm = document.createElement("div");
	elm.id = mk_cell_id(id, "cell");
	elm.classList.add("admin-cell");
	$("admin").appendChild(elm);

	admin_create_cell_elm(elm, id, "title");
	var area_elm = admin_create_cell_elm(elm, id, "area");
	var skew_elm = admin_create_cell_elm(area_elm, id, "skew");
	skew_elm.classList.add("admin-line-undef");
	var lag_elm = admin_create_cell_elm(area_elm, id, "lag");
	lag_elm.classList.add("admin-line-undef");

	return elm;
}

function admin_create_cell_elm(elm, id, item_name)
{
	var item_elm = document.createElement("div");
	item_elm.className = "admin-" + item_name;
	item_elm.id = mk_cell_id(id, item_name);
	elm.appendChild(item_elm);

	return item_elm;
}

function admin_rethink() 
{
	admin_rethink_get_base_vars();
	app.admin_best_cell_side = admin_rethink_find_best_cell_side();
	app.admin_cell_height = app.admin_best_cell_side;

	var cell_extra_width = admin_rethink_adjust_width();
	app.admin_cell_width = app.admin_best_cell_side + cell_extra_width;

	admin_rethink_recalc_coords();
}

function admin_rethink_get_base_vars()
{
	var rect = $("admin").getBoundingClientRect();
	app.admin_area_width = Math.floor(rect.right - rect.left);
	app.admin_area_height = Math.floor(rect.bottom - rect.top);
	app.admin_cell_count = Object.keys(app.admin_cells).length;
}

function admin_rethink_find_best_cell_side()
{
	var cell_side = app.admin_area_width;
	var jump = (cell_side + 5) / 2;
	var best_slack = -1;
	var best_side = -1;

	while (true) {		

		cell_side = Math.round(cell_side);
		var slack = admin_rethink_calc_slack(cell_side);

		var is_slack_valid = (slack >= 0);
		var is_slack_best = (slack < best_slack) || (best_slack < 0);
		if (is_slack_valid && is_slack_best) {
			best_slack = slack;
			best_side = cell_side;
		}

		if (jump < 1) break;

		if (slack > 0) {
			cell_side = cell_side + jump;
		} else {
			cell_side = cell_side - jump;
		}

		jump = jump / 2;
	}

	return best_side;
}

function admin_rethink_calc_slack(cell_side)
{
	if (cell_side >= (app.admin_area_width / 2)) return -1;
	var column_count = Math.floor(app.admin_area_width / cell_side);
	if (column_count < 1) return -1;

	var required_row_count = Math.ceil(app.admin_cell_count / column_count);
	var slack = app.admin_area_height - (cell_side * required_row_count);
	return slack;
}

function admin_rethink_adjust_width()
{
	if (app.admin_cell_count < 2) return 0;

	var column_count = Math.floor(app.admin_area_width / app.admin_best_cell_side) ;
	var cell_extra_width = admin_rethink_calc_extra_width(column_count);

	var orphan_cell_count = app.admin_cell_count % column_count;
	var empty_place_count = column_count - orphan_cell_count - 1;
	var filled_row_count = Math.floor(app.admin_cell_count / column_count);	
	var fill_empty_places = (empty_place_count >= filled_row_count);
	if (orphan_cell_count == 0) fill_empty_places = false;
	if (fill_empty_places) {
		cell_extra_width = admin_rethink_calc_extra_width(column_count - 1);
	}

	return cell_extra_width;
}

function admin_rethink_calc_extra_width(column_count)
{
	var cells_width = app.admin_best_cell_side * column_count;
	var horiz_slack = app.admin_area_width - cells_width;
	var cell_extra_width = Math.floor(horiz_slack / column_count);

	return cell_extra_width;
}

function admin_rethink_recalc_coords()
{
	var x = 0;
	var y = 0;
	var next_x = app.admin_cell_width;

	for (var id in app.admin_cells) {

		app.admin_cells[id]["x"] = x;
		app.admin_cells[id]["y"] = y;

		x = next_x;
		next_x += app.admin_cell_width;

		if (next_x > app.admin_area_width) {
			x = 0;
			next_x = app.admin_cell_width;
			y += app.admin_cell_height;
		}

	}
}

function mk_cell_dim()
{
	return app.adm_cell_width + "x" + app.adm_cell_height;
}

function admin_add(id)
{
	admin_rethink();

	var x = (app.admin_area_width - app.admin_cell_width) / 2;
	var y = (app.admin_area_height - app.admin_cell_height) / 2;
	app.admin_cells[id] = { "x": x, "y": y, "skew": "-", "lag": "-", "channels": 0 };
	admin_repaint();

	var cell = $(mk_cell_id(id, "cell"));
	cell.style.width = app.admin_cell_width / 4;
	cell.style.height = app.admin_cell_height / 4;

	admin_rethink();
	admin_repaint();
}

function admin_remove(id)
{
	delete app.admin_cells[id];

	var cell = $(mk_cell_id(id, "cell"));
	if (cell == null) return;
	cell.classList.add("admin-uncell");
	var title = $(mk_cell_id(id, "title"));
	title.classList.add("admin-untitle");

	cell.style.opacity = 0;
	cell.style.transition = "opacity 1s ease-out 0.4s";

	setTimeout(function() {
		cell.remove();
		admin_rethink();
		admin_repaint();
	}, 1300);
}
