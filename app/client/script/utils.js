function $(id)
{
	return document.getElementById(id);
}

function show(id)
{
	$(id).style.display = "block";
}

function hide(id)
{
	$(id).style.display = "none";
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

function prop(elm, prop_name)
{
	var elm = $(elm);
	var value = getComputedStyle(elm).getPropertyValue(prop_name);
	return +value.replace("px","");
}
