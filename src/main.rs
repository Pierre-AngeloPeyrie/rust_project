use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
use ggez::input::keyboard::{KeyCode,KeyInput};

struct Particle{
    position: Vec2,
    prev_position: Vec2,
    acceleration: Vec2
}

impl Particle{
    fn new(position: Vec2) -> Self{
        Particle{ 
            position, 
            prev_position: position + Vec2::new(-1., 0.), 
            acceleration: Vec2::ZERO
        }
    }

    fn update(&mut self, mut dt: f32){
        dt = if dt > 0.1 {0.0001}else {dt};
        let velocity = self.position - self.prev_position;
        self.prev_position = self.position;
        self.position += velocity + self.acceleration * dt * dt;
        self.acceleration = Vec2::ZERO;
        
    }

    fn constraint(&mut self, win_width: f32, win_height: f32){
        let velocity = self.position - self.prev_position;

        if self.position.x - 50. < 0.{
            self.position.x = 50.;
            self.prev_position.x = self.position.x + velocity.x;

        } else if self.position.x  + 50. > win_width{
            self.position.x = win_width - 50.;
            self.prev_position.x = self.position.x + velocity.x;
        };

        if self.position.y - 50. < 0.{
            self.position.y = 50.;
            self.prev_position.y = self.position.y + velocity.y;

        }else if self.position.y + 50. > win_height{
            self.position.y = win_height - 50.;
            self.prev_position.y = self.position.y + velocity.y;
        };
    
    }
    
    fn accelerate(&mut self, acc: Vec2){
        self.acceleration += acc;
    }
}

struct MainState {
    objects: Vec<Particle>,
    gravity: Vec2,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState{
            objects: Vec::new(),
            gravity: Vec2::new(0., 300.)
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let win_width = ctx.gfx.window().inner_size().width as f32;
        let win_height = ctx.gfx.window().inner_size().height as f32;  

        self.objects.iter_mut().for_each(|particle| particle.accelerate(self.gravity));
        self.objects.iter_mut().for_each(|particle| particle.update(ctx.time.delta().as_secs_f32()));
        self.objects.iter_mut().for_each(|particle| particle.constraint(win_width, win_height));

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
            50.0,
            0.1,
            Color::WHITE,
        )?;
        self.objects.iter_mut().for_each(|particle| canvas.draw(&circle, particle.position));

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _b: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Escape) =>{
                ctx.request_quit();
                Ok(())
            },
            Some(KeyCode::Space) =>{
                self.objects.push(Particle::new(Vec2::new(400., 100.)));
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