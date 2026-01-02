pub struct CircularBuffer{
    buffer: [f32;20],
    next: usize
}

impl CircularBuffer{
    pub fn new() -> CircularBuffer{
    	CircularBuffer {
    	    buffer: [0.;20],
    	    next: 0
    	}
    }
    
    pub fn push(&mut self, value: f32){
    	self.buffer[self.next] = value;
    	self.next =  (self.next + 1) % 20;
    }
    
    pub fn mean(& self) -> f32 {
    	self.buffer.into_iter().sum::<f32>()/20.
    }
}
