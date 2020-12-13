use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    End
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

impl State {

    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        self.menu_screen("Welcome to Flappy Dragon", ctx);
    }

    fn menu_screen(&mut self, msg: &str, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, msg);
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

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
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
        if (self.player.y > SCREEN_HEIGHT) {
            self.mode = GameMode::End
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        self.menu_screen("You are dead!", ctx);
    }

}

struct Player {
    x: i32,
    y: i32,
    velocity: f32
}

impl Player {

    fn new(x: i32, y: i32) -> Self {
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
        )
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

impl GameState for State {

    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx)
        }
    }

}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(ctx, State::new())
}
