ws = new WebSocket("ws://127.0.0.1:8080/");
messages = document.createElement('ul');

ws.onmessage = function (event) {

	document.body.innerHTML = event.data;
};

document.body.appendChild(messages);

setInterval(function() {

	ws.send("client->server");

}, 2500)