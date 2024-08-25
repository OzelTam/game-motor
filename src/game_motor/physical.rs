use sfml::graphics::{CircleShape, RectangleShape};


#[derive(Debug, Clone)]
pub enum CollisionType{
    Vectoral,
    Rectangular,
}

#[derive(Debug, Clone)]
pub struct PhysicalProperties{
    pub collision_enabled: bool,
    pub mass: f32,
    pub friction: f32,
    pub bounce: f32,
    pub static_object: bool,
    pub show_collider: bool,
    pub ghost: bool,
    pub solid_box_margin: (f32, f32),
    pub show_solid_box: bool,
    pub speed: f32,
    pub collision_type: CollisionType,
}

impl PhysicalProperties{
    pub fn default() -> Self{
        PhysicalProperties{
            collision_type: CollisionType::Rectangular,
            speed:1.0,
            collision_enabled: true,
            mass: 1.0,
            friction: 0.0,
            bounce: 0.0,
            static_object: false,
            show_collider: false,
            ghost: false,
            solid_box_margin:   (0.0, 0.0),
            show_solid_box: false,

        }
    }


    pub fn new_static()->Self{
        PhysicalProperties{
            collision_type: CollisionType::Rectangular,
            speed:1.0,
            collision_enabled: true,
            mass: 1.0,
            friction: 0.0,
            bounce: 0.0,
            static_object: true,
            show_collider: false,
            ghost: false,
            solid_box_margin:(0.0, 0.0),
            show_solid_box: false,
        }
    }
    

    pub fn new_debug()->Self{
        
        PhysicalProperties{
            collision_type: CollisionType::Rectangular,
            speed:1.0,
            bounce: 0.5,
            collision_enabled: true,
            friction: 0.0,
            ghost: false,
            mass: 1.0,
            show_collider: true,
            static_object: false,
            show_solid_box: true,
            solid_box_margin: (0.0, 0.0),
        }
    }

}

