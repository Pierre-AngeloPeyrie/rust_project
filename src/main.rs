use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
use ggez::input::keyboard::{KeyCode,KeyInput};

struct PhysicsObject{
    position: Vec2,
    prev_position: Vec2,
    acceleration: Vec2
}

impl PhysicsObject{
    fn new(position: Vec2) -> Self{
        PhysicsObject{ 
            position, 
            prev_position: position, 
            acceleration: Vec2::ZERO
        }
    }

    fn update(&mut self, dt: f32){
        let velocity = self.position - self.prev_position;
        self.prev_position = self.position;
        self.position = self.position + velocity + self.acceleration * dt * dt;
        self.acceleration = Vec2::ZERO;
        dbg!(velocity.y);
    }
    fn accelerate(&mut self, acc: Vec2){
        self.acceleration += acc;
    }
}



struct MainState {
    object: PhysicsObject,
    gravity: Vec2,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState{
            object: PhysicsObject::new(Vec2::new(400., 100.)),
            gravity: Vec2::new(0., 10.)
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.object.accelerate(self.gravity);
        self.object.update(ctx.time.delta().as_secs_f32());
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            0.1,
            Color::WHITE,
        )?;
        canvas.draw(&circle, self.object.position);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _b: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Escape) =>{
                ctx.request_quit();
                Ok(())
            },
            _ => Ok(()), // Do nothing
        }
    }

}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}