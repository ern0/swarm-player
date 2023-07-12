function init_audio()
{
	app.aucx = new AudioContext();

	if (app.aucx == null) {
		setTimeout(init_audio, 100);
		return;
	} 

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

}

function beep(delay_ms = 0)
{
	t0 = app.aucx.currentTime + (delay_ms / 1000);
	t1 = t0 + 0.2;
	f1 = Math.round(app.frequency * 0.99)|0;
	f2 = Math.round(app.frequency * 1.01)|0;

	app.osc1 = app.aucx.createOscillator();
	app.osc1.type = "sine";
	app.osc1.frequency.value = f1;
	app.osc1.connect(app.aucx.destination);
	app.osc1.start(t0);
	app.osc1.stop(t1);

	app.osc2 = app.aucx.createOscillator();
	app.osc2.type = "sine";
	app.osc2.frequency.value = f2;
	app.osc2.connect(app.aucx.destination);
	app.osc2.start(t0);
	app.osc2.stop(t1);

}

