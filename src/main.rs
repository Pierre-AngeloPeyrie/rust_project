use ggez::event;
use ggez::graphics::{self, Color, DrawParam};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode,KeyInput};

mod spacial_partition;
use spacial_partition::Grid;
mod misc;
use misc::{pos_win_from_rel,gen_vec_range};

use std::sync::{Arc,RwLock,mpsc};
use std::thread;
//use std::time::Instant;

struct MainState {
    positions: Arc<RwLock<Vec<Vec2>>>,
    velocity: Vec<Vec2>,
    gravity: Vec2,
    particle_radius: f32,
    grid: Arc<RwLock<Grid>>,
    num_part_to_spawn : usize,
}

impl MainState {
    fn new(ctx: &Context, part_radius: f32) -> GameResult<MainState> {
        let s = MainState{
            positions: Arc::new(RwLock::new(Vec::new())),
            velocity: Vec::new(),
            gravity: Vec2::new(0., 300.),
            particle_radius: part_radius,
            grid: Arc::new(RwLock::new(Grid::new(ctx.gfx.window().inner_size().width as f32, ctx.gfx.window().inner_size().height as f32, part_radius*2.),)),
            num_part_to_spawn: 1,
        };
        Ok(s)
    }

    fn add_particle(&mut self, position: Vec2, number: usize){
        for i in 0..number{
            self.positions.write().unwrap().push(position + Vec2::new(0., self.particle_radius * 5. * (i as f32)));
            self.velocity.push(Vec2::new(300., 400.));
        }        
    }

    fn update_positions(&mut self, dt: f32){
        let mut positions = self.positions.write().unwrap();
        for i in 0..self.velocity.len(){
            self.velocity[i] += self.gravity * dt;
            positions[i] += self.velocity[i] * dt;
        }        
    }

    fn collisions(&mut self){
        let (tx, rx) = mpsc::channel();
        let n_threads = 10;
        let mut ranges = gen_vec_range(n_threads, self.grid.read().unwrap().get_num_columns());
        for _ in 0..n_threads{
            let thread_tx = tx.clone();
            let radius = self.particle_radius;
            let grid_ref = Arc::clone(&self.grid);
            let positions_ref = Arc::clone(&self.positions);
            let thread_range = ranges.pop().unwrap();
            thread::spawn(move || {
                //let start = Instant::now();
                let eps = 0.0001;
                let grid = grid_ref.read().unwrap();
                let positions = positions_ref.read().unwrap();
                thread_range.for_each(|i| (1..(grid.get_num_rows() - 1)).for_each(|j| (0..3).for_each(|di| (0..3).for_each(|dj| {
                    for id1 in grid.get_cell(i, j){
                        for id2 in grid.get_cell(i + di - 1, j + dj - 1){ 
                            let collision_vector = positions[*id2] - positions[*id1];
                            let distance = collision_vector.length();
                            if id1 != id2 && distance > eps && distance < 2.*radius{ 
                                let delta = 2.*radius - distance;
                                let n = collision_vector.normalize() ;
                                thread_tx.send((*id1, *id2, delta, n)).unwrap();
                            }
                        }
                    }
                }))));
                //dbg!(start.elapsed());
            });
        }

        drop(tx);

        let positions = self.positions.read().unwrap();
        let mut new_positions = positions.clone();
        
        for (id1, id2, delta, n) in rx{
            if n.is_nan(){
                println!("qfvds");
            }
            
            let v1 = n.dot(self.velocity[id1]);
            let v2 =  n.dot(self.velocity[id2]);
            let prop = (v1/(v1-v2)).clamp(0., 1.);

            let momentum = v1 + v2;
            let energy = v1*v1 + v2*v2;
            let (b,c) = (
                -2. * momentum, 
                momentum*momentum - energy
            );
            
            let new_v2 = (-b + f32::sqrt((b*b - 8.*c).abs()))/4.;
            let new_v1 = momentum - new_v2;
        
            new_positions[id1] = positions[id1] - delta * n * 0.5;
            new_positions[id2] = positions[id2] + delta * n * 0.5;
            self.velocity[id1] += (new_v1 - v1) * n *0.5;
            self.velocity[id2] += (new_v2 - v2 )* n *0.5;
        }
        drop(positions);

        *self.positions.write().unwrap() = new_positions;   

    }

    fn constraint(&mut self, win_width: f32, win_height: f32){
        let mut positions = self.positions.write().unwrap();

        for i in 0..self.velocity.len(){     
            if positions[i].x - self.particle_radius < 0.{
                positions[i].x = self.particle_radius;
                self.velocity[i].x = -self.velocity[i].x *0.9;
            } else if positions[i].x  + self.particle_radius > win_width{
                positions[i].x = win_width - self.particle_radius;
                self.velocity[i].x = -self.velocity[i].x *0.9;
            };

            if positions[i].y - self.particle_radius < 0.{
                positions[i].y = self.particle_radius;
                self.velocity[i].y = -self.velocity[i].y *0.9;
            }else if positions[i].y + self.particle_radius > win_height{
                positions[i].y = win_height - self.particle_radius;
                self.velocity[i].y = -self.velocity[i].y *0.9;
            };
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let win_width = ctx.gfx.window().inner_size().width as f32;
        let win_height = ctx.gfx.window().inner_size().height as f32;  

        
        self.grid.write().unwrap().update(&self.positions.read().unwrap());
        
        
            
        
        self.update_positions(ctx.time.delta().as_secs_f32());   
        self.constraint(win_width, win_height);  
        for _ in 0..1{
            self.collisions();

        }
        
                 
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
        self.positions.read().unwrap().iter().for_each(|position| canvas.draw(&circle, *position));

        canvas.draw(graphics::Text::new(format!("FPS : {:.0}, number of balls : {} ",ctx.time.fps(), self.velocity.len()) )
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
                self.add_particle(pos_win_from_rel(ctx, 0.5, 0.2),self.num_part_to_spawn);
                Ok(())
            },
            Some(KeyCode::LControl) =>{
                self.num_part_to_spawn += 1;
                Ok(())
            },
            _ => Ok(()), // Do nothing
        }
    }

}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
                            .window_mode(ggez::conf::WindowMode::default().dimensions(50., 700.));
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(&ctx,20.)?;
    event::run(ctx, event_loop, state)
}