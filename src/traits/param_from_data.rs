use amethyst::ecs::{Entity, SystemData};

pub trait ParamaterFromData<'s, T>: Sized {
    // 現在の情報も付与してデータを作成する
    type SystemData: SystemData<'s>;
    fn make_collision_data(
        entity: Entity,
        data: Option<&T>,
        system_data: &Self::SystemData,
    ) -> Option<Self>;
}

impl<'s, T> ParamaterFromData<'s, T> for () {
    type SystemData = ();
    fn make_collision_data(_: Entity, _: Option<&T>, (): &Self::SystemData) -> Option<()> {
        Some(())
    }
}
