//! **An operand is a read** (D10, "reading is the only dependency"):
//! every operand field of a node holds the [`VarId`] of the variable it
//! reads — an operation's output ([`crate::VarDef::Output`]) — and is
//! addressed by the field it is ([`OperandSlot`], one arm of
//! [`crate::SlotId`]), typed by the kinds the field admits
//! ([`SlotKind`]).
//!
//! What a caller writes is an [`Operand`]: a node, which is sugar for
//! that node's one output (`Operand::Node`), a port spelled out, or a
//! variable by id or by name. The slot door
//! ([`crate::DocEdit::SetParam`] with a [`crate::SlotValue::Read`])
//! lowers it to the id the document stores.

use crate::doc::VarName;
use crate::node::RecipeNodeId;
use crate::var::{VarId, VarKind};

/// **An operand as an author writes it**: what the edit door lowers to
/// the read a node stores.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Operand {
    /// A node, read through its one output (spec Q5). A node with
    /// several outputs — a revolve's body and axis, a split's two halves
    /// — refuses this spelling whatever the seat
    /// ([`crate::EditError::AmbiguousOutput`], naming its ports): the
    /// read names its port ([`Operand::Output`]).
    Node(RecipeNodeId),
    /// Port `port` of `node`'s signature.
    Output {
        /// The operation.
        node: RecipeNodeId,
        /// The port, an index into its signature.
        port: u8,
    },
    /// A variable, by id.
    Var(VarId),
    /// A variable, by name.
    Name(VarName),
}

impl From<RecipeNodeId> for Operand {
    fn from(node: RecipeNodeId) -> Self {
        Self::Node(node)
    }
}

impl From<VarId> for Operand {
    fn from(var: VarId) -> Self {
        Self::Var(var)
    }
}

impl From<&RecipeNodeId> for Operand {
    fn from(node: &RecipeNodeId) -> Self {
        Self::Node(*node)
    }
}

impl Operand {
    /// Port `port` of `node`.
    #[must_use]
    pub fn output(node: RecipeNodeId, port: u8) -> Self {
        Self::Output { node, port }
    }
}

impl core::fmt::Display for Operand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Node(node) => write!(f, "node {node}"),
            Self::Output { node, port } => write!(f, "port {port} of node {node}"),
            Self::Var(var) => write!(f, "{var}"),
            Self::Name(name) => write!(f, "{name}"),
        }
    }
}

/// **An operand field's address** (spec Q7: named by field, never by
/// position): the operand half of a node's slot vocabulary, addressed
/// as [`crate::SlotId::Operand`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum OperandSlot {
    /// The profile an extrude, revolve or sweep reads.
    Profile,
    /// Section `i` of a loft.
    Section(u32),
    /// A sweep's path profile.
    Path,
    /// The axis a revolve turns about, or a circular rule's.
    Axis,
    /// The frame a tube is built in, a profile is drawn on, or an
    /// in-plane axis is written in: the one operand kind a field of
    /// each reads, so one slot (its field is `frame` on a tube and
    /// `plane` on the other two).
    Frame,
    /// The body a blend, a shell or a split reshapes.
    Target,
    /// A split's plane.
    Tool,
    /// The body a subtract cuts.
    From,
    /// The body a subtract cuts away.
    Cut,
    /// Member `i` of a union's or an intersect's spelled list.
    Member(u32),
    /// The family a union or an intersect reads whole.
    Members,
    /// What a transform, a pattern or a placed union places.
    Input,
    /// What a part projection picks from.
    Of,
    /// The body a face frame reads its face out of.
    At,
    /// The body a world placement places.
    Body,
}

impl OperandSlot {
    /// The field as a reader says it.
    #[must_use]
    pub fn label(self) -> String {
        match self {
            Self::Profile => "profile".to_owned(),
            Self::Section(i) => format!("section {}", u64::from(i) + 1),
            Self::Path => "path".to_owned(),
            Self::Axis => "axis".to_owned(),
            Self::Frame => "frame".to_owned(),
            Self::Target => "target".to_owned(),
            Self::Tool => "tool".to_owned(),
            Self::From => "from".to_owned(),
            Self::Cut => "tool".to_owned(),
            Self::Member(i) => format!("member {}", u64::from(i) + 1),
            Self::Members => "members".to_owned(),
            Self::Input => "input".to_owned(),
            Self::Of => "source".to_owned(),
            Self::At | Self::Body => "body".to_owned(),
        }
    }

