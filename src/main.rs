use macroquad::prelude::*;

#[macroquad::main("AG1")]
async fn main() {
    let mut player = Player { pos: (0.0, 0.0) };
    let mut played_ticks = 0;
    loop {
        clear_background(RED);
        update_world(&mut player, &mut played_ticks);
        draw_frame(&player);
        next_frame().await;
    }
}

const SPEED: f32 = 100.0 / 64.0; // 100 units per second

struct Player {
    pos: (f32, f32),
}

fn update_world(player: &mut Player, played_ticks: &mut i32) {
    let total_ticks = (get_time() * 64.0) as i32;
    let ticks = total_ticks - *played_ticks;
    for _n in 0..ticks {
        update_one_tick(player);
    }
    *played_ticks = total_ticks;
}

fn update_one_tick(player: &mut Player) {
    if is_key_down(KeyCode::A) {
        player.pos.0 -= SPEED;
    }
    if is_key_down(KeyCode::D) {
        player.pos.0 += SPEED;
    }
    if is_key_down(KeyCode::W) {
        player.pos.1 -= SPEED;
    }
    if is_key_down(KeyCode::S) {
        player.pos.1 += SPEED;
    }
}

fn draw_frame(player: &Player) {
    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_player(player);
    draw_fps();
}

fn draw_player(player: &Player) {
    let color = if is_key_down(KeyCode::A) {
        YELLOW
    } else {
        WHITE
    };

    draw_circle(
        screen_width() / 2.0 + player.pos.0,
        screen_height() / 2.0 + player.pos.1,
        15.0,
        color,
    );
}

fn draw_fps() {
    let fps = get_fps();
    draw_text(&fps.to_string(), 20.0, 20.0, 30.0, WHITE);
}
