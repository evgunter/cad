//! Wire (serde) representations for the two types that must NOT
//! deserialize field-by-field:
//!
//! - [`Expr`] persists as a plain AST tree and is REBUILT through the
//!   dimension-checking smart constructors on load — a corrupt or
//!   hand-edited file can never smuggle an ill-dimensioned tree (or a
//!   non-finite literal) past the construction door. The cached
//!   dimension is deliberately not persisted: it re-derives.
//! - [`ProfileDesc`] wraps the profile crate's foreign `Profile<f64>`;
//!   the kernel crates gain no serde dependency (G1 layering), so the
//!   wire shape is written here structurally (plane placement
//!   columns plus loops of `(x, y, bulge)` vertices — exactly the
//!   `ProfileDesc::tokens` traversal).

use geom_core::{Affine3, Mat3, Point2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::doc::ParamName;
use crate::expr::{Dimension, Expr, ExprKind};
use crate::profile_desc::ProfileDesc;

/// The persisted expression tree (spec D1: the recipe is the save; an
/// expression is its constructor calls).
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum WireExpr {
    /// A continuous literal with its dimension.
    Literal {
        /// The exact value (D2: bit-exact round-trip).
        value: f64,
        /// The literal's dimension.
        dim: Dimension,
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
            ExprKind::Literal(v) => WireExpr::Literal {
                value: *v,
                dim: e.dim(),
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
            WireExpr::Literal { value, dim } => Expr::literal(*value, *dim),
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

/// One profile vertex: position and bulge.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireVertex {
    /// Sketch-plane x.
    x: f64,
    /// Sketch-plane y.
    y: f64,
    /// The arc bulge (0 = straight segment).
    bulge: f64,
}

/// The profile payload's wire shape (module docs).
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireProfile {
    /// The sketch plane placement.
    plane: WirePlacement,
    /// The loops: outer first, then holes, description order.
    loops: Vec<Vec<WireVertex>>,
}

impl Serialize for ProfileDesc {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let a = &self.0.plane.placement;
        let col = |v: Vec3<f64>| [v.x, v.y, v.z];
        let wire = WireProfile {
            plane: WirePlacement {
                basis: [col(a.linear.c0), col(a.linear.c1), col(a.linear.c2)],
                origin: col(a.translation),
            },
            loops: self
                .0
                .loops
                .iter()
                .map(|lp| {
                    lp.vertices
                        .iter()
                        .map(|v| WireVertex {
                            x: v.pos.x,
                            y: v.pos.y,
                            bulge: v.bulge,
                        })
                        .collect()
                })
                .collect(),
        };
        wire.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for ProfileDesc {
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
            .map(|lp| {
                ProfileLoop::new(
                    lp.into_iter()
                        .map(|v| ProfileVertex {
                            pos: Point2::new(v.x, v.y),
                            bulge: v.bulge,
                        })
                        .collect(),
                )
            })
            .collect();
        Ok(ProfileDesc(Profile::new(
            SketchPlane::new(placement),
            loops,
        )))
    }
}
