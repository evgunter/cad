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
use crate::expr::{Dimension, Expr, ExprKind};
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
        /// The display-unit symbol (quantity's closed table), absent
        /// for canonically-authored literals. Optional ON THE WIRE
        /// (`deny_unknown_fields` kept — absence is legal, an unknown
        /// FIELD still refuses); an unknown SYMBOL refuses at rebuild.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
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
                unit: lit.display_unit.map(|u| u.symbol.to_string()),
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
            WireExpr::Literal { value, dim, unit } => match unit {
                None => Expr::literal(*value, *dim),
                // Strict door: the symbol must be in quantity's closed
                // table and its quantity must match the dimension —
                // both re-checked by the same constructor authoring
                // uses (never a field-by-field trust).
                Some(symbol) => match quantity::unit_by_symbol(symbol) {
                    None => Err(crate::expr::DimensionError::UnknownDisplayUnit {
                        symbol: symbol.clone(),
                    }),
                    Some(u) => Expr::literal_with_unit(*value, *dim, u),
                },
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
            .map_err(|e| D::Error::custom(format!("ill-dimensioned expression refused: {e:?}")))
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

/// One chain step on the wire — `ProgramStep`'s structural mirror.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum WireStep {
    /// `.at(p)`.
    At([Expr; 2]),
    /// `.at_on(p, centre, winding)`.
    AtOn {
        /// The anchor.
        p: [Expr; 2],
        /// The carrier centre.
        centre: [Expr; 2],
        /// Travel sense.
        winding: WireWinding,
    },
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
    /// `.turn(δ)`.
    Turn(Expr),
    /// `line(len)`.
    Line(Expr),
    /// `line_to(target)`.
    LineTo(WireTarget),
    /// `arc_to(target, bulge)`.
    ArcTo {
        /// Where the leg ends.
        target: WireTarget,
        /// The authored bulge.
        bulge: Expr,
    },
    /// `arc_via(via, target)`.
    ArcVia {
        /// The through-point.
        via: [Expr; 2],
        /// Where the leg ends.
        target: WireTarget,
    },
    /// `arc_center(centre, target, winding)`.
    ArcCenter {
        /// The carrier centre.
        centre: [Expr; 2],
        /// Where the leg ends.
        target: WireTarget,
        /// Travel sense.
        winding: WireWinding,
    },
    /// `tangent_arc_to(target)`.
    TangentArcTo(WireTarget),
    /// `.fillet(r)`.
    Fillet(Expr),
    /// `.to(anchor)`.
    FarEndTo([Expr; 2]),
    /// `.to(Start)`.
    CloseTo,
    /// `.to_on(Start, centre, winding)`.
    CloseToOn {
        /// The arrival carrier centre.
        centre: [Expr; 2],
        /// Travel sense.
        winding: WireWinding,
    },
}

impl WireStep {
    fn from_step(s: &ProgramStep) -> Self {
        use ProgramStep as P;
        match s {
            P::At(p) => WireStep::At(p.clone()),
            P::AtOn { p, centre, winding } => WireStep::AtOn {
                p: p.clone(),
                centre: centre.clone(),
                winding: WireWinding::from_sweep(*winding),
            },
            P::Angle(e) => WireStep::Angle(e.clone()),
            P::Toward { dx, dy } => WireStep::Toward {
                dx: dx.clone(),
                dy: dy.clone(),
            },
            P::Tangent => WireStep::Tangent,
            P::Turn(e) => WireStep::Turn(e.clone()),
            P::Line(e) => WireStep::Line(e.clone()),
            P::LineTo(t) => WireStep::LineTo(WireTarget::from_target(t)),
            P::ArcTo { target, bulge } => WireStep::ArcTo {
                target: WireTarget::from_target(target),
                bulge: bulge.clone(),
            },
            P::ArcVia { via, target } => WireStep::ArcVia {
                via: via.clone(),
                target: WireTarget::from_target(target),
            },
            P::ArcCenter {
                centre,
                target,
                winding,
            } => WireStep::ArcCenter {
                centre: centre.clone(),
                target: WireTarget::from_target(target),
                winding: WireWinding::from_sweep(*winding),
            },
            P::TangentArcTo(t) => WireStep::TangentArcTo(WireTarget::from_target(t)),
            P::Fillet(e) => WireStep::Fillet(e.clone()),
            P::FarEndTo(p) => WireStep::FarEndTo(p.clone()),
            P::CloseTo => WireStep::CloseTo,
            P::CloseToOn { centre, winding } => WireStep::CloseToOn {
                centre: centre.clone(),
                winding: WireWinding::from_sweep(*winding),
            },
        }
    }

    fn into_step(self) -> ProgramStep {
        use ProgramStep as P;
        match self {
            WireStep::At(p) => P::At(p),
            WireStep::AtOn { p, centre, winding } => P::AtOn {
                p,
                centre,
                winding: winding.into_sweep(),
            },
            WireStep::Angle(e) => P::Angle(e),
            WireStep::Toward { dx, dy } => P::Toward { dx, dy },
            WireStep::Tangent => P::Tangent,
            WireStep::Turn(e) => P::Turn(e),
            WireStep::Line(e) => P::Line(e),
            WireStep::LineTo(t) => P::LineTo(t.into_target()),
            WireStep::ArcTo { target, bulge } => P::ArcTo {
                target: target.into_target(),
                bulge,
            },
            WireStep::ArcVia { via, target } => P::ArcVia {
                via,
                target: target.into_target(),
            },
            WireStep::ArcCenter {
                centre,
                target,
                winding,
            } => P::ArcCenter {
                centre,
                target: target.into_target(),
                winding: winding.into_sweep(),
            },
            WireStep::TangentArcTo(t) => P::TangentArcTo(t.into_target()),
            WireStep::Fillet(e) => P::Fillet(e),
            WireStep::FarEndTo(p) => P::FarEndTo(p),
            WireStep::CloseTo => P::CloseTo,
            WireStep::CloseToOn { centre, winding } => P::CloseToOn {
                centre,
                winding: winding.into_sweep(),
            },
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
