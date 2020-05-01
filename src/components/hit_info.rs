use crate::{
    paramater::{CollisionParamater, CollisionType},
    traits::{HitType, UpdateHitInfo, UpdateHitInfoType},
};
use amethyst::ecs::{Component, DenseVecStorage, Entity};

// 攻撃側の押し出し判定以外の接触判定をまとめるコンポーネント
pub struct HitInfo {
    // ダメージを与えた対象(複数にダメージを与える可能性があるのでvector)
    pub(crate) damaged_owners: Vec<Entity>,

    // 攻撃してきた相手
    pub(crate) attack_owner: Option<Entity>,
}

impl Component for HitInfo {
    type Storage = DenseVecStorage<Self>;
}

impl Default for HitInfo {
    fn default() -> Self {
        HitInfo {
            damaged_owners: Vec::with_capacity(16),
            attack_owner: None,
        }
    }
}

impl UpdateHitInfoType for HitInfo {
    type Paramater = CollisionParamater;
    type CancelInfo = ();

    fn check_hit_type(param1: &Self::Paramater, param2: &Self::Paramater) -> HitType {
        match (param1.collision_type, param2.collision_type) {
            (CollisionType::Damaged, CollisionType::Blow { .. }) => HitType::Damage,
            (CollisionType::Damaged, CollisionType::Projectile { .. }) => HitType::Damage,
            (CollisionType::Damaged, CollisionType::Throw) => HitType::Damage,
            (CollisionType::Blow { .. }, CollisionType::Damaged) => HitType::Attack,
            (CollisionType::Projectile { .. }, CollisionType::Damaged) => HitType::Attack,
            (CollisionType::Throw, CollisionType::Damaged) => HitType::Attack,
            _ => unreachable!(
                "{:?} vs {:?} hit type undefined",
                param1.collision_type, param2.collision_type
            ),
        }
    }
}

impl<'s> UpdateHitInfo<'s> for HitInfo {
    type SystemData = ();

    // ヒット情報の更新．
    // ダメージの上書きや他ダメージによる攻撃，ダメージのキャンセルのための情報を返す
    fn attack_update(
        &mut self,
        damage_owner: Entity, //
        _attack_param: &Self::Paramater,
        _damage_param: &Self::Paramater,
        (): &Self::SystemData,
    ) -> Vec<(Entity, Self::CancelInfo)> {
        log::debug!("update attack: {:?}", damage_owner);
        vec![]
    }

    fn damage_update(
        &mut self,
        attack_owner: Entity,
        _attack_param: &Self::Paramater,
        _damage_param: &Self::Paramater,
        (): &Self::SystemData,
    ) -> Vec<(Entity, Self::CancelInfo)> {
        log::debug!("update damage: {:?}", attack_owner);
        vec![]
    }

    // 他エンティティの更新時にキャンセルされた場合に呼び出す．
    fn cancel(&mut self, targeted: Entity, _cancel_info: Self::CancelInfo) {
        // 対象に攻撃した，された場合はキャンセル
        if self.attack_owner.map(|e| e == targeted).unwrap_or(false) {
            // 攻撃主情報破棄
            self.attack_owner = None;
        }
        if self.damaged_owners.contains(&targeted) {
            // 被ダメージ情報破棄
            self.damaged_owners.clear();
        }
    }
}
