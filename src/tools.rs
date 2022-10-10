use crate::canvas::{Canvas, screen_to_canvas};

pub enum ToolType {
    Pen,
    Ereaser
}

#[derive(PartialEq)]
pub enum ToolState {
    Up,
    Down
}

pub struct Tool{
    pub mode: ToolType,
    pub size: usize,
    pub state: ToolState,
    pub position: (u32, u32)
}

impl Tool{

    pub fn new() -> Self {
        Tool{
            mode: ToolType::Pen,
            size: 5,
            state: ToolState::Up,
            position: (0,0)
        }
    }

}

pub fn draw(canvas: &mut Canvas, tool: &Tool, tool_size: i32, pos: (i32, i32)){
    // get all pixels inside a circle with r = tool_size and center = pos
    // make sure you dont overflow in width to prevent wraparound
    // make sure to prevent overflow of index
    // set pixel for all valid target pixels 

    // Version 1: https://stackoverflow.com/questions/14487322/get-all-pixel-array-inside-circle
    // geht bestimmt besser aber besser als nix

    // calculate bounding box around circle

    // store the indices of the correct pixels within canvas_data
    let mut bounding_box: Vec<usize> = Vec::new();

    for x_offset in -tool_size..tool_size {
        for y_offset in -tool_size..tool_size {
            let x = (pos.0 + x_offset) as u32;
            let y = (pos.1 + y_offset) as u32;

            bounding_box.push(screen_to_canvas(x, y, canvas));
        }
    }

    for index in bounding_box {
        //let canvas_pos = screen_to_canvas(pos.0, pos.1, canvas);
        match tool.mode {
            ToolType::Pen => {
                canvas.set_pixel(index, 0x00000000);
            },
            ToolType::Ereaser => {
                canvas.set_pixel(index, 0xFFFFFFFF);
            },
        }
    }
}