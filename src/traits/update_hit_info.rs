use amethyst::ecs::{Component, Entity, SystemData};

pub enum HitType {
    Attack,
    Damage,
}

// ヒット情報更新に関連する情報
pub trait UpdateHitInfoType: 'static + Send + Sync + Default + Component {
    type Paramater; // 更新に必要なパラメータ
    type CancelInfo; // ヒット情報更新時に他のエンティティの情報をキャンセルするための情報

    fn check_hit_type(param1: &Self::Paramater, param2: &Self::Paramater) -> HitType;
}

// ダメージ，攻撃ヒット時の情報を更新する
pub trait UpdateHitInfo<'s>: UpdateHitInfoType {
    type SystemData: SystemData<'s>; // ダメージ補正などヒット情報に必要な情報

    // ヒット情報の更新．
    // ダメージの上書きや他ダメージによる攻撃，ダメージのキャンセルのための情報を返す
    fn attack_update(
        &mut self,
        damage_owner: Entity, //
        attack_param: &Self::Paramater,
        damage_param: &Self::Paramater,
        data: &Self::SystemData,
    ) -> Vec<(Entity, Self::CancelInfo)>;

    fn damage_update(
        &mut self,
        attack_owner: Entity,
        attack_param: &Self::Paramater,
        damage_param: &Self::Paramater,
        data: &Self::SystemData,
    ) -> Vec<(Entity, Self::CancelInfo)>;

    // 他エンティティの更新時にキャンセルされた場合に呼び出す．
    fn cancel(&mut self, targeted: Entity, cancel_info: Self::CancelInfo);
}
