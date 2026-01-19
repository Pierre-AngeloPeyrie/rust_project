use ggez::glam::Vec2;

pub struct Grid{
    columns: Vec<Vec<Vec<usize>>>,
    cell_size: f32,
}

impl Grid {
    pub fn new(width: f32, height: f32,cell_size: f32 ) -> Grid{
        Grid{ 
            columns: vec![vec![Vec::new(); (height / cell_size).ceil() as usize + 2 ]; (width  / cell_size).ceil() as usize + 2 ], 
            cell_size,           
        }
    }

    pub fn get_num_columns(&self) -> usize{
        self.columns.len()
    }
    pub fn get_num_rows(&self) -> usize{
        self.columns[0].len()
    }

    pub fn get_cell(&self,i: usize,j: usize) -> &Vec<usize>{
        &self.columns[i][j]
    }
    pub fn update(&mut self, entities: &Vec<Vec2>,){
        self.columns.iter_mut().flatten().for_each(|cell| cell.clear()); 

        for i in 0..entities.len(){
            let col = (entities[i].x / self.cell_size) as usize + 1;
            let row  = (entities[i].y / self.cell_size) as usize + 1;
            if col < self.get_num_columns() && row < self.get_num_rows(){
                self.columns[col][row].push(i);
            }
            
        };   
    }  
}