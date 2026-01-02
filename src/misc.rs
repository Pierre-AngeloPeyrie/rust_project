use ggez::{glam::Vec2, graphics::Color, Context};
use std::{f32::consts::PI, ops::Range};

    
pub fn pos_win_from_rel(context: &mut Context, rw: f32, rh: f32) -> Vec2 {
    Vec2::new(context.gfx.window().inner_size().width as f32 * rw , context.gfx.window().inner_size().height as f32 * rh)
}

pub fn angle_to_color(angle: f32)->Color{
    Color::new(channel_curve((angle + 2.*PI/3.)%(2.*PI)), channel_curve(angle % (2.*PI)), channel_curve((angle + 4.*PI/3.)%(2.*PI)), 1.)
    //let (r,g,b) = (keep_in_bound(hue + 0.3333), hue, keep_in_bound(hue - 0.3333));
    
}

fn channel_curve(angle: f32) -> f32{
    match angle {
        0.0..1.0471976 => angle / (PI/3.), // 0..PI/3
        1.0471976..3.1415927 => 1., // PI/3..PI
        3.1415927..4.1887903 => 1. - (angle-PI)/(PI/3.), // PI..4PI/3
        4.1887903..6.2831855 => 0., // 4PI/3..2PI
        _ => panic!("invalid angle value : {angle}")
    }
}

pub fn gen_vec_range(n_part: usize, size: usize)->Vec<Range<usize>>{
    let mut res = vec![1..size/n_part];
    if size % n_part == 0{
        (1..n_part).for_each(|i| res.push(size/n_part*i..size/n_part*(i+1)));
    }else{
        (1..n_part-1).for_each(|i| res.push(size/n_part*i..size/n_part*(i+1)));
        res.push(size/n_part*(n_part-1)..size);
        
    };
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_range() {
        println!("{:?}",gen_vec_range(2, 100));
        assert!(true);
    }
}
