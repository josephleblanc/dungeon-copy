// use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::components::creature::Creature;
use crate::components::feats::combat_feats::{ImprovedCritical, WeaponFocus};
use crate::components::player::PlayerComponent;
use crate::components::player_animation::PlayerAnimation;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::combat::attack_of_opportunity::aoo_round_modifier::CombatReflexes;
use crate::plugins::item::equipment::weapon::EquippedWeapons;
use crate::plugins::player::{PLAYER_SIZE_HEIGHT, PLAYER_SIZE_WIDTH};
use crate::resources::equipment::weapon::{WeaponBundle, WeaponName};
use crate::resources::equipment::Armory;
use crate::resources::game_data::GameData;
use crate::resources::profile::Profile;

use super::control::ActionPriority;
// use crate::resources::upgrade::upgrade_controller::UpgradeController;

const PLAYER_ORIGIN_SIZE_WIDTH: f32 = 16.0;
const PLAYER_ORIGIN_SIZE_HEIGHT: f32 = 28.0;

/// Used to keep track of the player entity id so it can be referenced elsewhere,
/// or despawned during cleanup.
#[derive(Resource)]
pub struct PlayerEntity {
    pub entity: Entity,
}

/// Inititates a new player character.
/// This is used to create the player character when the dungeon is spawned.
pub fn initiate_player(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ingame_materials: Res<InGameMaterials>,
    game_data: Res<GameData>,
    mut commands: Commands,
    profile: Res<Profile>,
    armory: Res<Armory>,
) {
    let class = profile.hero_class.clone();
    let gender = profile.gender.clone();
    let hero = game_data.get_hero(class.clone());

    let longsword = armory.get(&WeaponName::Longsword).unwrap().clone();
    let weapon_focus = WeaponFocus::new(1, vec![longsword.weapon_name]);

    let improved_critical = ImprovedCritical::new(vec![WeaponName::Longsword]);

    // let skill = game_data.get_skill(class.clone());

    let player = PlayerComponent::new(class.clone(), game_data.clone());
    let player_attributes = hero.attributes;
    let player_bab = hero.base_attack_bonus;

    let hero_tileset = ingame_materials
        .heroes_materials
        .get_texture(class.clone(), gender);

    let texture_atlas = TextureAtlas::from_grid(
        hero_tileset,
        Vec2::new(PLAYER_ORIGIN_SIZE_WIDTH, PLAYER_ORIGIN_SIZE_HEIGHT),
        9,
        1,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut weapon_entity: Option<Entity> = None;
    let player_entity = commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE_WIDTH, PLAYER_SIZE_HEIGHT)),
                anchor: Anchor::BottomCenter,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.15),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.25, 0.75),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
                ..default()
            });
        })
        .with_children(|builder| {
            weapon_entity = Some(builder.spawn(WeaponBundle { weapon: longsword }).id());
        })
        .insert(player)
        .insert(CombatReflexes)
        .insert(player_attributes)
        .insert(player_bab)
        .insert(weapon_focus)
        .insert(improved_critical)
        .insert(EquippedWeapons {
            main_hand: weapon_entity.unwrap(),
            off_hand: vec![],
        })
        .insert(Creature)
        .insert(ActionPriority)
        .insert(PlayerAnimation::new())
        .insert(Name::new("Player"))
        .id();

    // commands.insert_resource(UpgradeController::new());
    // TODO: Decide whether there is sufficient justification for a PlayerEntity
    // resource to exist. If there is only ever one Player, then it could just
    // as easily be a component on the player entity, which can be queried for
    // when needed.
    commands.insert_resource(PlayerEntity {
        entity: player_entity,
    });
}
