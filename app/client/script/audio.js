function init_audio()
{
	app.frequency = 50;
	app.audio_context = new AudioContext();

	if (app.audio_context == null) {
		setTimeout(init_beep, 100);
	}
}

function beep()
{
	app.oscillator = app.audio_context.createOscillator();
	app.oscillator.type = "sine";
	app.oscillator.frequency.value = app.frequency;
	app.oscillator.connect(app.audio_context.destination);
	app.oscillator.start();

	setTimeout(function() {
		app.oscillator.stop();
	}, 100);
}

function set_beep_freq() 
{
	var base_hz = 220;
	var pitch = app.client_id * 3;
	app.frequency = base_hz * (2 ** (pitch / 12));
}
