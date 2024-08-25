use std::{cell::RefCell, collections::HashMap};

use sfml::{graphics::{
    CircleShape, Color, Drawable, FloatRect, IntRect, RcSprite, RcTexture, RectangleShape,
    RenderStates, RenderTarget, Shape, Sprite, Texture, Transformable,
}, system::Vector2f, window::sensor};

use super::prelude::*;

pub trait EntityTrait {
    fn get_sprite_mut(&mut self) -> &mut RcSprite;
    fn get_sprite(&self) -> &RcSprite;
    fn set_texture_rect(&mut self, rect: IntRect);
    fn texture_rect(&self) -> IntRect;
    fn global_bounds(&self) -> FloatRect;
    fn local_bounds(&self) -> FloatRect;
    fn color(&self) -> Color;
    fn set_color(&mut self, color: Color);
    fn set_texture(&mut self, texture: &RcTexture, reset_rect: bool);
    fn get_animation(&self, name: &str) -> Option<&Animation>;
    fn get_current_animation(&self) -> Option<&Animation>;
    fn get_current_animation_mut(&mut self) -> Option<&mut Animation>;
    fn get_animation_mut(&mut self, name: &str) -> Option<&mut Animation>;
    fn get_id(&self) -> String;

    fn set_current_animation(&mut self, name: &str);
    fn set_current_animation_flipped(&mut self, name: &str, flip_x: bool, flip_y: bool);
    fn add_animation(&mut self, name: &str, animation: Animation);
    fn current_animation(&self) -> &str;

    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_towards(&mut self, target: Vector2f);
    fn set_speed(&mut self, speed: f32);
    fn get_speed(&self) -> f32;



    fn is_animation_changed(&self) -> bool;
    fn set_animation_changed(&mut self, value: bool);

    fn get_animations(&self) -> &HashMap<String, Animation>;
    fn get_flip(&self) -> (bool, bool);

    fn get_physics(&self) -> &PhysicalProperties;
    fn get_physics_mut(&mut self) -> &mut PhysicalProperties;
    fn set_physics(&mut self, physics: PhysicalProperties);
    fn get_movement_vector(&self) -> Vector2f;



    fn get_hitbox(&self) ->RectangleShape;
    fn get_hitbox_with_margin(&self, margin:(f32,f32))->RectangleShape;

    fn mark_for_deletion(&mut self);
    fn is_marked_for_deletion(&self) -> bool;

    fn set_previous_position(&mut self, position: Vector2f);
    fn update_previous_position(&mut self);

}


pub struct Entity {
    id: String,
    animations: HashMap<String, Animation>,
    current_animation: String,
    sprite: RcSprite,
    animation_changed: bool,
    flip_x: bool,
    flip_y: bool,
    physics: PhysicalProperties,
    deletion_flag: bool,
    previous_position: Vector2f,
}

impl Entity {
    pub fn new(id: &str) -> Self {
        Entity {
            previous_position: Vector2f::new(0.0, 0.0),
            id: id.to_string(),
            animations: HashMap::new(),
            current_animation: "..".to_string(),
            sprite: RcSprite::new(),
            animation_changed: true,
            flip_x: false,
            flip_y: false,
            physics: PhysicalProperties::default(),
            deletion_flag: false,

        }
    }
}

impl EntityTrait for Entity {

    fn set_previous_position(&mut self, position: Vector2f) {
        self.previous_position = position;
    }

    fn update_previous_position(&mut self) {
        self.previous_position = self.sprite.position();
    }

    fn get_movement_vector(&self) -> Vector2f {
          self.sprite.position() - self.previous_position
    }

    fn set_speed(&mut self, speed: f32) {
        self.physics.speed = speed;
    }

    fn get_speed(&self) -> f32 {
        self.physics.speed
    }

    fn is_marked_for_deletion(&self) -> bool {
        self.deletion_flag
    }

    fn mark_for_deletion(&mut self) {
        self.deletion_flag = true;
    }

    

    fn move_towards(&mut self, target:Vector2f) {
        let pos = self.sprite.position();
        let direction = target - pos;
        let length = direction.length_sq().sqrt();
        let normalized = direction / length;
        let velocity = normalized * self.get_speed() * get_render_ms();
        self.move_(velocity);
    }

    fn get_hitbox(&self) -> RectangleShape {
        let mut rect = RectangleShape::new();        
        rect.set_size(self.sprite.global_bounds().size());
        rect.set_origin(rect.size() / 2.0);
        rect.set_position(self.sprite.position() + (rect.size() / 2.0));
        rect.set_fill_color(Color::TRANSPARENT);
        rect.set_outline_color(Color::BLUE);
        rect.set_outline_thickness(1.0);
        rect
    }


    fn get_hitbox_with_margin(&self, margin:(f32,f32 ))->RectangleShape{
        let mut rect = RectangleShape::new();
        let size = self.sprite.global_bounds().size() - Vector2f::new(margin.0 *2.0, margin.1 *2.0);
        rect.set_size(size);
        rect.set_origin(rect.size() / 2.0);
        rect.set_position(self.sprite.position() + (self.sprite.global_bounds().size() / 2.0));
        rect.set_fill_color(Color::TRANSPARENT);
        rect.set_outline_color(Color::RED);
        rect.set_outline_thickness(1.0);
        rect
    }

