//! **What the document types cannot say for themselves.** Everything
//! else in the recipe derives serde where it is declared; four things
//! cannot, and this module is exactly those four.
//!
//! # Two expression languages that must NOT deserialize field-by-field
//!
//! - [`Expr`] persists as a plain AST tree and is REBUILT through the
//!   dimension-checking smart constructors on load — a corrupt or
//!   hand-edited file can never smuggle an ill-dimensioned tree (or a
//!   non-finite literal) past the construction door. The cached
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

use crate::doc::ParamName;
use crate::expr::{Dimension, DimensionError, Expr, ExprKind};
use crate::measure::{MeasureExpr, MeasureKind, MeasurePrimitive};
use crate::node::RecipeNodeId;

/// The persisted expression tree (spec D1: the recipe is the save; an
/// expression is its constructor calls).
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum WireExpr {
    /// A continuous literal with its dimension and (optionally) the
    /// display unit it was authored in (LIB-SWITCH §4g — presentation
    /// metadata; the value stays canonical meters/radians).
    Literal {
        /// The exact value (D2: bit-exact round-trip), canonical units.
        value: f64,
        /// The literal's dimension.
        dim: Dimension,
        /// The display-unit symbol (quantity's closed table). Always
        /// written, because every literal names the notation it was
        /// authored in — the dimensionless row's symbol is the empty
        /// string, which is what a `Scalar` literal carries. An unknown
        /// symbol refuses typed at rebuild.
        unit: String,
    },
    /// An exact integer Count literal.
    Count(i64),
    /// A parameter reference with its declared dimension.
    Param {
        /// The referenced document parameter.
        name: ParamName,
        /// The dimension declared at construction.
        dim: Dimension,
    },
    /// Same-dimension addition.
    Add(Box<WireExpr>, Box<WireExpr>),
    /// Same-dimension subtraction.
    Sub(Box<WireExpr>, Box<WireExpr>),
    /// Negation.
    Neg(Box<WireExpr>),
    /// Product.
    Mul(Box<WireExpr>, Box<WireExpr>),
    /// Quotient.
    Div(Box<WireExpr>, Box<WireExpr>),
    /// Sine.
    Sin(Box<WireExpr>),
    /// Cosine.
    Cos(Box<WireExpr>),
    /// Tangent.
    Tan(Box<WireExpr>),
    /// Four-quadrant arctangent (y, x).
    Atan2(Box<WireExpr>, Box<WireExpr>),
    /// Lattice minimum.
    Min(Box<WireExpr>, Box<WireExpr>),
    /// Lattice maximum.
    Max(Box<WireExpr>, Box<WireExpr>),
    /// Explicit Count→Scalar promotion.
    CountToScalar(Box<WireExpr>),
}

impl From<&Expr> for WireExpr {
    fn from(e: &Expr) -> Self {
        let b = |x: &Expr| Box::new(WireExpr::from(x));
        match e.kind() {
            ExprKind::Literal(lit) => WireExpr::Literal {
                value: lit.value,
                dim: e.dim(),
                unit: lit.unit_def().symbol().to_string(),
            },
            ExprKind::CountLiteral(v) => WireExpr::Count(*v),
            ExprKind::Param(name) => WireExpr::Param {
                name: name.clone(),
                dim: e.dim(),
            },
            ExprKind::Add(x, y) => WireExpr::Add(b(x), b(y)),
            ExprKind::Sub(x, y) => WireExpr::Sub(b(x), b(y)),
            ExprKind::Neg(x) => WireExpr::Neg(b(x)),
            ExprKind::Mul(x, y) => WireExpr::Mul(b(x), b(y)),
            ExprKind::Div(x, y) => WireExpr::Div(b(x), b(y)),
            ExprKind::Sin(x) => WireExpr::Sin(b(x)),
            ExprKind::Cos(x) => WireExpr::Cos(b(x)),
            ExprKind::Tan(x) => WireExpr::Tan(b(x)),
            ExprKind::Atan2(x, y) => WireExpr::Atan2(b(x), b(y)),
            ExprKind::Min(x, y) => WireExpr::Min(b(x), b(y)),
            ExprKind::Max(x, y) => WireExpr::Max(b(x), b(y)),
            ExprKind::CountToScalar(x) => WireExpr::CountToScalar(b(x)),
        }
    }
}

