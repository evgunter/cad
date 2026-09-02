//! Wire (serde) representations for the two types that must NOT
//! deserialize field-by-field:
//!
//! - [`Expr`] persists as a plain AST tree and is REBUILT through the
//!   dimension-checking smart constructors on load — a corrupt or
//!   hand-edited file can never smuggle an ill-dimensioned tree (or a
//!   non-finite literal) past the construction door. The cached
//!   dimension is deliberately not persisted: it re-derives.
//! - [`ProfileProgram`] persists STRUCTURALLY (plane placement columns
//!   plus per-loop step lists whose continuous args are [`Expr`]s) and
//!   its kernel-foreign tags (`ArcSweep`) via wire mirrors — the
//!   kernel crates gain no serde dependency (G1 layering). Crucially,
//!   deserialization can NEVER mint a `profile::ProfileLoop`: the wire
//!   rebuilds the PROGRAM only; loops exist only through the replay
//!   driver at evaluation (serde is transport, the driver is the door
//!   — LIB-SWITCH §4h, the strict-door rule at the program layer).

use geom_core::{Affine3, Mat3, Vec3};
use profile::{ArcSweep, SketchPlane};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::doc::ParamName;
use crate::expr::{Dimension, DimensionError, Expr, ExprKind};
use crate::measure::{MeasureExpr, MeasureKind, MeasurePrimitive};
use crate::program::{LoopProgram, ProfileProgram, ProgramStep, ProgramTarget};

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

/// A sketch-plane placement: the affine frame's four columns
/// (basis c0, c1, c2, then translation), each an exact (x, y, z).
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WirePlacement {
    /// The linear part's columns.
    basis: [[f64; 3]; 3],
    /// The translation column.
    origin: [f64; 3],
}

/// A structural travel-sense tag (`profile::ArcSweep`'s wire mirror;
/// the kernel crate stays serde-free).
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum WireWinding {
    /// Counterclockwise.
    Ccw,
    /// Clockwise.
    Cw,
}

impl WireWinding {
    fn from_sweep(w: ArcSweep) -> Self {
        match w {
            ArcSweep::Ccw => WireWinding::Ccw,
            ArcSweep::Cw => WireWinding::Cw,
        }
    }
    fn into_sweep(self) -> ArcSweep {
        match self {
            WireWinding::Ccw => ArcSweep::Ccw,
            WireWinding::Cw => ArcSweep::Cw,
        }
    }
}

/// A structural side tag (`profile::ArcSide`'s wire mirror).
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum WireSide {
    /// Centre on the left of travel.
    Left,
    /// Centre on the right of travel.
    Right,
}

impl WireSide {
    fn from_side(s: profile::ArcSide) -> Self {
        match s {
            profile::ArcSide::Left => WireSide::Left,
            profile::ArcSide::Right => WireSide::Right,
        }
    }
    fn into_side(self) -> profile::ArcSide {
        match self {
            WireSide::Left => profile::ArcSide::Left,
            WireSide::Right => profile::ArcSide::Right,
        }
    }
}

/// A step target on the wire (`Start` is structural).
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum WireTarget {
    /// The entry vertex — the closing form.
    Start,
    /// An authored point (two Length expressions; every `Expr` field
    /// on this wire rebuilds through the dimension door — per-ROLE
    /// dimension agreement is the shared validator's walk).
    Point([Expr; 2]),
}

impl WireTarget {
    fn from_target(t: &ProgramTarget) -> Self {
        match t {
            ProgramTarget::Start => WireTarget::Start,
            ProgramTarget::Point(p) => WireTarget::Point(p.clone()),
        }
    }
    fn into_target(self) -> ProgramTarget {
        match self {
            WireTarget::Start => ProgramTarget::Start,
            WireTarget::Point(p) => ProgramTarget::Point(p),
        }
    }
}

