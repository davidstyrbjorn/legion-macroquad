use crate::prelude::*;

pub fn build_menu_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(idle_bounce_system())
        .add_system(handle_button_system())
        .build()
}

pub fn render_rectangle_system(ecs: &mut World) {
    let renderables: Vec<(&Position, &Size, &Rectangle)> =
        <(&Position, &Size, &Rectangle)>::query()
            .iter(ecs)
            .collect();

    renderables.iter().for_each(|(position, size, rectangle)| {
        draw_rectangle(
            position.x,
            position.y,
            size.width,
            size.height,
            rectangle.color,
        );
    });
}

pub fn render_text_system(ecs: &mut World) {
    let renderables: Vec<(&Text, &Position)> = <(&Text, &Position)>::query().iter(ecs).collect();
    renderables.iter().for_each(|(text, position)| {
        draw_text(&text.text, position.x, position.y, text.size, text.color);
    });
}

#[system(for_each)]
pub fn idle_bounce(
    position: &mut Position,
    size: &Size,
    idle_bounce: &mut IdleBounce,
    #[resource] delta_time: &DeltaTime,
) {
    const SPEED: f32 = 200.0;
    position.x += idle_bounce.dx * SPEED * delta_time.0;
    position.y += idle_bounce.dy * SPEED * delta_time.0;

    if position.x < 0.0 || position.x > SCREEN_WIDTH as f32 - size.width {
        idle_bounce.dx *= -1.0;
    }
    if position.y < 0.0 || position.y > SCREEN_HEIGHT as f32 - size.height {
        idle_bounce.dy *= -1.0;
    }
}

#[system(for_each)]
pub fn handle_button(
    button: &mut Button,
    position: &Position,
    size: &Size,
    #[resource] mouse_position: &MousePosition,
) {
    println!("x: {}, y: {}", mouse_position.x, mouse_position.y);
}
