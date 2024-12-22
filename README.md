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

### How to handle server authority

I see two paths for making the server authoritative.

- Server executes battle and sends event stream to client, which is purely a passive viewer.
- Server executes battle for actual result, but sends the initial conditions (including RNG seeds) to the client which executes the same code in real time instead of sped up.

The shared-initial-state approach requires less overall serialization and thus a smaller "battle"
payload, but is complicated by the fact that the game logic must behave identically on the client
and server.

The event-stream approach means the client can be a dumb viewer, but requires implementing an
event stream serde and tracking all the events w/ 100% accuracy.

Originally, I was definitely sold on the dumb viewer approach, but I'm actually leaning more and more
towards the initial state approach instead, just because then it becomes a bit more 
"normal game development".  Basically the server would run with a reduced plugin set that skips
anything visual while the client would enable all the visual plugins.  As mentioned above the main
trick is making sure that the game logic RNG seeds are shared and that all gameplay RNG is deterministic,
which is to say that the visual plugins shouldn't "taint" the game logic RNG.  I'd actually probably go
as far as to make the RNG semi-deterministic, e.g. the `Crit` component could use the entity id and 
frame number as the seed.

### If I wanted to make a actual game with this.

I'd probably:

- Do all the non-battle UI in a SPA or HTMX app.
- Run the battle in wasm as either a dumb viewer or initial condition loader.
- Hide and show the battle wasm for each battle.
- Maybe use parts of the same wasm app for rug and inventory management.

### Status:

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