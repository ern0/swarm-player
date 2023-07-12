function init_audio()
{
	app.frequency = 200 + (Math.random() * 900);
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