/// One chain step on the wire — `ProgramStep`'s structural mirror, and
/// the vocabulary's last stop. A verb reaching here is a FORMAT
/// change, not a mapping (the checked-in corpus regenerates; see the
/// persist module docs), so this enum going quietly short is worse
/// than its being a third spelling — a spelling can be reconciled
/// later; a format that has reached someone's disk (Band 4, once a
/// document ships) cannot.
///
/// It cannot go short of `ProgramStep`: [`WireStep::from_step`] and
/// [`WireStep::into_step`] are exhaustive on `ProgramStep` and on
/// `WireStep` respectively, so neither can gain a variant the other
/// lacks. What those two matches cannot see is a verb `profile`'s
/// transition table gained and `ProgramStep` never learned, or an arm
/// that maps one verb onto another's wire shape; both are checked by
/// the round-trip census in `tests/switch_program_vocabulary.rs`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum WireStep {
    /// `.at(p)`.
    At([Expr; 2]),
    /// `.angle(θ)`.
    Angle(Expr),
    /// `.toward(dx, dy)`.
    Toward {
        /// x component.
        dx: Expr,
        /// y component.
        dy: Expr,
    },
    /// `.tangent()`.
    Tangent,
    /// `.cusp()`.
    Cusp,
    /// `.turn(δ)`.
    Turn(Expr),
    /// `line(len)`.
    Line(Expr),
    /// `line_to(target)`.
    LineTo(WireTarget),
    /// `arc_to(spec)` — the unified §2c arc-spec record.
    ArcTo(WireArcData),
    /// `tangent_arc_to(target)`.
    TangentArcTo(WireTarget),
    /// `arc_continue(target)` — the declared-subdivision step.
    ArcContinue([Expr; 2]),
    /// `.fillet(r)`.
    Fillet(Expr),
    /// `fillet_arc(r, spec)`.
    FilletArc {
        /// The fillet radius.
        radius: Expr,
        /// The arc-arrival spec.
        spec: WireArcData,
    },
    /// `arc_fillet(spec, r)`.
    ArcFillet {
        /// The fused incoming-arc spec.
        spec: WireArcData,
        /// The fillet radius.
        radius: Expr,
    },
    /// `arc_fillet_arc(spec, r, spec₂)`.
    ArcFilletArc {
        /// The fused incoming-arc spec.
        spec: WireArcData,
        /// The fillet radius.
        radius: Expr,
        /// The arc-arrival spec.
        spec2: WireArcData,
    },
    /// `.to(anchor)`.
    FarEndTo([Expr; 2]),
    /// `.to(Start)`.
    CloseTo,
}

/// An arc spec on the wire (`ProgramArcData`'s structural mirror), and
/// the arc-mode vocabulary's last stop — a mode reaching here is a
/// format change for the same reason a verb is.
///
/// It cannot go short of `ProgramArcData`: [`WireArcData::from_spec`]
/// and [`WireArcData::into_spec`] are exhaustive on the document type
/// and on this one. What those two cannot see is a mode `profile`'s
/// vocabulary gained and `ProgramArcData` never learned, or an arm
/// mapping one mode onto another's wire shape; both are checked by
/// the mode census in `tests/switch_program_vocabulary.rs`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum WireArcData {
    /// `Radius { r, side }`.
    Radius {
        /// The carrier radius.
        r: Expr,
        /// Which side of the tangent the centre sits on.
        side: WireSide,
    },
    /// `Bulge { p, b }`.
    Bulge {
        /// The authored endpoint.
        target: WireTarget,
        /// The authored bulge.
        b: Expr,
    },
    /// `Via { q, p }`.
    Via {
        /// The through-point.
        q: [Expr; 2],
        /// The authored endpoint.
        target: WireTarget,
    },
    /// `Center { c, winding, p }`.
    Center {
        /// The carrier centre.
        c: [Expr; 2],
        /// Travel sense.
        winding: WireWinding,
        /// The authored anchor/endpoint.
        target: WireTarget,
    },
    /// `Sweep { r, side, angle }`.
    Sweep {
        /// The carrier radius.
        r: Expr,
        /// Which side the centre sits on.
        side: WireSide,
        /// The swept central angle.
        angle: Expr,
    },
    /// `ArcLen { r, side, len }`.
    ArcLen {
        /// The carrier radius.
        r: Expr,
        /// Which side the centre sits on.
        side: WireSide,
        /// The arc length.
        len: Expr,
    },
}

