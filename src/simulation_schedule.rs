use bevy::prelude::*;

#[derive(Hash, Debug, PartialEq, Eq, Clone, SystemSet)]
pub enum InSimulationSchedule {
    UserInput,
    EntityUpdates,
}


pub struct SimulationSchedulePlugin;


impl Plugin for SimulationSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
            InSimulationSchedule::UserInput,
            InSimulationSchedule::EntityUpdates,
        ).chain());
    }
}