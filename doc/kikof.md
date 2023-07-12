SwarmPlayer Kikof
=================

SwarmPlayer is a wild demo. 
The audience is asked to 
join to our WiFi hotspot, 
then a web app automatically starts.
The server organizes the mobile devices
to a music player swarm, 
they will play songs and real-time MIDI
performed live on the stage.

Functional specification
------------------------

- A central machine emits MIDI (-like) data
- The clients (smartphones, notebooks with WiFi) are web apps,
  join to our own hotspot, with captive portal.
- The central machine plays a pre-written music,
  but MIDI keyboard and Wind Controller can be also used.
- The clients do not play all notes, the server
  decides which notes to play on which client.
- A dedicated client for voices, which mobile devices can't play, e.g. bass, drum.

Technical issues
----------------

- I made captive portal on ESP8266, but
  for 50-100 clients, we need a different solution.
- Time sync is needed between server and clients. Or maybe not, depends on the speed of the network, MIDI data is small.
- I think, websockets is the appropiate solution, needs implement server side.
- Needs a concept for displatching notes, which handles suddenly disappearing and appearing clients.
- Before the party, it needs to be tested,
at least with 20-25 clients.
- I want to write the server in Rust.
- Needs to plan the show script, e.g.
it would start with a unisono buzzer song,
then switch to multitimbral song, finally,
the audience should play on the MIDI keyboard,
while I play on the wind controller.

Features NOT to implement
-------------------------

- Asking the audience to show their phones to a camera, so stereo positions could be calibrated.
- The clients will be passive, play-only,
no input is accepted.

Investigation
-------------

First, I'll make a PoC,
the central machine will send something
to as many clients as I can collect,
just to know if time sync is required.
