use crate::traits::ExtrudeFilter;
use amethyst::core::ecs::{Entity, ReaderId, System, Write, WriteStorage};
use amethyst_aabb::{
    event::{ContactEvent, ContactEventChannel},
    types::{Contact, Vector},
};
use movement_transform::components::Movement;

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
        WriteStorage<'s, Movement>,
        Write<'s, ContactEventChannel<T>>,
        T::SystemData,
    );

    fn run(&mut self, (mut movements, mut channel, filter_params): Self::SystemData) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("extrude");
        if self.reader.is_none() == true {
            self.reader = channel.register_reader().into();
        }

        for &ContactEvent {
            entity1,
            entity2,
            contact: Contact { depth, normal, .. },
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
            let extrude_length = normal.into_inner() * depth / 2.;
            log::info!(
                "extrude: normal = [{}, {}] depth = {}",
                normal.x,
                normal.y,
                depth,
            );
            extrude(&mut movements, entity1, -extrude_length);
            extrude(&mut movements, entity2, extrude_length);
        }
    }
}

fn extrude(
    movements: &mut WriteStorage<Movement>,
    e: Entity,
    extrude_length: Vector,
) -> Option<()> {
    let movement = movements.get_mut(e)?;
    movement
        .transform_mut()
        .append_translation_xyz(extrude_length.x, extrude_length.y, 0.);
    Some(())
}