impl WireArcData {
    fn from_spec(s: &crate::program::ProgramArcData) -> Self {
        use crate::program::ProgramArcData as S;
        match s {
            S::Radius { r, side } => WireArcData::Radius {
                r: r.clone(),
                side: WireSide::from_side(*side),
            },
            S::Bulge { target, b } => WireArcData::Bulge {
                target: WireTarget::from_target(target),
                b: b.clone(),
            },
            S::Via { q, target } => WireArcData::Via {
                q: q.clone(),
                target: WireTarget::from_target(target),
            },
            S::Center { c, winding, target } => WireArcData::Center {
                c: c.clone(),
                winding: WireWinding::from_sweep(*winding),
                target: WireTarget::from_target(target),
            },
            S::Sweep { r, side, angle } => WireArcData::Sweep {
                r: r.clone(),
                side: WireSide::from_side(*side),
                angle: angle.clone(),
            },
            S::ArcLen { r, side, len } => WireArcData::ArcLen {
                r: r.clone(),
                side: WireSide::from_side(*side),
                len: len.clone(),
            },
        }
    }

    fn into_spec(self) -> crate::program::ProgramArcData {
        use crate::program::ProgramArcData as S;
        match self {
            WireArcData::Radius { r, side } => S::Radius {
                r,
                side: side.into_side(),
            },
            WireArcData::Bulge { target, b } => S::Bulge {
                target: target.into_target(),
                b,
            },
            WireArcData::Via { q, target } => S::Via {
                q,
                target: target.into_target(),
            },
            WireArcData::Center { c, winding, target } => S::Center {
                c,
                winding: winding.into_sweep(),
                target: target.into_target(),
            },
            WireArcData::Sweep { r, side, angle } => S::Sweep {
                r,
                side: side.into_side(),
                angle,
            },
            WireArcData::ArcLen { r, side, len } => S::ArcLen {
                r,
                side: side.into_side(),
                len,
            },
        }
    }
}

impl WireStep {
    fn from_step(s: &ProgramStep) -> Self {
        use ProgramStep as P;
        match s {
            P::At(p) => WireStep::At(p.clone()),
            P::Angle(e) => WireStep::Angle(e.clone()),
            P::Toward { dx, dy } => WireStep::Toward {
                dx: dx.clone(),
                dy: dy.clone(),
            },
            P::Tangent => WireStep::Tangent,
            P::Cusp => WireStep::Cusp,
            P::Turn(e) => WireStep::Turn(e.clone()),
            P::Line(e) => WireStep::Line(e.clone()),
            P::LineTo(t) => WireStep::LineTo(WireTarget::from_target(t)),
            P::ArcTo(spec) => WireStep::ArcTo(WireArcData::from_spec(spec)),
            P::TangentArcTo(t) => WireStep::TangentArcTo(WireTarget::from_target(t)),
            P::ArcContinue(p) => WireStep::ArcContinue(p.clone()),
            P::Fillet(e) => WireStep::Fillet(e.clone()),
            P::FilletArc { radius, spec } => WireStep::FilletArc {
                radius: radius.clone(),
                spec: WireArcData::from_spec(spec),
            },
            P::ArcFillet { spec, radius } => WireStep::ArcFillet {
                spec: WireArcData::from_spec(spec),
                radius: radius.clone(),
            },
            P::ArcFilletArc {
                spec,
                radius,
                spec2,
            } => WireStep::ArcFilletArc {
                spec: WireArcData::from_spec(spec),
                radius: radius.clone(),
                spec2: WireArcData::from_spec(spec2),
            },
            P::FarEndTo(p) => WireStep::FarEndTo(p.clone()),
            P::CloseTo => WireStep::CloseTo,
        }
    }

