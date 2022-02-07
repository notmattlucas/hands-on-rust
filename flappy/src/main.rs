use bracket_lib::prelude::*;
use crate::GameMode::*;

const SCREEN_WIDTH:i32 = 80;
const SCREEN_HEIGHT:i32 = 50;
const FRAME_DURATION:f32 = 75.0;

struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    score: i32,
    obstacle: Obstacle
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32
}

impl Obstacle {
    fn new(x:i32, score:i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score)
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // Draw top half
        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x,
                y,
                RED,
                BLACK,
                to_cp437('|')
            )
        }

        // Draw bottom half
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                RED,
                BLACK,
                to_cp437('|')
            )
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        does_x_match && (player.y < self.gap_y - half_size || player.y > self.gap_y + half_size)
    }
}

impl Player {
    fn new(x:i32, y:i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        );
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

enum GameMode {
    Menu,
    Playing,
    End
}

impl GameState for State {

    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        match &self.mode {
            Menu => self.main_menu(ctx),
            Playing => self.play(ctx),
            End => self.dead(ctx)
        }
    }

}

impl State {

    fn new() -> Self {
        State {
            mode: Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            score: 0,
            obstacle: Obstacle::new(0, SCREEN_WIDTH)
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        self.check_option(ctx);
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time= 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        self.obstacle.render(ctx, self.player.x);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(
                self.player.x + SCREEN_WIDTH, self.score
            )
        }
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        self.check_option(ctx)
    }

    fn check_option(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.mode = Playing;
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
