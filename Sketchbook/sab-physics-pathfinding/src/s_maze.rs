use pathfinding::grid::Grid;
use pathfinding::directed::bfs::*;

use crate::sab_physics_pathfinding::components::*;

pub fn setup() {
    let mut grid : Grid = Grid::new(9, 9);
    for x in 0..grid.width {
        for y in 0..grid.height {
            if random::<f32>()<0.65 {
                grid.add_vertex((x, y));
            }
        }
    }
    grid.add_borders();

    for x in 0..grid.width {
        for y in 0..grid.height {
            if !grid.has_vertex((x,y)) {
                spawn_maze_wall((x,y));
            }
        }
    }
    println!("{grid:?}");
    // for v in grid.iter() {
    //     spawn_maze_wall(v);
    // }

    query((translation(), roller_steer(), roller_targetcell())).each_frame(move |rollers|{
        for (roller, (pos, steer, goal)) in rollers {
            let cell = get_position_cell(pos);
            if let Some(nextcell) = entity::get_component(roller, roller_nextcell()) {
                let nextcell : IVec2 = nextcell;
                let nextpos = nextcell.extend(0).as_vec3();
                let to_nextpos = nextpos - pos;
                let to_dist = to_nextpos.length();
                let speed = 10.0 * delta_time();
                if to_dist <= speed {
                    entity::set_component(roller, translation(), nextpos);
                    entity::remove_component(roller, roller_nextcell());
                } else {
                    entity::set_component(roller, translation(), pos + to_nextpos.normalize()*speed);
                }
            } else {
                if let Some(path) = bfs(
                    &get_position_cell(pos),
                    |v| grid.neighbours(*v),
                    |v| *v == (goal.x as usize, goal.y as usize)
                ) {
                    let path : Vec<(usize,usize)> = path;
                    match path.len() {
                        0 => { println!("Empty path returned (invalid)"); }
                        1 => { println!("At end of path"); }
                        _ => { entity::add_component(roller, roller_nextcell(), vertex_to_ivec2(path[1]) ); }
                    }
                } else {
                    println!("No valid path exists!");
                }
            }
        }
    });
}

fn spawn_maze_wall(v:(usize,usize)) -> EntityId {
    Entity::new()
        .with(translation(), vec3(v.0 as f32, v.1 as f32, 0.5))
        .with(cube(), ())
        .with(cube_collider(), vec3(1., 1., 1.))
        .spawn()
}

fn get_position_cell(pos:Vec3) -> (usize,usize) { (pos.x.round() as usize, pos.y.round() as usize) }

fn vertex_to_ivec2(v:(usize,usize)) -> IVec2 { ivec2(v.0 as i32, v.1 as i32) }

use ambient_api::{prelude::*, core::{transform::components::translation, primitives::components::cube, physics::components::cube_collider}};