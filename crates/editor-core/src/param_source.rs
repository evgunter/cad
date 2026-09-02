//! **The single spelling of lowered expression identity**, and the
//! attach-at-mint pass that puts it on the fields a verb's parameter
//! reached.
//!
//! # The token
//!
//! [`lower`] turns a slot expression into the opaque
//! [`topo::ParamSource`] the kernel compares. The lowering
//! is a **canonical prefix encoding of the expression tree**: one tag
//! byte per AST node, operands in [`Expr::child`] order, literals as
//! their `f64` BITS, parameters as their names. It is injective — every
//! distinct expression has a distinct byte string — so token equality
//! IS expression equality rather than a claim about it, and [`invert`]
//! reads a token back to the slot address that produced it.
//!
//! **Bit-semantic, matching [`Expr::bit_eq`]**: `0.0` and `-0.0` are
//! different expressions here, and the display unit a literal was
//! authored in is not part of identity at all (D7).
//!
//! **Why a function of the expression and not an index into a table.**
//! A counter handing out ids in evaluation order would be a fact about
//! the RUN, and the memo serves bodies built by an earlier run: a node
//! whose content key is unchanged keeps its geometry, records and all,
//! while its siblings re-run. Two tokens from two different runs' tables
//! would then meet inside one body, and index `0` of the old table would
//! compare EQUAL to index `0` of the new one while naming different
//! expressions — a false `Declared`, which is the one failure this
//! channel must not have. The `GeomSource` records avoid it the same
//! way, by being functions of stable recipe identity (a node id) rather
//! than of an evaluation counter, and this is that discipline at
//! expression granularity. Determinism (D9) is then immediate: the same
//! recipe lowers to the same bytes on every run, in any node order,
//! under any scheduling.
//!
//! # The attach
//!
//! [`attach_blend`] walks the verb's DECLARED parameter→field flow
//! (`verbs::ParamFlow`) and stamps the token on exactly the fields it
//! names, found through the operation's own birth record. Nothing here
//! knows what a fillet is: it knows a flow, a record, and the rule that
//! a role family's carriers are the faces that family's rows name.
//!
//! The kernel never mints, composes or interprets one of these — see
//! `topo::param_source`.

use geom::Surface;
use geom_core::Real;
use sweep::blend::naming::BlendNaming;
use topo::{Body, FaceKey, ParamSource, SurfaceField};
use verbs::{FieldRole, ParamFlow, RoleFamily, ScalarParam};

use crate::expr::{Dimension, Expr, ExprKind};

// The tag alphabet. Fixed arity per tag is what makes the prefix
// encoding injective: a reader knows how many operands to expect from
// the tag alone, so no separators and no lengths are needed above the
// leaves.
const T_LITERAL: u8 = 0x01;
const T_COUNT_LITERAL: u8 = 0x02;
const T_PARAM: u8 = 0x03;
const T_ADD: u8 = 0x10;
const T_SUB: u8 = 0x11;
const T_NEG: u8 = 0x12;
const T_MUL: u8 = 0x13;
const T_DIV: u8 = 0x14;
const T_SIN: u8 = 0x15;
const T_COS: u8 = 0x16;
const T_TAN: u8 = 0x17;
const T_ATAN2: u8 = 0x18;
const T_MIN: u8 = 0x19;
const T_MAX: u8 = 0x1a;
const T_COUNT_TO_SCALAR: u8 = 0x1b;

/// The dimension byte carried by the two leaves that are not
/// determined by their own tag.
fn dim_code(dim: Dimension) -> u8 {
    match dim {
        Dimension::Length => 0,
        Dimension::Angle => 1,
        Dimension::Count => 2,
        Dimension::Scalar => 3,
    }
}

