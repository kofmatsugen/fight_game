use serde::{
    de::{SeqAccess, Visitor},
    ser::{SerializeSeq, Serializer},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

bitflags::bitflags! {
    // アニメーション遷移条件
    #[derive(Default)]
    pub struct Condition : u64 {
        const KNOCKBACK = 1 << 0;   // ノックバック中
        const AIR = 1 << 1;         // 空中
    }
}

impl<'de> Deserialize<'de> for Condition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ConditionVisitor)
    }
}

struct ConditionVisitor;

// 条件フラグをデシリアライズする用のenum
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
enum ConditionValue {
    Knockback,
    Air,
}

// シリアライズ用フラグ優先順位
const SERIALIZE_FLAGS: [(Condition, ConditionValue); 2] = [
    (Condition::KNOCKBACK, ConditionValue::Knockback),
    (Condition::AIR, ConditionValue::Air),
];

impl ConditionValue {
    fn convert_flag(self) -> Condition {
        match self {
            ConditionValue::Knockback => Condition::KNOCKBACK,
            ConditionValue::Air => Condition::AIR,
        }
    }

    fn from_flag(mut flag: Condition) -> Vec<Self> {
        SERIALIZE_FLAGS
            .iter()
            .filter_map(|&(f, v)| {
                let val = if flag.contains(f) == true {
                    Some(v)
                } else {
                    None
                };

                flag.remove(f);

                val
            })
            .collect()
    }
}

impl<'de> Visitor<'de> for ConditionVisitor {
    type Value = Condition;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("not supported format")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut cancel = Condition::empty();
        while let Some(elem) = seq.next_element()? {
            let flag = ConditionValue::convert_flag(elem);
            cancel |= flag;
        }
        Ok(cancel)
    }
}

impl Serialize for Condition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let values = ConditionValue::from_flag(*self);
        let mut seq = serializer.serialize_seq(Some(values.len()))?;
        for v in values {
            seq.serialize_element(&v)?;
        }
        seq.end()
    }
}
