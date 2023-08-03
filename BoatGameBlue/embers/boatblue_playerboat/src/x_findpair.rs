use ambient_api::{ecs::{
    ComponentsTuple,
    EventQuery,
}, prelude::EntityId};
use flume;

pub async fn find_first_pair_async<T: ComponentsTuple + Copy, U: ComponentsTuple + Copy> (
    q1 : EventQuery<T>, condforq1 : impl Fn(EntityId) -> bool + 'static,
    q2 : EventQuery<U>, condforq2 : impl Fn(EntityId) -> bool + 'static
) -> (EntityId, EntityId) {
    let (tx1, rx1) = flume::bounded::<EntityId>(1);
    let (tx2, rx2) = flume::bounded::<EntityId>(1);

    let see1 = q1.bind(move |ents| for (id,_) in ents {
        if condforq1(id) {
            tx1.send(id).unwrap(); // unwrap?
            break;
        }
    });

    let see2 = q2.bind(move |ents| for (id,_) in ents {
        if condforq2(id) {
            tx2.send(id).unwrap();
            break;
        }
    });

    let ent1 = rx1.recv_async().await.unwrap();
    see1.stop();
    let ent2 = rx2.recv_async().await.unwrap();
    see2.stop();

    (ent1, ent2)
}