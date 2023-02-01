use crate::map::ItemMap;
use crate::player::Player;
use bevy::log::warn;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource, Default)]
pub struct Inventory {
    items: HashMap<String, u32>,
}

#[derive(Component)]
pub struct InventoryText;

impl Inventory {
    pub fn add_item(&mut self, item: &str) {
        self.add_item_stack(item, 1);
    }

    pub fn add_item_stack(&mut self, item: &str, stack_size: u32) {
        self.items
            .insert(item.to_string(), self.get_item_count(item) + stack_size);
    }

    #[allow(dead_code)]
    pub fn remove_item(&mut self, item: &str) {
        self.remove_item_stack(item, 1);
    }

    #[allow(dead_code)]
    pub fn remove_item_stack(&mut self, item: &str, stack_size: u32) {
        let count = self.get_item_count(item);
        if count > 0 {
            self.items.insert(item.to_string(), count - stack_size);
        } else {
            warn!("Tried to remove item when count is zero");
        }
    }

    pub fn get_item_count(&self, item: &str) -> u32 {
        *self.items.get(item).unwrap_or(&0)
    }
}

pub fn pickup_items(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    mut item_map: ResMut<ItemMap>,
    player_query: Query<&Transform, With<Player>>,
) {
    for transform in player_query.iter() {
        let pos = IVec2::new(
            (transform.translation.x / 16.0) as i32,
            (transform.translation.y / 16.0) as i32,
        );

        if let Some((item_name, entity)) = item_map.item_map.remove(&pos) {
            inventory.add_item(&item_name);
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "KEYS - ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                    font_size: 60.0,
                    color: Color::BLACK,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 60.0,
                color: Color::BLACK,
            }),
        ])
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        InventoryText,
    ));
}

pub fn text_update_system(
    mut query: Query<&mut Text, With<InventoryText>>,
    inventory: Res<Inventory>,
) {
    for mut text in &mut query {
        text.sections[1].value = format!("{count}", count = inventory.get_item_count("key"));
    }
}

#[cfg(test)]
mod inventory_tests {
    use super::*;

    #[test]
    fn default_inventory_is_empty() {
        let inv = Inventory::default();

        assert_eq!(inv.items.len(), 0);
    }

    #[test]
    fn get_item_count_returns_0() {
        let inv = Inventory::default();

        assert_eq!(inv.get_item_count(&"foo".to_string()), 0);
    }

    #[test]
    fn add_item_increases_count() {
        let mut inv = Inventory::default();
        inv.add_item("foo");

        assert_eq!(inv.items.len(), 1);
        assert_eq!(inv.get_item_count(&"foo".to_string()), 1);
    }

    #[test]
    fn remove_item_decreases_count() {
        let mut inv = Inventory::default();
        inv.add_item_stack("foo", 3);
        inv.remove_item("foo");

        assert_eq!(inv.items.len(), 1);
        assert_eq!(inv.get_item_count(&"foo".to_string()), 2);
    }
}
