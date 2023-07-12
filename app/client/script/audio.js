function init_audio()
{
	app.aucx = new AudioContext();

	if (app.aucx.state == "suspended") {
		setTimeout(init_audio, 100);
		return;
	} 

	log("Audio enabled");
	set_beep_freq();
}

function set_beep_freq() 
{
	if (app.client_id == -1) {
		setTimeout(set_beep_freq, 100);
		return;
	}

	var base_hz = 220;
	var pitch = (app.client_id % 12) * 3;
	app.frequency = base_hz * (2 ** (pitch / 12));

	f1 = Math.round(app.frequency * 0.99)|0;
	f2 = Math.round(app.frequency * 1.01)|0;

	app.gain1 = app.aucx.createGain();
	app.gain1.gain.value = 0;
	app.gain1.connect(app.aucx.destination);

	app.osc1 = app.aucx.createOscillator();
	app.osc1.type = "sine";
	app.osc1.frequency.value = f1;
	app.osc1.connect(app.gain1);
	app.osc1.start();

	app.gain2 = app.aucx.createGain();
	app.gain2.gain.value = 0;
	app.gain2.connect(app.aucx.destination);

	app.osc2 = app.aucx.createOscillator();
	app.osc2.type = "sine";
	app.osc2.frequency.value = f2;
	app.osc2.connect(app.gain2);
	app.osc2.start();

}

function beep(delay_ms)
{
	var start = app.aucx.currentTime + (delay_ms / 1000);
	var stop = start + 0.2;

	app.gain1.gain.setValueAtTime(1, start);
	app.gain1.gain.setValueAtTime(0, stop);

	app.gain2.gain.setValueAtTime(1, start);
	app.gain2.gain.setValueAtTime(0, stop);
}