fn encode(expr: &Expr, out: &mut Vec<u8>) {
    // EXHAUSTIVE over the AST vocabulary with no wildcard arm (D3): a
    // new expression node cannot reach the kernel as an unlabelled
    // token, it breaks this match first.
    match expr.kind() {
        ExprKind::Literal(lit) => {
            out.push(T_LITERAL);
            out.extend_from_slice(&lit.value.to_bits().to_be_bytes());
            out.push(dim_code(expr.dim()));
        }
        ExprKind::CountLiteral(n) => {
            out.push(T_COUNT_LITERAL);
            out.extend_from_slice(&n.to_be_bytes());
        }
        ExprKind::Param(name) => {
            out.push(T_PARAM);
            let bytes = name.0.as_bytes();
            // A length prefix, because a name is the one payload with
            // no fixed width; `u32` is beyond any name a document can
            // hold and the cast is checked below.
            let len = u32::try_from(bytes.len()).unwrap_or(u32::MAX);
            out.extend_from_slice(&len.to_be_bytes());
            out.extend_from_slice(&bytes[..len as usize]);
            out.push(dim_code(expr.dim()));
        }
        ExprKind::Add(a, b) => binary(T_ADD, a, b, out),
        ExprKind::Sub(a, b) => binary(T_SUB, a, b, out),
        ExprKind::Mul(a, b) => binary(T_MUL, a, b, out),
        ExprKind::Div(a, b) => binary(T_DIV, a, b, out),
        ExprKind::Atan2(a, b) => binary(T_ATAN2, a, b, out),
        ExprKind::Min(a, b) => binary(T_MIN, a, b, out),
        ExprKind::Max(a, b) => binary(T_MAX, a, b, out),
        ExprKind::Neg(a) => unary(T_NEG, a, out),
        ExprKind::Sin(a) => unary(T_SIN, a, out),
        ExprKind::Cos(a) => unary(T_COS, a, out),
        ExprKind::Tan(a) => unary(T_TAN, a, out),
        ExprKind::CountToScalar(a) => unary(T_COUNT_TO_SCALAR, a, out),
    }
}

fn binary(tag: u8, a: &Expr, b: &Expr, out: &mut Vec<u8>) {
    out.push(tag);
    encode(a, out);
    encode(b, out);
}

fn unary(tag: u8, a: &Expr, out: &mut Vec<u8>) {
    out.push(tag);
    encode(a, out);
}

/// **The lowered identity of one slot expression.**
///
/// Two slots anywhere in the document lower to the same token exactly
/// when they hold the same expression — which is what makes "both walls
/// offset by the same declared `t`" equal by construction (both are
/// `r − t`) while `r` and `r − t` differ.
pub(crate) fn lower(expr: &Expr) -> ParamSource {
    let mut bytes = Vec::new();
    encode(expr, &mut bytes);
    ParamSource::from_lowered(&bytes)
}

/// **The token read back, as the address that produced it** — the
/// inversion this side of the line owes an opaque token.
///
/// The kernel cannot answer "which expression is this?" and is not
/// meant to; here the document is in hand, so the answer is a real
/// address: the first slot of the first node whose expression lowers to
/// `token`, scanned in the document's own deterministic node order
/// ([`Doc::order`](crate::doc::Doc::order)). A
/// token minted from a slot this document no longer holds — the scope
/// caveat of `topo::source`, which binds this channel verbatim — has no
/// address and answers `None`.
///
/// The scan is the honest shape for a diagnosis door: a stored
/// token→address table would be a second spelling of the recipe with
/// nothing forcing it to agree, and diagnosis is not on a hot path.
#[must_use]
pub fn invert<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    token: &ParamSource,
) -> Option<crate::expr::ExprPath> {
    for &node in doc.order() {
        let Some(n) = doc.node(node) else { continue };
        for slot in n.slots() {
            let Some(expr) = n.expr(slot) else { continue };
            if lower(expr) == *token {
                return Some(crate::expr::ExprPath {
                    node,
                    slot,
                    path: Vec::new(),
                });
            }
        }
    }
    None
}

