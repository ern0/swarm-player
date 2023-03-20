function log(message) {
	console.log(message);
}

function elm(id)
{
	return document.getElementById(id);
}

function show(id)
{
	elm(id).style.display = "block";
}

function hide(id)
{
	elm(id).style.display = "none";
}
