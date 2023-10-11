use bevy::prelude::*;
use std::slice::Iter;

use crate::materials::font::FontMaterials;
use crate::materials::ingame::InGameMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::game_mode::GameMode;
use crate::resources::hero::gender::Gender;
use crate::resources::hero::hero_class::HeroClass;
use crate::resources::profile::Profile;
use crate::scenes::SceneState;

const RETURN_BUTTON_SIZE: f32 = 50.0;
const BOX_TILE_SIZE: f32 = 60.0;

#[derive(Component)]
struct ReturnButtonComponent;

#[derive(Component, PartialEq, Clone)]
pub enum ButtonComponent {
    FemaleFighter,
    MaleFighter,
    FemaleWizard,
    MaleWizard,
}

impl ButtonComponent {
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::FemaleFighter,
            ButtonComponent::MaleFighter,
            ButtonComponent::FemaleWizard,
            ButtonComponent::MaleWizard,
        ]
        .iter()
    }
}

#[derive(Resource)]
struct AnimationController {
    run_animation: bool,
    hero_image: HeroImageComponent,
    timer: Timer,
}

pub struct HeroSelectScenePlugin;

#[derive(Component, Clone, PartialEq, Eq)]
enum HeroImageComponent {
    MaleFighter,
    MaleWizard,
    FemaleFighter,
    FemaleWizard,
}

#[derive(Resource)]
struct HeroSelectSceneData {
    sprite_bundle: Entity,
    user_interface_root: Entity,
}

impl Plugin for HeroSelectScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::HeroSelectScene), setup);
        // app.add_systems(Update, (
        //     return_button_handle,
        //     hero_select_handle,
        //     hero_image_animation_handle,
        // ).run_if(in_state(SceneState::HeroSelectScene)));
        app.add_systems(OnExit(SceneState::HeroSelectScene), cleanup);
    }
}

fn setup(
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    ingame_materials: Res<InGameMaterials>,
    scenes_materials: Res<ScenesMaterials>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
    mut commands: Commands,
) {
    let sprite_bundle = commands
        .spawn(SpriteBundle {
            texture: scenes_materials.sub_background_image.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            menu_box(parent, &scenes_materials.menu_box_materials);
            heroes_images(parent, &ingame_materials, texture_atlases)
        })
        .insert(Name::new("SpriteBundle"))
        .id();

    let user_interface_root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            select_hero_text(parent, &font_materials, &dictionary);
            return_button(parent, &scenes_materials);
            heroes_buttons(parent);
        })
        .insert(Name::new("UIRoot"))
        .id();

    commands.insert_resource(HeroSelectSceneData {
        sprite_bundle,
        user_interface_root,
    });

    commands.insert_resource(AnimationController {
        run_animation: false,
        hero_image: HeroImageComponent::MaleFighter,
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
    });
}

fn cleanup(mut commands: Commands, hero_select_scene_data: Res<HeroSelectSceneData>) {
    commands
        .entity(hero_select_scene_data.user_interface_root)
        .despawn_recursive();

    commands
        .entity(hero_select_scene_data.sprite_bundle)
        .despawn_recursive();

    commands.remove_resource::<AnimationController>();
}

