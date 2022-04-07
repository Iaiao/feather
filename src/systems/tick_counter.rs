use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.insert_resource(TickCounter(0)).add_stage_before(
        CoreStage::First,
        "TODO label",
        SystemStage::single(tick_counter_system),
    );
}

fn tick_counter_system(mut counter: ResMut<TickCounter>) {
    *counter = TickCounter(**counter + 1);
}
