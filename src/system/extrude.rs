use crate::traits::ExtrudeFilter;
use amethyst::core::{
    ecs::{Entity, Read, ReaderId, System, Write, WriteStorage},
    Time, Transform,
};
use amethyst_aabb::{
    event::{ContactEvent, ContactEventChannel},
    types::Vector,
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
    T: 'static + Send + Sync + ExtrudeFilter<'s>,
{
    type SystemData = (
        WriteStorage<'s, Transform>,
        Write<'s, ContactEventChannel<T>>,
        Read<'s, Time>,
        T::SystemData,
    );

    fn run(&mut self, (mut transforms, mut channel, time, filter_params): Self::SystemData) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("extrude");
        if self.reader.is_none() == true {
            self.reader = channel.register_reader().into();
        }

        for &ContactEvent {
            entity1,
            entity2,
            delta,
            ..
        } in channel.read(self.reader.as_mut().unwrap()).filter(
            |ContactEvent {
                 entity1,
                 entity2,
                 args1,
                 args2,
                 ..
             }| T::extrude_filter(*entity1, args1, *entity2, args2, &filter_params),
        ) {
            // 格ゲーでは押し出し判定は横方向のみ
            let extrude_length = delta / 2.;
            log::trace!(
                "[{} F] extrude: normal = [{}, {}] ",
                time.frame_number(),
                extrude_length.x,
                extrude_length.y,
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
    transform.append_translation_xyz(extrude_length.x, 0., 0.);
    Some(())
}
