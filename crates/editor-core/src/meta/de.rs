//! [`from_value`] — the serde-native producer boundary, deserialize
//! side (spec D7): a typed view is rebuilt from the canonical
//! [`MetaValue`] tree. The tree is self-describing, so this is a
//! straight `deserialize_any` forwarding: each node visits as exactly
//! what it is; producer `Deserialize` impls do their own shape
//! checking and report their own errors.

use serde::de::{self, Deserializer, IntoDeserializer, Visitor};

use super::{MetaError, MetaValue};

/// Rebuilds a typed producer view from a [`MetaValue`] tree (spec
/// D7's `from_value` boundary).
///
/// # Errors
///
/// [`MetaError::Message`] carrying the producer `Deserialize` impl's
/// refusal (wrong shape, unknown variant, missing field, …).
pub fn from_value<'de, T: serde::Deserialize<'de>>(value: &'de MetaValue) -> Result<T, MetaError> {
    T::deserialize(ValueDe(value))
}

#[derive(Clone, Copy)]
struct ValueDe<'de>(&'de MetaValue);

impl<'de> IntoDeserializer<'de, MetaError> for ValueDe<'de> {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self {
        self
    }
}

impl<'de> Deserializer<'de> for ValueDe<'de> {
    type Error = MetaError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, MetaError> {
        match self.0 {
            MetaValue::Null => visitor.visit_unit(),
            MetaValue::Bool(b) => visitor.visit_bool(*b),
            MetaValue::Int(i) => visitor.visit_i64(*i),
            MetaValue::Float(f) => visitor.visit_f64(*f),
            MetaValue::Str(s) => visitor.visit_borrowed_str(s),
            MetaValue::Bytes(b) => visitor.visit_borrowed_bytes(b),
            MetaValue::List(items) => {
                let mut seq = de::value::SeqDeserializer::new(items.iter().map(ValueDe));
                let out = visitor.visit_seq(&mut seq)?;
                seq.end()?;
                Ok(out)
            }
            MetaValue::Map(entries) => {
                let mut map = de::value::MapDeserializer::new(
                    entries.iter().map(|(k, v)| (k.as_str(), ValueDe(v))),
                );
                let out = visitor.visit_map(&mut map)?;
                map.end()?;
                Ok(out)
            }
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, MetaError> {
        // `Null` is absence (the ser side maps `None` to `Null`);
        // anything else is a present value.
        match self.0 {
            MetaValue::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, MetaError> {
        // External tagging, mirroring the ser side: `Str(variant)` for
        // unit variants, one-entry `Map { variant: payload }` for the
        // rest.
        match self.0 {
            MetaValue::Str(s) => visitor.visit_enum(s.as_str().into_deserializer()),
            MetaValue::Map(entries) if entries.len() == 1 => {
                // len == 1 guarantees the entry exists; avoid indexing.
                let Some((tag, payload)) = entries.iter().next() else {
                    return Err(MetaError::Message("empty variant map".into()));
                };
                visitor.visit_enum(EnumDe {
                    tag,
                    payload: ValueDe(payload),
                })
            }
            _ => Err(MetaError::Message(
                "expected a variant string or a one-entry variant map".into(),
            )),
        }
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, MetaError> {
        visitor.visit_newtype_struct(self)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str
        string bytes byte_buf unit unit_struct seq tuple tuple_struct
        map struct identifier ignored_any
    }
}

struct EnumDe<'de> {
    tag: &'de str,
    payload: ValueDe<'de>,
}

impl<'de> de::EnumAccess<'de> for EnumDe<'de> {
    type Error = MetaError;
    type Variant = ValueDe<'de>;
    fn variant_seed<S: de::DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, ValueDe<'de>), MetaError> {
        let tag = seed.deserialize(self.tag.into_deserializer())?;
        Ok((tag, self.payload))
    }
}

impl<'de> de::VariantAccess<'de> for ValueDe<'de> {
    type Error = MetaError;
    fn unit_variant(self) -> Result<(), MetaError> {
        match self.0 {
            MetaValue::Null => Ok(()),
            _ => Err(MetaError::Message("expected no variant payload".into())),
        }
    }
    fn newtype_variant_seed<S: de::DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<S::Value, MetaError> {
        seed.deserialize(self)
    }
    fn tuple_variant<V: Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, MetaError> {
        self.deserialize_any(visitor)
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, MetaError> {
        self.deserialize_any(visitor)
    }
}
