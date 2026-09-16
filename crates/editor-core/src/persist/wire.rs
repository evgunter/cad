//! Wire (serde) representations for the two types that must NOT
//! deserialize field-by-field:
//!
//! - [`Expr`] persists as a plain AST tree and is REBUILT through the
//!   dimension-checking smart constructors on load — a corrupt or
//!   hand-edited file can never smuggle an ill-dimensioned tree (or a
//!   non-finite literal) past the construction door. The cached
//!   dimension is deliberately not persisted: it re-derives.
//! - [`ProfileProgram`] persists STRUCTURALLY (the `plane` NODE ID —
//!   twelve placement columns until the sketch plane became a node —
//!   plus per-loop step lists whose continuous args are [`Expr`]s).
//!   Crucially, deserialization can NEVER mint a
//!   `profile::ProfileLoop`: the wire rebuilds the PROGRAM only; loops
//!   exist only through the replay driver at evaluation (serde is
//!   transport, the driver is the door — LIB-SWITCH §4h, the
//!   strict-door rule at the program layer).
//!
//! # What is NOT here
//!
//! The document's own step vocabulary. [`crate::program::ProgramStep`]
//! and its two companions derive serde where they are declared, so the
//! document form IS the persisted form: there is one spelling of a
//! verb in this crate and nothing to keep in step with anything. The
//! consequence to hold onto is that a rename in `program.rs` is a
//! FORMAT change — pinned as literals by
//! `tests/switch_program_vocabulary.rs`, which is what used to be
//! bought by the vocabulary stopping here.
//!
//! What stays on this side of the layer is the part `program.rs`
//! cannot say: the two kernel-foreign tags a spec carries
//! (`profile::ArcSweep`, `profile::ArcSide`), which the orphan rule
//! puts out of reach of a derive and G1 layering keeps out of the
//! kernel crate, so they persist through the [`arc_sweep`] and
//! [`arc_side`] adapters below.

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::doc::ParamName;
use crate::expr::{Dimension, DimensionError, Expr, ExprKind};
use crate::measure::{MeasureExpr, MeasureKind, MeasurePrimitive};
use crate::node::RecipeNodeId;
use crate::program::{LoopProgram, ProfileProgram};

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

/// `profile::ArcSweep` on the wire.
///
/// A travel sense is the kernel's type, so `editor-core` cannot derive
/// serde for it — the orphan rule, and G1 layering says the kernel
/// crate does not gain the derive either. What is left is an adapter:
/// a private local enum with the persisted spelling, and the pair of
/// functions [`ProgramArcData`]'s `winding` field names through
/// `#[serde(with = …)]`.
///
/// [`ProgramArcData`]: crate::program::ProgramArcData
pub(crate) mod arc_sweep {
    use profile::ArcSweep;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// The persisted spelling of a travel sense.
    #[derive(Debug, Serialize, Deserialize)]
    enum Wire {
        /// Counterclockwise.
        Ccw,
        /// Clockwise.
        Cw,
    }

    /// Writes the tag.
    ///
    /// # Errors
    ///
    /// The serializer's own.
    pub(crate) fn serialize<S: Serializer>(w: &ArcSweep, ser: S) -> Result<S::Ok, S::Error> {
        match w {
            ArcSweep::Ccw => Wire::Ccw,
            ArcSweep::Cw => Wire::Cw,
        }
        .serialize(ser)
    }

    /// Reads the tag. Total: the wire enum has no form the kernel
    /// enum lacks, so anything that parses converts.
    ///
    /// # Errors
    ///
    /// The deserializer's own — a tag outside the two above.
    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<ArcSweep, D::Error> {
        Ok(match Wire::deserialize(de)? {
            Wire::Ccw => ArcSweep::Ccw,
            Wire::Cw => ArcSweep::Cw,
        })
    }
}

/// `profile::ArcSide` on the wire — [`arc_sweep`]'s twin, there for
/// the same reason.
pub(crate) mod arc_side {
    use profile::ArcSide;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// The persisted spelling of a side.
    #[derive(Debug, Serialize, Deserialize)]
    enum Wire {
        /// Centre on the left of travel.
        Left,
        /// Centre on the right of travel.
        Right,
    }

    /// Writes the tag.
    ///
    /// # Errors
    ///
    /// The serializer's own.
    pub(crate) fn serialize<S: Serializer>(s: &ArcSide, ser: S) -> Result<S::Ok, S::Error> {
        match s {
            ArcSide::Left => Wire::Left,
            ArcSide::Right => Wire::Right,
        }
        .serialize(ser)
    }

    /// Reads the tag. Total, for [`arc_sweep::deserialize`]'s reason.
    ///
    /// # Errors
    ///
    /// The deserializer's own — a tag outside the two above.
    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<ArcSide, D::Error> {
        Ok(match Wire::deserialize(de)? {
            Wire::Left => ArcSide::Left,
            Wire::Right => ArcSide::Right,
        })
    }
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
fn plane_ref<'de, D: Deserializer<'de>>(de: D) -> Result<RecipeNodeId, D::Error> {
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

/// The profile payload's wire shape (module docs): the FRAME NODE it
/// is drawn on + loop PROGRAMS. No derived value is on this wire —
/// segments, bulges and joints are all replay products (V3: caches are
/// not persisted).
///
/// **It is [`ProfileProgram`]'s shape field for field**, and it is
/// still a separate type because its `plane` carries a DOOR the
/// document type has no room for: the reading below. The loop
/// programs need no such thing, so they are on this wire as
/// themselves — the document vocabulary is the persisted vocabulary.
///
/// **`plane` was four placement columns and is now a node id.** That
/// is a BREAKING change to the format, which this format's one door
/// handles by refusing typed: a document written before it names a
/// `plane` object where this build expects a number, and `plane_ref`'s
/// visitor refuses that shape — naming the placement in its own
/// `expecting` — onto [`super::PersistError::Unreadable`] with the
/// regenerate recourse. `deny_unknown_fields` on this struct is not
/// what fires: `plane` is a field this build knows, so the refusal is
/// the field type's and not the attribute's.
/// No migration, by the module header's ruling — nothing has shipped,
/// and every checked-in document is regenerable.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireProfile {
    /// The frame datum node this profile is drawn on.
    #[serde(deserialize_with = "plane_ref")]
    plane: RecipeNodeId,
    /// The loop programs: outer first, then holes, description order.
    loops: Vec<LoopProgram>,
}

impl Serialize for ProfileProgram {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        WireProfile {
            plane: self.plane,
            loops: self.loops.clone(),
        }
        .serialize(ser)
    }
}

impl<'de> Deserialize<'de> for ProfileProgram {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let wire = WireProfile::deserialize(de)?;
        Ok(ProfileProgram {
            plane: wire.plane,
            loops: wire.loops,
        })
    }
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
