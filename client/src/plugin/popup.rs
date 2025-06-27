use crate::prelude::widget::blocking_overlay;
use crate::prelude::*;
use crate::theme::widget::text_base;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use std::mem::take;

pub struct PopupPlugin;
impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenPopupEvent>()
            .add_event::<ClosePopupEvent>()
            .insert_resource(PopupCounter(10))
            .insert_resource(PopupStack(Vec::new()))
            .add_observer(open_popup_system)
            .add_observer(close_popup_system)
            .add_observer(close_all_popup_system);
    }
}

#[derive(Resource)]
struct PopupCounter(i32);

#[derive(Resource)]
struct PopupStack(Vec<Entity>);

#[derive(Event)]
pub struct OpenPopupEvent {
    pub content_builder: Box<dyn Fn(&mut RelatedSpawnerCommands<ChildOf>) + Send + Sync>,
    pub blocking: bool,
}

#[derive(Event)]
pub struct ClosePopupEvent;

#[derive(Event)]
pub struct CloseAllPopupEvent;

fn open_popup_system(
    trigger: Trigger<OpenPopupEvent>,
    mut cmds: Commands,
    mut popup_counter: ResMut<PopupCounter>,
    mut popup_stack: ResMut<PopupStack>,
) {
    let event = trigger.event();
    popup_counter.0 += 1;

    let popup_root = if event.blocking {
        cmds.spawn((blocking_overlay(popup_counter.0),)).id()
    } else {
        cmds.spawn(overlay(popup_counter.0)).id()
    };

    let content_panel = cmds
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Node::DEFAULT.full_size()
            },
            Pickable::IGNORE,
            if event.blocking {
                BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5))
            } else {
                BackgroundColor(Color::NONE)
            },
            // children![(text_base("弹窗", Vw(2.), Color::WHITE),)],
        ))
        .id();

    cmds.entity(content_panel).with_children(|parent| {
        (event.content_builder)(parent);
    });

    cmds.entity(popup_root).add_child(content_panel);
    popup_stack.0.push(popup_root);
}

fn close_popup_system(
    _: Trigger<ClosePopupEvent>,
    mut cmds: Commands,
    mut popup_stack: ResMut<PopupStack>,
) {
    let Some(entity) = popup_stack.0.pop() else {
        return;
    };
    cmds.entity(entity).despawn();
}

fn close_all_popup_system(
    _: Trigger<CloseAllPopupEvent>,
    mut cmds: Commands,
    mut popup_stack: ResMut<PopupStack>,
) {
    while let Some(entity) = popup_stack.0.pop() {
        cmds.entity(entity).despawn();
    }
}