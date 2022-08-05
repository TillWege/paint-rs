use std::cmp::{max, min};

pub struct Canvas {
    height: u32,
    width: u32,
    data: Vec<u32>
}

impl Canvas {
    
    pub fn new(p_height: u32, p_width: u32) -> Self {
        Self {
            width: p_width,
            height: p_height,
            data: vec![0xFFFFFFFF; (p_height * p_width) as usize]
        }
    }

    fn update(&mut self) {
        // todo
    }

    pub fn canvas_to_frame(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as u32;
            let y = (i / self.width as usize) as u32;

            let index: usize = (x + (y * self.width)) as usize;

            let px = self.data[index];

            let b1 : u8 = ((px >> 24) & 0xff) as u8;
            let b2 : u8 = ((px >> 16) & 0xff) as u8;
            let b3 : u8 = ((px >> 8) & 0xff) as u8;
            let b4 : u8 = (px & 0xff) as u8;

            pixel.copy_from_slice(&[b1, b2, b3, b4]);
        }
    }

    pub fn set_pixel(&mut self, index: u32, clr: u32){
        self.data[index as usize] = clr;
    }
}

pub fn screen_to_canvas(x: u32, y: u32, canvas: &Canvas) -> u32 {
    // really sketchy when resized lol
    let x_index = x;
    let y_index = y * canvas.width;
    let res = x_index + y_index;
    return min(res, (canvas.data.len() - 1) as u32);
}