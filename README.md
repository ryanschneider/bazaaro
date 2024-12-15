Bazaaro - Simple "The Bazaar" Clone
===================================

This is my attempt at emulating combat from The Bazaar.

The intent is that this remains a fully "headless" app which would be used in
an actual "Bazaar-alike" _on the server side_ by:

- Loading the two opponents in `loading` (_currently hardcoded_)
- Tracing all the combat events in `fighting` (_tracing not implemented_)
- Running the combat fast-forwarded (currently 100000x)
- Stream the combat events to the client, which runs in viewer mode (_not implemented_)

In short, when the user clicks on an opponent, a fight is started on the server, and
the results are streamed to the client in JSON/protobuf/bevy-remote-proto(?)/whatever,
since the server runs way faster than realtime the combat should be "done" in milliseconds.

Currently combat is basic, just weapons w/ cooldown have been implemented:

```
ready to fight!
3.015625s: Our Hero attacked Evil Henchman with Hand Axe for 5!
3.015625s: Evil Henchman attacked Our Hero with Hand Axe for 5!
5.015625s: Our Hero attacked Evil Henchman with Great Sword for 30!
6.015625s: Our Hero attacked Evil Henchman with Hand Axe for 5!
6.015625s: Evil Henchman attacked Our Hero with Hand Axe for 5!
9.015625s: Our Hero attacked Evil Henchman with Hand Axe for 5!
9.015625s: Evil Henchman attacked Our Hero with Hand Axe for 5!
10.015625s: Our Hero attacked Evil Henchman with Great Sword for 30!
12.015625s: Our Hero attacked Evil Henchman with Hand Axe for 5!
12.015625s: Evil Henchman attacked Our Hero with Hand Axe for 5!
15.015625s: Our Hero attacked Evil Henchman with Hand Axe for 5!
15.015625s: Our Hero attacked Evil Henchman with Great Sword for 30!
15.015625s: Evil Henchman attacked Our Hero with Hand Axe for 5!
18.015625s: Our Hero attacked Evil Henchman with Hand Axe for 5!
18.015625s: Evil Henchman attacked Our Hero with Hand Axe for 5!
20.015625s: Our Hero attacked Evil Henchman with Great Sword for 30!
We won in 20.015625s!  Simulated in 246.042µs
```

Status:

- The basics of systems for shielding, burn, poison, regen, heal, etc are all there just not fleshed out.  
- Haste and slow will require reworking the cooldown tics (currently tracked in 100ms "tick" timesteps).
- TODOs:
  - Finish main loop (goto `GameState::Results`)
  - Figure out why `::Results` transition takes so long.
  - Exit w/ results.
  - Timing off by a frame? (probably system ordering issues)
  - Sandstorm
  - Lots more events are needed for various "on XXX" effects.
  - Critical hits
  - Multi-cast (trigger UseEvent once for next N frames)
  - Freeze
  - Destroy