/// **Which stored field of THIS carrier a declared field role names.**
///
/// The role is the flow's vocabulary — "the rolling ball's radius on a
/// blend face" — and the field is the description's. One role can be
/// two fields because one quantity has two spellings: the rolling
/// radius is a cylinder's `radius` where the spine is straight and a
/// torus's `minor_radius` where it is curved.
///
/// `None` is a real answer, not a gap: a carrier kind the role does not
/// name has no field to stamp, so the channel simply does not reach it
/// and the consuming family reads absence — which refuses typed. The
/// match is exhaustive on the kind axis, so a new surface arm is a
/// compile-time visit here and cannot be silently skipped.
fn field_of<T: Real>(role: FieldRole, carrier: &Surface<T>) -> Option<SurfaceField> {
    match role {
        FieldRole::BlendCarrierRadius => match carrier {
            Surface::Cylinder { .. } => Some(SurfaceField::CylinderRadius),
            Surface::Torus { .. } => Some(SurfaceField::TorusMinorRadius),
            // A blend face on any other carrier is not a rolling-ball
            // band whose radius IS the parameter — a chamfer's strip is
            // a plane positioned by its setback, and the spline arms
            // store a control net, not a radius.
            Surface::Plane { .. }
            | Surface::Sphere { .. }
            | Surface::Cone { .. }
            | Surface::Nurbs(_)
            | Surface::Approx(_) => None,
        },
        FieldRole::CornerCarrierRadius => match carrier {
            Surface::Sphere { .. } => Some(SurfaceField::SphereRadius),
            Surface::Plane { .. }
            | Surface::Cylinder { .. }
            | Surface::Torus { .. }
            | Surface::Cone { .. }
            | Surface::Nurbs(_)
            | Surface::Approx(_) => None,
        },
        FieldRole::BandCarrierMinorRadius => match carrier {
            Surface::Torus { .. } => Some(SurfaceField::TorusMinorRadius),
            Surface::Plane { .. }
            | Surface::Cylinder { .. }
            | Surface::Sphere { .. }
            | Surface::Cone { .. }
            | Surface::Nurbs(_)
            | Surface::Approx(_) => None,
        },
    }
}

/// The faces a role family's rows name in a blend birth record.
fn family_faces(family: RoleFamily, rec: &BlendNaming) -> Vec<FaceKey> {
    match family {
        RoleFamily::Blends => rec.blends.iter().map(|&(f, _)| f).collect(),
        RoleFamily::Corners => rec.corners.iter().map(|&(f, _)| f).collect(),
        RoleFamily::Bands => rec.bands.iter().map(|(f, _)| *f).collect(),
    }
}

/// **Attach-at-mint for a blend verb**: stamp `token` on every stored
/// field the verb's declared flow says `param` reached.
///
/// The flow is the kernel-side declaration
/// (`verbs::VerbKind::param_flow`) and this is its only consumer: the
/// document layer knows the expression, the verb knows where its
/// parameter lands, and the two meet exactly here. A flow with no rows
/// for `param` — the chamfer's setback, which positions planes and is
/// stored in none of them — attaches nothing, which is the declaration
/// being obeyed rather than a case being skipped.
pub(crate) fn attach_blend<T: Real>(
    body: &mut Body<T>,
    flow: &[ParamFlow],
    param: ScalarParam,
    token: &ParamSource,
    rec: &BlendNaming,
) {
    let Some(row) = flow.iter().find(|row| row.param == param) else {
        return;
    };
    let mut stamps: Vec<(topo::SurfaceKey, SurfaceField)> = Vec::new();
    for &role in row.fields {
        for face in family_faces(role.family(), rec) {
            let Some(surface_key) = body.get_face(face).map(|f| f.surface) else {
                continue;
            };
            let Some(carrier) = body.get_surface(surface_key) else {
                continue;
            };
            if let Some(field) = field_of(role, carrier) {
                stamps.push((surface_key, field));
            }
        }
    }
    for (surface_key, field) in stamps {
        // The key and the field were read off this body a moment ago,
        // so neither refusal door can fire; the door is fallible for
        // callers who did not.
        let _ = body.set_surface_field_source(surface_key, field, token.clone());
    }
}