    /// The kinds this field admits.
    #[must_use]
    pub fn kind(self) -> SlotKind {
        match self {
            Self::Profile | Self::Section(_) | Self::Path => SlotKind::Is(VarKind::Profile),
            Self::Axis => SlotKind::Is(VarKind::Axis),
            Self::Frame => SlotKind::Is(VarKind::Frame),
            Self::Tool => SlotKind::Is(VarKind::Plane),
            Self::Target | Self::From | Self::Cut | Self::Member(_) | Self::At | Self::Body => {
                SlotKind::Is(VarKind::Body)
            }
            Self::Members => SlotKind::Is(VarKind::Bodies),
            Self::Input | Self::Of => SlotKind::Placeable,
        }
    }
}

impl core::fmt::Display for OperandSlot {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.label())
    }
}

/// **The kinds a slot admits** ([`crate::SlotId::kind`], total over
/// every slot): what a read in the slot may read, and what an
/// expression in a scalar slot lowers to a read of.
///
/// A scalar slot and an operand seat that holds one kind are
/// [`SlotKind::Is`]. One operand seat admits a set of kinds:
/// [`SlotKind::Placeable`] is exactly `{Body, Bodies}` (a placer places
/// one body or a list of them).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SlotKind {
    /// Exactly this kind: a seat's own, or a scalar slot's dimension.
    Is(VarKind),
    /// `Body` or `Bodies`.
    Placeable,
}

impl SlotKind {
    /// Whether `var` may sit here, by its kind.
    #[must_use]
    pub fn admits(self, var: &crate::Var) -> bool {
        let kind = var.kind();
        match self {
            Self::Is(is) => kind == is,
            Self::Placeable => matches!(kind, VarKind::Body | VarKind::Bodies),
        }
    }
}

impl core::fmt::Display for SlotKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Is(kind) => write!(f, "{} {kind}", crate::sentence::article(&kind.to_string())),
            Self::Placeable => f.write_str("a body or a list of bodies"),
        }
    }
}

/// **A `Bodies` argument** (REFERENCES DM4; D10): the one argument a
/// union and an intersect take, defined by index or by enumeration.
/// Every reader of a `Bodies` takes either form, and a mix cannot be
/// written: a `Bodies` read in a member's place is ill-typed at the door
/// (`SlotVarKind`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Bodies<R> {
    /// A family, read whole: its members in index order, each named by
    /// its index under the one read.
    Family(R),
    /// Independent reads spelled at the slot, in the order written, each
    /// member named by its own read.
    Spelled(Vec<R>),
}

impl<R> Bodies<R> {
    /// The reads, with the operand slot each sits in, in list order.
    pub fn rows(&self) -> Vec<(OperandSlot, &R)> {
        match self {
            Self::Family(read) => vec![(OperandSlot::Members, read)],
            Self::Spelled(reads) => reads
                .iter()
                .enumerate()
                .map(|(i, read)| {
                    (
                        OperandSlot::Member(u32::try_from(i).unwrap_or(u32::MAX)),
                        read,
                    )
                })
                .collect(),
        }
    }

    /// [`Self::rows`], writable.
    pub fn rows_mut(&mut self) -> Vec<(OperandSlot, &mut R)> {
        match self {
            Self::Family(read) => vec![(OperandSlot::Members, read)],
            Self::Spelled(reads) => reads
                .iter_mut()
                .enumerate()
                .map(|(i, read)| {
                    (
                        OperandSlot::Member(u32::try_from(i).unwrap_or(u32::MAX)),
                        read,
                    )
                })
                .collect(),
        }
    }

    /// The reads, in list order.
    pub fn reads(&self) -> impl Iterator<Item = &R> {
        let (family, spelled) = match self {
            Self::Family(read) => (Some(read), &[][..]),
            Self::Spelled(reads) => (None, reads.as_slice()),
        };
        family.into_iter().chain(spelled)
    }

    /// The same argument, each read mapped through `f`, in list order;
    /// the first error stops it.
    ///
    /// # Errors
    ///
    /// Whatever `f` refuses first.
    pub fn try_map<Q, E>(
        &self,
        mut f: impl FnMut(OperandSlot, &R) -> Result<Q, E>,
    ) -> Result<Bodies<Q>, E> {
        Ok(match self {
            Self::Family(read) => Bodies::Family(f(OperandSlot::Members, read)?),
            Self::Spelled(reads) => Bodies::Spelled(
                reads
                    .iter()
                    .enumerate()
                    .map(|(i, read)| {
                        f(
                            OperandSlot::Member(u32::try_from(i).unwrap_or(u32::MAX)),
                            read,
                        )
                    })
                    .collect::<Result<_, _>>()?,
            ),
        })
    }
}