    fn get_physics(&self) -> &PhysicalProperties {
        &self.physics
    }

    fn get_physics_mut(&mut self) -> &mut PhysicalProperties {
        &mut self.physics
    }

    fn set_physics(&mut self, physics: PhysicalProperties) {
        self.physics = physics;
    }

    fn current_animation(&self) -> &str {
        &self.current_animation
    }

    fn get_flip(&self) -> (bool, bool) {
        (self.flip_x, self.flip_y)
    }

    fn get_animations(&self) -> &HashMap<String, Animation> {
        &self.animations
    }

    fn get_sprite_mut(&mut self) -> &mut RcSprite {
        &mut self.sprite
    }

    fn get_sprite(&self) -> &RcSprite {
        &self.sprite
    }

    fn set_animation_changed(&mut self, value: bool) {
        self.animation_changed = value;
    }

    fn is_animation_changed(&self) -> bool {
        self.animation_changed
    }

    fn move_down(&mut self) {
        self.move_((0.0, self.get_speed() * get_render_ms()));
    }

    fn move_up(&mut self) {
        self.move_((0.0, -self.get_speed() * get_render_ms()));
    }

    fn move_left(&mut self) {
        self.move_((-self.get_speed() * get_render_ms(), 0.0));
    }

    fn move_right(&mut self) {
        self.move_((self.get_speed() * get_render_ms(), 0.0));
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn local_bounds(&self) -> FloatRect {
        self.sprite.local_bounds()
    }

    fn global_bounds(&self) -> FloatRect {
        self.sprite.global_bounds()
    }

    fn set_color(&mut self, color: Color) {
        self.sprite.set_color(color);
    }

    fn set_texture(&mut self, texture: &RcTexture, reset_rect: bool) {
        self.sprite.set_texture(texture, reset_rect);
    }

    fn color(&self) -> Color {
        self.sprite.color()
    }

    fn texture_rect(&self) -> IntRect {
        self.sprite.texture_rect()
    }

    fn set_texture_rect(&mut self, rect: IntRect) {
        self.sprite.set_texture_rect(rect);
    }

    fn get_animation(&self, name: &str) -> Option<&Animation> {
        self.animations.get(name)
    }

    fn get_current_animation(&self) -> Option<&Animation> {
        self.animations.get(&self.current_animation)
    }

    fn get_current_animation_mut(&mut self) -> Option<&mut Animation> {
        self.animations.get_mut(&self.current_animation)
    }

    fn get_animation_mut(&mut self, name: &str) -> Option<&mut Animation> {
        self.animations.get_mut(name)
    }

    fn set_current_animation_flipped(&mut self, name: &str, flip_x: bool, flip_y: bool) {
        self.set_current_animation(name);
        self.flip_x = flip_x;
        self.flip_y = flip_y;
    }

    fn set_current_animation(&mut self, name: &str) {

        if name.is_empty() || name == "deleted" {
            self.mark_for_deletion();
            return;
        }

        if !self.animations.contains_key(name) {
            println!("Animation not found: {}", name);
            return;
        }

        if self.current_animation == name {
            return;
        }

        self.flip_x = false;
        self.flip_y = false;
        self.animation_changed = true;
        self.current_animation = name.to_string();
    }

    fn add_animation(&mut self, name: &str, animation: Animation) {
        if self.animations.contains_key(name) {
            panic!("Animation already exists: {}", name);
        }

        if self.animations.is_empty() {
            self.current_animation = name.to_string();
        }

        self.animations.insert(name.to_string(), animation);
    }
}

impl Drawable for Entity {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        self.sprite.draw(target, states);
    }
}

impl Transformable for Entity {
    fn get_scale(&self) -> sfml::system::Vector2f {
        self.sprite.get_scale()
    }

    fn inverse_transform(&self) -> &sfml::graphics::Transform {
        self.sprite.inverse_transform()
    }

    fn scale<F: Into<sfml::system::Vector2f>>(&mut self, factors: F) {
        self.sprite.scale(factors);
    }

    fn transform(&self) -> &sfml::graphics::Transform {
        self.sprite.transform()
    }

    fn set_position<P: Into<sfml::system::Vector2f>>(&mut self, position: P) {
        self.sprite.set_position(position);
    }

    fn position(&self) -> sfml::system::Vector2f {
        self.sprite.position()
    }

    fn origin(&self) -> sfml::system::Vector2f {
        self.sprite.origin()
    }

    fn move_<O: Into<sfml::system::Vector2f>>(&mut self, offset: O) {
        self.sprite.move_(offset);
    }

    fn rotate(&mut self, angle: f32) {
        self.sprite.rotate(angle);
    }

    fn rotation(&self) -> f32 {
        self.sprite.rotation()
    }

    fn set_origin<O: Into<sfml::system::Vector2f>>(&mut self, origin: O) {
        self.sprite.set_origin(origin);
    }

    fn set_rotation(&mut self, angle: f32) {
        self.sprite.set_rotation(angle);
    }

    fn set_scale<S: Into<sfml::system::Vector2f>>(&mut self, scale: S) {
        self.sprite.set_scale(scale);
    }
}
