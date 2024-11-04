### Experimental personal project to learn Rust and Game dev basics simultaneously.


### Learning sources
 - [LEARN RUST](https://www.rust-lang.org/learn) WOULD BE ENOUGH. [BUT THERE IS ALSO THIS](https://github.com/ctjhoa/rust-learning) WITH EXTRA RESOURCES
 - [SFML & SFML DOCUMENTATION](https://github.com/jeremyletang/rust-sfml)
 - [SETTING UP SFML](https://www.youtube.com/watch?v=nnojR-8PT4M&t=747s)
 - TO HAVE AN IDEA ABOUT [HOW GAME ENGINES WORK](https://www.youtube.com/playlist?list=PLs6oRBoE2-Q_fX_rzraQekRoL7Kr7s5xi)

### Samples

```Rust
 let mut player = Entity::new("main"); // Defines a player

 let mut scn = Scene::new("main"); // Defines a scene

 scn.load_texture("player", "assets/Warrior_Yellow.png"); // Loads texture with id of "player" from filepath "assets/Warrior_Yellow.png" to scene

 let p_ridle = Animation::new_loop("player", 100.0, (192, 192), Some((0, 5)));  // Creates a looped animation, each frame is 192x192px and loops between 0, to 5 (rectangles indx. in texture) each frame stays 100ms

 let def = PhysicalProperties::default(); // Create default physics

 player.set_physics(PhysicalProperties { // Set physical properties of player to simulate
        collision_enabled: true,
        show_collider: true,
        show_solid_box: true,
        solid_box_margin: (60.0, 60.0),
        collision_type: CollisionType::Vectoral,
        speed: 0.3,
        mass:1.0,
        ..def.clone() // fill other properties same as default
    });

 player.add_animation("p_ridle", p_ridle); // Add animation to player

 scn.add_entity(player); // Add player to screen

 let mut game = Game::new("Game", Style::CLOSE, scn); // Create game with active scene.
 game.run((900, 600)); // Run game with window size

/*
Thre are also some events like:

scn.on_keystate_changed,
scn.on_update,
scn.on_collision,
scn.on_start,

And physics is simulated using Axis Aligned Box collisions.
*/

```
