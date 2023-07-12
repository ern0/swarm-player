
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
	admin_rethink();
	admin_repaint();
}

function admin_repaint()
{
	var rect = $("admin").getBoundingClientRect();

	app.adm_area_width = rect.right - rect.left;
	app.adm_area_height = rect.bottom - rect.top;

}

function admin_repaint_cell(id)
{
	var elm_id = mk_elm_id(id);
	var elm = $(elm_id);
	if (elm == null) elm = admin_create_cell(elm_id);

	elm.innerHTML = id;
	elm.style.width = app.adm_cell_width + "px";
	elm.style.height = app.adm_cell_height + "px";
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
	//TODO: implement rethink()
	app.adm_cell_width = 50;
	app.adm_cell_height = 50;

}

function mk_cell_dim()
{
	return app.adm_cell_width + "x" + app.adm_cell_height;
}

function admin_add(id)
{
	var before = mk_cell_dim();
	app.admin_cells[id] = null;
	admin_rethink();
	var after = mk_cell_dim();
	if (before != after) admin_repaint();

	admin_repaint_cell(id);
}

function admin_remove(id)
{
	var before = mk_cell_dim();
	delete app.admin_cells[id];
	admin_rethink();
	var after = mk_cell_dim();
	if (before != after) admin_repaint();

	//TODO: maybe requires repaint() some other cases, too

	$(mk_elm_id(id)).remove();
}
