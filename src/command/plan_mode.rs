use bevy::prelude::*;

use crate::{command::RegisterUndoSystemCommand, plan::PlanMode};

pub struct PlanModeCommandPlugin;

impl Plugin for PlanModeCommandPlugin {
    fn build(&self, app: &mut App) {
        app.register_undo_system_command(change_plan_mode);
    }
}

pub struct ChangePlanMode(pub PlanMode);

fn change_plan_mode(
    In(ChangePlanMode(new_plan_mode)): In<ChangePlanMode>,
    mut plan_mode: ResMut<PlanMode>,
) -> ChangePlanMode {
    // get state
    let old_plan_mode = *plan_mode;
    // apply
    *plan_mode = new_plan_mode;
    // build undo
    ChangePlanMode(old_plan_mode)
}
