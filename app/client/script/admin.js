
function init_admin() {

	if (!app.is_admin) return;

	show("admin");
	elm("client").style.width = "30%";
	document.body.style.fontSize = "3vmin";

	var elms = document.getElementsByTagName("button");
	Array.prototype.filter.call(elms, function(elm) {
		elm.style.fontSize = "3vmin";
	});

}

window.onresize = function() {


}
