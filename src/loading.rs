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
    let _hero = {
        let burning_great_sword = {
            let mut e = commands.spawn(GenericWeapon::new(
                "Burning Great Sword",
                10,
                Duration::from_secs(5),
            ));
            e.insert_if_new(Burn::new(15));
            e.id()
        };

        let mut items = Items::default();
        items.slots.extend(vec![
            Some(commands.spawn(HandAxe::default()).id()),
            Some(burning_great_sword),
        ]);

        let hero = commands
            .spawn((
                Name::new("Our Hero"),
                Character,
                Health::starting(250),
                Hero,
            ))
            .id();
        items.attach_to(hero, &mut commands);
        hero
    };

    // And our opponent!
    let _villain = {
        let shield_talisman = {
            let mut e = commands.spawn(GenericUsable::new(
                "Shielded Talisman",
                Duration::from_secs_f32(5.5),
            ));
            e.insert_if_new(Shield::new(10));
            e.id()
        };
        let poisoned_dagger = commands.spawn(PoisonedDagger::default()).id();
        let mut items = Items::default();
        items.slots.extend(vec![
            Some(commands.spawn(HandAxe::default()).id()),
            Some(shield_talisman),
            Some(poisoned_dagger),
        ]);
        let villain = commands
            .spawn((
                Name::new("Evil Henchman"),
                Character,
                Health::starting(150),
                Villain,
            ))
            .id();
        items.attach_to(villain, &mut commands);
        villain
    };

    // and start fighting!
    next_state.set(GameState::Fight);
}
