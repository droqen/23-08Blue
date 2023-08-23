use ambient_api::prelude::*;

#[main]
pub fn main() {
    move_message::setup();
    // cube_maze_spawning_v1::setup();
    cube_maze_spawning_v2_maze_generator::setup();
    floor_spawning::setup();
    player_features::setup();
}

mod player_features {
    use crate::packages::this::concepts::make_slowmover;
    use ambient_api::{
        core::player::components::{is_player, user_id},
        prelude::*,
    };
    pub fn setup() {
        spawn_query((is_player(), user_id())).bind(|players| {
            for (player, (_, uid)) in players {
                entity::add_components(player, make_slowmover());
            }
        });
    }
}

mod floor_spawning {
    use ambient_api::{
        core::{
            primitives::components::quad,
            rendering::components::color,
            transform::{components::scale, concepts::make_transformable},
        },
        prelude::*,
    };
    pub fn setup() {
        Entity::new()
            .with_merge(make_transformable())
            .with(quad(), ())
            .with(color(), vec4(0.5, 0.5, 0.5, 1.0))
            .with(
                scale(),
                (ivec2(crate::MAZE_WIDTH, crate::MAZE_HEIGHT).as_vec2() * 2.0 + 1.0).extend(1.0),
            )
            .spawn();
    }
}

mod mover_prediction;

const BODY_HIT_RADIUS: f32 = 0.25;

mod move_message {
    use crate::{
        mover_prediction,
        packages::{
            this::components::{mover_pos_end, mover_pos_start, mover_time_start},
            this::messages::PlayerMove,
        },
        BODY_HIT_RADIUS,
    };
    use ambient_api::prelude::*;
    pub fn setup() {
        println!("PlayerMove setup");
        PlayerMove::subscribe(|src, msg| {
            if let Some(player_mover) = src.client_entity_id() {
                let start = mover_prediction::get_mover_pos2(player_mover, 0.0);
                let target = msg.target;
                let to_target = target - start;

                // use raycast to determine where the actual end-of-movement is
                let hit_option =
                    physics::raycast_first(start.extend(0.), (target - start).extend(0.));
                let end = match hit_option {
                    Some(hit) => {
                        let to_hit: Vec3 = hit.position - start.extend(0.);
                        let to_hit_dist: f32 = to_hit.length();
                        if hit.distance <= BODY_HIT_RADIUS {
                            start
                        } else {
                            start
                                + (to_hit.normalize()
                                    * to_target.length().min(to_hit_dist - BODY_HIT_RADIUS))
                                .truncate()
                        }
                    }
                    _ => target,
                };

                println!("PlayerMove received");

                entity::add_components(
                    player_mover,
                    Entity::new()
                        .with(mover_pos_start(), start)
                        .with(mover_pos_end(), end)
                        .with(mover_time_start(), game_time().as_secs_f32()),
                );
            } else {
                println!("PlayerMove received, but no player found");
                // error - no player found there
            }
        });
    }
}

mod cube_maze_spawning_v1 {
    use ambient_api::{
        core::{
            physics::components::cube_collider, primitives::components::cube,
            transform::components::translation,
        },
        prelude::*,
    };
    pub fn setup() {
        // spawn maze
        for x in -10..10 + 1 {
            for y in -10..10 + 1 {
                if random::<f32>() < 0.5 && ((x as i32).abs() + (y as i32).abs() > 2) {
                    Entity::new()
                        .with(cube(), ())
                        .with(cube_collider(), Vec3::splat(1.))
                        .with(translation(), ivec2(x, y).as_vec2().extend(0.5))
                        .spawn();
                }
            }
        }
    }
}

const MAZE_WIDTH: i32 = 15;
const MAZE_HEIGHT: i32 = 11;

mod cube_maze_spawning_v2_maze_generator {
    use ambient_api::{
        core::{
            physics::components::cube_collider, primitives::components::cube,
            transform::components::translation,
        },
        prelude::*,
    };
    use maze_generator::prelude::*;
    use maze_generator::recursive_backtracking::RbGenerator;

    pub fn setup() {
        // let mut generator = RbGenerator::new(Some([42; 32]));
        let mut generator = RbGenerator::new(None);
        let maze = generator
            .generate(crate::MAZE_WIDTH, crate::MAZE_HEIGHT)
            .unwrap();
        println!("maze:\n{:?}", maze);
        // spawn maze
        for x in -1..2 * maze.size.0 {
            for y in -1..2 * maze.size.1 {
                let mut wall = true;
                if x >= 0 && x <= 2 * (maze.size.0 - 1) && y >= 0 && y <= 2 * (maze.size.1 - 1) {
                    // inside boundaries
                    let xongrid = (x % 2 == 0);
                    let yongrid = (y % 2 == 0);
                    if xongrid != yongrid {
                        let mazex = (x as f32 / 2.).floor() as i32;
                        let mazey = (y as f32 / 2.).floor() as i32;
                        if let Some(field) = maze.get_field(&Coordinates { x: mazex, y: mazey }) {
                            wall = !field.has_passage(match xongrid {
                                true => &Direction::South,
                                false => &Direction::East,
                            }); // cut passage
                        }

                        if wall && random::<f32>() < 0.2 {
                            wall = false; // cut a few random passages.
                        }
                    } else {
                        wall = !xongrid;
                    }
                }

                if wall {
                    Entity::new()
                        .with(cube(), ())
                        .with(cube_collider(), Vec3::splat(1.))
                        .with(
                            translation(),
                            (ivec2(x, y).as_vec2() - vec2(maze.size.0 as f32, maze.size.1 as f32)
                                + 1.)
                                .extend(0.5),
                        )
                        .spawn();
                }
            }
        }
    }
}
