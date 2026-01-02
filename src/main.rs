use ggez::event;
use ggez::graphics::{self, Color, DrawParam};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode,KeyInput};

mod spacial_partition;
use spacial_partition::Grid;
mod circular_buffer;
use circular_buffer::CircularBuffer;
mod misc;
use misc::pos_win_from_rel;

struct MainState {
    positions: Vec<Vec2>,
    prev_positions: Vec<Vec2>,
    gravity: Vec2,
    particle_radius: f32,
    fps_buffer: CircularBuffer,
    grid: Grid,
}

impl MainState {
    fn new(ctx: &Context, part_radius: f32) -> GameResult<MainState> {
        let s = MainState{
            positions: Vec::new(),
            prev_positions: Vec::new(),
            gravity: Vec2::new(0., 300.),
            particle_radius: part_radius,
            fps_buffer: CircularBuffer::new(),
            grid: Grid::new(ctx.gfx.window().inner_size().width as f32, ctx.gfx.window().inner_size().height as f32, part_radius*2.),
        };
        Ok(s)
    }

    fn add_particle(&mut self, position: Vec2){
        self.positions.push(position);
        self.prev_positions.push(position + Vec2::new(-1., -0.5));
        self.positions.push(position + Vec2::new(0., self.particle_radius * 2.));
        self.prev_positions.push(position + Vec2::new(-1., -0.5)+ Vec2::new(0., self.particle_radius * 2.));
    }

    fn update_positions(&mut self, dt: f32){
        for i in 0..self.positions.len(){
            let velocity = self.positions[i] - self.prev_positions[i];
            self.prev_positions[i] = self.positions[i];
            self.positions[i] += velocity + self.gravity * dt * dt;
        }        
    }

    fn collisions(&mut self){
        (1..(self.grid.get_num_columns() - 1)).for_each(|i| (1..(self.grid.get_num_rows() - 1)).for_each(|j| (0..3).for_each(|di| (0..3).for_each(|dj| {
        for id1 in self.grid.get_cell(i, j){
            for id2 in self.grid.get_cell(i + di - 1, j + dj - 1){ 
                let collision_vector = self.positions[*id1] - self.positions[*id2];
                let distance = collision_vector.length();
                if distance < 2. * self.particle_radius && id1 != id2{
                    let delta = 2. * self.particle_radius - distance;
                    let n = collision_vector/distance;
                    self.positions[*id1] += 0.5 * delta * n;
                    self.positions[*id2] -= 0.5 * delta * n;
                }
            }
        }
        }))));
    }

    fn constraint(&mut self, win_width: f32, win_height: f32){
        for i in 0..self.positions.len(){     
            if self.positions[i].x - self.particle_radius < 0.{
                self.positions[i].x = self.particle_radius;
            } else if self.positions[i].x  + self.particle_radius > win_width{
                self.positions[i].x = win_width - self.particle_radius;
            };

            if self.positions[i].y - self.particle_radius < 0.{
                self.positions[i].y = self.particle_radius;
            }else if self.positions[i].y + self.particle_radius > win_height{
                self.positions[i].y = win_height - self.particle_radius;
            };
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let win_width = ctx.gfx.window().inner_size().width as f32;
        let win_height = ctx.gfx.window().inner_size().height as f32;  
        let sub_steps = 2;
        for _ in 0..sub_steps{
            self.grid.update(&self.positions);
            self.update_positions(ctx.time.delta().as_secs_f32()/sub_steps as f32);
            self.constraint(win_width, win_height);
            self.collisions();
        }
        self.fps_buffer.push(1./ctx.time.delta().as_secs_f32());
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
            self.particle_radius,
            0.1,
            Color::WHITE,
        )?;
        self.positions.iter().for_each(|position| canvas.draw(&circle, *position));

        canvas.draw(graphics::Text::new(format!("FPS : {:.0}, number of balls : {} ",self.fps_buffer.mean(), self.positions.len()) )
              .set_scale(20.),
              DrawParam::default()
              .dest(pos_win_from_rel(ctx, 0.01, 0.01))
            );


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
                self.add_particle(Vec2::new(100., 100.));
                Ok(())
            },
            _ => Ok(()), // Do nothing
        }
    }

}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(&ctx,3.)?;
    event::run(ctx, event_loop, state)
}