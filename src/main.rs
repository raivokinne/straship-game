use rand::Rng;
use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;
const STARSHIP_SPEED: f32 = 0.05;
const METEORITE_SPEED: f32 = 0.05;

struct Starship {
    position: Vector2,
    size: Vector2,
    speed: f32,
    texture: Option<Texture2D>,
    lives: i32,
}

struct Meteorite {
    position: Vector2,
    size: Vector2,
    speed: f32,
    texture: Option<Texture2D>,
}

struct Game {
    starship: Starship,
    meteorites: Vec<Meteorite>,
    score: i32,
    over: bool,
    pause: bool,
    background: Option<Texture2D>,
}

impl Starship {
    fn new() -> Starship {
        Starship {
            position: Vector2::new(400.0, 400.0),
            size: Vector2::new(100.0, 100.0),
            speed: STARSHIP_SPEED,
            texture: None,
            lives: 3,
        }
    }
}

impl Meteorite {
    fn new(position: Vector2) -> Meteorite {
        Meteorite {
            position,
            size: Vector2::new(150.0, 150.0),
            speed: METEORITE_SPEED,
            texture: None,
        }
    }
}

impl Game {
    fn new() -> Game {
        Game {
            starship: Starship::new(),
            meteorites: Vec::new(),
            score: 0,
            over: false,
            pause: false,
            background: None,
        }
    }

    fn start(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.background = Some(rl.load_texture(thread, "assets/images/bg.png").unwrap());
        self.starship.texture = Some(rl.load_texture(thread, "assets/images/ship.png").unwrap());
        self.spawn_meteorite(rl, thread);
    }

    fn spawn_meteorite(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut rng = rand::thread_rng();
        let x_pos = rng.gen_range(0..SCREEN_WIDTH - 300) as f32;
        let meteorite = Meteorite::new(Vector2::new(x_pos, 0.0));
        self.meteorites.push(meteorite);
        self.meteorites.last_mut().unwrap().texture =
            Some(rl.load_texture(thread, "assets/images/meteor.png").unwrap());
    }

    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if !self.over {
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                self.pause = !self.pause;
            }

            if self.pause {
                return;
            }

            if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                self.starship.position.x += self.starship.speed;
            }
            if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                self.starship.position.x -= self.starship.speed;
            }

            if self.starship.position.x < 0.0 {
                self.starship.position.x = 0.0;
            }
            if self.starship.position.x + self.starship.size.x > SCREEN_WIDTH as f32 {
                self.starship.position.x = SCREEN_WIDTH as f32 - self.starship.size.x;
            }

            for meteorite in &mut self.meteorites {
                meteorite.position.y += meteorite.speed;

                let starship_rect = Rectangle::new(
                    self.starship.position.x,
                    self.starship.position.y,
                    self.starship.size.x,
                    self.starship.size.y,
                );

                let meteorite_rect = Rectangle::new(
                    meteorite.position.x,
                    meteorite.position.y,
                    meteorite.size.x,
                    meteorite.size.y,
                );

                if starship_rect.check_collision_recs(&meteorite_rect) {
                    self.starship.lives -= 1;
                    if self.starship.lives <= 0 {
                        self.over = true;
                    }

                    self.meteorites.clear();
                    self.spawn_meteorite(rl, thread);
                    self.score = 0;
                    break;
                }
            }

            self.meteorites
                .retain(|meteorite| meteorite.position.y < SCREEN_HEIGHT as f32);

            if self.meteorites.is_empty() {
                self.spawn_meteorite(rl, thread);
                self.score += 1;
                self.starship.speed += 0.01;
                for meteorite in &mut self.meteorites {
                    meteorite.speed += 0.02;
                }
            }
        } else {
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                self.reset(rl, thread);
            }
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        if !self.over {
            d.draw_texture(self.background.as_ref().unwrap(), 0, 0, Color::WHITE);

            self.draw_starship(d);
            self.draw_meteorites(d);

            d.draw_text(&format!("Score: {}", self.score), 10, 10, 20, Color::BLACK);
            d.draw_text(
                &format!("Lives: {}", self.starship.lives),
                10,
                40,
                20,
                Color::BLACK,
            );

            if self.pause {
                d.draw_text(
                    "Paused",
                    SCREEN_WIDTH / 2 - d.measure_text("Paused", 40) / 2,
                    SCREEN_HEIGHT / 2 - 20,
                    40,
                    Color::RED,
                );
            }
        } else {
            d.clear_background(Color::RAYWHITE);
            d.draw_text(
                "Game Over",
                SCREEN_WIDTH / 2 - d.measure_text("Game Over", 40) / 2,
                SCREEN_HEIGHT / 2 - 20,
                40,
                Color::RED,
            );
            d.draw_text(
                "Press Enter to restart",
                SCREEN_WIDTH / 2 - d.measure_text("Press Enter to restart", 20) / 2,
                SCREEN_HEIGHT / 2 + 20,
                20,
                Color::BLACK,
            );
        }
    }

    fn draw_starship(&self, d: &mut RaylibDrawHandle) {
        if let Some(texture) = &self.starship.texture {
            d.draw_texture_ex(
                texture,
                self.starship.position,
                0.0,
                self.starship.size.x / texture.width() as f32,
                Color::WHITE,
            );
        } else {
            d.draw_rectangle_v(self.starship.position, self.starship.size, Color::BLUE);
        }
    }

    fn draw_meteorites(&self, d: &mut RaylibDrawHandle) {
        for meteorite in &self.meteorites {
            if let Some(texture) = &meteorite.texture {
                d.draw_texture_ex(
                    texture,
                    meteorite.position,
                    0.0,
                    meteorite.size.x / texture.width() as f32,
                    Color::WHITE,
                );
            } else {
                d.draw_circle_v(meteorite.position, meteorite.size.x, Color::BROWN);
            }
        }
    }

    fn reset(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.starship = Starship::new();
        self.meteorites.clear();
        self.score = 0;
        self.over = false;
        self.pause = false;
        self.start(rl, thread);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Starship Game")
        .build();

    let mut game = Game::new();
    game.start(&mut rl, &thread);

    while !rl.window_should_close() {
        game.update(&mut rl, &thread);

        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d);
    }
}
