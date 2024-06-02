use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawParam, Mesh, Text, TextFragment};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::glam::Vec2;
use mint::Point2;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 5.0;
const BALL_RADIUS: f32 = 10.0;

#[derive(PartialEq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(PartialEq)]
enum GameState {
    Home,
    Menu,
    Playing,
    Paused,
}

struct MainState {
    left_paddle_y: f32,
    right_paddle_y: f32,
    ball_position: Vec2,
    ball_velocity: Vec2,
    difficulty: Difficulty,
    game_state: GameState,
    menu_selection: usize,
    pause_selection: usize,
    left_paddle_dragging: bool,
    right_paddle_dragging: bool,
}

impl MainState {
    fn new() -> Self {
        MainState {
            left_paddle_y: WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            right_paddle_y: WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            ball_position: Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            ball_velocity: Vec2::new(0.0, 0.0),
            difficulty: Difficulty::Medium,
            game_state: GameState::Home,
            menu_selection: 1,
            pause_selection: 0,
            left_paddle_dragging: false,
            right_paddle_dragging: false,
        }
    }

    fn start_game(&mut self) {
        self.ball_velocity = match self.difficulty {
            Difficulty::Easy => Vec2::new(2.0, 2.0),
            Difficulty::Medium => Vec2::new(4.0, 4.0),
            Difficulty::Hard => Vec2::new(6.0, 6.0),
        };
        self.game_state = GameState::Playing;
    }

    fn restart_game(&mut self) {
        self.left_paddle_y = WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0;
        self.right_paddle_y = WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0;
        self.ball_position = Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        self.start_game();
    }

