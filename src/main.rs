use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
use ggez::input::keyboard::{KeyCode,KeyInput};

struct MainState {
    positions: Vec<Vec2>,
    prev_positions: Vec<Vec2>,
    gravity: Vec2,
    space_center: Vec2,
    space_radius: f32,
    particle_radius: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState{
            positions: Vec::new(),
            prev_positions: Vec::new(),
            gravity: Vec2::new(0., 300.),
            space_center: Vec2::new(400., 300.),
            space_radius: 290.,
            particle_radius: 20.,

        };
        Ok(s)
    }

    fn add_particle(&mut self, position: Vec2){
        self.positions.push(position);
        self.prev_positions.push(position);
    }

    fn update_positions(&mut self, dt: f32){
        for i in 0..self.positions.len(){
            let velocity = self.positions[i] - self.prev_positions[i];
            self.prev_positions[i] = self.positions[i];
            self.positions[i] += velocity + self.gravity * dt * dt;
        }        
    }

    fn collisions(&mut self){
        for i in 0..self.positions.len(){
            for j in i+1..self.positions.len(){
                let collision_vector = self.positions[i] - self.positions[j];
                let distance = collision_vector.length();
                if distance < 2. * self.particle_radius{
                    let delta = 2. * self.particle_radius - distance;
                    let n = collision_vector/distance;
                    self.positions[i] += 0.5 * delta * n;
                    self.positions[j] -= 0.5 * delta * n;
                }
            }
        }
    }

    fn constraint(&mut self){
        for i in 0..self.positions.len(){
            let center_to_part= self.positions[i] - self.space_center;
            let distance = center_to_part.length();
            if distance > self.space_radius - self.particle_radius {
                self.positions[i] = self.space_center + center_to_part/distance * (self.space_radius - self.particle_radius)
            }   
        }
        
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let sub_steps = 4;
        for _ in 0..sub_steps{
            self.update_positions(ctx.time.delta().as_secs_f32()/sub_steps as f32);
            self.constraint();
            self.collisions();
        }
        

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );
        let space_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.space_radius,
            0.1,
            Color::BLACK,
        )?;

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.particle_radius,
            0.1,
            Color::WHITE,
        )?;
        canvas.draw(&space_mesh, self.space_center);
        self.positions.iter().for_each(|position| canvas.draw(&circle, *position));

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
                self.add_particle(Vec2::new(600., 200.));
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