use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color};

struct MainState {
    pos_x: f32,
    pos_y: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { pos_x: 100.0, pos_y: 100.0 };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update game logic here
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.pos_x, self.pos_y],
            50.0,
            2.0,
            Color::from_rgb(255, 0, 0),
        )?;
        graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => self.pos_y -= 10.0,
            KeyCode::Down => self.pos_y += 10.0,
            KeyCode::Left => self.pos_x -= 10.0,
            KeyCode::Right => self.pos_x += 10.0,
            _ => (),
        }
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("sprite_game", "SolutionsAsService");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}