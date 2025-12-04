use crate::characters::*;
use crate::rng::{ClientRng, RngProvider, ServerRng, TestRng};
use crate::GameState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Range;
use std::time::Duration;

/// Bazaaro-specific RNG key that uses stable identifiers for deterministic RNG
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RngKey {
    /// Game tick when the random decision is made
    pub tick: u64,
    /// Which character (Hero or Villain)
    pub character: BazaaroCharacter,
    /// Index of the item in the character's item list
    pub index: usize,
}

/// Character identifier for stable RNG keys
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum BazaaroCharacter {
    Hero,
    Villain,
}

/// Newtype wrapper for RNG providers specific to Bazaaro
/// This eliminates the need for generic type parameters throughout the codebase
#[derive(Resource)]
pub struct BazaaroRng(Box<dyn RngProvider<RngKey> + Send + Sync>);

impl BazaaroRng {
    /// Create a server-side RNG with the given seed
    pub fn server(seed: u64) -> Self {
        Self(Box::new(ServerRng::<RngKey>::new(seed)))
    }

    /// Create a client-side RNG from the server's log
    pub fn client(log: HashMap<RngKey, u32>) -> Self {
        Self(Box::new(ClientRng::from_log(log)))
    }

    /// Create a test RNG with predetermined values
    pub fn test(values: HashMap<RngKey, u32>) -> Self {
        Self(Box::new(TestRng::with_values(values)))
    }
}

impl RngProvider<RngKey> for BazaaroRng {
    fn result(&mut self, key: RngKey) -> u32 {
        self.0.result(key)
    }

    fn result_range(&mut self, key: RngKey, range: Range<u32>) -> u32 {
        self.0.result_range(key, range)
    }

    fn result_bool(&mut self, key: RngKey, probability: f32) -> bool {
        self.0.result_bool(key, probability)
    }
}

pub struct FightingPlugin;
impl Plugin for FightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FightingTickers::new());
        app.add_systems(
            OnEnter(GameState::Fight),
            (setup_fight,).in_set(SystemSets::OnEnter),
        );
        app.add_systems(
            FixedPreUpdate,
            (tick,)
                .in_set(SystemSets::Ticking)
                .run_if(in_state(GameState::Fight))
                .after(SystemSets::OnEnter),
        );
        app.add_systems(
            FixedUpdate,
            check_winner
                .run_if(in_state(GameState::Fight))
                .after(SystemSets::Ticking),
        );
        // lets make sure we detect and transition away from the ::Fight
        // state immediately
        app.add_systems(
            FixedUpdate,
            battle_over
                .run_if(in_state(GameState::Fight))
                .after(check_winner),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum SystemSets {
    OnEnter,
    Ticking,
}

#[derive(Resource)]
pub struct Battle {
    #[allow(dead_code)]
    pub start: f64,
    pub elapsed: Duration,
    pub over: bool,
    pub hero: Entity,
    pub villain: Entity,
    pub tick: u64,
}

impl Battle {
    pub fn opponent(&self, val: Entity) -> Entity {
        if val == self.hero {
            self.villain
        } else if val == self.villain {
            self.hero
        } else {
            panic!("invalid opponent: {:?}", val)
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn setup_fight(
    mut commands: Commands,
    mut time: ResMut<Time<Virtual>>,
    mut q_hero: Query<(Entity, &mut Health), (With<Hero>, Without<Villain>)>,
    mut q_villain: Query<(Entity, &mut Health), (With<Villain>, Without<Hero>)>,
) -> Result {
    time.set_relative_speed(100000.0);

    let (hero, mut hero_health) = q_hero.single_mut()?;
    hero_health.reset();

    let (villain, mut villain_health) = q_villain.single_mut()?;
    villain_health.reset();

    commands.insert_resource(Battle {
        start: time.elapsed_secs_f64(),
        elapsed: Duration::default(),
        over: false,
        hero,
        villain,
        tick: 0,
    });
    info!("ready to fight!");
    Ok(())
}

#[derive(Resource)]
pub struct FightingTickers {
    pub per_tick: Timer,
}

impl FightingTickers {
    pub fn new() -> Self {
        Self {
            per_tick: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

#[derive(Default, Event)]
pub struct TickEvent;

pub fn tick(
    mut tickers: ResMut<FightingTickers>,
    mut battle: ResMut<Battle>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if tickers.per_tick.tick(time.delta()).just_finished() {
        battle.tick += 1;
        debug!("{:?}: ticked! (tick {})", battle.elapsed, battle.tick);
        commands.trigger(TickEvent);
    }

    battle.elapsed += time.delta();
}

fn check_winner(
    changed: Query<(Entity, &Health), Changed<Health>>,
    query: Query<&Health>,
    mut battle: ResMut<Battle>,
    time_real: Res<Time<Real>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if changed.is_empty() {
        return;
    }

    let Ok(hero) = query.get(battle.hero) else {
        return;
    };
    let hero_alive = hero.current > 0;

    let Ok(villain) = query.get(battle.villain) else {
        return;
    };
    let villain_alive = villain.current > 0;

    let wall_time = time_real.elapsed();

    match (hero_alive, villain_alive) {
        (true, false) => {
            info!(
                "We won in {:?}!  Simulated in {:?}",
                battle.elapsed, wall_time
            );
            battle.over = true;
            next_state.set(GameState::Results);
        }
        (false, true) => {
            info!(
                "We lost in {:?}!  Simulated in {:?}",
                battle.elapsed, wall_time
            );
            battle.over = true;
            next_state.set(GameState::Results);
        }
        (false, false) => {
            info!(
                "We tied in {:?}!  Simulated in {:?}",
                battle.elapsed, wall_time
            );
            battle.over = true;
            next_state.set(GameState::Results);
        }
        (true, true) => {
            info!(
                "{:?}: Hero: {:?} Villain: {:?}",
                battle.elapsed, hero.current, villain.current,
            );
        }
    };
}

fn battle_over(world: &mut World) {
    if !world.get_resource_mut::<Battle>().unwrap().over {
        return;
    }
    let _ = world.try_run_schedule(StateTransition);
}
