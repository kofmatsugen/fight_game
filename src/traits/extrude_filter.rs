use amethyst::ecs::{Entity, SystemData};

pub trait ExtrudeFilter<'s> {
    type SystemData: SystemData<'s>;

    // 押し出し判定を行うフィルタ
    fn extrude_filter(
        _entity1: Entity,
        _p1: &Self,
        _entity2: Entity,
        _p2: &Self,
        _data: &Self::SystemData,
    ) -> bool {
        true
    }
}