/// **A read at a body seat** (REFERENCES DM3): the variable read, and
/// one `Count` index per index of the family it reads (VR4) when the
/// read picks one member. `xs[i]` is `BodyRead { read: xs, at: [i] }`;
/// a plain read carries no index. An indexed read is not a variable:
/// it reads the family `xs`, and an operation keys the member's names
/// by `xs` (DM4).
///
/// `S` is the node's slot form, so the stored read holds the family's
/// [`VarId`] and each index's variable, and the authored one an
/// [`Operand`] and each index's [`crate::Formula`]. Each index is a
/// slot of its node ([`crate::SlotId::Index`]).
///
/// Serialized as the bare read when it carries no index, and as
/// `{"read": …, "at": [...]}` otherwise, and debug-printed the same
/// way: a plain read as its read.
#[derive(Clone, PartialEq)]
pub struct BodyRead<S: crate::expr::Slot> {
    /// The variable read: a body, or for an indexed read the family.
    pub read: S::Read,
    /// One `Count` per index of the family, empty for a plain read.
    pub at: Vec<S>,
}

impl<S: crate::expr::Slot> BodyRead<S> {
    /// A plain read of `read`.
    #[must_use]
    pub fn plain(read: impl Into<S::Read>) -> Self {
        Self {
            read: read.into(),
            at: Vec::new(),
        }
    }

    /// One member of the family `read`, at one index per index of the
    /// family: `xs[i]` is `BodyRead::indexed(xs, vec![i])`.
    #[must_use]
    pub fn indexed(read: impl Into<S::Read>, at: Vec<S>) -> Self {
        Self {
            read: read.into(),
            at,
        }
    }

    /// Whether the read picks one member of a family.
    #[must_use]
    pub fn is_indexed(&self) -> bool {
        !self.at.is_empty()
    }

    /// The kinds the variable read may have at `seat`: the seat's own
    /// for a plain read, a family for an indexed one.
    #[must_use]
    pub fn kind(&self, seat: OperandSlot) -> SlotKind {
        if self.is_indexed() {
            SlotKind::Is(VarKind::Bodies)
        } else {
            seat.kind()
        }
    }

    /// The indices, each with its position, shared.
    pub fn rows(&self) -> Vec<(u8, &S)> {
        self.at
            .iter()
            .enumerate()
            .map(|(k, e)| (u8::try_from(k).unwrap_or(u8::MAX), e))
            .collect()
    }

    /// The indices, each with its position, exclusive.
    pub fn rows_mut(&mut self) -> Vec<(u8, &mut S)> {
        self.at
            .iter_mut()
            .enumerate()
            .map(|(k, e)| (u8::try_from(k).unwrap_or(u8::MAX), e))
            .collect()
    }

    /// This read in another slot form: the read by `read` (handed its
    /// seat), each index by `f`.
    ///
    /// # Errors
    ///
    /// The first refusal of `read` or `f`.
    pub fn try_map<S2: crate::expr::Slot, E>(
        &self,
        seat: OperandSlot,
        f: &mut impl FnMut(&S) -> Result<S2, E>,
        read: &mut impl FnMut(OperandSlot, &S::Read) -> Result<S2::Read, E>,
    ) -> Result<BodyRead<S2>, E> {
        Ok(BodyRead {
            read: read(seat, &self.read)?,
            at: self.at.iter().map(&mut *f).collect::<Result<_, _>>()?,
        })
    }
}

impl<S: crate::expr::Slot> core::fmt::Debug for BodyRead<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.at.is_empty() {
            return self.read.fmt(f);
        }
        f.debug_struct("BodyRead")
            .field("read", &self.read)
            .field("at", &self.at)
            .finish()
    }
}

impl<S: crate::expr::Slot> From<VarId> for BodyRead<S>
where
    S::Read: From<VarId>,
{
    fn from(var: VarId) -> Self {
        Self::plain(var)
    }
}

impl<S: crate::expr::Slot> From<RecipeNodeId> for BodyRead<S>
where
    S::Read: From<RecipeNodeId>,
{
    fn from(node: RecipeNodeId) -> Self {
        Self::plain(node)
    }
}

impl<S: crate::expr::Slot> From<&RecipeNodeId> for BodyRead<S>
where
    S::Read: From<RecipeNodeId>,
{
    fn from(node: &RecipeNodeId) -> Self {
        Self::plain(*node)
    }
}

impl<S: crate::expr::Slot> From<Operand> for BodyRead<S>
where
    S::Read: From<Operand>,
{
    fn from(read: Operand) -> Self {
        Self::plain(read)
    }
}

