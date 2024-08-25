use std::{io::SeekFrom, time::Instant};

use sfml::{
    graphics::{Color, Drawable, RenderTarget, RenderWindow, Transformable},
    window::{Event, Key, Style, VideoMode},
};

use crate::{EntityTrait, Scene};

use super::scene;


pub static mut RENDER_MS: f32 = 16.66;


pub fn get_render_ms() -> f32{
    unsafe{
        RENDER_MS
    }
}

fn set_render_ms(ms: f32){
    unsafe{
        RENDER_MS = ms;
    }
}


pub enum KeyState {
    Pressed(Key),
    Released(Key),
}



pub struct Game<T>
where
    T: Drawable + Transformable + EntityTrait,
{
    title: String,
    style: Style,
    scene: Scene<T>,
}

impl<T> Game<T>
where
    T: Drawable + Transformable + EntityTrait,
{
    pub fn new(title: &str, style: Style, scene: Scene<T>) -> Self {
        
        Game {
            title: title.to_string(),
            style,
            scene,
        }
    }

    
    fn set_key(&mut self, state: KeyState) {
        
        let code = match state {
            KeyState::Pressed(code) => code,
            KeyState::Released(code) => code,
        };

        let key_state = self.scene.check_key(code);
        
        match state {
            KeyState::Pressed(code) => {
                if !key_state {
                    self.scene.set_key(code, true);
                    (self.scene.on_keystate_changed)(&mut self.scene, state);
                }
            }
            KeyState::Released(code) => {
                if key_state {
                    self.scene.set_key(code, false);
                    (self.scene.on_keystate_changed)(&mut self.scene, state);
                }
            }
        }

    }


    pub fn run<W>(&mut self, v_mode_or_size: W) where W: Into<VideoMode> {



        let mut wnd = RenderWindow::new(
            v_mode_or_size,
            self.title.as_str(),
            self.style,
            &Default::default(),
        );

        wnd.set_framerate_limit(240);

        

        while wnd.is_open() {
            let now = Instant::now();

            wnd.clear(Color::BLACK);
          

            // UPDATE ENTITIES
            let (collisions, solid_collisions) = self.scene.update_entities();
              
             // FIRE FIRST RENDER EVENT
             if self.scene.first_render{
                (self.scene.on_start)(&mut self.scene);
                self.scene.first_render = false;
            }
           
            // FIRE ON_EVENT
            while let Some(event) = wnd.poll_event() {
                match event {
                    Event::Closed => wnd.close(),
                    Event::KeyPressed { code,.. } => {
                       self.set_key(KeyState::Pressed(code));
                    },
                    Event::KeyReleased { code,.. } => {
                        self.set_key(KeyState::Released(code));
                    },
                    _ => {}
                }
                (self.scene.on_event)(&mut self.scene, event);
            }

            // FIRE ON_UPDATE
            (self.scene.on_update)(&mut self.scene);

            // FIRE ON_COLLISION
            for (e1,e2,rect) in collisions{
                (self.scene.on_collision)(&mut self.scene, e1, e2, rect);
            }

            self.scene.push_back_solid_colisions(solid_collisions);
            self.scene.render(&mut wnd);

            wnd.display();
            let elapsed = now.elapsed().as_millis() as f32; // GET RENDER TIME
            set_render_ms(elapsed); // SET RENDER TIME
        }
    }
}
