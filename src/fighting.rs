use crate::characters::*;
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
    #[allow(dead_code)]
    pub start: f64,
    pub elapsed: Duration,
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
}

#[allow(clippy::type_complexity)]
pub fn setup_fight(
    mut commands: Commands,
    mut time: ResMut<Time<Virtual>>,
    mut q_hero: Query<(Entity, &mut Health), (With<Hero>, Without<Villain>)>,
    mut q_villain: Query<(Entity, &mut Health), (With<Villain>, Without<Hero>)>,
) {
    time.set_relative_speed(100000.0);

    let (hero, mut hero_health) = q_hero.single_mut();
    hero_health.reset();

    let (villain, mut villain_health) = q_villain.single_mut();
    villain_health.reset();

    commands.insert_resource(Battle {
        start: time.elapsed_secs_f64(),
        elapsed: Duration::default(),
        over: false,
        hero,
        villain,
    });
    eprintln!("ready to fight!");
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
    battle.elapsed += time.delta();

    if tickers.per_tick.tick(time.delta()).just_finished() {
        // eprintln!("{:?}: ticked!", battle.elapsed);
        commands.trigger(TickEvent);
    }
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

    let duration = battle.elapsed;
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
