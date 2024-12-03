use crate::prelude::*;

pub fn spawn_red_rectangle(ecs: &mut World, x: f32, y: f32, dx: f32, dy: f32) {
    ecs.push((
        Position { x, y },
        Size {
            width: 50.0,
            height: 50.0,
        },
        Rectangle { color: RED },
        IdleBounce { dx, dy },
    ));
}

pub fn spawn_text(ecs: &mut World, text: &str, x: f32, y: f32, size: f32, color: Color) {
    ecs.push((
        Position { x, y },
        Text {
            text: String::from(text),
            color,
            size,
        },
    ));
}

fn simple_callback() {}

pub fn spawn_button(ecs: &mut World) {
    ecs.push((
        Position { x: 0.0, y: 0.0 },
        Size {
            width: 100.0,
            height: 100.0,
        },
        Button {
            cb: Some(simple_callback),
            background: GRAY,
            on_hover: BLACK,
        },
    ));
}

pub fn spawn_main_menu(ecs: &mut World) {
    spawn_red_rectangle(ecs, 110.0, 101.0, 1.0, 1.0);
    spawn_red_rectangle(ecs, 55.0, 53.0, -1.25, 1.0);
    spawn_red_rectangle(ecs, 305.0, 282.0, -1.5, -1.0);
    spawn_red_rectangle(ecs, 720.0, 400.0, 0.75, -1.0);
    spawn_red_rectangle(ecs, 720.0, 520.0, -1.0, -1.5);

    let text_center = get_text_center("Macroquad + Legion!", Option::None, 64, 1.0, 0.0);
    spawn_text(
        ecs,
        "Macroquad + Legion!",
        (SCREEN_WIDTH as f32 / 2.0) - text_center.x,
        SCREEN_HEIGHT as f32 / 3.0 - text_center.y,
        64.0,
        BLUE,
    );

    spawn_button(ecs);
}
