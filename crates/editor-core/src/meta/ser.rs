//! [`to_value`] — the serde-native producer boundary, serialize side
//! (spec D7): any `T: Serialize` erases into the canonical
//! [`MetaValue`] tree. Faithful mapping: integers stay exact `Int`
//! (overflow refused), floats stay `Float` (non-finite refused at the
//! boundary — D2), `serialize_bytes` becomes `Bytes`, structs and maps
//! become `Map` (string keys only), sequences become `List`.
//! Enum representation mirrors serde's external tagging: unit variant
//! → `Str(name)`, newtype/tuple/struct variant → one-entry
//! `Map { name: payload }` — the same self-describing shape the file
//! format uses, so a producer's erased tree reads naturally.

use std::collections::BTreeMap;

use serde::Serialize;
use serde::ser::{self, Serializer};

use super::{MAX_NESTING, MAX_PRODUCER_NESTING, MetaError, MetaValue};

/// Erases a producer value into the canonical [`MetaValue`] tree
/// (spec D7's `to_value` boundary).
///
/// # Errors
///
/// [`MetaError`] on out-of-`i64` integers, non-finite floats,
/// non-string map keys, a value that would nest past
/// [`MAX_NESTING`](super::MAX_NESTING), a producer that nests past
/// [`MAX_PRODUCER_NESTING`](super::MAX_PRODUCER_NESTING) however little
/// of it the value keeps, or a producer `Serialize` impl's own error.
/// Either depth is refused before the producer is read any deeper, so
/// a producer of any depth, a self-referential one included, is read
/// only that far.
pub fn to_value<T: Serialize>(value: &T) -> Result<MetaValue, MetaError> {
    value.serialize(ValueSer { level: 1, read: 1 })
}

/// The serializer for a value at `level` of the tree, the root at 1,
/// reached through `read` nested `serialize` calls, the root's
/// included. An option or a newtype is read through without a level of
/// its own, so `read` is never below `level`.
#[derive(Clone, Copy)]
struct ValueSer {
    level: usize,
    read: usize,
}

impl ValueSer {
    /// The serializer for a producer this one reads through, at the
    /// same level, refused past [`MAX_PRODUCER_NESTING`] before it is
    /// read.
    fn through(self) -> Result<Self, MetaError> {
        if self.read >= MAX_PRODUCER_NESTING {
            return Err(MetaError::ProducerTooDeep {
                bound: MAX_PRODUCER_NESTING,
            });
        }
        Ok(Self {
            read: self.read + 1,
            ..self
        })
    }

    /// The serializer for a child of a value at this level, refused
    /// past [`MAX_NESTING`] or [`MAX_PRODUCER_NESTING`] before the
    /// child is read.
    fn child(self) -> Result<Self, MetaError> {
        if self.level >= MAX_NESTING {
            return Err(MetaError::NestedTooDeep { bound: MAX_NESTING });
        }
        let through = self.through()?;
        Ok(Self {
            level: self.level + 1,
            ..through
        })
    }
}

fn float(v: f64) -> Result<MetaValue, MetaError> {
    if v.is_finite() {
        Ok(MetaValue::Float(v))
    } else {
        Err(MetaError::NonFinite)
    }
}

impl Serializer for ValueSer {
    type Ok = MetaValue;
    type Error = MetaError;
    type SerializeSeq = SeqSer;
    type SerializeTuple = SeqSer;
    type SerializeTupleStruct = SeqSer;
    type SerializeTupleVariant = TaggedSeqSer;
    type SerializeMap = MapSer;
    type SerializeStruct = MapSer;
    type SerializeStructVariant = TaggedMapSer;