impl WireExpr {
    /// Rebuilds the checked [`Expr`], re-running every dimension check
    /// and the non-finite-literal refusal (load door; module docs).
    pub(crate) fn rebuild(&self) -> Result<Expr, crate::expr::DimensionError> {
        let b = |x: &WireExpr| x.rebuild();
        match self {
            // Strict door: the symbol must be in quantity's closed
            // table and its quantity must match the dimension — both
            // re-checked by the same constructor authoring uses (never
            // a field-by-field trust).
            WireExpr::Literal { value, dim, unit } => match quantity::unit_by_symbol(unit) {
                None => Err(crate::expr::DimensionError::UnknownDisplayUnit {
                    symbol: unit.clone(),
                }),
                Some(u) => Expr::literal_with_unit(*value, *dim, u),
            },
            WireExpr::Count(v) => Ok(Expr::count(*v)),
            WireExpr::Param { name, dim } => Ok(Expr::param(name.clone(), *dim)),
            WireExpr::Add(x, y) => Expr::add(b(x)?, b(y)?),
            WireExpr::Sub(x, y) => Expr::sub(b(x)?, b(y)?),
            WireExpr::Neg(x) => Ok(Expr::neg(b(x)?)),
            WireExpr::Mul(x, y) => Expr::mul(b(x)?, b(y)?),
            WireExpr::Div(x, y) => Expr::div(b(x)?, b(y)?),
            WireExpr::Sin(x) => Expr::sin(b(x)?),
            WireExpr::Cos(x) => Expr::cos(b(x)?),
            WireExpr::Tan(x) => Expr::tan(b(x)?),
            WireExpr::Atan2(x, y) => Expr::atan2(b(x)?, b(y)?),
            WireExpr::Min(x, y) => Expr::min(b(x)?, b(y)?),
            WireExpr::Max(x, y) => Expr::max(b(x)?, b(y)?),
            WireExpr::CountToScalar(x) => Expr::count_to_scalar(b(x)?),
        }
    }
}

impl Serialize for Expr {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        WireExpr::from(self).serialize(ser)
    }
}

impl<'de> Deserialize<'de> for Expr {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let wire = WireExpr::deserialize(de)?;
        wire.rebuild()
            .map_err(|e| D::Error::custom(format!("ill-dimensioned expression refused: {e}")))
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
        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<RecipeNodeId, E> {
            Ok(RecipeNodeId(v))
        }
    }
    de.deserialize_u64(PlaneRef)
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
pub(crate) enum WireMeasureExpr {
    /// A closed-form measurement leaf.
    Primitive(MeasurePrimitive),
    /// An ordinary document expression leaf.
    Value(Box<WireExpr>),
    /// Same-dimension addition.
    Add(Box<WireMeasureExpr>, Box<WireMeasureExpr>),
    /// Same-dimension subtraction.
    Sub(Box<WireMeasureExpr>, Box<WireMeasureExpr>),
    /// Negation.
    Neg(Box<WireMeasureExpr>),
    /// Product (at least one Scalar operand).
    Mul(Box<WireMeasureExpr>, Box<WireMeasureExpr>),
    /// Quotient (Scalar divisor).
    Div(Box<WireMeasureExpr>, Box<WireMeasureExpr>),
    /// Same-dimension minimum.
    Min(Box<WireMeasureExpr>, Box<WireMeasureExpr>),
    /// Same-dimension maximum.
    Max(Box<WireMeasureExpr>, Box<WireMeasureExpr>),
}

impl From<&MeasureExpr> for WireMeasureExpr {
    fn from(e: &MeasureExpr) -> Self {
        let b = |x: &MeasureExpr| Box::new(WireMeasureExpr::from(x));
        match e.kind() {
            MeasureKind::Primitive(p) => WireMeasureExpr::Primitive(*p),
            MeasureKind::Value(v) => WireMeasureExpr::Value(Box::new(WireExpr::from(v))),
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

impl WireMeasureExpr {
    /// Rebuilds through the DIMENSION-CHECKING constructors — the load
    /// door is the construction door, so a file cannot carry a tree the
    /// authoring API refuses.
    fn rebuild(&self) -> Result<MeasureExpr, DimensionError> {
        let b = |x: &WireMeasureExpr| x.rebuild();
        match self {
            WireMeasureExpr::Primitive(p) => Ok(MeasureExpr::primitive(*p)),
            WireMeasureExpr::Value(v) => Ok(MeasureExpr::value(v.rebuild()?)),
            WireMeasureExpr::Add(x, y) => MeasureExpr::add(b(x)?, b(y)?),
            WireMeasureExpr::Sub(x, y) => MeasureExpr::sub(b(x)?, b(y)?),
            WireMeasureExpr::Neg(x) => Ok(MeasureExpr::neg(b(x)?)),
            WireMeasureExpr::Mul(x, y) => MeasureExpr::mul(b(x)?, b(y)?),
            WireMeasureExpr::Div(x, y) => MeasureExpr::div(b(x)?, b(y)?),
            WireMeasureExpr::Min(x, y) => MeasureExpr::min(b(x)?, b(y)?),
            WireMeasureExpr::Max(x, y) => MeasureExpr::max(b(x)?, b(y)?),
        }
    }
}

impl Serialize for MeasureExpr {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        WireMeasureExpr::from(self).serialize(ser)
    }
}

impl<'de> Deserialize<'de> for MeasureExpr {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let wire = WireMeasureExpr::deserialize(de)?;
        wire.rebuild().map_err(|e| {
            D::Error::custom(format!("ill-dimensioned measure expression refused: {e}"))
        })
    }
}
