use sfml::{graphics::{Rect, Sprite, Texture}, system::Vector2};

#[derive(Clone)]
pub enum AnimationType {
    Loop{rect_index_range: Option<(u32, u32)>},
    FiniteLoop{loops: u32, current_loop: u32, return_state: String, rect_index_range: Option<(u32, u32)>},
    Once{return_state: String, rect_index_range: Option<(u32, u32)>},
    Static,
}

#[derive(Clone)]
pub struct Animation{
    pub type_: AnimationType,
    pub textute_id: String,
    pub rect_size: Option<(i32, i32)>,
    pub frame_duration_ms: f32,
    pub current_total_duration: f32,
    pub current_frame: u32,

}



fn flip_rect(rect: Rect<i32>, flip_x: bool, flip_y: bool)->Rect<i32>{
    let mut new_rect = rect;
    if flip_x{
        new_rect.left += new_rect.width;
        new_rect.width = -new_rect.width;
    }
    if flip_y{
        new_rect.top += new_rect.height;
        new_rect.height = -new_rect.height;
    }
    new_rect
}

impl Animation{
    pub fn new_loop(texture_id: &str, frame_duration_ms: f32,rect_size:(i32,i32), rect_index_range: Option<(u32, u32)>) -> Self{
        Animation{
            type_: AnimationType::Loop{rect_index_range},
            textute_id: texture_id.to_string(),
            rect_size:Some(rect_size),
            frame_duration_ms,
            current_frame: 0,
            current_total_duration: 0.0,
        }
    }

    pub fn new_finite_loop(return_state: &str, texture_id: &str, frame_duration_ms: f32, loops: u32,rect_size:(i32,i32), rect_index_range: Option<(u32, u32)>) -> Self{
        Animation{
            type_: AnimationType::FiniteLoop{loops, current_loop: 0, return_state: return_state.to_string(), rect_index_range},
            textute_id: texture_id.to_string(),
            rect_size:Some(rect_size),
            frame_duration_ms,
            current_frame: 0,
            current_total_duration: 0.0,
        }
    }

    pub fn new_once(return_state: &str, texture_id:&str , frame_duration_ms: f32,rect_size:(i32,i32),  rect_index_range: Option<(u32, u32)>) -> Self{
        Animation{
            type_: AnimationType::Once{return_state: return_state.to_string(), rect_index_range},
            textute_id: texture_id.to_string(),
            rect_size:Some(rect_size),
            frame_duration_ms,
            current_frame: 0,
            current_total_duration: 0.0,
        }
    }

    pub fn new_static(texture_id: &str) -> Self{
        Animation{
            type_: AnimationType::Static,
            textute_id: texture_id.to_string(),
            rect_size:None,
            frame_duration_ms: 0.0,
            current_frame: 0,
            current_total_duration: 0.0,
        }
    }

    pub fn new_static_with_rect(texture_id: &str, rect_size:(i32,i32)) -> Self{
        Animation{
            type_: AnimationType::Static,
            textute_id: texture_id.to_string(),
            rect_size:Some(rect_size),
            frame_duration_ms: 0.0,
            current_frame: 0,
            current_total_duration: 0.0,
        }
    }

    pub fn set_rect_size(&mut self, rect_size: (i32, i32)){
        self.rect_size = Some(rect_size);
    }

    //----------------------------------------------
    pub fn set_rect_index_range(&mut self, rect_index_range: (u32, u32)){
        let correct_range = rect_index_range.0 < rect_index_range.1;
        let corrected_range = if correct_range {Some(rect_index_range)} else {Some((rect_index_range.1, rect_index_range.0))};

        match &mut self.type_{
            AnimationType::Loop{rect_index_range} => {
                *rect_index_range = corrected_range;
            },
            AnimationType::FiniteLoop{rect_index_range, ..} => {
                *rect_index_range = corrected_range;
            },
            AnimationType::Once{rect_index_range, ..} => {
                *rect_index_range = corrected_range;
            },
            _ => {}
        }
    }
    
