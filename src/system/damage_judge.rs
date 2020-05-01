use crate::traits::{ExtrudeFilter, HitType, UpdateHitInfo, UpdateHitInfoType};
use amethyst::core::ecs::{Entity, ReaderId, System, Write, WriteStorage};
use amethyst_aabb::event::{ContactEvent, ContactEventChannel};

// ダメージ処理をするための情報を統合するシステム
pub struct DamageJudgeSystem<H>
where
    H: UpdateHitInfoType,
{
    reader: Option<ReaderId<ContactEvent<H::Paramater>>>,
}

impl<H> DamageJudgeSystem<H>
where
    H: UpdateHitInfoType,
{
    pub fn new() -> Self {
        DamageJudgeSystem { reader: None }
    }
}

impl<'s, H> System<'s> for DamageJudgeSystem<H>
where
    H: UpdateHitInfo<'s>,
    H::Paramater: ExtrudeFilter<'s>,
{
    type SystemData = (
        Write<'s, ContactEventChannel<H::Paramater>>,
        WriteStorage<'s, H>,
        <H::Paramater as ExtrudeFilter<'s>>::SystemData,
        H::SystemData,
    );

    fn run(&mut self, (mut channel, mut hits, filter_params, hit_info_params): Self::SystemData) {
        if self.reader.is_none() == true {
            self.reader = channel.register_reader().into();
        }

        for ContactEvent {
            entity1,
            entity2,
            args1,
            args2,
            ..
        } in channel.read(self.reader.as_mut().unwrap()).filter(
            |ContactEvent {
                 entity1,
                 entity2,
                 args1,
                 args2,
                 ..
             }| {
                // 押出判定を行わないもの限定
                <H::Paramater as ExtrudeFilter<'s>>::extrude_filter(
                    *entity1,
                    args1,
                    *entity2,
                    args2,
                    &filter_params,
                ) == false
            },
        ) {
            let (attack, damage, attack_param, damage_param) = match H::check_hit_type(args1, args2)
            {
                HitType::Attack => (*entity1, *entity2, args1, args2),
                HitType::Damage => (*entity2, *entity1, args2, args1),
            };

            match update_info(
                attack,
                damage,
                attack_param,
                damage_param,
                &hit_info_params,
                &mut hits,
            ) {
                Ok(_) => {}
                Err(err) => log::error!("update info error: {:?}", err),
            }
        }
    }
}

fn update_info<'s, H>(
    attack: Entity,
    damage: Entity,
    attack_param: &H::Paramater,
    damage_param: &H::Paramater,
    data: &H::SystemData,
    hits: &mut WriteStorage<H>,
) -> amethyst::Result<()>
where
    H: UpdateHitInfo<'s>,
{
    let mut attack_hit_info = H::default();
    let damage_cancels = attack_hit_info.attack_update(damage, attack_param, damage_param, data);
    hits.insert(attack, attack_hit_info)?;

    let mut damage_hit_info = H::default();
    let attack_cancels = damage_hit_info.damage_update(damage, attack_param, damage_param, data);
    hits.insert(damage, damage_hit_info)?;

    for (e, cancel) in damage_cancels {
        if let Some(hit) = hits.get_mut(e) {
            hit.cancel(attack, cancel);
        }
    }
    for (e, cancel) in attack_cancels {
        if let Some(hit) = hits.get_mut(e) {
            hit.cancel(damage, cancel);
        }
    }

    Ok(())
}
