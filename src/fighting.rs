use crate::characters::*;
use crate::effects::burn::Burn;
use crate::items::weapons::Weapon;
use crate::GameState;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Default, Component)]
pub struct Shielded(u32);

impl Shielded {
    pub fn absorb(&mut self, dmg: u32) -> u32 {
        if dmg > self.0 {
            let remaining = dmg - self.0;
            self.0 = 0;
            remaining
        } else {
            0
        }
    }
}

#[derive(Default, Component)]
pub struct Regeneration;

#[derive(Default, Component)]
pub struct Burned(u32);

#[derive(Default, Component)]
pub struct Poisoned;

#[derive(Default, Bundle)]
pub struct DefaultEffects {
    shielded: Shielded,
    regeneration: Regeneration,
    burned: Burned,
    poisoned: Poisoned,
}

pub struct FightingPlugin;
impl Plugin for FightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FightingTickers::new());
        app.add_systems(
            OnEnter(GameState::Fight),
            (setup_fight,).in_set(SystemSets::OnEnter),
        );
        app.add_observer(apply_burn);
        app.add_observer(on_attack);
        app.add_observer(on_burned);
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

pub fn apply_burn(
    _: Trigger<MajorTickEvent>,
    time: Res<Time>,
    battle: Res<Battle>,
    mut q_burn: Query<(&Name, &mut Burned, &mut Health, &mut Shielded)>,
) {
    q_burn
        .iter_mut()
        .for_each(|(name, mut burned, mut health, mut shielded)| {
            let burn_amt = burned.0;
            if burn_amt == 0 {
                return;
            }

            eprintln!(
                "{:?}: Burning {:?} for {}",
                battle.elapsed(time.elapsed_secs_f64()),
                name,
                burn_amt,
            );
            // burn shields
            let burn_amt = shielded.absorb(burn_amt);
            if burn_amt == 0 {
                return;
            }
            // then health
            health.current = health.current.saturating_sub(burn_amt);

            // and remove one burn
            burned.0 = burned.0.saturating_sub(1);
        });
}

#[derive(Event)]
pub struct AttackEvent {
    attacker: Entity,
    defender: Entity,
    with: Entity,
}

impl AttackEvent {
    pub fn new(attacker: Entity, defender: Entity, with: Entity) -> Self {
        Self {
            attacker,
            defender,
            with,
        }
    }
}

fn on_attack(
    trigger: Trigger<AttackEvent>,
    time: Res<Time>,
    battle: Res<Battle>,
    q_attacker: Query<&Name>,
    mut q_defender: Query<(&mut Health, &mut Shielded, Option<&Name>), With<Character>>,
    q_weapon: Query<(&Weapon, Option<&Name>)>,
) {
    let AttackEvent {
        attacker,
        defender,
        with,
    } = trigger.event();
    let Ok((mut health, mut shielded, defender_name)) = q_defender.get_mut(*defender) else {
        return;
    };
    let defender_name: &str = defender_name
        .map(|name| name.as_str())
        .unwrap_or("defender");
    let attacker_name: &str = q_attacker.get(*attacker).map_or("attacker", |n| n.as_str());

    let Ok((weapon, weapon_name)) = q_weapon.get(*with) else {
        return;
    };
    let damage = weapon.damage;
    let damage = shielded.absorb(damage);
    health.current = health.current.saturating_sub(damage);
    let weapon_name: &str = weapon_name.map_or("some weapon", |n| n.as_str());

    eprintln!(
        "{:?}: {:?} attacked {:?} with {} for {}!",
        battle.elapsed(time.elapsed_secs_f64()),
        attacker_name,
        defender_name,
        weapon_name,
        damage
    );
}

fn on_burned(
    trigger: Trigger<BurnEvent>,
    time: Res<Time>,
    battle: Res<Battle>,
    q_attacker: Query<&Name>,
    mut q_defender: Query<(&mut Burned, Option<&Name>), With<Character>>,
    q_burner: Query<(&Burn, Option<&Name>)>,
) {
    let BurnEvent {
        attacker,
        defender,
        with,
    } = trigger.event();
    let Ok((mut burned, defender_name)) = q_defender.get_mut(*defender) else {
        return;
    };
    let defender_name: &str = defender_name
        .map(|name| name.as_str())
        .unwrap_or("defender");
    let attacker_name: &str = q_attacker.get(*attacker).map_or("attacker", |n| n.as_str());

    let Ok((source, source_name)) = q_burner.get(*with) else {
        return;
    };
    let burn = source.amount;
    burned.0 += burn;
    let source_name: &str = source_name.map_or("some burner", |n| n.as_str());

    eprintln!(
        "{:?}: {:?} burned {:?} with {} for {}!",
        battle.elapsed(time.elapsed_secs_f64()),
        attacker_name,
        defender_name,
        source_name,
        burn
    );
}

#[derive(Event)]
pub struct BurnEvent {
    attacker: Entity,
    defender: Entity,
    with: Entity,
}

impl BurnEvent {
    pub fn new(attacker: Entity, defender: Entity, with: Entity) -> Self {
        Self {
            attacker,
            defender,
            with,
        }
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
