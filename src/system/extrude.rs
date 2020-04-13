use crate::paramater::{CollisionType, ContactParamter};
use amethyst::core::{
    components::Transform,
    ecs::{Entity, ReaderId, System, Write, WriteStorage},
};
use amethyst_collision::events::{ContactEvent, ContactEvents};

type EventChannel = ContactEvents<ContactParamter>;
type Event = ContactEvent<ContactParamter>;

// 押出処理
pub struct ExtrudeSystem {
    reader: Option<ReaderId<Event>>,
}

impl ExtrudeSystem {
    pub fn new() -> Self {
        ExtrudeSystem { reader: None }
    }
}

impl<'s> System<'s> for ExtrudeSystem {
    type SystemData = (WriteStorage<'s, Transform>, Write<'s, EventChannel>);

    fn run(&mut self, (mut transforms, mut channel): Self::SystemData) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("extrude");
        if self.reader.is_none() == true {
            self.reader = channel.register_reader().into();
        }

        for Event {
            entity1,
            entity2,
            normal,
            depth,
            ..
        } in
            channel
                .read(self.reader.as_mut().unwrap())
                .filter(|Event { args1, args2, .. }| {
                    match (&args1.collision_type, &args2.collision_type) {
                        (CollisionType::Extrusion, CollisionType::Extrusion) => true,
                        _ => false,
                    }
                })
        {
            // 格ゲーでは押し出し判定は横方向のみ
            let extrude_length = normal.x * depth;
            log::info!("extrude: {} ({:?}, {:?})", extrude_length, entity1, entity2);
            extrude(&mut transforms, *entity1, -extrude_length);
            extrude(&mut transforms, *entity2, extrude_length);
        }
    }
}

fn extrude(transforms: &mut WriteStorage<Transform>, e: Entity, extrude_length: f32) -> Option<()> {
    let transform = transforms.get_mut(e)?;
    transform.translation_mut().x += extrude_length;
    Some(())
}