    fn get_rect_index_range(&self)->Option<(u32, u32)>{
        match &self.type_{
            AnimationType::Loop{rect_index_range} => *rect_index_range,
            AnimationType::FiniteLoop{rect_index_range, ..} => *rect_index_range,
            AnimationType::Once{rect_index_range, ..} => *rect_index_range,
            _ => None,
        }
    }

    pub fn frame_index_mod(&self, frame_index:u32)->Option<u32>{
        if self.get_rect_index_range().is_none(){
            return None;
        }
        let (start, end) = self.get_rect_index_range().unwrap();
        Some(start + frame_index % (end - start + 1))
    }
    
    pub fn increment_duration(&mut self, duration: f32){
        self.current_total_duration += duration;
    }

    pub fn reset_duration(&mut self){
        self.current_total_duration = 0.0;
    }

    pub fn time_to_next_frame(&self)->bool{
        self.current_total_duration >= self.frame_duration_ms
    }


    // Returns true if modulated
    pub fn safe_increment_frame(&mut self, texture_size:Vector2<u32>)->bool{
        if let Some((start, end)) = self.get_rect_index_range(){
            if self.current_frame > (end-start){
                self.current_frame = 0;
                return true;
            }
            else{
                self.current_frame += 1;
                return false;
            }
        }
        else{
            let rect = self.rect_size.unwrap();
            let (rect_width, rect_height) = (rect.0, rect.1);
            let (texture_width, texture_height) = (texture_size.x, texture_size.y);
            let (rects_per_row, rects_per_column) = (texture_width as i32 / rect_width, texture_height as i32 / rect_height);
            if self.current_frame >= (rects_per_row as i32* rects_per_column -1) as u32{
                self.current_frame = 0;
                return true;
            }
            else{
                self.current_frame += 1;
                return false;
            }
        }
    }

    pub fn increment_frame(&mut self){
        self.current_frame += 1;
    }

    pub fn get_current_rect(&self, texture_size:Vector2<u32>)->Option<Rect<i32>>{
        self.get_rect(self.current_frame, texture_size)
    }
    
    pub fn get_current_rect_flipped(&self, texture_size:Vector2<u32>, flip_x: bool, flip_y: bool)->Option<Rect<i32>>{
        let rect = self.get_current_rect(texture_size);
        if rect.is_none(){
            return None;
        }
        Some(flip_rect(rect.unwrap(), flip_x, flip_y))
    }

    fn get_rect(&self, frame_index:u32, texture_size:Vector2<u32>)->Option<Rect<i32>>{

       if self.rect_size.is_none(){
           return None;
       }

        let rect_size = self.rect_size.unwrap();
        let (rect_width, rect_height) = (rect_size.0, rect_size.1);
        let (texture_width, texture_height) = (texture_size.x as i32, texture_size.y as i32);
        let (rects_per_row, rects_per_column) = (texture_width / rect_width, texture_height / rect_height);
        
        if self.get_rect_index_range().is_none(){
            let frame_index = frame_index as i32;
            let left = (frame_index % rects_per_row as i32) * rect_width ;
            let top = (frame_index / rects_per_row as i32) * rect_height;
            Some(Rect::new(left as i32, top as i32, rect_width as i32, rect_height as i32))
        }
        else{
            let (start, end) = self.get_rect_index_range().unwrap();
            // Check if range is valid
            if start >= end || end >= (rects_per_row * rects_per_column) as u32{
                return None;
            }

            // frame index to rect index
            let rect_index = self.frame_index_mod(frame_index).unwrap() as i32;
            let left = (rect_index % rects_per_row as i32) * rect_width;
            let top = (rect_index / rects_per_row as i32) * rect_height;
            Some(Rect::new(left as i32, top as i32, rect_width as i32, rect_height as i32))
            
        }
    

    }

    pub fn increment_loop(&mut self){
        match &mut self.type_{
            AnimationType::FiniteLoop{current_loop, loops, ..} => {
                *current_loop += 1;
            },
            _ => {}
        }
    }

}