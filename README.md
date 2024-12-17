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

Currently combat is basic, just weapons/burn w/ cooldown have been implemented:

```
ready to fight!
15.625ms: Hero: 250 Villain: 150
3.015625s: "Our Hero" attacked "Evil Henchman" with Hand Axe for 5!
3.015625s: "Evil Henchman" attacked "Our Hero" with Hand Axe for 5!
3.015625s: Hero: 245 Villain: 145
5.015625s: "Our Hero" attacked "Evil Henchman" with Burning Great Sword for 10!
5.015625s: "Our Hero" burned "Evil Henchman" with Burning Great Sword for 10!
5.015625s: Hero: 245 Villain: 135
6s: Burning "Evil Henchman" for 10
6s: Hero: 245 Villain: 125
6.015625s: "Our Hero" attacked "Evil Henchman" with Hand Axe for 5!
6.015625s: "Evil Henchman" attacked "Our Hero" with Hand Axe for 5!
6.015625s: Hero: 240 Villain: 120
7s: Burning "Evil Henchman" for 9
7s: Hero: 240 Villain: 111
8s: Burning "Evil Henchman" for 8
8s: Hero: 240 Villain: 103
9s: Burning "Evil Henchman" for 7
9s: Hero: 240 Villain: 96
9.015625s: "Our Hero" attacked "Evil Henchman" with Hand Axe for 5!
9.015625s: "Evil Henchman" attacked "Our Hero" with Hand Axe for 5!
9.015625s: Hero: 235 Villain: 91
10s: Burning "Evil Henchman" for 6
10s: Hero: 235 Villain: 85
10.015625s: "Our Hero" attacked "Evil Henchman" with Burning Great Sword for 10!
10.015625s: "Our Hero" burned "Evil Henchman" with Burning Great Sword for 10!
10.015625s: Hero: 235 Villain: 75
11s: Burning "Evil Henchman" for 15
11s: Hero: 235 Villain: 60
12s: Burning "Evil Henchman" for 14
12s: Hero: 235 Villain: 46
12.015625s: "Our Hero" attacked "Evil Henchman" with Hand Axe for 5!
12.015625s: "Evil Henchman" attacked "Our Hero" with Hand Axe for 5!
12.015625s: Hero: 230 Villain: 41
13s: Burning "Evil Henchman" for 13
13s: Hero: 230 Villain: 28
14s: Burning "Evil Henchman" for 12
14s: Hero: 230 Villain: 16
15s: Burning "Evil Henchman" for 11
15s: Hero: 230 Villain: 5
15.015625s: "Our Hero" attacked "Evil Henchman" with Hand Axe for 5!
15.015625s: "Evil Henchman" attacked "Our Hero" with Hand Axe for 5!
15.015625s: "Our Hero" attacked "Evil Henchman" with Burning Great Sword for 10!
15.015625s: "Our Hero" burned "Evil Henchman" with Burning Great Sword for 10!
We won in 15.015625s!  Simulated in 281.042µs
```

Status:

- The basics of systems for shielding, burn, poison, regen, heal, etc are all there just not fleshed out.  
- Haste and slow will require reworking the cooldown tics (currently tracked in 100ms "tick" timesteps).
- TODOs:
  - Finish main loop (goto `GameState::Results`)
  - Exit w/ results.
  - Timing off by a frame? (probably system ordering issues)
  - Sandstorm
  - Lots more events are needed for various "on XXX" effects.
  - Critical hits
  - Multi-cast (trigger UseEvent once for next N frames)
  - Freeze
  - Destroy