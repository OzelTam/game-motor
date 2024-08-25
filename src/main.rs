#![allow(dead_code)]

use core::f32;
use std::{env, mem::{self, ManuallyDrop}};

use sfml::{
    graphics::{glsl::Vec2, Color, FloatRect, IntRect, RenderWindow, Transformable},
    system::Vector2,
    window::{Event, Key, Style, VideoMode},
};
mod game_motor;
use game_motor::{entity, prelude::*, scene};

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    // Define Entities
    let mut player = Entity::new("main");
    let mut goblin = Entity::new("goblin1");
    let mut ground = Entity::new("ground");



    // Define Scene
    let mut scn = Scene::new("main");
    // Load Textures
    scn.load_texture("player", "assets/Warrior_Yellow.png");
    scn.load_texture(
        "trch_goblin",
        "assets/Torch_Blue.png",
    );

    scn.load_texture_custom("ground", "assets/Water.png", |mut tx| {
        tx.set_repeated(true);
        tx.set_smooth(true);
        tx
    });


    scn.load_texture("expl", "assets/Explosions.png");


    scn.set_gravity(Vec2::new(0.0, 0.3));
    // Create Animations
    let p_ridle = Animation::new_loop("player", 100.0, (192, 192), Some((0, 5)));
    let p_rwalk = Animation::new_loop("player", 100.0, (192, 192), Some((6, 11)));
    let p_rattack1 = Animation::new_once("p_ridle", "player", 100.0, (192, 192), Some((12, 17)));
    let p_rattack2 = Animation::new_once("p_ridle", "player", 100.0, (192, 192), Some((18, 23)));
    let p_fattack1 = Animation::new_once("p_ridle", "player", 100.0, (192, 192), Some((24, 29)));
    let p_fattack2 = Animation::new_once("p_ridle", "player", 100.0, (192, 192), Some((30, 35)));

    let g_idle = Animation::new_loop("trch_goblin", 100.0, (192, 192), Some((0, 6)));
    let g_walk = Animation::new_loop("trch_goblin", 100.0, (192, 192), Some((7, 12)));
    let g_attack1 = Animation::new_once("g_idle", "trch_goblin", 100.0, (192, 192), Some((13, 18)));

    let e_explode = Animation::new_once("deleted", "expl", 100.0, (192, 192), Some((0, 8)));

    let g_water = Animation::new_static("ground");

    // Set physics properties
    let def = PhysicalProperties::default();

    player.set_physics(PhysicalProperties {
        collision_enabled: true,
        show_collider: true,
        show_solid_box: true,
        solid_box_margin: (60.0, 60.0),
        collision_type: CollisionType::Vectoral,
        speed: 0.3,
        mass:1.0,
        // static_object: true,
        ..def.clone()
    });

    goblin.set_physics(PhysicalProperties {
        show_collider: true,
        show_solid_box: true,
        solid_box_margin: (60.0, 60.0),
        speed: 0.3,
        collision_type: CollisionType::Vectoral,
        ..def.clone()
    });

    ground.set_physics(PhysicalProperties {
        static_object: true,
        collision_type: CollisionType::Rectangular  ,
        ..def.clone()
    });



    // Add Animations to Entity
    player.add_animation("p_ridle", p_ridle);
    player.add_animation("p_rwalk", p_rwalk);
    player.add_animation("p_rattack1", p_rattack1);
    player.add_animation("p_rattack2", p_rattack2);
    player.add_animation("p_fattack1", p_fattack1);
    player.add_animation("p_fattack2", p_fattack2);

    goblin.add_animation("g_idle", g_idle);
    goblin.add_animation("g_walk", g_walk);
    goblin.add_animation("g_attack1", g_attack1);
    goblin.add_animation("explode", e_explode);

    ground.add_animation("water", g_water);
    // Add Entities to Scene
    scn.add_entity(ground);
    scn.add_entity(player);
    scn.add_entity(goblin);
    // Event Handlers
    scn.on_keystate_changed = on_keystate_changed;
    scn.on_update = on_update;
    scn.on_collision = on_collision;
    scn.on_start = |scn| {
        scn.entity("goblin1").borrow_mut().set_position(Vector2::new(400.0, 50.0));
        scn.entity("main").borrow_mut().set_position(Vec2::new(400.0, 300.0));
        scn.entity("ground").borrow_mut().set_texture_rect(IntRect::new(0, 0, 1500, 180));
        scn.entity("ground").borrow_mut().set_position(Vector2::new(150.0, 450.0));
        
    };

    // Define Game
    let mut game = Game::new("Game", Style::CLOSE, scn);
    // let mut game = Game::new("Game", (800, 600), Style::CLOSE, scn);

    // Start Game
    game.run((900, 600));
}