impl<S: crate::expr::Slot> serde::Serialize for BodyRead<S> {
    fn serialize<Z: serde::Serializer>(&self, ser: Z) -> Result<Z::Ok, Z::Error> {
        use serde::ser::SerializeStruct as _;
        if self.at.is_empty() {
            return self.read.serialize(ser);
        }
        let mut st = ser.serialize_struct("BodyRead", 2)?;
        st.serialize_field("read", &self.read)?;
        st.serialize_field("at", &self.at)?;
        st.end()
    }
}

impl<'de, S: crate::expr::Slot> serde::Deserialize<'de> for BodyRead<S> {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        use serde::de::value::{MapAccessDeserializer, StringDeserializer};
        use serde::de::{Error as _, MapAccess, Visitor};

        /// A map whose first key was read to tell the two forms apart,
        /// handed on whole to the bare read's own deserializer.
        struct Prepended<A> {
            first: Option<String>,
            rest: A,
        }

        impl<'de, A: MapAccess<'de>> MapAccess<'de> for Prepended<A> {
            type Error = A::Error;
            fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(
                &mut self,
                seed: K,
            ) -> Result<Option<K::Value>, A::Error> {
                match self.first.take() {
                    Some(key) => seed
                        .deserialize(StringDeserializer::<A::Error>::new(key))
                        .map(Some),
                    None => self.rest.next_key_seed(seed),
                }
            }
            fn next_value_seed<V: serde::de::DeserializeSeed<'de>>(
                &mut self,
                seed: V,
            ) -> Result<V::Value, A::Error> {
                self.rest.next_value_seed(seed)
            }
        }

        /// A bare read spelled as a string, handed on to the read's own
        /// deserializer: a newtype around a string (a [`VarId`]) reads
        /// through it as the string.
        struct Spelled<'a, E>(&'a str, core::marker::PhantomData<E>);

        impl<'de, E: serde::de::Error> serde::Deserializer<'de> for Spelled<'_, E> {
            type Error = E;
            fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_str(self.0)
            }
            fn deserialize_newtype_struct<V: Visitor<'de>>(
                self,
                _name: &'static str,
                visitor: V,
            ) -> Result<V::Value, E> {
                visitor.visit_newtype_struct(self)
            }
            serde::forward_to_deserialize_any! {
                bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                bytes byte_buf option unit unit_struct seq tuple
                tuple_struct map struct enum identifier ignored_any
            }
        }

        struct Either<S>(core::marker::PhantomData<S>);

        impl<'de, S: crate::expr::Slot> Visitor<'de> for Either<S> {
            type Value = BodyRead<S>;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a read, or a read and its indices (`{\"read\": …, \"at\": [...]}`)")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(BodyRead {
                    read: <S::Read as serde::Deserialize<'de>>::deserialize(Spelled::<E>(
                        v,
                        core::marker::PhantomData,
                    ))?,
                    at: Vec::new(),
                })
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let Some(first) = map.next_key::<String>()? else {
                    return Ok(BodyRead {
                        read: <S::Read as serde::Deserialize<'de>>::deserialize(
                            MapAccessDeserializer::new(map),
                        )?,
                        at: Vec::new(),
                    });
                };
                if first != "read" && first != "at" {
                    let rest = Prepended {
                        first: Some(first),
                        rest: map,
                    };
                    return Ok(BodyRead {
                        read: <S::Read as serde::Deserialize<'de>>::deserialize(
                            MapAccessDeserializer::new(rest),
                        )?,
                        at: Vec::new(),
                    });
                }
                let (mut read, mut at) = (None, None);
                let mut key = Some(first);
                while let Some(k) = key {
                    match k.as_str() {
                        "read" if read.is_none() => read = Some(map.next_value::<S::Read>()?),
                        "at" if at.is_none() => at = Some(map.next_value::<Vec<S>>()?),
                        "read" | "at" => return Err(A::Error::duplicate_field("read or at")),
                        other => return Err(A::Error::unknown_field(other, &["read", "at"])),
                    }
                    key = map.next_key::<String>()?;
                }
                let read = read.ok_or_else(|| A::Error::missing_field("read"))?;
                let at: Vec<S> = at.ok_or_else(|| A::Error::missing_field("at"))?;
                if at.is_empty() {
                    return Err(A::Error::invalid_length(
                        0,
                        &"at least one index (a plain read is written bare)",
                    ));
                }
                Ok(BodyRead { read, at })
            }
        }

        de.deserialize_any(Either(core::marker::PhantomData))
    }
}
