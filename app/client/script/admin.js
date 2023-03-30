
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
	for (var cell in app.admin_cells) {
		admin_remove(cell);
	}

	for (var i = 0; i < 15; i++) admin_add(i); //TODO: remove
}

function admin_socket_close()
{
	//TODO: gray out all cells or something
}

function mk_cell_id(id)
{
	return "cell_" + id;
}

function admin_add_self() 
{
	admin_add(app.client_id);
}

function admin_elm_self()
{
	var elm_id = mk_cell_id(app.client_id);
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
	var elm_id = mk_cell_id(id);
	var elm = $(elm_id);
	if (elm == null) elm = admin_create_cell(elm_id);

	elm.innerHTML = id;
	elm.style.width = (app.admin_cell_width - ADMIN_CELL_AURA) + "px";
	elm.style.height = (app.admin_cell_height - ADMIN_CELL_AURA) + "px";

	var cell_pos = app.admin_cells[id];
	elm.style.left = cell_pos[0] + "px";
	elm.style.top = cell_pos[1] + "px";

	var t = Math.random();
	var d = Math.random();
	elm.style.transition = "all " + t + "s linear " + d + "s";
}

function admin_create_cell(elm_id)
{
	var elm = document.createElement("div");
	elm.id = elm_id;
	elm.classList.add("cell");
	$("admin").appendChild(elm);

	return elm;
}

function admin_rethink() 
{
	admin_rethink_get_base_vars();
	admin_rethink_find_best_side();
	app.admin_cell_height = app.admin_best_cell_side;

	var cell_extra_width = admin_rethink_adjust_width();
	app.admin_cell_width = app.admin_best_cell_side + cell_extra_width;

	admin_rethink_renumber();
}

function admin_rethink_get_base_vars()
{
	var rect = $("admin").getBoundingClientRect();
	app.admin_area_width = Math.floor(rect.right - rect.left);
	app.admin_area_height = Math.floor(rect.bottom - rect.top);
	app.admin_cell_count = Object.keys(app.admin_cells).length;
}

function admin_rethink_find_best_side()
{
	var cell_side = app.admin_area_width;
	var jump = (cell_side + 5) / 2;
	var best_slack = -1;
	app.admin_best_cell_side = -1;

	while (true) {		

		cell_side = Math.round(cell_side);
		var slack = admin_rethink_calc_slack(cell_side);

		var is_slack_valid = (slack >= 0);
		var is_slack_best = (slack < best_slack) || (best_slack < 0);
		if (is_slack_valid && is_slack_best) {
			best_slack = slack;
			app.admin_best_cell_side = cell_side;
		}

		if (jump < 1) break;

		if (slack > 0) {
			cell_side = cell_side + jump;
		} else {
			cell_side = cell_side - jump;
		}

		jump = jump / 2;
	}
}

function admin_rethink_calc_slack(cell_side)
{
	var column_count = Math.floor(app.admin_area_width / cell_side);
	if (column_count < 1) return -1;

	var required_row_count = Math.ceil(app.admin_cell_count / column_count);
	var slack = app.admin_area_height - (cell_side * required_row_count);
	return slack;
}

function admin_rethink_adjust_width()
{
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

function admin_rethink_renumber()
{
	var column_count = Math.floor(app.admin_area_width / app.admin_best_cell_side);

	var x = 0;
	var y = 0;
	for (var id in app.admin_cells) {

		app.admin_cells[id] = [
			x * app.admin_cell_width, 
			y * app.admin_cell_height]; 

		x++;
		if (x < column_count) continue;
		x = 0;
		y++;
	}
}

function mk_cell_dim()
{
	return app.adm_cell_width + "x" + app.adm_cell_height;
}

function admin_add(id)
{
	app.admin_cells[id] = null;
	admin_rethink();
	admin_repaint();
}

function admin_remove(id)
{
	delete app.admin_cells[id];
	$(mk_cell_id(id)).remove();
	admin_rethink();
	admin_repaint();
}
