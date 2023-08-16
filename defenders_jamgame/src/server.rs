use ambient_api::{
    core::{
        primitives::components::cube,
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

use embers::defenders_jamgame::{
    components::{is_walker, global_score},
    messages::DeleteGuy,
};

#[main]
pub fn main() {

    Entity::new()
        .with(global_score(), 0)
        .spawn();

    query((translation(), is_walker())).each_frame(|walkers|for(walker,(pos,_))in walkers{
        entity::mutate_component(walker, translation(), |pos|*pos+=vec3(1., 1., 0.)*delta_time().max(0.1)*0.25);
        if pos.x > 10. {
            entity::despawn(walker);
        }
    });

    run_async(async { loop {
        spawn_walker();
        sleep(0.1).await;
    }});
    
    DeleteGuy::subscribe(|_src,msg|{
        entity::despawn(msg.guy);
    });
}

fn spawn_walker() -> EntityId {
    let bkwds = 5.;
    let sideways = random::<f32>() * 10. - 5.;
    Entity::new()
        .with_merge(make_transformable()).with(translation(), vec3(-bkwds - sideways, -bkwds + sideways, 0.))
        .with_default(cube())
        .with_default(is_walker())
        .spawn()
}