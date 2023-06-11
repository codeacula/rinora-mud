use bevy::prelude::*;

struct TickEvent;

fn tick(time: Res<Time>, mut timer: ResMut<TickTimer>, mut ev_tick: EventWriter<TickEvent>) {
  if timer.0.tick(time.delta()).just_finished() {
    ev_tick.send(TickEvent);
  }
}

fn on_world_tick(mut ev_tick: EventReader<TickEvent>) {
  for _ev in ev_tick.iter() {
    println!("Tick!");
  }
}

#[derive(Resource)]
pub struct TickTimer(Timer);

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(TickTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
    .add_event::<TickEvent>()
    .add_system(tick)
    .add_system(on_world_tick);
  }
}