fn on_collision(scene: &mut Scene<Entity>, e1: String, e2: String, inter: FloatRect) {
    let touch_area = if  inter.width < inter.height  {
        inter.width
    } else {
        inter.height
    };

    if touch_area > 100.0 {
        // println!("HİT HİM HİT HİMMMMM !!!");

        if scene
            .entity(e1.as_str())
            .borrow()
            .current_animation()
            .contains("attack")
        {
            scene
                .entity(e2.as_str())
                .borrow_mut()
                .set_current_animation("explode");
        }
    }

    // if scene.entity(e1.as_str()).borrow().current_animation().contains("attack")  {
    //     scene.entity(e2.as_str()).borrow_mut().set_current_animation("explode");
    // }
}

fn on_update(scene: &mut Scene<Entity>) {
    if let Some(goblin) = scene.try_entity("goblin1") {
        if scene.check_key(Key::Numpad8) {
            goblin.borrow_mut().move_up();
        }
        if scene.check_key(Key::Numpad5) {
            goblin.borrow_mut().move_down();
        }
        if scene.check_key(Key::Numpad4) {
            goblin.borrow_mut().move_left();
        }
        if scene.check_key(Key::Numpad6) {
            goblin.borrow_mut().move_right();
        }
    }

    if scene.check_key(Key::W) {
        scene.entity("main").borrow_mut().move_up();
    }

    if scene.check_key(Key::S) {
        scene.entity("main").borrow_mut().move_down();
    }
    if scene.check_key(Key::A) {
        scene.entity("main").borrow_mut().move_left();
    }
    if scene.check_key(Key::D) {
        scene.entity("main").borrow_mut().move_right();
    }

    

}

fn on_keystate_changed(scene: &mut Scene<Entity>, key_state: KeyState) {
    if let Some(goblin) = scene.try_entity("goblin1") {
        match key_state {
            KeyState::Pressed(Key::Numpad8) => {
                goblin.borrow_mut().set_current_animation("g_walk");
            }
            KeyState::Pressed(Key::Numpad5) => {
                goblin
                    .borrow_mut()
                    .set_current_animation("g_walk");
            }
            KeyState::Pressed(Key::Numpad4) => {
                goblin
                    .borrow_mut()
                    .set_current_animation_flipped("g_walk", true, false);
            }
            KeyState::Pressed(Key::Numpad6) => {
                goblin.borrow_mut().set_current_animation("g_walk");
            }
            KeyState::Released(Key::Numpad8) => {
                goblin.borrow_mut().set_current_animation("g_idle");
            }
            KeyState::Released(Key::Numpad5) => {
                goblin.borrow_mut().set_current_animation("g_idle");
            }
            KeyState::Released(Key::Numpad4) => {
                goblin.borrow_mut().set_current_animation("g_idle");
            }
            KeyState::Released(Key::Numpad6) => {
                goblin.borrow_mut().set_current_animation("g_idle");
            }

            KeyState::Pressed(Key::Numpad0) => {
                goblin.borrow_mut().set_current_animation("g_attack1");
            }

            _ => {}
        }
    }

        match key_state {
            KeyState::Pressed(Key::W) => {
                scene
                    .entity("main")
                    .borrow_mut()
                    .set_current_animation("p_rwalk");
            }
            KeyState::Pressed(Key::S) => {
                scene
                    .entity("main")
                    .borrow_mut()
                    .set_current_animation("p_rwalk");
            }
            KeyState::Pressed(Key::A) => {
                scene
                    .entity("main")
                    .borrow_mut()
                    .set_current_animation_flipped("p_rwalk", true, false);
            }
            KeyState::Pressed(Key::D) => {
                scene
                    .entity("main")
                    .borrow_mut()
                    .set_current_animation("p_rwalk");
            }

            // ------------------------------------------------
            KeyState::Released(Key::W) => {
                scene
                    .entity("main")
                    .borrow_mut()
                    .set_current_animation("p_ridle");
            }
            KeyState::Released(Key::S) => {
                scene
                    .entity("main")
                    .borrow_mut()
                    .set_current_animation("p_ridle");
            }
            KeyState::Released(Key::A) => {
                scene
                    .entity("main")
                    .borrow_mut()
                    .set_current_animation("p_ridle");
            }
            KeyState::Released(Key::D) => {
                scene
                    .entity("main")
                    .borrow_mut()
                    .set_current_animation("p_ridle");
            }

            // ------------------------------------------------
            KeyState::Pressed(Key::Space) => {
                scene
                    .entity("main")
                    .borrow_mut()
                    .set_current_animation("p_rattack1");
            }
            KeyState::Pressed(Key::LShift) => {
                scene
                    .entity("main")
                    .borrow_mut()
                    .set_current_animation("p_rattack2");
            }

            _ => {}
        
    }
}
