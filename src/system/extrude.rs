use amethyst::core::{
    components::Transform,
    ecs::{Entity, ReaderId, System, Write, WriteStorage},
};
use amethyst_aabb::{
    event::{ContactEvent, ContactEventChannel},
    types::{Contact, Vector},
};

// 押出処理
pub struct ExtrudeSystem<T>
where
    T: 'static + Send + Sync,
{
    reader: Option<ReaderId<ContactEvent<T>>>,
}

impl<T> ExtrudeSystem<T>
where
    T: 'static + Send + Sync,
{
    pub fn new() -> Self {
        ExtrudeSystem { reader: None }
    }
}

impl<'s, T> System<'s> for ExtrudeSystem<T>
where
    T: 'static + Send + Sync,
{
    type SystemData = (
        WriteStorage<'s, Transform>,
        Write<'s, ContactEventChannel<T>>,
    );

    fn run(&mut self, (mut transforms, mut channel): Self::SystemData) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("extrude");
        if self.reader.is_none() == true {
            self.reader = channel.register_reader().into();
        }

        for &ContactEvent {
            entity1,
            entity2,
            contact:
                Contact {
                    depth,
                    normal,
                    world1,
                    world2,
                },
            ..
        } in channel.read(self.reader.as_mut().unwrap())
        {
            // 格ゲーでは押し出し判定は横方向のみ
            let extrude_length = normal.into_inner() * depth;
            log::info!(
                "extrude: normal = [{}, {}] depth = {}",
                normal.x,
                normal.y,
                depth,
            );
            log::info!(
                "extrude: point1 = ({}, {}), point2 = ({}, {})",
                world1.x,
                world1.y,
                world2.x,
                world2.y,
            );
            extrude(&mut transforms, entity1, -extrude_length);
            extrude(&mut transforms, entity2, extrude_length);
        }
    }
}

fn extrude(
    transforms: &mut WriteStorage<Transform>,
    e: Entity,
    extrude_length: Vector,
) -> Option<()> {
    let transform = transforms.get_mut(e)?;
    transform.translation_mut().x += extrude_length.x;
    transform.translation_mut().y += extrude_length.y;
    Some(())
}
