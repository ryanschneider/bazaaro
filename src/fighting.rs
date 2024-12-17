use crate::characters::*;
use crate::effects::DefaultEffects;
use crate::GameState;
use bevy::prelude::*;
use std::time::Duration;

pub struct FightingPlugin;
impl Plugin for FightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FightingTickers::new());
        app.add_systems(
            OnEnter(GameState::Fight),
            (setup_fight,).in_set(SystemSets::OnEnter),
        );
        app.add_systems(
            FixedUpdate,
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
    pub start: f64,
    pub over: bool,
    pub hero: Entity,
    pub villain: Entity,
}

impl Battle {
    pub fn opponent(&self, val: Entity) -> Entity {
        if val == self.hero {
            self.villain
        } else if val == self.villain {
            self.hero
        } else {
            panic!("invalid opponent: {}", val)
        }
    }

    pub fn elapsed(&self, now: f64) -> Duration {
        Duration::from_secs_f64(now - self.start)
    }
}

pub fn setup_fight(
    mut commands: Commands,
    mut time: ResMut<Time<Virtual>>,
    mut q_hero: Query<(Entity, &mut Health), (With<Hero>, Without<Villain>)>,
    mut q_villain: Query<(Entity, &mut Health), (With<Villain>, Without<Hero>)>,
) {
    time.set_relative_speed(100000.0);

    let (hero, mut hero_health) = q_hero.single_mut();
    commands
        .entity(hero)
        .insert_if_new(DefaultEffects::default());
    hero_health.reset();

    let (villain, mut villain_health) = q_villain.single_mut();
    commands
        .entity(villain)
        .insert_if_new(DefaultEffects::default());
    villain_health.reset();

    commands.insert_resource(Battle {
        start: time.elapsed_secs_f64(),
        over: false,
        hero,
        villain,
    });
    eprintln!("ready to fight!");
}

#[derive(Resource)]
pub struct FightingTickers {
    pub per_tick: Timer,
    pub per_second: Timer,
}

impl FightingTickers {
    pub fn new() -> Self {
        Self {
            per_tick: Timer::from_seconds(0.1, TimerMode::Repeating),
            per_second: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

#[derive(Default, Event)]
pub struct TickEvent;

#[derive(Default, Event)]
pub struct MajorTickEvent;

pub fn tick(mut tickers: ResMut<FightingTickers>, time: Res<Time>, mut commands: Commands) {
    if tickers.per_tick.tick(time.delta()).just_finished() {
        commands.trigger(TickEvent);
    }
    if tickers.per_second.tick(time.delta()).just_finished() {
        commands.trigger(MajorTickEvent);
    }
}

fn check_winner(
    changed: Query<(Entity, &Health), Changed<Health>>,
    query: Query<&Health>,
    mut battle: ResMut<Battle>,
    time: Res<Time>,
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

    let duration = battle.elapsed(time.elapsed_secs_f64());
    let wall_time = time_real.elapsed();

    match (hero_alive, villain_alive) {
        (true, false) => {
            eprintln!("We won in {:?}!  Simulated in {:?}", duration, wall_time);
            battle.over = true;
            next_state.set(GameState::Results);
        }
        (false, true) => {
            eprintln!("We lost in {:?}!  Simulated in {:?}", duration, wall_time);
            battle.over = true;
            next_state.set(GameState::Results);
        }
        (false, false) => {
            eprintln!("We tied in {:?}!  Simulated in {:?}", duration, wall_time);
            battle.over = true;
            next_state.set(GameState::Results);
        }
        (true, true) => {
            eprintln!(
                "{:?}: Hero: {} Villain: {}",
                duration, hero.current, villain.current,
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
