
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

function mk_elm_id(id)
{
	return "cell_" + id;
}

function admin_add_self() 
{
	admin_add(app.client_id);
}

function admin_elm_self()
{
	var elm_id = mk_elm_id(app.client_id);
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
	var aura = 8;  // 2x border=2 + 2x margin=2
	var elm_id = mk_elm_id(id);
	var elm = $(elm_id);
	if (elm == null) elm = admin_create_cell(elm_id);

	elm.innerHTML = id;
	elm.style.width = (app.admin_cell_width - aura) + "px";
	elm.style.height = (app.admin_cell_height - aura) + "px";
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
	var rect = $("admin").getBoundingClientRect();
	app.admin_area_width = Math.floor(rect.right - rect.left);
	app.admin_area_height = Math.floor(rect.bottom - rect.top);
	app.admin_cell_count = Object.keys(app.admin_cells).length;

	var cell_side = app.admin_area_width;
	var jump = (cell_side + 5) / 2;
	var best_slack = -1;
	var best_cell_side = -1;

	while (true) {		

		cell_side = Math.round(cell_side);
		var slack = admin_rethink_calc_slack(cell_side);

		var is_slack_valid = (slack >= 0);
		var is_slack_best = (slack < best_slack) || (best_slack < 0);
		if (is_slack_valid && is_slack_best) {
			best_slack = slack;
			best_cell_side = cell_side;
		}

		if (jump < 1) break;

		if (slack > 0) {
			cell_side = cell_side + jump;
		} else {
			cell_side = cell_side - jump;
		}

		jump = jump / 2;

	}

	var cell_count_in_row = Math.floor(app.admin_area_width / best_cell_side);
	var cells_width = best_cell_side * cell_count_in_row;
	var horiz_slack = app.admin_area_width - cells_width;
	var cell_extra_width = Math.floor(horiz_slack / cell_count_in_row);

	//TODO: orphans vs rows

	app.admin_cell_width = best_cell_side + cell_extra_width;
	app.admin_cell_height = best_cell_side;
}

function admin_rethink_calc_slack(cell_side)
{
	var cell_count_in_row = Math.floor(app.admin_area_width / cell_side);
	if (cell_count_in_row < 1) return -1;

	var required_row_count = Math.ceil(app.admin_cell_count / cell_count_in_row);
	var slack = app.admin_area_height - (cell_side * required_row_count);

	return slack;
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
	$(mk_elm_id(id)).remove();
	admin_rethink();
	admin_repaint();
}