fn menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let box_array = menu_box_materials.build_box(13, 9).unwrap();

    let start_x = -340.0;
    let start_y = 230.0;

    for (row_index, row) in box_array.into_iter().enumerate() {
        for (column_index, image) in row.into_iter().enumerate() {
            root.spawn(SpriteBundle {
                texture: image,
                transform: Transform {
                    translation: Vec3::new(
                        start_x + BOX_TILE_SIZE * column_index as f32,
                        start_y - BOX_TILE_SIZE * row_index as f32,
                        0.1,
                    ),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BOX_TILE_SIZE, BOX_TILE_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

fn heroes_images(
    root: &mut ChildBuilder,
    ingame_materials: &InGameMaterials,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut index = 0;
    let hero_image_positions: [[f32; 2]; 8] = [
        [-250.0, 75.0],
        [-250.0, -100.0],
        [-75.0, 75.0],
        [-75.0, -100.0],
        [100.0, 75.0],
        [100.0, -100.0],
        [275.0, 75.0],
        [275.0, -100.0],
    ];

    for hero_class in HeroClass::iterator() {
        for gender in Gender::iterator() {
            let hero_tileset;
            let hero_image;
            let component_name;

            match hero_class {
                HeroClass::Fighter => match gender {
                    Gender::Male => {
                        hero_tileset = ingame_materials.heroes_materials.male_fighter.clone();
                        hero_image = HeroImageComponent::MaleFighter;
                        component_name = format!("{}_{}", "Fighter", "Male");
                    }
                    Gender::Female => {
                        hero_tileset = ingame_materials.heroes_materials.male_fighter.clone();
                        hero_image = HeroImageComponent::FemaleFighter;
                        component_name = format!("{}_{}", "Fighter", "Female");
                    }
                },
                HeroClass::Wizard => match gender {
                    Gender::Male => {
                        hero_tileset = ingame_materials.heroes_materials.male_wizard.clone();
                        hero_image = HeroImageComponent::MaleWizard;
                        component_name = format!("{}_{}", "Wizard", "Male");
                    }
                    Gender::Female => {
                        hero_tileset = ingame_materials.heroes_materials.female_wizard.clone();
                        hero_image = HeroImageComponent::FemaleWizard;
                        component_name = format!("{}_{}", "Wizard", "Female");
                    }
                },
            };

            let texture_atlas =
                TextureAtlas::from_grid(hero_tileset, Vec2::new(16.0, 28.0), 9, 1, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let x = hero_image_positions[index][0];
            let y = hero_image_positions[index][1];

            root.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(x, y, 0.2),
                    scale: Vec3::splat(4.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new(component_name))
            .insert(hero_image);
            index += 1;
        }
    }
}

fn select_hero_text(
    root: &mut ChildBuilder,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();
    root.spawn(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(390.0),
            top: Val::Px(95.0),
            ..Default::default()
        },
        text: Text::from_section(
            glossary.shared_text.select_hero,
            TextStyle {
                font: font,
                font_size: 50.0,
                color: Color::BLACK,
            },
        )
        .with_alignment(TextAlignment::Center),
        ..Default::default()
    })
    .insert(Name::new("SelectHeroText"));
}

fn return_button(root: &mut ChildBuilder, scenes_materials: &ScenesMaterials) {
    let handle_image = scenes_materials.icon_materials.home_icon_normal.clone();
    root.spawn(ButtonBundle {
        style: Style {
            left: Val::Px(RETURN_BUTTON_SIZE / 2.0),
            top: Val::Px(RETURN_BUTTON_SIZE / 2.0),
            right: Val::Auto,
            bottom: Val::Auto,
            width: Val::Px(RETURN_BUTTON_SIZE),
            height: Val::Px(RETURN_BUTTON_SIZE),
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        image: UiImage::new(handle_image),
        ..Default::default()
    })
    .insert(Name::new("ReturnButton"))
    .insert(ReturnButtonComponent);
}

fn heroes_buttons(root: &mut ChildBuilder) {
    let button_positions: [[f32; 2]; 8] = [
        [210.0, 170.0],
        [380.0, 170.0],
        [560.0, 170.0],
        [740.0, 170.0],
        [210.0, 350.0],
        [380.0, 350.0],
        [560.0, 350.0],
        [740.0, 350.0],
    ];

    for (index, value) in ButtonComponent::iterator().enumerate() {
        let component_name = match index {
            1 => "MaleElf",
            2 => "MaleKnight",
            3 => "MaleLizard",
            4 => "MaleWizard",
            5 => "FemaleElf",
            6 => "FemaleKnight",
            7 => "FemaleLizard",
            _ => "FemaleWizard",
        };

        root.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(button_positions[index][0]),
                top: Val::Px(button_positions[index][1]),
                right: Val::Auto,
                bottom: Val::Auto,
                width: Val::Px(100.0),
                height: Val::Px(150.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new(component_name))
        .insert(value.clone());
    }
}

fn hero_select_handle(
    mut button_query: Query<(&Interaction, &ButtonComponent), (Changed<Interaction>, With<Button>)>,
    mut profile: ResMut<Profile>,
    mut animation_controller: ResMut<AnimationController>,
    mut state: ResMut<NextState<SceneState>>,
) {
    for (interaction, button) in button_query.iter_mut() {
        match interaction {
            Interaction::None => animation_controller.run_animation = false,
            Interaction::Hovered => {
                animation_controller.run_animation = true;
                match button {
                    ButtonComponent::MaleFighter => {
                        animation_controller.hero_image = HeroImageComponent::MaleFighter
                    }
                    ButtonComponent::FemaleFighter => {
                        animation_controller.hero_image = HeroImageComponent::FemaleFighter
                    }
                    ButtonComponent::MaleWizard => {
                        animation_controller.hero_image = HeroImageComponent::MaleWizard
                    }
                    ButtonComponent::FemaleWizard => {
                        animation_controller.hero_image = HeroImageComponent::FemaleWizard
                    }
                };
            }
            Interaction::Pressed => {
                profile.set_hero(button.clone());
                if profile.game_mode == GameMode::ClassicMode {
                    state.set(SceneState::PreClassicMode);
                } else {
                    // state.set(SceneState::PreSurvivalMode);
                }
            }
        }
    }
}
