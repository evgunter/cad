//! **What the document types cannot say for themselves.** Everything
//! else in the recipe derives serde where it is declared; four things
//! cannot, and this module is exactly those four.
//!
//! # Two expression languages that must NOT deserialize field-by-field
//!
//! - [`Expr`] persists as a plain AST tree and is REBUILT through the
//!   dimension-checking smart constructors on load — a corrupt or
//!   hand-edited file can never smuggle an ill-dimensioned tree (or an
//!   unreduced constant) past the construction door, and the
//!   checker's refusal reaches the caller WHOLE rather than as
//!   prose — how a typed value leaves a `Deserialize` impl at all
//!   is [`super::refusal`]'s subject. The cached
//!   dimension is deliberately not persisted: it re-derives.
//! - [`MeasureExpr`] is the same rule over the leaves the measurement
//!   language adds, and a SEPARATE wire form for the reason the type is
//!   separate: a shared one would make a primitive leaf representable
//!   in a slot expression.
//!
//! # One FIELD that must not, inside a type that otherwise does
//!
//! [`ProfileProgram`](crate::program::ProfileProgram) derives serde on
//! its own declaration — its loop programs are the document vocabulary
//! and persist as themselves. Its `plane` does not: a document written
//! before the sketch plane became a node carries a placement object
//! there, and [`plane_ref`] is the visitor that refuses it in terms
//! naming what moved. What the derive still buys unchanged is the
//! strict door at the program layer: deserialization can NEVER mint a
//! `profile::ProfileLoop`. The wire rebuilds the PROGRAM only; loops
//! exist through the replay driver at evaluation and nowhere else
//! (serde is transport, the driver is the door — LIB-SWITCH §4h).
//! Unlike the two expression languages above, then, this rebuild
//! TRUSTS the program's structure — only its slot expressions pass a
//! constructor — and what re-checks it is the load door's snapshot
//! program walk in `persist::check`, which refuses the lattice class
//! alone ([`ProgramFault`](crate::ProgramFault)'s doc accounts for
//! the rest).
//!
//! # Two KERNEL-FOREIGN tags
//!
//! `profile::ArcSweep` and `profile::ArcSide` ride a
//! [`ProgramArcData`](crate::program::ProgramArcData) field. The orphan
//! rule puts them out of reach of a derive here and G1 layering keeps
//! serde out of the kernel crate, so they persist through the
//! [`arc_sweep`] and [`arc_side`] adapters, both minted from one macro.
//!
//! # What is NOT here, and the consequence
//!
//! The document's own step vocabulary. `ProgramStep`, `ProgramTarget`,
//! `ProgramArcData` and `LoopProgram` derive serde where they are
//! declared, so the document form IS the persisted form: there is one
//! spelling of a verb in this crate and nothing to keep in step with
//! anything. What that costs is that a RENAME in `program.rs` is a
//! FORMAT change — held by two rows in
//! `tests/switch_program_vocabulary.rs` and `tests/wire_rv_bytes.rs`,
//! which is what used to be bought by the vocabulary stopping here.
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::doc::VarName;
use crate::expr::{AuthoredLeaf, Dimension, DimensionError, Expr, ExprKind, Slot};
use crate::formula::Formula;
use crate::measure::{MeasureExpr, MeasureKind, MeasurePrimitive};
use crate::node::RecipeNodeId;
use crate::var::VarId;

use super::nesting::Child;

