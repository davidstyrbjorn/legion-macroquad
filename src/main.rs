mod components;
mod spawner;
mod system;

mod prelude {
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::system::*;

    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub use legion::{Resources, Schedule, World};
    pub use macroquad::color::*;
    pub use macroquad::shapes::*;
    pub use macroquad::text::*;
    pub use macroquad::time::*;
    pub use macroquad::window::*;

    pub const SCREEN_WIDTH: u32 = 800;
    pub const SCREEN_HEIGHT: u32 = 600;

    // Resources
    pub struct DeltaTime(pub f32);
    pub struct MousePosition {
        pub x: f32,
        pub y: f32,
    }
}

use macroquad::input::mouse_position;
use miniquad::window::set_window_size;

pub use crate::prelude::*;

enum CurrentState {
    MENU,
    PLAYING,
}

struct State {
    ecs: World,
    resources: Resources,
    menu_systems: Schedule,
    current_state: CurrentState,
}

impl State {
    fn new() -> Self {
        let mut ecs: World = World::default();
        let resources = Resources::default();

        spawn_main_menu(&mut ecs);

        Self {
            ecs,
            resources,
            menu_systems: build_menu_scheduler(),
            current_state: CurrentState::MENU,
        }
    }
}

#[macroquad::main("Macroquad + Legion!")]
async fn main() {
    let mut state = State::new();
    set_window_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    loop {
        clear_background(WHITE);

        // We have to insert delta time as a resource here so it can be used by the threaded systems
        state.resources.insert(DeltaTime(get_frame_time()));

        let mouse_pos = mouse_position();
        state.resources.insert(MousePosition {
            x: mouse_pos.0,
            y: mouse_pos.1,
        });

        // Update game data schedule
        match state.current_state {
            CurrentState::MENU => {
                state
                    .menu_systems
                    .execute(&mut state.ecs, &mut state.resources);
            }
            CurrentState::PLAYING => {}
        }

        // Rendering systems
        render_rectangle_system(&mut state.ecs);
        render_text_system(&mut state.ecs);

        // Switch gameplay schedule and upate accordingly
        next_frame().await
    }
}
