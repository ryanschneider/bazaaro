use super::*;
use crate::effects::burn::Burn;
use crate::effects::shield::Shield;
use characters::*;
use items::armory::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_characters);
    }
}

fn load_characters(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    // Spawn our Hero
    let _hero = commands
        .spawn((
            Name::new("Our Hero"),
            Character,
            Health::starting(250),
            Hero,
        ))
        .with_related_entities::<ItemOf>(|commands| {
            let _burning_great_sword = {
                let mut e = commands.spawn(GenericWeapon::new(
                    "Burning Great Sword",
                    10,
                    Duration::from_secs(5),
                ));
                e.insert_if_new(Burn::new(15));
                e.id()
            };

            let _healing_potion = commands.spawn(HealingPotion::default()).id();
            let _gorgons_head = commands.spawn(GorgonsHead::default()).id();
            let _freezing_crystal = commands.spawn(FreezingCrystal::default()).id();
            let _haste_potion = commands.spawn(HastePotion::default()).id();
        });

    // And our opponent!
    let _villain = commands
        .spawn((
            Name::new("Evil Henchman"),
            Character,
            Health::starting(150),
            Villain,
        ))
        .with_related_entities::<ItemOf>(|commands| {
            let _shield_talisman = {
                let mut e = commands.spawn(GenericUsable::new(
                    "Shielded Talisman",
                    Duration::from_secs_f32(5.5),
                ));
                e.insert_if_new(Shield::new(10));
                e.id()
            };
            let _poisoned_dagger = commands.spawn(PoisonedDagger::default()).id();
            let _healing_potion = commands.spawn(HealingPotion::default()).id();
        });

    // and start fighting!
    next_state.set(GameState::Fight);
}