/// **One persisted expression form, minted from its own leaves.** The
/// stored [`Expr`] and the authored [`Formula`] are one tree with
/// different leaf sets, and persist as one wire vocabulary with
/// different leaf sets: the shared variants are written here once, and
/// each invocation adds its form's own variants and their two
/// conversions. A variant name the forms share is spelled once, so a
/// formula in an edit log and the expression a snapshot stores from it
/// write the same words.
macro_rules! wire_tree {
    (
        $(#[$meta:meta])*
        $wire:ident for $form:ty {
            $(
                $(#[$vmeta:meta])*
                $variant:ident { $( $(#[$fmeta:meta])* $field:ident : $fty:ty ),* $(,)? }
            )*
        }
        to_wire($leaf:ident, $dim:ident) => $to:expr;
        rebuild { $($rebuild:tt)* }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub(crate) enum $wire {
            /// An exact rational constant, in lowest terms (a ratio that
            /// reduces refuses at rebuild).
            Ratio {
                /// The numerator, carrying the sign.
                num: i64,
                /// The denominator, at least 1.
                den: u64,
            },
            /// An exact integer constant (a count).
            Integer(i64),
            /// One full rotation.
            Turn,
            /// A reader of a variable, by id, with the kind it caches.
            Var {
                /// The variable read.
                var: crate::var::VarId,
                /// The dimension the reader reads at.
                dim: Dimension,
            },
            $(
                $(#[$vmeta])*
                $variant { $( $(#[$fmeta])* $field: $fty ),* },
            )*
            /// Same-dimension addition.
            Add(Child<$wire>, Child<$wire>),
            /// Same-dimension subtraction.
            Sub(Child<$wire>, Child<$wire>),
            /// Negation.
            Neg(Child<$wire>),
            /// Product.
            Mul(Child<$wire>, Child<$wire>),
            /// Quotient.
            Div(Child<$wire>, Child<$wire>),
            /// Sine.
            Sin(Child<$wire>),
            /// Cosine.
            Cos(Child<$wire>),
            /// Tangent.
            Tan(Child<$wire>),
            /// Four-quadrant arctangent (y, x).
            Atan2(Child<$wire>, Child<$wire>),
            /// Lattice minimum.
            Min(Child<$wire>, Child<$wire>),
            /// Lattice maximum.
            Max(Child<$wire>, Child<$wire>),
            /// Explicit Count→Scalar promotion.
            CountToScalar(Child<$wire>),
        }

        impl From<&$form> for $wire {
            fn from(e: &$form) -> Self {
                let b = |x: &$form| Child::new($wire::from(x));
                match e.kind() {
                    ExprKind::Ratio(r) => $wire::Ratio {
                        num: r.num(),
                        den: r.den(),
                    },
                    ExprKind::Integer(v) => $wire::Integer(*v),
                    ExprKind::Turn => $wire::Turn,
                    ExprKind::Var(var) => $wire::Var {
                        var: *var,
                        dim: e.dim(),
                    },
                    ExprKind::Leaf($leaf) => {
                        let $dim = e.dim();
                        $to
                    }
                    ExprKind::Add(x, y) => $wire::Add(b(x), b(y)),
                    ExprKind::Sub(x, y) => $wire::Sub(b(x), b(y)),
                    ExprKind::Neg(x) => $wire::Neg(b(x)),
                    ExprKind::Mul(x, y) => $wire::Mul(b(x), b(y)),
                    ExprKind::Div(x, y) => $wire::Div(b(x), b(y)),
                    ExprKind::Sin(x) => $wire::Sin(b(x)),
                    ExprKind::Cos(x) => $wire::Cos(b(x)),
                    ExprKind::Tan(x) => $wire::Tan(b(x)),
                    ExprKind::Atan2(x, y) => $wire::Atan2(b(x), b(y)),
                    ExprKind::Min(x, y) => $wire::Min(b(x), b(y)),
                    ExprKind::Max(x, y) => $wire::Max(b(x), b(y)),
                    ExprKind::CountToScalar(x) => $wire::CountToScalar(b(x)),
                }
            }
        }

        impl $wire {
            /// Rebuilds the checked tree, re-running every dimension
            /// check and the constant's range and reduction (load door;
            /// module docs).
            pub(crate) fn rebuild(&self) -> Result<$form, DimensionError> {
                let b = |x: &$wire| x.rebuild();
                match self {
                    $wire::Ratio { num, den } => {
                        crate::expr::Ratio::reduced(*num, *den).map(<$form>::ratio_leaf)
                    }
                    $wire::Integer(v) => Ok(<$form>::integer_leaf(*v)),
                    $wire::Turn => Ok(<$form>::turn_leaf()),
                    $wire::Var { var, dim } => Ok(<$form>::var(*var, *dim)),
                    $($rebuild)*
                    $wire::Add(x, y) => <$form>::add(b(x)?, b(y)?),
                    $wire::Sub(x, y) => <$form>::sub(b(x)?, b(y)?),
                    $wire::Neg(x) => <$form>::neg(b(x)?),
                    $wire::Mul(x, y) => <$form>::mul(b(x)?, b(y)?),
                    $wire::Div(x, y) => <$form>::div(b(x)?, b(y)?),
                    $wire::Sin(x) => <$form>::sin(b(x)?),
                    $wire::Cos(x) => <$form>::cos(b(x)?),
                    $wire::Tan(x) => <$form>::tan(b(x)?),
                    $wire::Atan2(x, y) => <$form>::atan2(b(x)?, b(y)?),
                    $wire::Min(x, y) => <$form>::min(b(x)?, b(y)?),
                    $wire::Max(x, y) => <$form>::max(b(x)?, b(y)?),
                    $wire::CountToScalar(x) => <$form>::count_to_scalar(b(x)?),
                }
            }
        }

        impl Serialize for $form {
            fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                $wire::from(self).serialize(ser)
            }
        }

        impl<'de> Deserialize<'de> for $form {
            fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
                let wire = $wire::deserialize(de)?;
                wire.rebuild().map_err(|e| {
                    // The typed refusal leaves through the slot; the
                    // serde message is the human half of the same fact
                    // (`persist::refusal`).
                    super::refusal::record(&e);
                    D::Error::custom(format!("ill-dimensioned expression refused: {e}"))
                })
            }
        }

        impl Wired for $form {
            type Wire = $wire;
            fn to_wire(&self, _dim: Dimension) -> $wire {
                $wire::from(self)
            }
            fn rebuild_leaf(wire: &$wire) -> Result<(Self, Dimension), DimensionError> {
                let leaf = wire.rebuild()?;
                let dim = leaf.dim();
                Ok((leaf, dim))
            }
        }
    };
}

/// **A persisted slot form and its wire vocabulary**: what a tree
/// generic over its form ([`MeasureExpr`]) persists its value leaves
/// as. A leaf is written with the dimension the tree reads it at, and
/// rebuilt with the dimension it holds: an expression holds its own,
/// and a stored slot, a bare variable id, holds the one written beside
/// it.
pub(crate) trait Wired: Sized {
    /// The form's wire value.
    type Wire: Serialize + for<'de> Deserialize<'de>;
    /// The wire value of a leaf read at `dim`.
    fn to_wire(&self, dim: Dimension) -> Self::Wire;
    /// The leaf a wire value rebuilds, through its form's checking
    /// constructors, and its dimension.
    fn rebuild_leaf(wire: &Self::Wire) -> Result<(Self, Dimension), DimensionError>;
}

/// **A stored value leaf on the wire**: the variable, and the
/// dimension the measure reads it at.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WireVarLeaf {
    var: VarId,
    dim: Dimension,
}

impl Wired for VarId {
    type Wire = WireVarLeaf;
    fn to_wire(&self, dim: Dimension) -> WireVarLeaf {
        WireVarLeaf { var: *self, dim }
    }
    fn rebuild_leaf(wire: &WireVarLeaf) -> Result<(Self, Dimension), DimensionError> {
        Ok((wire.var, wire.dim))
    }
}

wire_tree! {
    /// The persisted stored expression (spec D1: the recipe is the
    /// save; an expression is its constructor calls). It has no name
    /// leaf, so a snapshot that holds one is unreadable.
    WireExpr for Expr {}
    to_wire(leaf, _dim) => match *leaf {};
    rebuild {}
}

wire_tree! {
    /// The persisted authored formula, as an edit log carries it: the
    /// stored vocabulary plus a variable by name, a fresh-table read
    /// and a written quantity.
    WireFormula for Formula {
        /// A variable by name, with the dimension it is read at.
        Name {
            /// The name.
            name: VarName,
            /// The dimension it is read at.
            dim: Dimension,
        }
        /// Entry `index` of the edit's fresh table, with the dimension
        /// it is read at.
        Fresh {
            /// The table index.
            index: u16,
            /// The dimension it is read at.
            dim: Dimension,
        }
        /// A written quantity with its dimension, the display unit it
        /// was authored in (LIB-SWITCH §4g — presentation metadata;
        /// the value stays canonical meters/radians) and its
        /// distribution.
        Quantity {
            /// The exact value (D2: bit-exact round-trip), canonical units.
            value: f64,
            /// The quantity's dimension.
            dim: Dimension,
            /// The display-unit symbol (quantity's closed table).
            /// Always written, because every quantity names the
            /// notation it was authored in — the dimensionless row's
            /// symbol is the empty string. An unknown symbol refuses
            /// typed at rebuild.
            unit: String,
            /// Its distribution, if one was written.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            distribution: Option<crate::distribution::Distribution>,
        }
    }
    to_wire(leaf, dim) => match leaf {
        AuthoredLeaf::Name(name) => WireFormula::Name {
            name: name.clone(),
            dim,
        },
        &AuthoredLeaf::Fresh(index) => WireFormula::Fresh { index, dim },
        AuthoredLeaf::Quantity(q) => WireFormula::Quantity {
            value: q.value(),
            dim,
            unit: q.unit().symbol().to_string(),
            distribution: q.distribution().copied(),
        },
    };
    rebuild {
        WireFormula::Name { name, dim } => Ok(Formula::named(name.clone(), *dim)),
        WireFormula::Fresh { index, dim } => Ok(Formula::fresh(*index, *dim)),
        // Strict door: the symbol must be in quantity's closed table
        // and its quantity must match the dimension — re-checked by the
        // same constructor authoring uses (never a field-by-field
        // trust). The distribution is the variable's: the door that
        // mints it refuses one breaking E2, as it does a declared one.
        WireFormula::Quantity { value, dim, unit, distribution } => {
            match quantity::unit_by_symbol(unit) {
                None => Err(DimensionError::UnknownDisplayUnit {
                    symbol: unit.clone(),
                }),
                Some(u) => Formula::literal_with_unit(*value, *dim, u)
                    .map(|q| q.carrying(*distribution)),
            }
        }
    }
}

/// **One kernel-foreign two-variant tag's persistence, minted from its
/// two words.**
///
/// A tag like `profile::ArcSweep` is the kernel's type, so this crate
/// cannot derive serde for it (the orphan rule) and G1 layering says
/// the kernel crate does not gain the derive either. What is left is a
/// `#[serde(with = …)]` adapter: a private local enum carrying the
/// persisted spelling, plus the two functions the attribute names.
///
/// That adapter is the same six lines for every such tag, so it is
/// written once here rather than per tag. Each invocation below is the
/// module name, the kernel type and the two variant words — which is
/// all that ever differs — so a third tag pair is one more line and
/// cannot drift from the shape of the other two.
///
/// **What it does not cover:** a tag with other than two variants, or
/// one whose persisted word differs from its Rust variant name. Both
/// would need the macro grown rather than another invocation, and
/// neither exists on this wire.
macro_rules! foreign_tag {
    ($(
        $(#[$meta:meta])*
        $module:ident => $tag:path { $a:ident, $b:ident }
    )*) => {
        $(
            $(#[$meta])*
            pub(crate) mod $module {
                use super::*;
                use $tag as Tag;

                /// The persisted spelling of the tag.
                #[derive(Debug, Serialize, Deserialize)]
                enum Wire {
                    /// The first form.
                    $a,
                    /// The second form.
                    $b,
                }

                /// Writes the tag.
                ///
                /// # Errors
                ///
                /// The serializer's own.
                pub(crate) fn serialize<S: Serializer>(
                    t: &Tag,
                    ser: S,
                ) -> Result<S::Ok, S::Error> {
                    match t {
                        Tag::$a => Wire::$a,
                        Tag::$b => Wire::$b,
                    }
                    .serialize(ser)
                }

                /// Reads the tag. Total: the wire enum has no form the
                /// kernel enum lacks, so anything that parses converts.
                ///
                /// # Errors
                ///
                /// The deserializer's own — a word outside the two.
                pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
                    de: D,
                ) -> Result<Tag, D::Error> {
                    Ok(match Wire::deserialize(de)? {
                        Wire::$a => Tag::$a,
                        Wire::$b => Tag::$b,
                    })
                }
            }
        )*
    };
}

foreign_tag! {
    /// `profile::ArcSweep` on the wire: a travel sense.
    arc_sweep => profile::ArcSweep { Ccw, Cw }

    /// `profile::ArcSide` on the wire: which side of the tangent the
    /// carrier's centre sits on.
    arc_side => profile::ArcSide { Left, Right }
}

/// **An `Option` field that must be PRESENT on the wire**, `null` for
/// `None`: one `Option::deserialize`, with no fallback. A derived
/// `Deserialize` reads an absent `Option` field as `None`, which for an
/// instance's `gauge` and `offset` would load a file written before
/// gauges as an unplaced instance; routed through here, the absent key
/// is serde's own "missing field" refusal, which names it.
pub(crate) fn present<'de, D: Deserializer<'de>, T: serde::Deserialize<'de>>(
    de: D,
) -> Result<Option<T>, D::Error> {
    Option::deserialize(de)
}

/// The profile's `plane`, read so that a document written before the
/// sketch plane became a node refuses in terms that NAME what moved.
///
/// Those files carry a twelve-float placement object in this field.
/// Serde's own report for that is `invalid type: map, expected u64` —
/// true, and useless: it says nothing about which field of which node
/// changed shape, which is the whole job of an `Unreadable` refusal
/// (a reader has to know what to regenerate). The visitor's `expecting`
/// is where that sentence goes.
pub(crate) fn plane_ref<'de, D: Deserializer<'de>>(de: D) -> Result<RecipeNodeId, D::Error> {
    struct PlaneRef;
    impl serde::de::Visitor<'_> for PlaneRef {
        type Value = RecipeNodeId;
        fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(
                "a `plane` node id: the frame node (`Datum::Frame` or `Datum::FaceFrame`) \
                 a profile is drawn on (a document that carries a sketch-plane PLACEMENT \
                 here predates the frame node and cannot be read by this build)",
            )
        }
        fn visit_str<E: serde::de::Error>(self, text: &str) -> Result<RecipeNodeId, E> {
            crate::MintId::parse(text)
                .map(RecipeNodeId)
                .ok_or_else(|| E::invalid_value(serde::de::Unexpected::Str(text), &self))
        }
    }
    de.deserialize_str(PlaneRef)
}

/// The persisted MEASUREMENT expression (ERROR-DESIGN E3): the same
/// arithmetic the document expression has, over the two leaves this
/// language adds.
///
/// A separate wire enum rather than a grown [`WireExpr`], for the same
/// reason [`MeasureExpr`] is a separate type: a primitive leaf is
/// meaningless in a slot expression, and a shared wire form would make
/// one representable there — a file could then carry a `distance` leaf
/// in an extrude's distance, and the refusal would have to be invented
/// at every rebuild site instead of being unrepresentable.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum WireMeasureExpr<W> {
    /// A closed-form measurement leaf.
    Primitive(MeasurePrimitive),
    /// An ordinary document expression leaf, in the measure's form.
    Value(Box<W>),
    /// Same-dimension addition.
    Add(Child<WireMeasureExpr<W>>, Child<WireMeasureExpr<W>>),
    /// Same-dimension subtraction.
    Sub(Child<WireMeasureExpr<W>>, Child<WireMeasureExpr<W>>),
    /// Negation.
    Neg(Child<WireMeasureExpr<W>>),
    /// Product (at least one Scalar operand).
    Mul(Child<WireMeasureExpr<W>>, Child<WireMeasureExpr<W>>),
    /// Quotient (Scalar divisor).
    Div(Child<WireMeasureExpr<W>>, Child<WireMeasureExpr<W>>),
    /// Same-dimension minimum.
    Min(Child<WireMeasureExpr<W>>, Child<WireMeasureExpr<W>>),
    /// Same-dimension maximum.
    Max(Child<WireMeasureExpr<W>>, Child<WireMeasureExpr<W>>),
}

impl<S: Slot + Wired> From<&MeasureExpr<S>> for WireMeasureExpr<S::Wire> {
    fn from(e: &MeasureExpr<S>) -> Self {
        let b = |x: &MeasureExpr<S>| Child::new(WireMeasureExpr::from(x));
        match e.kind() {
            MeasureKind::Primitive(p) => WireMeasureExpr::Primitive(*p),
            MeasureKind::Value(v) => WireMeasureExpr::Value(Box::new(v.to_wire(e.dim()))),
            MeasureKind::Add(x, y) => WireMeasureExpr::Add(b(x), b(y)),
            MeasureKind::Sub(x, y) => WireMeasureExpr::Sub(b(x), b(y)),
            MeasureKind::Neg(x) => WireMeasureExpr::Neg(b(x)),
            MeasureKind::Mul(x, y) => WireMeasureExpr::Mul(b(x), b(y)),
            MeasureKind::Div(x, y) => WireMeasureExpr::Div(b(x), b(y)),
            MeasureKind::Min(x, y) => WireMeasureExpr::Min(b(x), b(y)),
            MeasureKind::Max(x, y) => WireMeasureExpr::Max(b(x), b(y)),
        }
    }
}

impl<W> WireMeasureExpr<W> {
    /// Rebuilds through the DIMENSION-CHECKING constructors — the load
    /// door is the construction door, so a file cannot carry a tree the
    /// authoring API refuses.
    fn rebuild<S: Slot + Wired<Wire = W>>(&self) -> Result<MeasureExpr<S>, DimensionError> {
        let b = |x: &WireMeasureExpr<W>| x.rebuild();
        match self {
            WireMeasureExpr::Primitive(p) => Ok(MeasureExpr::primitive(*p)),
            WireMeasureExpr::Value(v) => {
                let (leaf, dim) = S::rebuild_leaf(v)?;
                Ok(MeasureExpr::value_at(leaf, dim))
            }
            WireMeasureExpr::Add(x, y) => MeasureExpr::add(b(x)?, b(y)?),
            WireMeasureExpr::Sub(x, y) => MeasureExpr::sub(b(x)?, b(y)?),
            WireMeasureExpr::Neg(x) => MeasureExpr::neg(b(x)?),
            WireMeasureExpr::Mul(x, y) => MeasureExpr::mul(b(x)?, b(y)?),
            WireMeasureExpr::Div(x, y) => MeasureExpr::div(b(x)?, b(y)?),
            WireMeasureExpr::Min(x, y) => MeasureExpr::min(b(x)?, b(y)?),
            WireMeasureExpr::Max(x, y) => MeasureExpr::max(b(x)?, b(y)?),
        }
    }
}

/// The measure's serde, one text for each slot form.
macro_rules! measure_serde {
    ($($form:ty),*) => {$(
        impl Serialize for MeasureExpr<$form> {
            fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                WireMeasureExpr::from(self).serialize(ser)
            }
        }

        impl<'de> Deserialize<'de> for MeasureExpr<$form> {
            fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
                let wire = WireMeasureExpr::<<$form as Wired>::Wire>::deserialize(de)?;
                wire.rebuild().map_err(|e| {
                    super::refusal::record(&e);
                    D::Error::custom(format!("ill-dimensioned measure expression refused: {e}"))
                })
            }
        }
    )*};
}

measure_serde!(VarId, Expr, Formula);
