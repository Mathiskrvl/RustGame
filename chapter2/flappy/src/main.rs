#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 50.0;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: i32,
}

struct Player {
    x: i32,
    y: f32,
    velocity: f32,
}

struct Obstacle {
    x: i32,
    gap_y: f32,
    size: f32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40) as f32,
            size: f32::max(2.0, 20.0 - score as f32),
        }
    }
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2.0;

        for y in 0..(self.gap_y - half_size) as i32 {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        for y in (self.gap_y + half_size) as i32..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2.0;
        let x_match = player.x + 5 == self.x;
        let player_hit_top = player.y < self.gap_y - half_size;
        let player_hit_bottom = player.y > self.gap_y + half_size;
        x_match && (player_hit_bottom || player_hit_top)
    }
}

impl Player {
    fn new(x: i32, y: f32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(5, self.y as i32, YELLOW, BLACK, to_cp437('@'));
    }
    fn gravity_and_move(&mut self) {
        if self.velocity < 3.0 {
            self.velocity += 0.5;
        }
        self.y += self.velocity;
        self.x += 1;
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }
    fn flap(&mut self) {
        self.velocity = -3.0;
    }
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(0, 25.0),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y as i32> SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }
    fn restart(&mut self) {
        self.player = Player::new(0, 25.0);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(8, format!("Votre score est {}", self.score));
        ctx.print_centered(11, "(P) Play Game");
        ctx.print_centered(12, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}
