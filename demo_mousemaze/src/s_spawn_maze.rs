use ambient_api::{
    prelude::*,
    core::transform::components::translation,
};
use crate::embers::ww_gen::components::{block_size, block_door_side, block_decor_cube, };
use pathfinding::grid::Grid;
pub fn setup() {
    let mut grid = Grid::new(15, 15);
    mkgrid_citi(&mut grid);

    for ver in grid.iter() {
        let mut mousehole : Option<u8> = None;

        if random::<f32>() < 0.10 {
            if !grid.has_vertex((ver.0+1, ver.1)) && ver.0+1 < grid.width { mousehole = Some(0); }
            if !grid.has_vertex((ver.0, ver.1+1)) && ver.1+1 < grid.height { mousehole = Some(1); }
        }

        let mut block = Entity::new()
            .with(translation(), (vec2(ver.0 as f32, ver.1 as f32) * vec2(6.0, 6.0)).extend(0.0))
            .with(block_size(), vec2(6.0, 6.0).extend(10.0) )
            .with(block_decor_cube(), ())
            ;
        if mousehole.is_some() { block = block.with(block_door_side(), mousehole.unwrap()); }
        block.spawn();
    }

//     Entity::new()
//         .with(translation(), vec3(0., 0., 0.))
//         .with(block_size(), vec3(6.0, 6.0, 10.0))
//         .with(block_decor_cube(), ())
//         .spawn();

//     Entity::new()
//         .with(translation(), vec3(6.0, -6.0, 0.))
//         .with(block_size(), vec3(6.0, 6.0, 10.0))
//         .with(block_door_side(), 0)
//         .with(block_decor_cube(), ())
//         .spawn();

//     Entity::new()
//         .with(translation(), vec3(12.0, -12.0, 0.))
//         .with(block_size(), vec3(6.0, 6.0, 10.0))
//         .with(block_door_side(), 1)
//         .with(block_decor_cube(), ())
//         .spawn();

}

fn mkgrid_bord(mut grid : &mut Grid) {
    grid.add_borders();
    for x in 0..15 { for y in 0..15 { 
        if random::<f32>()<0.75 {
            grid.add_vertex((x,y));
        }
    }}
}

fn mkgrid_citi(mut grid : &mut Grid) {
    for x in 0..15 { for y in 0..15 {
        let mut score = 0;
        // if random::<f32>()<0.25 { score -= 1; }
        if random::<f32>()<0.25 { score += 1; }
        if x%2==0 { score += 1; }
        if y%2==0 { score += 1; }
        if score >= 2 {
            grid.add_vertex((x,y));
        }
    }}
}