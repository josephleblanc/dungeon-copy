use bevy::prelude::*;
use std::slice::Iter;

use crate::{
    materials::font::FontMaterials,
    resources::{dictionary::Dictionary, glossary::ActionBar},
};

use self::submenu_button::{setup_attack_buttons, setup_move_buttons};

use super::ui_root::UserInterfaceRoot;

pub mod submenu_button;

#[derive(Resource, Debug, Clone, Copy)]
pub struct ActionBarData {
    action_bar_root: Entity,
}

pub fn cleanup(mut commands: Commands, action_bar_root: Res<ActionBarData>) {
    commands
        .entity(action_bar_root.action_bar_root)
        .despawn_recursive();
    commands.remove_resource::<ActionBarData>();
}

pub fn setup(
    mut commands: Commands,
    ui_root: Res<UserInterfaceRoot>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
) {
    let mut action_bar_root: Option<Entity> = None;
    commands
        .get_entity(ui_root.entity)
        .unwrap()
        .with_children(|builder| {
            action_bar_root = Some(
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            left: Val::Percent(15.0),
                            width: Val::Percent(70.0),
                            height: Val::Percent(15.0),
                            ..default()
                        },
                        background_color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
                        ..default()
                    })
                    .with_children(|builder| {
                        action_bar(builder, &font_materials, &dictionary);
                    })
                    .insert(Name::from("Action Bar Root"))
                    .id(),
            );
        });

    commands.insert_resource(ActionBarData {
        action_bar_root: action_bar_root.unwrap(),
    });
    commands.insert_resource(SelectedAction(ActionBarButton::Move));
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ActionBarButton {
    Attack,
    Move,
}

impl ActionBarButton {
    pub fn iterator() -> Iter<'static, Self> {
        [ActionBarButton::Attack, ActionBarButton::Move].iter()
    }

    pub fn to_string_glossary(self, action_bar: &ActionBar) -> String {
        match self {
            Self::Attack => action_bar.attack.clone(),
            Self::Move => action_bar.move_action.clone(),
        }
    }
}

impl std::fmt::Display for ActionBarButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionBarButton::Attack => write!(f, "Attack"),
            ActionBarButton::Move => write!(f, "Move"),
        }
    }
}

pub fn action_bar(
    root: &mut ChildBuilder,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let font_size = 22.0;

    let glossary = dictionary.get_glossary();
    let ingame_glossary = glossary.action_bar;

    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        color: Color::WHITE,
    };

    let submenu_style = Style {
        flex_direction: FlexDirection::ColumnReverse,
        bottom: Val::Px(70.0),
        left: Val::Px(20.0),
        position_type: PositionType::Absolute,
        ..default()
    };

    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        for action_button in ActionBarButton::iterator() {
            let component_name = action_button.to_string_glossary(&ingame_glossary);

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::from(format!(
                    "{} Button and submenu container",
                    action_button
                )))
                .with_children(|builder| {
                    builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    border: UiRect::all(Val::Px(5.0)),
                                    padding: UiRect::all(Val::Px(15.0)),
                                    margin: UiRect::all(Val::Px(15.)),
                                    ..Default::default()
                                },
                                background_color: Color::DARK_GREEN.into(),
                                border_color: Color::BLACK.into(),
                                ..Default::default()
                            },
                            *action_button,
                            Name::new("Action Button"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(component_name, text_style.clone())
                                        .with_alignment(TextAlignment::Center)
                                        .with_no_wrap(),
                                    ..Default::default()
                                },
                                Name::new("Action Button Text"),
                            ));
                        });
                })
                .with_children(|builder| {
                    match action_button {
                        ActionBarButton::Attack => {
                            setup_attack_buttons(builder, dictionary, &text_style, &submenu_style);
                        }
                        ActionBarButton::Move => {
                            setup_move_buttons(builder, dictionary, &text_style, &submenu_style);
                        }
                    };
                });
        }
    })
    .insert(Name::new("Action Button Bar"));
}

#[derive(Resource, Copy, Clone, Debug, Deref, DerefMut, Eq, PartialEq)]
pub struct SelectedAction(pub ActionBarButton);

pub fn handle_buttons(
    mut button_query: Query<
        (&Interaction, &ActionBarButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut current_mode: ResMut<SelectedAction>,
) {
    for (interaction, button, mut bg_color) in button_query.iter_mut() {
        use ActionBarButton::*;
        match interaction {
            Interaction::Pressed => match *button {
                Attack => {
                    if **current_mode != Attack {
                        **current_mode = Attack;
                    }
                }
                Move => {
                    if **current_mode != Move {
                        **current_mode = Move;
                    }
                }
            },
            Interaction::Hovered => *bg_color = Color::GREEN.into(),
            Interaction::None => *bg_color = Color::DARK_GREEN.into(),
        }
    }
}

pub fn handle_button_borders(
    mut button_query: Query<
        (&ActionBarButton, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    current_mode: ResMut<SelectedAction>,
) {
    for (button, mut border_color) in button_query.iter_mut() {
        if *button == **current_mode {
            *border_color = Color::WHITE.into();
        } else {
            *border_color = Color::BLACK.into();
        }
    }
}
