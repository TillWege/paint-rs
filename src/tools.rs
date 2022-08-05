use crate::canvas::{Canvas, screen_to_canvas};

pub enum Tool{
    Pen,
    Ereaser
}

pub fn draw(canvas: &mut Canvas, tool: &Tool, pos: (u32, u32)){

    let canvas_pos = screen_to_canvas(pos.0, pos.1, canvas);
    match tool {
        Tool::Pen => {
            canvas.set_pixel(canvas_pos, 0x00000000);
        },
        Tool::Ereaser => {
            canvas.set_pixel(canvas_pos, 0xFFFFFFFF);
        },
    }
}