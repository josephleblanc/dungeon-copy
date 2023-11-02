use crate::config::TILE_SIZE;
use crate::plugins::interact::{Interactable, InteractingType};
use crate::plugins::monster::animation::MonsterAnimationComponent;
use crate::plugins::monster::collisions::MonsterBox;
use crate::resources::monster::MonsterLibrary;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::materials::ingame::InGameMaterials;
use crate::resources::animation_state::AnimationState;
use crate::resources::monster::Monster;

pub fn spawn_training_dummy(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ingame_materials: Res<InGameMaterials>,
    monster_dictionary: Res<MonsterLibrary>,
    mut commands: Commands,
) {
    let training_dummy = monster_dictionary
        .get(&Monster::TrainingDummy)
        .unwrap()
        .clone();

    let texture_atlas = get_texture(&training_dummy.monster, &ingame_materials);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let component_name = "Monster TrainingDummy";

    // this is the width/height of the sprite image source.
    let origin_width = 32.0;
    let origin_height = 36.0;

    let x_spawn_pos = TILE_SIZE + TILE_SIZE / 2.0;
    let y_spawn_pos = -TILE_SIZE;

    let interactable_box_lower =
        Vec2::new(x_spawn_pos - TILE_SIZE / 2.0, y_spawn_pos - TILE_SIZE / 2.0);
    let interactable_box_upper =
        Vec2::new(x_spawn_pos + TILE_SIZE / 2.0, y_spawn_pos + TILE_SIZE / 2.0);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(origin_width * 3.5, origin_height * 3.5)),
                anchor: Anchor::BottomCenter,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x_spawn_pos, y_spawn_pos, 0.16),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(training_dummy.clone())
        .insert(MonsterBox {
            width: origin_width * 3.5,
            height: origin_height * 3.5,
        })
        .insert(MonsterAnimationComponent {
            total_tiles: 8,
            animation_state: AnimationState::Idle,
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        })
        .insert(Interactable::new_from_trans(
            interactable_box_lower,
            interactable_box_upper,
            InteractingType::Enemy,
        ))
        .insert(Name::new(component_name));
}

fn get_texture(monster: &Monster, ingame_materials: &InGameMaterials) -> TextureAtlas {
    let monster_tileset = ingame_materials.monsters_materials.get_texture(*monster);

    let columns = 8;

    // this is the width/height of the sprite image source.
    // later I'll put them into a more useful Component or something
    // TODO: Find a home for this data
    let origin_width = 32.0;
    let origin_height = 36.0;

    TextureAtlas::from_grid(
        monster_tileset,
        Vec2::new(origin_width, origin_height),
        columns,
        1,
        None,
        None,
    )
}
