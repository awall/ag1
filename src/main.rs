use macroquad::prelude::*;

#[macroquad::main("AG1")]
async fn main() {
    
    let mut world = World {
        player: Player { pos: (0.0, 0.0) },
        monster: Monster { pos: (100.0, 100.0) },
    };

    let mut played_ticks = 0;
    loop {
        clear_background(RED);
        update_world(&mut world, &mut played_ticks);
        draw_world(&world);
        next_frame().await;
    }
}

const PLAYER_SPEED: f32 = 200.0 / 64.0; // 100 units per second
const MONSTER_SPEED: f32 = 100.0 / 64.0; // 100 units per second

struct Player {
    pos: (f32, f32),
}

struct Monster {
    pos: (f32, f32),
}

struct World {
    player: Player,
    monster: Monster,
}

fn update_world(world: &mut World, played_ticks: &mut i32) {
    let total_ticks = (get_time() * 64.0) as i32;
    let ticks = total_ticks - *played_ticks;
    for _n in 0..ticks {
        tick_world(world);
    }
    *played_ticks = total_ticks;
}

fn tick_world(world: &mut World) {
    tick_player(&mut world.player);
    tick_monster(&mut world.monster, world.player.pos);
}

fn tick_player(player: &mut Player) {
    if is_key_down(KeyCode::A) {
        player.pos.0 -= PLAYER_SPEED;
    }
    if is_key_down(KeyCode::D) {
        player.pos.0 += PLAYER_SPEED;
    }
    if is_key_down(KeyCode::W) {
        player.pos.1 -= PLAYER_SPEED;
    }
    if is_key_down(KeyCode::S) {
        player.pos.1 += PLAYER_SPEED;
    }
}

fn tick_monster(monster: &mut Monster, player_pos: (f32, f32)) {
    let x = player_pos.0 - monster.pos.0;
    let y = player_pos.1 - monster.pos.1;
    let magnitude = (x * x + y * y).sqrt();

    let dx = MONSTER_SPEED * x / magnitude;
    let dy = MONSTER_SPEED * y / magnitude;

    monster.pos.0 += dx;
    monster.pos.1 += dy;
}

fn draw_world(world: &World) {
    draw_monster(&world.monster);
    draw_player(&world.player);
    draw_fps();
}

fn draw_monster(monster: &Monster) {
    draw_circle(
        screen_width() / 2.0 + monster.pos.0,
        screen_height() / 2.0 + monster.pos.1,
        20.0,
        BROWN,
    );
}

fn draw_player(player: &Player) {
    draw_circle(
        screen_width() / 2.0 + player.pos.0,
        screen_height() / 2.0 + player.pos.1,
        15.0,
        YELLOW,
    );
}

fn draw_fps() {
    let fps = get_fps();
    draw_text(&fps.to_string(), 20.0, 20.0, 30.0, WHITE);
}
