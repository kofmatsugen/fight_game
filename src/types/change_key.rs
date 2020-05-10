use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use serde::{Deserialize, Serialize};

// アニメーションデータから遷移するためのデータ
// コマンド入力より上位
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ChangeKey<T>
where
    T: AnimationFile,
{
    // アニメーションパック指定(同じパック内の場合は省略可)
    pub(crate) pack: Option<T::PackKey>,
    // アニメーションキー指定(必須)
    pub(crate) animation: T::AnimationKey,
}
