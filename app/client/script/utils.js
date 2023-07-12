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

function sgnfmt(value, role)
{
	if (role == "sign") {
		return (value > 0 ? "+" : "") + value;
	}

	var result = " ";
	result += (value < 0 ? "-" : "+");
	result += " ";
	result += Math.abs(value);
	result += " ";

	return result;
}
