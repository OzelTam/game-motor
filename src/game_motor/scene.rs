use core::f32;
use std::{cell::RefCell, char::MAX, cmp, collections::HashMap, f32::INFINITY};

use indexmap::IndexMap;
use sfml::{
    audio::SoundBuffer, graphics::{
        glsl::Vec2, CircleShape, Color, Drawable, FloatRect, IntRect, RcFont, RcTexture, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, TextStyle, Texture, Transformable
    }, system::Vector2f, window::{Event, Key}
};

use super::{animation, entity, prelude::*};

pub struct Scene<T>
where
    T: Drawable + Transformable,
{
    pub name: String,
    pub on_keystate_changed: fn(&mut Self, KeyState) -> (),
    pub on_update: fn(&mut Self) -> (),
    pub on_event: fn(&mut Self, Event) -> (),
    pub on_entity_state_change: fn(&mut Self, String) -> (),
    pub on_start: fn(&mut Self),
    pub first_render: bool,
    pub on_collision: fn(&mut Self, String, String, FloatRect) -> (),
    textures: HashMap<String, RcTexture>,
    pub fonts: ResourceHolder<RcFont, String>,
    pub sounds: ResourceHolder<SoundBuffer, String>,
    pub entities: IndexMap<String, RefCell<T>>,
    keyboard: HashMap<Key, bool>,
    pub gravity: Vec2,
}

