use crate::{
    paramater::{CollisionParamater, CollisionType},
    traits::{HitType, UpdateHitInfo, UpdateHitInfoType},
    types::DamageCollisionId,
};
use amethyst::ecs::{Component, DenseVecStorage, Entity};
use amethyst_sprite_studio::traits::animation_file::AnimationFile;

// 攻撃側の押し出し判定以外の接触判定をまとめるコンポーネント
pub struct HitInfo<T>
where
    T: AnimationFile,
{
    // ダメージを与えた対象(複数にダメージを与える可能性があるのでvector)
    pub(crate) damaged_owners: Vec<Entity>,

    // 攻撃してきた相手
    pub(crate) attack_owner: Option<Entity>,

    // 判定ID(全く同じ攻撃)を受けないために保存
    pub(crate) damage_collision_ids: Vec<DamageCollisionId<T>>,

    // ヒットストップをかけるフレーム数
    pub(crate) hitstop: Option<usize>,

    // ノックバックフレーム
    pub(crate) knockback: Option<usize>,
}

impl<T> Component for HitInfo<T>
where
    T: AnimationFile,
{
    type Storage = DenseVecStorage<Self>;
}

impl<T> Default for HitInfo<T>
where
    T: AnimationFile,
{
    fn default() -> Self {
        HitInfo {
            damaged_owners: Vec::with_capacity(16),
            attack_owner: None,
            damage_collision_ids: Vec::with_capacity(16),
            hitstop: None,
            knockback: None,
        }
    }
}

impl<T> UpdateHitInfoType for HitInfo<T>
where
    T: AnimationFile,
{
    type Paramater = CollisionParamater<T>;
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

impl<'s, T> UpdateHitInfo<'s> for HitInfo<T>
where
    T: AnimationFile,
{
    type SystemData = ();

    // ヒット情報の更新．
    // ダメージの上書きや他ダメージによる攻撃，ダメージのキャンセルのための情報を返す
    fn attack_update(
        &mut self,
        damage_owner: Entity, //
        CollisionParamater {
            collision_type: attack_type,
            damage_collision_id: _attack_collision_id,
        }: &Self::Paramater,
        _damage_param: &Self::Paramater,
        (): &Self::SystemData,
    ) -> Vec<(Entity, Self::CancelInfo)> {
        log::info!(
            "update attack: {:?}, id = {:?}",
            damage_owner,
            _attack_collision_id
        );
        let mut _cancels = Vec::with_capacity(16);

        match attack_type {
            CollisionType::Blow { hit_level, .. } | CollisionType::Projectile { hit_level, .. } => {
                let hitstop = hit_level.hitstop();
                self.hitstop = hitstop.into();
            }
            _ => {}
        }

        _cancels
    }

    fn damage_update(
        &mut self,
        attack_owner: Entity,
        CollisionParamater {
            collision_type: attack_type,
            damage_collision_id: attack_collision_id,
        }: &Self::Paramater,
        _damage_param: &Self::Paramater,
        (): &Self::SystemData,
    ) -> Vec<(Entity, Self::CancelInfo)> {
        log::info!("update damage: {:?}", attack_owner);
        let mut _cancels = Vec::with_capacity(16);
        match attack_type {
            CollisionType::Blow {
                hit_level, ground, ..
            }
            | CollisionType::Projectile {
                hit_level, ground, ..
            } => {
                let hitstop = hit_level.hitstop();
                self.hitstop = hitstop.into();
                self.knockback = ground.frame.into();
                log::info!(
                    "hitstop = {}, knockback = {}, collision_id = {:?}",
                    hitstop,
                    ground.frame,
                    attack_collision_id
                );
            }
            _ => {}
        }
        if let &Some(attack_collision_id) = attack_collision_id {
            self.damage_collision_ids.push(attack_collision_id);
        }

        _cancels
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