    fn serialize_bool(self, v: bool) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Bool(v))
    }
    fn serialize_i8(self, v: i8) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Int(v.into()))
    }
    fn serialize_i16(self, v: i16) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Int(v.into()))
    }
    fn serialize_i32(self, v: i32) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Int(v.into()))
    }
    fn serialize_i64(self, v: i64) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Int(v))
    }
    fn serialize_i128(self, v: i128) -> Result<MetaValue, MetaError> {
        i64::try_from(v)
            .map(MetaValue::Int)
            .map_err(|_| MetaError::IntOutOfRange)
    }
    fn serialize_u8(self, v: u8) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Int(v.into()))
    }
    fn serialize_u16(self, v: u16) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Int(v.into()))
    }
    fn serialize_u32(self, v: u32) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Int(v.into()))
    }
    fn serialize_u64(self, v: u64) -> Result<MetaValue, MetaError> {
        i64::try_from(v)
            .map(MetaValue::Int)
            .map_err(|_| MetaError::IntOutOfRange)
    }
    fn serialize_u128(self, v: u128) -> Result<MetaValue, MetaError> {
        i64::try_from(v)
            .map(MetaValue::Int)
            .map_err(|_| MetaError::IntOutOfRange)
    }
    fn serialize_f32(self, v: f32) -> Result<MetaValue, MetaError> {
        float(v.into())
    }
    fn serialize_f64(self, v: f64) -> Result<MetaValue, MetaError> {
        float(v)
    }
    fn serialize_char(self, v: char) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Str(v.to_string()))
    }
    fn serialize_str(self, v: &str) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Str(v.to_owned()))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Bytes(v.to_vec()))
    }
    fn serialize_none(self) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Null)
    }
    fn serialize_some<T: Serialize + ?Sized>(self, v: &T) -> Result<MetaValue, MetaError> {
        v.serialize(self.through()?)
    }
    fn serialize_unit(self) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Null)
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Null)
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
    ) -> Result<MetaValue, MetaError> {
        Ok(MetaValue::Str(variant.to_owned()))
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        v: &T,
    ) -> Result<MetaValue, MetaError> {
        v.serialize(self.through()?)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        v: &T,
    ) -> Result<MetaValue, MetaError> {
        tag(variant, v.serialize(self.child()?)?)
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<SeqSer, MetaError> {
        Ok(SeqSer {
            items: Vec::with_capacity(len.unwrap_or(0)),
            at: self,
        })
    }
    fn serialize_tuple(self, len: usize) -> Result<SeqSer, MetaError> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<SeqSer, MetaError> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<TaggedSeqSer, MetaError> {
        Ok(TaggedSeqSer {
            variant,
            items: Vec::with_capacity(len),
            at: self.child()?,
        })
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<MapSer, MetaError> {
        Ok(MapSer {
            entries: BTreeMap::new(),
            pending: None,
            at: self,
        })
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<MapSer, MetaError> {
        self.serialize_map(None)
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<TaggedMapSer, MetaError> {
        Ok(TaggedMapSer {
            variant,
            entries: BTreeMap::new(),
            at: self.child()?,
        })
    }
}

/// A one-entry map `{ variant: value }`, serde's external tagging.
fn tag(variant: &str, value: MetaValue) -> Result<MetaValue, MetaError> {
    MetaValue::map(BTreeMap::from([(variant.to_owned(), value)]))
}

/// A list's items, and the serializer of the list itself.
struct SeqSer {
    items: Vec<MetaValue>,
    at: ValueSer,
}

impl ser::SerializeSeq for SeqSer {
    type Ok = MetaValue;
    type Error = MetaError;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), MetaError> {
        self.items.push(v.serialize(self.at.child()?)?);
        Ok(())
    }
    fn end(self) -> Result<MetaValue, MetaError> {
        MetaValue::list(self.items)
    }
}

impl ser::SerializeTuple for SeqSer {
    type Ok = MetaValue;
    type Error = MetaError;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), MetaError> {
        ser::SerializeSeq::serialize_element(self, v)
    }
    fn end(self) -> Result<MetaValue, MetaError> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SeqSer {
    type Ok = MetaValue;
    type Error = MetaError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), MetaError> {
        ser::SerializeSeq::serialize_element(self, v)
    }
    fn end(self) -> Result<MetaValue, MetaError> {
        ser::SerializeSeq::end(self)
    }
}

/// A tuple variant's items, and the serializer of the list that holds
/// them, one level below the variant's tag.
struct TaggedSeqSer {
    variant: &'static str,
    items: Vec<MetaValue>,
    at: ValueSer,
}

impl ser::SerializeTupleVariant for TaggedSeqSer {
    type Ok = MetaValue;
    type Error = MetaError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), MetaError> {
        self.items.push(v.serialize(self.at.child()?)?);
        Ok(())
    }
    fn end(self) -> Result<MetaValue, MetaError> {
        tag(self.variant, MetaValue::list(self.items)?)
    }
}

/// A map's entries, and the serializer of the map itself.
struct MapSer {
    entries: BTreeMap<String, MetaValue>,
    pending: Option<String>,
    at: ValueSer,
}

impl ser::SerializeMap for MapSer {
    type Ok = MetaValue;
    type Error = MetaError;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), MetaError> {
        match key.serialize(self.at.child()?)? {
            MetaValue::Str(s) => {
                self.pending = Some(s);
                Ok(())
            }
            _ => Err(MetaError::NonStringKey),
        }
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), MetaError> {
        // serde's SerializeMap contract calls serialize_key first; a
        // missing pending key is a protocol breach by the producer.
        let key = self
            .pending
            .take()
            .ok_or_else(|| MetaError::Message("serialize_value before serialize_key".into()))?;
        // Duplicate keys refuse (the from_value side cannot even
        // represent them — BTreeMap — so silent last-wins here would
        // be a save/load-shaped asymmetry at the producer boundary).
        if self.entries.contains_key(&key) {
            return Err(MetaError::DuplicateKey(key));
        }
        self.entries.insert(key, v.serialize(self.at.child()?)?);
        Ok(())
    }
    fn end(self) -> Result<MetaValue, MetaError> {
        MetaValue::map(self.entries)
    }
}

impl ser::SerializeStruct for MapSer {
    type Ok = MetaValue;
    type Error = MetaError;
    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        v: &T,
    ) -> Result<(), MetaError> {
        // Derives cannot repeat a field; a hand-written impl could.
        if self.entries.contains_key(key) {
            return Err(MetaError::DuplicateKey(key.to_owned()));
        }
        self.entries
            .insert(key.to_owned(), v.serialize(self.at.child()?)?);
        Ok(())
    }
    fn end(self) -> Result<MetaValue, MetaError> {
        MetaValue::map(self.entries)
    }
}

/// A struct variant's fields, and the serializer of the map that holds
/// them, one level below the variant's tag.
struct TaggedMapSer {
    variant: &'static str,
    entries: BTreeMap<String, MetaValue>,
    at: ValueSer,
}

impl ser::SerializeStructVariant for TaggedMapSer {
    type Ok = MetaValue;
    type Error = MetaError;
    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        v: &T,
    ) -> Result<(), MetaError> {
        if self.entries.contains_key(key) {
            return Err(MetaError::DuplicateKey(key.to_owned()));
        }
        self.entries
            .insert(key.to_owned(), v.serialize(self.at.child()?)?);
        Ok(())
    }
    fn end(self) -> Result<MetaValue, MetaError> {
        tag(self.variant, MetaValue::map(self.entries)?)
    }
}
