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

const PARTICLES_RADIUS: f32 = 5.;
const WINDOW_WIDTH: f32 = 1200.;
const WINDOW_HEIGHT: f32 = 900.;
const NUMBER_OF_THREADS: usize = 50;
const CELL_SIZE: f32 = PARTICLES_RADIUS * 5.;
const GRAVITY: Vec2 = Vec2::new(0., 300.);
const SUB_STEPS: usize = 16;

struct MainState {
    positions: Arc<RwLock<Vec<Vec2>>>,
    prev_positions: Vec<Vec2>,
    grid: Arc<RwLock<Grid>>,
    num_part_to_spawn : usize,
}

impl MainState {
    fn new(ctx: &Context) -> GameResult<MainState> {
        let s = MainState{
            positions: Arc::new(RwLock::new(Vec::new())),
            prev_positions: Vec::new(),
            grid: Arc::new(RwLock::new(Grid::new(ctx.gfx.window().inner_size().width as f32, ctx.gfx.window().inner_size().height as f32, CELL_SIZE),)),
            num_part_to_spawn: 1,
        };
        Ok(s)
    }

    fn add_particles(&mut self, position: Vec2, number: usize){
        for i in 0..number{
            self.positions.write().unwrap().push(position + Vec2::new(0., PARTICLES_RADIUS * 5. * (i as f32)));
            self.prev_positions.push(position + Vec2::new(-3., -0.5) + Vec2::new(0., PARTICLES_RADIUS * 5. * (i as f32)));
        }        
    }

    fn update_positions(&mut self, dt: f32){
        let mut positions = self.positions.write().unwrap();
        for i in 0..self.prev_positions.len(){
            let velocity = (positions[i] - self.prev_positions[i]).clamp_length_max(1.5) ;
            self.prev_positions[i] = positions[i];
            positions[i] += velocity + (GRAVITY - 0.01* velocity) * dt * dt;
        }        
    }

    fn collisions(&mut self){
        let (tx, rx) = mpsc::channel();
        let mut ranges = gen_vec_range(NUMBER_OF_THREADS, self.grid.read().unwrap().get_num_columns());

        for _ in 0..NUMBER_OF_THREADS{
            let thread_tx = tx.clone();
            let grid_ref = Arc::clone(&self.grid);
            let positions_ref = Arc::clone(&self.positions);
            let thread_range = ranges.pop().unwrap();
            thread::spawn(move || {
                let grid = grid_ref.read().unwrap();
                let positions = positions_ref.read().unwrap();
                thread_range.for_each(|i| (1..(grid.get_num_rows() - 1)).for_each(|j| (0..3).for_each(|di| (0..3).for_each(|dj| {
                    for id1 in grid.get_cell(i, j){
                        for id2 in grid.get_cell(i + di - 1, j + dj - 1){ 
                            let collision_vector = positions[*id1] - positions[*id2];
                            let distance = collision_vector.length();
                            if distance < 2.*PARTICLES_RADIUS && id1 != id2{
                                let delta = (2.*PARTICLES_RADIUS - distance) * 0.25; // 0.25 because divided by two to move the two particles equally 
                                let correction_vector = collision_vector/distance * delta; //and divided again bay two because the collision will be computed twice 
                                thread_tx.send((*id1, *id2, correction_vector)).unwrap();
                            }
                        }
                    }
                }))));
                
            });
        }
        drop(tx);
        let mut new_positions = self.positions.read().unwrap().clone();
        for (id1, id2, correction_vector) in rx{
            new_positions[id1] +=  correction_vector;
            new_positions[id2] -=  correction_vector;
        }


        *self.positions.write().unwrap() = new_positions;   
    }

    fn constraint(&mut self, win_width: f32, win_height: f32){
        let mut positions = self.positions.write().unwrap();

        for i in 0..self.prev_positions.len(){     
            if positions[i].x - PARTICLES_RADIUS < 0.{
                positions[i].x = PARTICLES_RADIUS;
            } else if positions[i].x  + PARTICLES_RADIUS > win_width{
                positions[i].x = win_width - PARTICLES_RADIUS;
            };

            if positions[i].y - PARTICLES_RADIUS < 0.{
                positions[i].y = PARTICLES_RADIUS;
            }else if positions[i].y + PARTICLES_RADIUS > win_height{
                positions[i].y = win_height - PARTICLES_RADIUS;
            };
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let win_width = ctx.gfx.window().inner_size().width as f32;
        let win_height = ctx.gfx.window().inner_size().height as f32;  
        let sub_dt = ctx.time.delta().as_secs_f32()/(SUB_STEPS as f32);
        for _ in 0..SUB_STEPS{           
            self.update_positions(sub_dt);            
            self.constraint(win_width, win_height);
            self.grid.write().unwrap().update(&self.positions.read().unwrap());
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
            PARTICLES_RADIUS,
            0.1,
            Color::WHITE,
        )?;
        self.positions.read().unwrap().iter().for_each(|position| canvas.draw(&circle, *position));

        canvas.draw(graphics::Text::new(format!("FPS : {:.0}, number of balls : {} ",ctx.time.fps(), self.prev_positions.len()) )
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
                self.add_particles(pos_win_from_rel( ctx, 0.1, 0.1),self.num_part_to_spawn);
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
    let cb = ggez::ContextBuilder::new("granular simulation ", "Peyrie Pierre-Angelo")
      .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(&ctx)?;
    event::run(ctx, event_loop, state)
}