    fn stop_game(&mut self) {
        self.game_state = GameState::Menu;
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if let GameState::Playing = self.game_state {
            // Update ball position
            self.ball_position += self.ball_velocity;

            // Check ball collision with top and bottom walls
            if self.ball_position.y - BALL_RADIUS < 0.0 || self.ball_position.y + BALL_RADIUS > WINDOW_HEIGHT {
                self.ball_velocity.y = -self.ball_velocity.y;
            }

            // Check ball collision with paddles
            let left_paddle_rect = graphics::Rect::new(0.0, self.left_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT);
            let right_paddle_rect = graphics::Rect::new(WINDOW_WIDTH - PADDLE_WIDTH, self.right_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT);
            if self.ball_position.x - BALL_RADIUS < PADDLE_WIDTH && left_paddle_rect.contains(Point2 { x: self.ball_position.x, y: self.ball_position.y }) {
                self.ball_velocity.x = -self.ball_velocity.x;
            }
            if self.ball_position.x + BALL_RADIUS > WINDOW_WIDTH - PADDLE_WIDTH && right_paddle_rect.contains(Point2 { x: self.ball_position.x, y: self.ball_position.y }) {
                self.ball_velocity.x = -self.ball_velocity.x;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        match self.game_state {
            GameState::Home => {
                // Draw home screen
                let title = Text::new(TextFragment::new("Welcome to Pong Game").color(Color::GREEN).scale(50.0));
                canvas.draw(&title, DrawParam::default().dest(Vec2::new(100.0, 100.0)));

                let instructions = Text::new(TextFragment::new("Press Enter or Click to go to the Menu").color(Color::WHITE).scale(20.0));
                canvas.draw(&instructions, DrawParam::default().dest(Vec2::new(150.0, 400.0)));
            }
            GameState::Menu => {
                // Draw title
                let title = Text::new(TextFragment::new("Pong Game").color(Color::WHITE).scale(50.0));
                canvas.draw(&title, DrawParam::default().dest(Vec2::new(250.0, 100.0)));

                // Draw menu text
                let menu_texts = [
                    ("Easy", self.menu_selection == 0),
                    ("Medium", self.menu_selection == 1),
                    ("Hard", self.menu_selection == 2),
                    ("Back to Home", self.menu_selection == 3),
                ];
                let mut y = 200.0;
                for (text, selected) in &menu_texts {
                    let color = if *selected { Color::YELLOW } else { Color::WHITE };
                    let menu_text = Text::new(TextFragment::new(text.to_string()).color(color).scale(30.0));
                    canvas.draw(&menu_text, DrawParam::default().dest(Vec2::new(300.0, y)));
                    y += 50.0;
                }

                // Draw instructions
                let instructions = Text::new(TextFragment::new("Use W/S or Up/Down to navigate, Enter or Click to select").color(Color::WHITE).scale(20.0));
                canvas.draw(&instructions, DrawParam::default().dest(Vec2::new(100.0, 400.0)));
            }
            GameState::Playing | GameState::Paused => {
                // Draw paddles
                let paddle_color = Color::WHITE;
                let left_paddle_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(0.0, self.left_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT), paddle_color)?;
                let right_paddle_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(WINDOW_WIDTH - PADDLE_WIDTH, self.right_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT), paddle_color)?;
                canvas.draw(&left_paddle_mesh, DrawParam::default());
                canvas.draw(&right_paddle_mesh, DrawParam::default());

                // Draw ball
                let ball_mesh = Mesh::new_circle(ctx, graphics::DrawMode::fill(), Point2 { x: self.ball_position.x, y: self.ball_position.y }, BALL_RADIUS, 2.0, Color::GREEN)?;
                canvas.draw(&ball_mesh, DrawParam::default());

                // Draw help text
                let help_text = Text::new(TextFragment::new("Controls:\nLeft Paddle: W/S or drag with mouse\nRight Paddle: Up/Down Arrows or drag with mouse\nPress P to Pause").color(Color::WHITE).scale(20.0));
                canvas.draw(&help_text, DrawParam::default().dest(Vec2::new(10.0, 10.0)));

                if self.game_state == GameState::Paused {
                    let pause_menu_texts = [
                        ("Resume", self.pause_selection == 0),
                        ("Restart", self.pause_selection == 1),
                        ("Stop Game", self.pause_selection == 2),
                        ("Back to Home", self.pause_selection == 3),
                    ];
                    let mut y = 200.0;
                    for (text, selected) in &pause_menu_texts {
                        let color = if *selected { Color::YELLOW } else { Color::WHITE };
                        let menu_text = Text::new(TextFragment::new(text.to_string()).color(color).scale(30.0));
                        canvas.draw(&menu_text, DrawParam::default().dest(Vec2::new(300.0, y)));
                        y += 50.0;
                    }

                    // Draw instructions
                    let instructions = Text::new(TextFragment::new("Use W/S or Up/Down to navigate, Enter or Click to select").color(Color::WHITE).scale(20.0));
                    canvas.draw(&instructions, DrawParam::default().dest(Vec2::new(100.0, 400.0)));
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _repeat: bool) {
        match self.game_state {
            GameState::Home => {
                if keycode == KeyCode::Return {
                    self.game_state = GameState::Menu;
                }
            }
            GameState::Menu => {
                match keycode {
                    KeyCode::W | KeyCode::Up => {
                        if self.menu_selection > 0 {
                            self.menu_selection -= 1;
                        }
                    }
                    KeyCode::S | KeyCode::Down => {
                        if self.menu_selection < 3 {
                            self.menu_selection += 1;
                        }
                    }
                    KeyCode::Return => {
                        match self.menu_selection {
                            0 => {
                                self.difficulty = Difficulty::Easy;
                                self.start_game();
                            }
                            1 => {
                                self.difficulty = Difficulty::Medium;
                                self.start_game();
                            }
                            2 => {
                                self.difficulty = Difficulty::Hard;
                                self.start_game();
                            }
                            3 => self.game_state = GameState::Home,
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            GameState::Playing => {
                match keycode {
                    KeyCode::W => {
                        if self.left_paddle_y > 0.0 {
                            self.left_paddle_y -= PADDLE_SPEED;
                        }
                    }
                    KeyCode::S => {
                        if self.left_paddle_y + PADDLE_HEIGHT < WINDOW_HEIGHT {
                            self.left_paddle_y += PADDLE_SPEED;
                        }
                    }
                    KeyCode::Up => {
                        if self.right_paddle_y > 0.0 {
                            self.right_paddle_y -= PADDLE_SPEED;
                        }
                    }
                    KeyCode::Down => {
                        if self.right_paddle_y + PADDLE_HEIGHT < WINDOW_HEIGHT {
                            self.right_paddle_y += PADDLE_SPEED;
                        }
                    }
                    KeyCode::P => {
                        self.game_state = GameState::Paused;
                    }
                    _ => (),
                }
            }
            GameState::Paused => {
                match keycode {
                    KeyCode::W | KeyCode::Up => {
                        if self.pause_selection > 0 {
                            self.pause_selection -= 1;
                        }
                    }
                    KeyCode::S | KeyCode::Down => {
                        if self.pause_selection < 3 {
                            self.pause_selection += 1;
                        }
                    }
                    KeyCode::Return => {
                        match self.pause_selection {
                            0 => self.game_state = GameState::Playing,
                            1 => self.restart_game(),
                            2 => self.stop_game(),
                            3 => self.game_state = GameState::Home,
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            match self.game_state {
                GameState::Home => {
                    self.game_state = GameState::Menu;
                }
                GameState::Menu => {
                    let menu_options = [
                        (300.0, 200.0),
                        (300.0, 250.0),
                        (300.0, 300.0),
                        (300.0, 350.0),
                    ];
                    for (i, &(mx, my)) in menu_options.iter().enumerate() {
                        if x >= mx && x <= mx + 200.0 && y >= my && y <= my + 50.0 {
                            self.menu_selection = i;
                            self.key_down_event(ctx, KeyCode::Return, false);
                            break;
                        }
                    }
                }
                GameState::Paused => {
                    let pause_options = [
                        (300.0, 200.0),
                        (300.0, 250.0),
                        (300.0, 300.0),
                        (300.0, 350.0),
                    ];
                    for (i, &(px, py)) in pause_options.iter().enumerate() {
                        if x >= px && x <= px + 200.0 && y >= py && y <= py + 50.0 {
                            self.pause_selection = i;
                            self.key_down_event(ctx, KeyCode::Return, false);
                            break;
                        }
                    }
                }
                GameState::Playing => {
                    let left_paddle_rect = graphics::Rect::new(0.0, self.left_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT);
                    let right_paddle_rect = graphics::Rect::new(WINDOW_WIDTH - PADDLE_WIDTH, self.right_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT);
                    if left_paddle_rect.contains(Point2 { x, y }) {
                        self.left_paddle_dragging = true;
                    } else if right_paddle_rect.contains(Point2 { x, y }) {
                        self.right_paddle_dragging = true;
                    }
                }
            }
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.left_paddle_dragging = false;
            self.right_paddle_dragging = false;
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, y: f32, _dx: f32, _dy: f32) {
        if self.left_paddle_dragging {
            self.left_paddle_y = y.clamp(0.0, WINDOW_HEIGHT - PADDLE_HEIGHT);
        } else if self.right_paddle_dragging {
            self.right_paddle_y = y.clamp(0.0, WINDOW_HEIGHT - PADDLE_HEIGHT);
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult<()> {
        if let Some(keycode) = input.keycode {
            self.key_down_event(ctx, keycode, _repeat);
        }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<()> {
        self.mouse_button_down_event(ctx, button, x, y);
        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<()> {
        self.mouse_button_up_event(ctx, button, x, y);
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) -> GameResult<()> {
        self.mouse_motion_event(ctx, x, y, dx, dy);
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("pong", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    let state = MainState::new();
    event::run(ctx, event_loop, state)
}
