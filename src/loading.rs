use super::*;
use characters::*;
use items::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_characters);
    }
}

fn load_characters(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    // Spawn our Hero
    let _hero = {
        let mut items = Items::default();
        items.slots.extend::<Vec<Option<Entity>>>(vec![
            Some(commands.spawn(HandAxe::default()).id()),
            Some(
                commands
                    .spawn(GenericWeapon::new(
                        "Great Sword",
                        30,
                        Duration::from_secs(5),
                    ))
                    .id(),
            ),
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
        let mut items = Items::default();
        items
            .slots
            .push(Some(commands.spawn(HandAxe::default()).id()));
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
