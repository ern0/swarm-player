function init_audio()
{
	app.aucx = new AudioContext();

	if (app.aucx.state == "suspended") {
		setTimeout(init_audio, 100);
		return;
	} 

	init_audio_profiling();
	start_audio_profiling();
}

function init_audio_profiling() 
{
	app.audio_profiling_start_offset = 0;
	app.audio_profiling_stop_offset = 10;
	app.audio_profiling_rounds = 20;
	app.audio_profiling_instances = 8;

	app.audio_profiling_count = [];	
	app.audio_profiling_finished = 0;
	app.audio_lag = 0;
}

function start_audio_profiling()
{
	app.audio_profiling_epoch = Date.now();
	
	for (var i = 0; i < app.audio_profiling_instances; i++) {
		app.audio_profiling_count[i] = 0;
		run_audio_profiling_instance(i);
	}
}

function run_audio_profiling_instance(i)
{
	var gain = app.aucx.createGain();
	gain.connect(app.aucx.destination);
	gain.gain.value = 0;

	var osc = app.aucx.createOscillator();
	osc.type = "square";
	osc.frequency.value = 440;
	osc.connect(gain);

	var now = app.aucx.currentTime;
	var offset = app.audio_profiling_start_offset;
	if (offset == 0) {
		osc.start();
	} else {
		var start = now;
		start += app.audio_profiling_start_offset / 1000;
		osc.start(start);
	}
	var stop = now;
	stop += app.audio_profiling_stop_offset / 1000;
	osc.stop(stop);

	osc.onended = function() {
		var index = i;
		eval_audio_profiling_instance(index);
	}
}

function eval_audio_profiling_instance(i) 
{
	app.audio_profiling_count[i]++;
	if (app.audio_profiling_count[i] < app.audio_profiling_rounds) {
		run_audio_profiling_instance(i);
		return;
	}

	app.audio_profiling_finished += 1;
	if (app.audio_profiling_finished == app.audio_profiling_instances) {
		eval_audio_profiling_summary();
	}
}

function eval_audio_profiling_summary() 
{
	var elapsed = Date.now() - app.audio_profiling_epoch;
	var estimated = app.audio_profiling_stop_offset *	app.audio_profiling_rounds;
	app.audio_lag = Math.round((elapsed - estimated) / app.audio_profiling_rounds);
	
	if (app.audio_lag < 0) app.audio_lag = 0;
	if (app.audio_lag > 10) {
		app.audio_lag = Math.round(app.audio_lag * MAGIC_AUDIO_LAG_FACTOR);
	}
	
	display("au", app.audio_lag);
	report_audio_lag();

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