    fn into_step(self) -> ProgramStep {
        use ProgramStep as P;
        match self {
            WireStep::At(p) => P::At(p),
            WireStep::Angle(e) => P::Angle(e),
            WireStep::Toward { dx, dy } => P::Toward { dx, dy },
            WireStep::Tangent => P::Tangent,
            WireStep::Cusp => P::Cusp,
            WireStep::Turn(e) => P::Turn(e),
            WireStep::Line(e) => P::Line(e),
            WireStep::LineTo(t) => P::LineTo(t.into_target()),
            WireStep::ArcTo(spec) => P::ArcTo(spec.into_spec()),
            WireStep::TangentArcTo(t) => P::TangentArcTo(t.into_target()),
            WireStep::ArcContinue(p) => P::ArcContinue(p),
            WireStep::Fillet(e) => P::Fillet(e),
            WireStep::FilletArc { radius, spec } => P::FilletArc {
                radius,
                spec: spec.into_spec(),
            },
            WireStep::ArcFillet { spec, radius } => P::ArcFillet {
                spec: spec.into_spec(),
                radius,
            },
            WireStep::ArcFilletArc {
                spec,
                radius,
                spec2,
            } => P::ArcFilletArc {
                spec: spec.into_spec(),
                radius,
                spec2: spec2.into_spec(),
            },
            WireStep::FarEndTo(p) => P::FarEndTo(p),
            WireStep::CloseTo => P::CloseTo,
        }
    }
}

/// One loop program on the wire: a chain, or a carrier form.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum WireLoopProgram {
    /// A chain-vocabulary step list.
    Chain(Vec<WireStep>),
    /// `circle(centre, r)`.
    Circle {
        /// The centre.
        centre: [Expr; 2],
        /// The radius.
        radius: Expr,
    },
    /// `circle_split(centre, r, n, phase)` (`n` structural).
    CircleSplit {
        /// The centre.
        centre: [Expr; 2],
        /// The radius.
        radius: Expr,
        /// The subdivision count.
        n: u32,
        /// The first vertex's angle.
        phase: Expr,
    },
}

/// The profile payload's wire shape (module docs): placement + loop
/// PROGRAMS. No derived value is on this wire — segments, bulges and
/// joints are all replay products (V3: caches are not persisted).
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireProfile {
    /// The sketch plane placement.
    plane: WirePlacement,
    /// The loop programs: outer first, then holes, description order.
    loops: Vec<WireLoopProgram>,
}

impl Serialize for ProfileProgram {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let a = &self.plane.placement;
        let col = |v: Vec3<f64>| [v.x, v.y, v.z];
        let wire = WireProfile {
            plane: WirePlacement {
                basis: [col(a.linear.c0), col(a.linear.c1), col(a.linear.c2)],
                origin: col(a.translation),
            },
            loops: self
                .loops
                .iter()
                .map(|lp| match lp {
                    LoopProgram::Chain(steps) => {
                        WireLoopProgram::Chain(steps.iter().map(WireStep::from_step).collect())
                    }
                    LoopProgram::Circle { centre, radius } => WireLoopProgram::Circle {
                        centre: centre.clone(),
                        radius: radius.clone(),
                    },
                    LoopProgram::CircleSplit {
                        centre,
                        radius,
                        n,
                        phase,
                    } => WireLoopProgram::CircleSplit {
                        centre: centre.clone(),
                        radius: radius.clone(),
                        n: *n,
                        phase: phase.clone(),
                    },
                })
                .collect(),
        };
        wire.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for ProfileProgram {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let wire = WireProfile::deserialize(de)?;
        let v3 = |c: [f64; 3]| Vec3::new(c[0], c[1], c[2]);
        let placement = Affine3::from_parts(
            Mat3::from_cols(
                v3(wire.plane.basis[0]),
                v3(wire.plane.basis[1]),
                v3(wire.plane.basis[2]),
            ),
            v3(wire.plane.origin),
        );
        let loops = wire
            .loops
            .into_iter()
            .map(|lp| match lp {
                WireLoopProgram::Chain(steps) => {
                    LoopProgram::Chain(steps.into_iter().map(WireStep::into_step).collect())
                }
                WireLoopProgram::Circle { centre, radius } => {
                    LoopProgram::Circle { centre, radius }
                }
                WireLoopProgram::CircleSplit {
                    centre,
                    radius,
                    n,
                    phase,
                } => LoopProgram::CircleSplit {
                    centre,
                    radius,
                    n,
                    phase,
                },
            })
            .collect();
        Ok(ProfileProgram {
            plane: SketchPlane::new(placement),
            loops,
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
#[serde(deny_unknown_fields)]
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
