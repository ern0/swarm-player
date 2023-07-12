function init_audio()
{
	app.aucx = new AudioContext();

	if (app.aucx.state == "suspended") {
		setTimeout(init_audio, 100);
		return;
	} 

	log("audio enabled");	
	set_beep_freq();
	
	init_audio_profiling();
	run_audio_profiling();
}

function init_audio_profiling() 
{
	app.audio_lag = 0;
	app.audio_profiling_count = 0;
	app.audio_profiling_start_offset = 5;
	app.audio_profiling_stop_offset = 8;
	app.audio_profiling_rounds = 50;
}

function run_audio_profiling(round)
{
	if (app.audio_profiling_count == 0) {
		app.audio_profiling_epoch = Date.now();
	}

	var gain = app.aucx.createGain();
	gain.connect(app.aucx.destination);
	gain.gain.value = 0.5;

	var osc = app.aucx.createOscillator();
	osc.type = "sine";
	osc.frequency.value = 440;
	osc.connect(gain);

	var now = app.aucx.currentTime;
	var start = now;
	start += app.audio_profiling_start_offset / 1000;
	osc.start(start);
	var stop = now;
	stop += app.audio_profiling_stop_offset / 1000;
	osc.stop(stop);

	osc.onended = eval_audio_profiling;
}

function eval_audio_profiling() 
{
	app.audio_profiling_count++;
	if (app.audio_profiling_count < app.audio_profiling_rounds) {
		run_audio_profiling();
		return;
	}

	var elapsed = Date.now() - app.audio_profiling_epoch;
	var estimated = app.audio_profiling_stop_offset *	app.audio_profiling_rounds;
	app.audio_lag = Math.round((elapsed - estimated) / app.audio_profiling_rounds);
	if (app.audio_lag < 20) app.audio_lag = 0;

	app.audio_lag = app.audio_lag * MAGIC_AUDIO_LAG_FACTOR;
	
	display("au", app.audio_lag);
	log("audio lag is " + app.audio_lag + " ms");
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

function beep(delay_ms)
{
	var f1 = Math.round(app.frequency * 0.99)|0;
	var f2 = Math.round(app.frequency * 1.01)|0;

	var delay_corrected = delay_ms - app.audio_lag;
	if (delay_corrected < 0) delay_corrected = 0;
	var start = app.aucx.currentTime + (delay_corrected / 1000);
	var stop = start + 0.2;

	app.osc1 = app.aucx.createOscillator();
	app.osc1.type = "sine";
	app.osc1.frequency.value = f1;
	app.osc1.connect(app.aucx.destination);

	app.osc1.start(start);
	app.osc1.stop(stop);

	app.osc2 = app.aucx.createOscillator();
	app.osc2.type = "sine";
	app.osc2.frequency.value = f2;
	app.osc2.connect(app.aucx.destination);

	app.osc2.start(start);
	app.osc2.stop(stop);
}