impl<T> Scene<T>
where
    T: Drawable + Transformable + EntityTrait,
{
    pub fn new(name: &str) -> Self {
        let mut scn = Scene {
            gravity: Vec2::new(0.0, 0.0),
            on_keystate_changed: |_s, _k| {},
            first_render: true,
            on_entity_state_change: |_s, _e| {},
            name: name.to_string(),
            on_start: |_s| {},
            on_update: |_s| {},
            on_event: |_s, _e| {},
            on_collision: |_s, _e1, _e2, _r| {},
            textures: HashMap::new(),
            fonts: ResourceHolder::default(),
            sounds: ResourceHolder::default(),
            entities: IndexMap::new(),
            keyboard: HashMap::new(),
        };

        scn.load_texture("empty", "assets/404.png");
        return scn;
    }

    pub fn set_gravity(&mut self, gravity: Vec2) {
        self.gravity = gravity;
    }

    pub fn load_texture(&mut self, name: &str, path: &str) {
        if let Ok(texture) = RcTexture::from_file(path) {
            self.textures.insert(name.to_string(), texture);
        }
    }

    pub fn load_texture_custom(&mut self, name: &str, path: &str, manup: fn(RcTexture)->RcTexture) {
        if let Ok(texture) = RcTexture::from_file(path) {
            let texture = manup(texture);
            self.textures.insert(name.to_string(), texture);
        }
    }

    pub fn get_texture(&self, name: &str) -> Option<&RcTexture> {
        if let Some(txt) = self.textures.get(name) {
            return Some(txt);
        }
        None
    }

    pub fn entity(&self, id: &str) -> &RefCell<T> {
        if !self.entities.contains_key(id) {
            panic!("Entity with id {} not found", id);
        }
        self.entities.get(id).unwrap()
    }

    pub fn try_entity(&self, id: &str) -> Option<&RefCell<T>> {
        self.entities.get(id)
    }

    pub fn add_entity(&mut self, entity: T) {
        self.entities.insert(entity.get_id(), RefCell::new(entity));
    }

    pub fn check_key(&self, key: Key) -> bool {
        if let Some(&value) = self.keyboard.get(&key) {
            return value;
        }
        false
    }

    pub fn any_key_pressed(&self) -> bool {
        for (_, &value) in self.keyboard.iter() {
            if value {
                return true;
            }
        }
        false
    }

    pub fn set_key(&mut self, key: Key, value: bool) {
        self.keyboard.insert(key, value);
    }

    fn animate(&self, entity: &RefCell<T>) {
        if entity.borrow().is_animation_changed() {
            let texture_id: String = entity.borrow().get_current_animation().unwrap().textute_id.clone(); // Clone the texture ID
            let texture: &RcTexture = self.get_texture(&texture_id).unwrap();
            let rect = entity
            .borrow()
            .get_current_animation()
            .unwrap()
            .get_current_rect(texture.size());
            entity.borrow_mut().set_texture(texture, true);
            if rect.is_some() {
                entity.borrow_mut().set_texture_rect(rect.unwrap());
            }
            entity.borrow_mut().set_animation_changed(false);
        }


        let animation_type: animation::AnimationType = entity
            .borrow()
            .get_current_animation()
            .unwrap()
            .type_
            .clone(); // Clone or copy the animation type if needed
        match animation_type {
            AnimationType::Static => {} // Do nothing for static animations
            AnimationType::Loop { rect_index_range } => {
                let next_frame = entity
                    .borrow()
                    .get_current_animation()
                    .unwrap()
                    .time_to_next_frame()
                    .clone();

                if next_frame {
                    let texture_id: String = entity
                        .borrow()
                        .get_current_animation()
                        .unwrap()
                        .textute_id
                        .clone();
                    let texture: &RcTexture = self.get_texture(&texture_id).unwrap();
                    let (flip_x, flip_y) = entity.borrow().get_flip();

                    let current_rect = entity
                        .borrow()
                        .get_current_animation()
                        .unwrap()
                        .get_current_rect_flipped(texture.size(), flip_x, flip_y);
                    entity
                        .borrow_mut()
                        .get_current_animation_mut()
                        .unwrap()
                        .safe_increment_frame(texture.size());
                    entity
                        .borrow_mut()
                        .get_current_animation_mut()
                        .unwrap()
                        .reset_duration();

                    if current_rect.is_some() {
                        entity.borrow_mut().set_texture_rect(current_rect.unwrap());
                    }
                }
                entity
                    .borrow_mut()
                    .get_current_animation_mut()
                    .unwrap()
                    .increment_duration(get_render_ms());
            }
            AnimationType::FiniteLoop {
                loops,
                current_loop,
                return_state,
                ..
            } => {
                let next_frame = entity
                    .borrow()
                    .get_current_animation()
                    .unwrap()
                    .time_to_next_frame()
                    .clone();

                if next_frame {
                    let texture_id: String = entity
                        .borrow()
                        .get_current_animation()
                        .unwrap()
                        .textute_id
                        .clone();
                    let texture: &RcTexture = self.get_texture(&texture_id).unwrap();
                    let (flip_x, flip_y) = entity.borrow().get_flip();

                    let current_rect = entity
                        .borrow()
                        .get_current_animation()
                        .unwrap()
                        .get_current_rect_flipped(texture.size(), flip_x, flip_y);
                    let looped = entity
                        .borrow_mut()
                        .get_current_animation_mut()
                        .unwrap()
                        .safe_increment_frame(texture.size());

                    if looped {
                        entity
                            .borrow_mut()
                            .get_current_animation_mut()
                            .unwrap()
                            .increment_loop();
                    }

                    if current_loop >= loops {
                        entity
                            .borrow_mut()
                            .set_current_animation(return_state.as_str());
                    }
                    entity
                        .borrow_mut()
                        .get_current_animation_mut()
                        .unwrap()
                        .reset_duration();
                    if current_rect.is_some() {
                        entity.borrow_mut().set_texture_rect(current_rect.unwrap());
                    }
                }

                entity
                    .borrow_mut()
                    .get_current_animation_mut()
                    .unwrap()
                    .increment_duration(get_render_ms());
            }
            AnimationType::Once {
                return_state,
                rect_index_range,
            } => {
                let next_frame = entity
                    .borrow()
                    .get_current_animation()
                    .unwrap()
                    .time_to_next_frame()
                    .clone();

                if next_frame {
                    let texture_id: String = entity
                        .borrow()
                        .get_current_animation()
                        .unwrap()
                        .textute_id
                        .clone();
                    let texture: &RcTexture = self.get_texture(&texture_id).unwrap();
                    let (flip_x, flip_y) = entity.borrow().get_flip();

                    let current_rect = entity
                        .borrow()
                        .get_current_animation()
                        .unwrap()
                        .get_current_rect_flipped(texture.size(), flip_x, flip_y);
                    let looped = entity
                        .borrow_mut()
                        .get_current_animation_mut()
                        .unwrap()
                        .safe_increment_frame(texture.size());

                    if looped {
                        entity
                            .borrow_mut()
                            .set_current_animation(return_state.as_str());
                    }

                    entity
                        .borrow_mut()
                        .get_current_animation_mut()
                        .unwrap()
                        .reset_duration();
                    if current_rect.is_some() {
                        entity.borrow_mut().set_texture_rect(current_rect.unwrap());
                    }
                }

                entity
                    .borrow_mut()
                    .get_current_animation_mut()
                    .unwrap()
                    .increment_duration(get_render_ms());
            }
        }
    }

    pub fn get_closest_entity_where(&self, entity_id: &str, condition: fn(&T) -> bool) -> Option<(&RefCell<T>, Vector2f)> {
        let entity = self.entity(entity_id);
        let entity = entity.borrow();
        let mut closest: Option<(&RefCell<T>, Vector2f)> = None;
        let mut closest_d = f32::MAX;

        for e in self.entities.values() {
            let e1 = e.borrow();
            if e1.get_id() == entity.get_id() || !condition(&e1) {
                continue;
            }

            let vec = closest_distance(entity.global_bounds(), e1.global_bounds());
            let distance = vec.length_sq().sqrt();
            if  distance < closest_d {
                closest_d = distance;
                closest = Some((e, vec));
            }
        }
        None
    }

    pub fn get_closest_entity(&self, entity_id: &str) -> Option<(&RefCell<T>, Vector2f)> {
        let entity = self.entity(entity_id);
        let entity = entity.borrow();
        let mut closest: Option<(&RefCell<T>, Vector2f)> = None;
        let mut closest_d = f32::MAX;

        for e in self.entities.values() {
            let e1 = e.borrow();
            if e1.get_id() == entity.get_id() {
                continue;
            }

            let vec = closest_distance(entity.global_bounds(), e1.global_bounds());
            let distance = vec.length_sq().sqrt();
            if  distance < closest_d {
                closest_d = distance;
                closest = Some((e, vec));
            }
        }

        return closest;
    }
    

    pub fn get_closest_solid_where(&self, entity_id: &str, condition: fn(&T) -> bool) -> Option<(&RefCell<T>, Vector2f)> {
        let entity = self.entity(entity_id);
        let entity = entity.borrow();
        let mut closest: Option<(&RefCell<T>, Vector2f)> = None;
        let mut closest_d = f32::MAX;

        for e in self.entities.values() {
            let e1 = e.borrow();
            if e1.get_id() == entity.get_id() || e1.get_physics().ghost || !condition(&e1) {
                continue;
            }

            let rect1 = entity.get_hitbox_with_margin(entity.get_physics().solid_box_margin).global_bounds();
            let rect2 = e1.get_hitbox_with_margin(e1.get_physics().solid_box_margin).global_bounds();
            let vec = closest_distance(rect1, rect2);
            let distance = vec.length_sq().sqrt();
            if  distance < closest_d {
                closest_d = distance;
                closest = Some((e, vec));
            }
        }

        return closest;
    }

    pub fn get_closest_solid(&self, entity_id: &str) -> Option<(&RefCell<T>, Vector2f)> {
        let entity = self.entity(entity_id);
        let entity = entity.borrow();
        let mut closest: Option<(&RefCell<T>, Vector2f)> = None;
        let mut closest_d = f32::MAX;

        for e in self.entities.values() {
            let e1 = e.borrow();
            if e1.get_id() == entity.get_id() || e1.get_physics().ghost {
                continue;
            }

            let rect1 = entity.get_hitbox_with_margin(entity.get_physics().solid_box_margin).global_bounds();
            let rect2 = e1.get_hitbox_with_margin(e1.get_physics().solid_box_margin).global_bounds();
            let vec = closest_distance(rect1, rect2);
            let distance = vec.length_sq().sqrt();
            if  distance < closest_d {
                closest_d = distance;
                closest = Some((e, vec));
            }
        }

        return closest;
    }


    pub fn get_closest_path_between(&self, entity_id1:&str, entity_id2:&str)->Vector2f
    {
        let entity1 = self.try_entity(entity_id1);
        let entity2 = self.try_entity(entity_id2);
        if entity1.is_none() || entity2.is_none(){
            return Vector2f::new(f32::INFINITY, f32::INFINITY);
        }

        let entity1 = entity1.unwrap().borrow();
        let entity2 = entity2.unwrap().borrow();

        let rect1 = entity1.get_hitbox_with_margin(entity1.get_physics().solid_box_margin).global_bounds();
        let rect2 = entity2.get_hitbox_with_margin(entity2.get_physics().solid_box_margin).global_bounds();
        return closest_distance(rect1, rect2);
    }

    fn check_solid_collisions(&self, e1: String, e2: String)->Option<FloatRect>{

        let e1_ = self.entity(e1.as_str()).borrow();
        let e2_ = self.entity(e2.as_str()).borrow();

        if e1_.get_physics().ghost || e2_.get_physics().ghost {
            return None;
        }

        let mut e1_bnd = e1_.global_bounds();
        let mut e2_bnd = e2_.global_bounds();

        let e1_margin = e1_.get_physics().solid_box_margin;
        let e2_margin = e2_.get_physics().solid_box_margin;

        e1_bnd.left = e1_bnd.left + e1_margin.0;
        e1_bnd.top = e1_bnd.top + e1_margin.1;

        e2_bnd.left = e2_bnd.left + e2_margin.0;
        e2_bnd.top = e2_bnd.top + e2_margin.1;

        e1_bnd.width = e1_bnd.width - e1_margin.0 * 2.0;
        e1_bnd.height = e1_bnd.height - e1_margin.1 * 2.0;

        e2_bnd.width = e2_bnd.width - e2_margin.0 * 2.0;
        e2_bnd.height = e2_bnd.height - e2_margin.1 * 2.0;



        return e1_bnd.intersection(&e2_bnd);

    }

    fn collision_push_back(&self, e1: String, e2: String, inter: FloatRect){   
        if self.entity(e1.as_str()).borrow().get_physics().static_object && self.entity(e2.as_str()).borrow().get_physics().static_object {
            return;
        }


        let coltype1 = self.entity(e1.as_str()).borrow().get_physics().collision_type.clone();
        let coltype2 = self.entity(e2.as_str()).borrow().get_physics().collision_type.clone();

        let collision_type: CollisionType;
        if matches!(coltype1, CollisionType::Rectangular) || matches!(coltype2, CollisionType::Rectangular){
            collision_type = CollisionType::Rectangular;
        }
        else{
            collision_type = CollisionType::Vectoral;
        }
       


        match collision_type {
            CollisionType::Rectangular => {
                let e1_static = self.entity(e1.as_str()).borrow().get_physics().static_object;
                let e2_static = self.entity(e2.as_str()).borrow().get_physics().static_object;
                if e1_static && e2_static {
                    return;
                }

                let push_force = if inter.width < inter.height { inter.width } else { inter.height };

                let c_axis = if inter.width < inter.height {Vec2::new(1.0, 0.0)} else {Vec2::new(0.0, 1.0)};
                let c_axis = c_axis * push_force;
                
                
                let vertical_touch = inter.width > inter.height;
                let e2_center = self.entity(e2.as_str()).borrow().global_bounds().position() + self.entity(e2.as_str()).borrow().global_bounds().size() / 2.0;
                let (e2_x, e2_y) = (e2_center.x, e2_center.y);
                let (col_x, col_y) = (inter.left, inter.top);


                let bottom_touch = e2_y < col_y;
                let right_touch = e2_x < col_x;


                let m1 = self.entity(e1.as_str()).borrow().get_physics().mass;
                let m2 = self.entity(e2.as_str()).borrow().get_physics().mass;
                let mut m1_weight = m2 / (m1 + m2);
                let mut m2_weight = m1 / (m1 + m2);

                if e1_static {
                    m1_weight = 0.0;
                    m2_weight = 1.0;
                }

                if e2_static {
                    m1_weight = 1.0;
                    m2_weight = 0.0;
                }


                if (vertical_touch && bottom_touch) || (!vertical_touch && right_touch){
                    let push_e1 = c_axis * m1_weight;
                    let push_e2 = c_axis * m2_weight;

                    self.entity(e1.as_str()).borrow_mut().move_(push_e1);
                    self.entity(e2.as_str()).borrow_mut().move_(-push_e2);
                }
                else if (vertical_touch && !bottom_touch) || (!vertical_touch && !right_touch){
                    let push_e1 = c_axis * m1_weight;
                    let push_e2 = c_axis * m2_weight;

                    self.entity(e1.as_str()).borrow_mut().move_(-push_e1);
                    self.entity(e2.as_str()).borrow_mut().move_(push_e2);
                }
            },
            CollisionType::Vectoral => {
                let touch_point = Vec2::new(inter.left + inter.width / 2.0, inter.top + inter.height / 2.0);
                let e1_size = self.entity(e1.as_str()).borrow().global_bounds().size();
                let e1_center = self.entity(e1.as_str()).borrow().global_bounds().position() + e1_size / 2.0;
                let e2_size = self.entity(e2.as_str()).borrow().global_bounds().size();
                let e2_center = self.entity(e2.as_str()).borrow().global_bounds().position() + e2_size / 2.0;
        
        
                let e1_push = e1_center - touch_point;
                let e1_push_length = if  e1_push.length_sq().sqrt() == 0.0 {1.0} else {e1_push.length_sq().sqrt()};
                let e1_push = e1_push / e1_push_length;
        
                let e2_push = e2_center - touch_point;
                let e2_push_length = if  e2_push.length_sq().sqrt() == 0.0 {1.0} else {e2_push.length_sq().sqrt()};
                let e2_push = e2_push / e2_push_length;
        
                let e1_velocity = self.entity(e1.as_str()).borrow().get_movement_vector();
                let e2_velocity = self.entity(e2.as_str()).borrow().get_movement_vector();
        
                let e1_push = e1_push * if  e2_velocity.length_sq().sqrt() == 0.0 {1.0} else {e2_velocity.length_sq().sqrt()};
                let e2_push = e2_push * if e1_velocity.length_sq().sqrt() == 0.0 {1.0} else {e1_velocity.length_sq().sqrt()};

                
                
                if self.entity(e1.as_str()).borrow().get_physics().static_object  {
                    let e2_push = e2_push * get_render_ms();
                    self.entity(e2.as_str()).borrow_mut().move_(e2_push);
                    return;
                }
                
                if self.entity(e2.as_str()).borrow().get_physics().static_object {
                    let e1_push = e1_push * get_render_ms();
                    self.entity(e1.as_str()).borrow_mut().move_(e1_push );
                    return;
                }
        
                let e1_mass = self.entity(e1.as_str()).borrow().get_physics().mass;
                let e2_mass = self.entity(e2.as_str()).borrow().get_physics().mass;
                let mass_2_weight = e1_mass / (e1_mass + e2_mass);
                let mass_1_weight = e2_mass / (e1_mass + e2_mass);

                
                let e1_push = e1_push * mass_1_weight * get_render_ms();
                let e2_push = e2_push * mass_2_weight  * get_render_ms();
                
             

                self.entity(e1.as_str()).borrow_mut().move_(e1_push);
                self.entity(e2.as_str()).borrow_mut().move_(e2_push);
            }
        }


    }

    pub fn push_back_solid_colisions(&self, solid_colls: Vec<(String, String, FloatRect)>){
        for (e1, e2, inter) in solid_colls.iter(){
            self.collision_push_back(e1.clone(), e2.clone(), inter.clone());
        }
    }

    fn check_collisions(&self, entity_index: usize) -> (Vec<(String, String, FloatRect)>, Vec<(String, String, FloatRect)>) {
        let e1 = self.entities.get_index(entity_index).unwrap().1.borrow();
        
        if !e1.get_physics().collision_enabled || (entity_index + 1) >= self.entities.len()  {
            return (Vec::new(), Vec::new());
        }
        
        let mut solid_collisions: Vec<(String, String, FloatRect)> = Vec::new();
        let mut collisions: Vec<(String, String, FloatRect)> = Vec::new();

        let slice = self
            .entities
            .get_range((entity_index + 1..self.entities.len()));

        if slice.is_none() {
            return (Vec::new(), Vec::new());
        }

        let slice = slice.unwrap();

        for (e2_id, e2) in slice.iter() {
            let e2 = e2.borrow();
            if !e2.get_physics().collision_enabled || e2.is_marked_for_deletion() {
                continue;
            }

            let e1_rect = e1.global_bounds();
            let e2_rect = e2.global_bounds();
            
           
            // If Colliding
            if let Some(int) = e1_rect.intersection(&e2_rect) {
                collisions.push((e1.get_id(), e2_id.clone(), int)); // Add to collisions
                
                // Check for solid collision
                if let Some(solid_collision) = self.check_solid_collisions(e1.get_id(), e2_id.clone()){
                    solid_collisions.push((e1.get_id(), e2_id.clone(), solid_collision));
                }
            }
        }

  
        return (collisions, solid_collisions);
    }


    pub fn render(&self, wnd: &mut RenderWindow) {
        for entity in self.entities.values() {
            let show_collider_box = entity.borrow().get_physics().show_collider;
            let show_solid_box = entity.borrow().get_physics().show_solid_box;

            wnd.draw(entity.borrow().get_sprite());
             // RENDER COLLIDER BOX
             if show_collider_box {
                let e = entity.borrow();
                let hitbox = e.get_hitbox();
                wnd.draw(&hitbox);
            }
            
            // RENDER SOLID BOX
            if show_solid_box {
                let margin = entity.borrow().get_physics().solid_box_margin;
                let e = entity.borrow();
                let hitbox = e.get_hitbox_with_margin(margin);
                wnd.draw(&hitbox);
            }
        }
    }

    fn apply_gravity(&self, entity: &RefCell<T>) {
        let mut entity = entity.borrow_mut();
        if entity.get_physics().static_object {
            return;
        }
        entity.move_down();
        // entity.move_(self.gravity * get_render_ms());
    }

    pub fn update_entities(&mut self) -> (Vec<(String, String, FloatRect)>, Vec<(String, String, FloatRect)>) {
        let mut i = 0;
        let mut delete_list: Vec<String> = Vec::new();
        
        let mut collisions: Vec<(String, String, FloatRect)> = Vec::new();
        let mut solid_collisions: Vec<(String, String, FloatRect)> = Vec::new();
        
        for entity in self.entities.values() {
            entity.borrow_mut().update_previous_position();
            
            if entity.borrow().is_marked_for_deletion() {
                delete_list.push(entity.borrow().get_id());
                continue;
            }
            
            if entity.borrow().get_current_animation().is_none() {
                let empty = self.get_texture("empty").unwrap();
                entity.borrow_mut().set_texture(empty, true);
                continue;
            }

            // ANIMATION STUFF
            self.animate(entity);
            
            // PHYSICS STUFF
            if entity.borrow().get_physics().collision_enabled {
                let (colls, solid_colls) = self.check_collisions(i); // Check for collisions and solid collisions
                collisions.extend(colls);  // Extend the collisions
                solid_collisions.extend(solid_colls); // Extend the solid collisions
            }

            // GRAVITY
            if self.gravity.length_sq().sqrt() > 0.0 {
                self.apply_gravity(entity);
            }
            
           
            i += 1;
        }

        for coll in collisions.iter() {
            (self.on_collision)(self, coll.0.clone(), coll.1.clone(), coll.2.clone());
        }
        
        for id_to_delete in delete_list {
            self.entities.remove(&id_to_delete);
        }

        

        return (collisions, solid_collisions);
    }
}

pub fn closest_distance(rect1:Rect<f32>, rect2:Rect<f32>)->Vector2f{
    let mut distance = Vector2f::new(0.0, 0.0);
    // Calculate horizontal distance
    if (rect1.left + rect1.width < rect2.left) {
        distance.x = rect2.left - (rect1.left + rect1.width);
    } else if (rect2.left + rect2.width < rect1.left) {
        distance.x = rect1.left - (rect2.left + rect2.width);
    } else {
        distance.x = 0.0; // Overlapping horizontally
    }

    // Calculate vertical distance
    if (rect1.top + rect1.height < rect2.top) {
        distance.y = rect2.top - (rect1.top + rect1.height);
    } else if (rect2.top + rect2.height < rect1.top) {
        distance.y = rect1.top - (rect2.top + rect2.height);
    } else {
        distance.y = 0.0; // Overlapping vertically
    }

    return distance;
    }