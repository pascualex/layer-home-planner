use bevy::prelude::*;

use crate::{
    command::{
        action::UncommittedCommands, point::MovePoint, system_command::RegisterSystemCommand,
    },
    plan::{point::Point, Element, PlanMode, PointMode, TrackMode},
};

pub struct PlanModeCommandPlugin;

impl Plugin for PlanModeCommandPlugin {
    fn build(&self, app: &mut App) {
        app
            // atomic commands
            .register_system_command(select_point)
            .register_system_command(track_point)
            .register_system_command(select_line)
            .register_system_command(unselect)
            .register_system_command(change_selection);
    }
}

// atomic commands

#[derive(Debug)]
pub struct SelectPoint(pub Entity);

fn select_point(In(SelectPoint(point)): In<SelectPoint>, mut plan_mode: ResMut<PlanMode>) {
    // apply
    *plan_mode = PlanMode::Point(point, PointMode::Normal);
}

#[derive(Debug)]
pub struct TrackPoint(pub Entity, pub TrackMode);

fn track_point(
    In(TrackPoint(point, old_selection)): In<TrackPoint>,
    mut plan_mode: ResMut<PlanMode>,
    point_query: Query<&Transform, With<Point>>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
) {
    // get state
    let transform = point_query.get(point).unwrap();
    let old_position = transform.translation.truncate();
    // apply
    *plan_mode = PlanMode::Point(point, PointMode::Track(old_selection));
    // add undo
    uncommitted_commands.add(MovePoint(point, old_position));
}

#[derive(Debug)]
pub struct SelectLine(pub Entity);

fn select_line(In(SelectLine(line)): In<SelectLine>, mut plan_mode: ResMut<PlanMode>) {
    // apply
    *plan_mode = PlanMode::Line(line);
}

#[derive(Debug)]
pub struct Unselect;

fn unselect(In(Unselect): In<Unselect>, mut plan_mode: ResMut<PlanMode>) {
    // apply
    *plan_mode = PlanMode::Normal;
}

#[derive(Debug)]
pub struct ChangeSelection(pub Element);

fn change_selection(
    In(ChangeSelection(new_selection)): In<ChangeSelection>,
    mut plan_mode: ResMut<PlanMode>,
) {
    // apply
    *plan_mode = match new_selection {
        Element::Point(point) => PlanMode::Point(point, PointMode::Normal),
        Element::Line(line) => PlanMode::Line(line),
        Element::None => PlanMode::Normal,
    };
}
