use ggez::{glam::Vec2, Context};
use std::ops::Range;

    
pub fn pos_win_from_rel(context: &mut Context, rw: f32, rh: f32) -> Vec2 {
    Vec2::new(context.gfx.window().inner_size().width as f32 * rw , context.gfx.window().inner_size().height as f32 * rh)
}


pub fn gen_vec_range(n_part: usize, size: usize)->Vec<Range<usize>>{
   let mut res = vec![1..size/n_part];
   (1..n_part-1).for_each(|i| res.push(size/n_part*i..size/n_part*(i+1)));
   res.push(size/n_part*(n_part-1)..(size-1));
   
   res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_range() {
        let ranges: Vec<Range<usize>> = gen_vec_range(3, 100);
        println!("{:?}",ranges);
        assert_eq!(ranges,vec![1..33, 33..66, 66..99]);
    }
}
