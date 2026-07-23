//! Node-to-kernel wiring (spec D3: wire, don't invent): each F4 node
//! maps to an EXISTING public kernel op; every editor-side geometric
//! judgment (direction normalization, the revolve axis's in-plane
//! projection, full-vs-partial classification) goes through the
//! kernel's decided-predicate door, never a raw comparison.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::{Affine3, Band, Decide, Mat3, Point2, Point3, Sign, Tolerance, Vec2, Vec3};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::transform::transform_rigid;
use topo::{Body, BooleanResult, intersect, subtract, union};

use super::slots::{self, SlotValues};
use super::{BooleanValue, DatumValue, NodeErrorKind, NodeResult, SplitSide, ValuePayload};
use crate::node::{Axis3, BooleanOp, Datum, Node, PatternKind, RecipeNodeId, SlotId};
use crate::profile_desc::ProfileDesc;

type Results<T> = BTreeMap<RecipeNodeId, NodeResult<T>>;
type OpResult<T> = Result<ValuePayload<T>, NodeErrorKind>;

/// Runs one node's op against its (already Ok) inputs and evaluated
/// slots.
pub(crate) fn run_op<T>(
    node: &Node<ProfileDesc>,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> OpResult<T>
where
    T: Decide + super::ContentBits,
{
    match node {
        Node::Datum(d) => wire_datum(d, vals),
        Node::Profile(desc) => wire_profile(desc),
        Node::Extrude { profile, .. } => wire_extrude(*profile, results, vals),
        Node::Revolve { profile, axis, .. } => wire_revolve(*profile, *axis, results, vals),
        Node::Split { target, tool } => wire_split(*target, *tool, results),
        Node::Boolean { op, a, b, declare } => wire_boolean(*op, *a, *b, *declare, results),
        Node::Transform { input, .. } => wire_transform(*input, results, vals),
        Node::Pattern { input, kind, .. } => wire_pattern(*input, kind, results, vals),
        Node::Declare { pairs } => Ok(ValuePayload::Declarations(pairs.clone())),
    }
}

/// The (Ok) value of an input node.
fn value_of<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
) -> Result<&super::NodeValue<T>, NodeErrorKind> {
    match results.get(&input) {
        Some(NodeResult::Ok(v)) => Ok(v),
        // Failed/Poisoned inputs never reach run_op (poison
        // propagation happens first); an absent entry is a dangling
        // reference.
        _ => Err(NodeErrorKind::MissingInput { input }),
    }
}

/// A single-body operand: a Body value, or a boolean's non-empty
/// result. Splits and patterns need PR 3's naming layer to select a
/// part — typed refusal, not a guess.
fn body_operand<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
) -> Result<Arc<Body<T>>, NodeErrorKind> {
    let v = value_of(results, input)?;
    match &v.payload {
        ValuePayload::Body(b) => Ok(Arc::clone(b)),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => Ok(Arc::clone(body)),
        ValuePayload::Boolean(BooleanValue::Empty) => Err(NodeErrorKind::EmptyOperand { input }),
        other => Err(NodeErrorKind::WrongOperand {
            input,
            expected: "body",
            found: other.kind_name(),
        }),
    }
}

/// The linear classification band (kernel-ambient tolerance).
fn band() -> Result<Band, NodeErrorKind> {
    Band::linear().map_err(NodeErrorKind::Band)
}

/// Normalizes a direction-valued vector; decided-zero length refuses,
/// in-band indeterminacy escalates (all through the one door).
fn unit<T: Decide>(v: Vec3<T>, role: &'static str) -> Result<Vec3<T>, NodeErrorKind> {
    match decide("eval_direction_norm", v.norm(), band()?) {
        Ok(Sign::Positive) => Ok(v.normalize()),
        Ok(_) => Err(NodeErrorKind::DegenerateDirection { role }),
        Err(source) => Err(NodeErrorKind::Escalated {
            predicate: "eval_direction_norm",
            source,
        }),
    }
}

/// A Length-valued `[Expr; 3]` triple as a point.
fn point3<T: Decide>(vals: &SlotValues<T>, f: fn(Axis3) -> SlotId) -> Option<Point3<T>> {
    let v = slots::vec3(vals, f)?;
    Some(Point3::new(v.x, v.y, v.z))
}

fn need_scalar<T: Decide>(vals: &SlotValues<T>, slot: SlotId) -> Result<T, NodeErrorKind> {
    slots::scalar(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })
}

fn need_vec3<T: Decide>(
    vals: &SlotValues<T>,
    f: fn(Axis3) -> SlotId,
) -> Result<Vec3<T>, NodeErrorKind> {
    slots::vec3(vals, f).ok_or(NodeErrorKind::MissingSlot { slot: f(Axis3::X) })
}

fn need_point3<T: Decide>(
    vals: &SlotValues<T>,
    f: fn(Axis3) -> SlotId,
) -> Result<Point3<T>, NodeErrorKind> {
    point3(vals, f).ok_or(NodeErrorKind::MissingSlot { slot: f(Axis3::X) })
}

fn wire_datum<T: Decide>(d: &Datum, vals: &SlotValues<T>) -> OpResult<T> {
    Ok(ValuePayload::Datum(match d {
        Datum::Plane { .. } => DatumValue::Plane {
            origin: need_point3(vals, SlotId::Origin)?,
            normal: unit(need_vec3(vals, SlotId::Normal)?, "datum plane normal")?,
        },
        Datum::Axis { .. } => DatumValue::Axis {
            origin: need_point3(vals, SlotId::Origin)?,
            dir: unit(need_vec3(vals, SlotId::Direction)?, "datum axis direction")?,
        },
        Datum::Point { .. } => DatumValue::Point {
            position: need_point3(vals, SlotId::Origin)?,
        },
    }))
}

fn wire_profile<T: Decide>(desc: &ProfileDesc) -> OpResult<T> {
    let validated = desc
        .embed::<T>()
        .validate(Tolerance::get())
        .map_err(NodeErrorKind::Profile)?;
    Ok(ValuePayload::Profile(Arc::new(validated)))
}

fn wire_extrude<T: Decide>(
    profile: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    let v = value_of(results, profile)?;
    let ValuePayload::Profile(vp) = &v.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: profile,
            expected: "profile",
            found: v.payload.kind_name(),
        });
    };
    let distance = need_scalar(vals, SlotId::Distance)?;
    let built = extrude(vp, Extrusion::Distance(distance)).map_err(NodeErrorKind::Extrude)?;
    Ok(ValuePayload::Body(Arc::new(built.body)))
}

fn wire_revolve<T: Decide>(
    profile: RecipeNodeId,
    axis: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    let pv = value_of(results, profile)?;
    let ValuePayload::Profile(vp) = &pv.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: profile,
            expected: "profile",
            found: pv.payload.kind_name(),
        });
    };
    let av = value_of(results, axis)?;
    let ValuePayload::Datum(DatumValue::Axis { origin, dir }) = &av.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: axis,
            expected: "datum axis",
            found: av.payload.kind_name(),
        });
    };
    // The kernel's RevolveAxis lives in SKETCH-PLANE coordinates: the
    // 3-D datum axis must lie in the profile's plane (decided; a
    // definite out-of-plane component is a typed refusal, spec D3's
    // "wire, don't invent" — projecting silently would be invention).
    let place = vp.plane().placement;
    let (u, v_axis, n) = (place.linear.c0, place.linear.c1, place.linear.c2);
    let plane_origin = Point3::new(
        place.translation.x,
        place.translation.y,
        place.translation.z,
    );
    let rel = *origin - plane_origin;
    let b = band()?;
    for (name, margin) in [
        ("revolve_axis_origin_in_plane", rel.dot(n)),
        ("revolve_axis_dir_in_plane", dir.dot(n)),
    ] {
        match decide(name, margin, b) {
            Ok(Sign::Zero) => {}
            Ok(_) => return Err(NodeErrorKind::AxisNotInSketchPlane { axis }),
            Err(source) => {
                return Err(NodeErrorKind::Escalated {
                    predicate: name,
                    source,
                });
            }
        }
    }
    let axis2 = RevolveAxis {
        origin: Point2::new(rel.dot(u), rel.dot(v_axis)),
        dir: Vec2::new(dir.dot(u), dir.dot(v_axis)),
    };
    // Full vs partial (kernel contract: exactly-full must SAY Full):
    // |θ| coincident with τ at tolerance classifies Full; anything
    // else wires Partial and the kernel's own angle classification
    // rules on it (out-of-range partials refuse loudly there).
    let angle = need_scalar(vals, SlotId::RevolveAngle)?;
    let abs_angle = angle.max(-angle);
    let revolution = match decide("revolve_full_vs_partial", abs_angle - T::tau(), b) {
        Ok(Sign::Zero) => Revolution::Full,
        Ok(_) => Revolution::Partial(angle),
        Err(source) => {
            return Err(NodeErrorKind::Escalated {
                predicate: "revolve_full_vs_partial",
                source,
            });
        }
    };
    let built = revolve(vp, axis2, revolution).map_err(NodeErrorKind::Revolve)?;
    Ok(ValuePayload::Body(Arc::new(built.body)))
}

fn wire_split<T: Decide>(
    target: RecipeNodeId,
    tool: RecipeNodeId,
    results: &Results<T>,
) -> OpResult<T> {
    let body = body_operand(results, target)?;
    let tv = value_of(results, tool)?;
    let ValuePayload::Datum(DatumValue::Plane { origin, normal }) = &tv.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: tool,
            expected: "datum plane",
            found: tv.payload.kind_name(),
        });
    };
    let plane = SplitPlane {
        origin: *origin,
        normal: *normal,
    };
    let result = split(&body, &plane).map_err(NodeErrorKind::Split)?;
    let side = |part: SplitPart<T>| match part {
        SplitPart::Body(b) => SplitSide::Body(Arc::new(b)),
        SplitPart::Empty => SplitSide::Empty,
    };
    Ok(ValuePayload::Split {
        above: side(result.above),
        below: side(result.below),
    })
}

fn wire_boolean<T: Decide>(
    op: BooleanOp,
    a: RecipeNodeId,
    b: RecipeNodeId,
    declare: Option<RecipeNodeId>,
    results: &Results<T>,
) -> OpResult<T> {
    // v1 (spec D3): a Declare input is validated for SHAPE and passed
    // through as data — threading its pairs into the kernel op is
    // PR 5's contract.
    if let Some(d) = declare {
        let dv = value_of(results, d)?;
        if !matches!(dv.payload, ValuePayload::Declarations(_)) {
            return Err(NodeErrorKind::WrongOperand {
                input: d,
                expected: "declarations",
                found: dv.payload.kind_name(),
            });
        }
    }
    let body_a = body_operand(results, a)?;
    let body_b = body_operand(results, b)?;
    let run = match op {
        BooleanOp::Union => union,
        BooleanOp::Intersect => intersect,
        BooleanOp::Subtract => subtract,
    };
    Ok(ValuePayload::Boolean(
        match run(&body_a, &body_b).map_err(NodeErrorKind::Boolean)? {
            BooleanResult::Empty => BooleanValue::Empty,
            BooleanResult::Body(bb) => BooleanValue::Body {
                body: Arc::new(bb.body),
                kind: bb.kind,
                contacts: Arc::new(bb.contacts),
            },
        },
    ))
}

fn wire_transform<T: Decide>(
    input: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    let body = body_operand(results, input)?;
    let translation = need_vec3(vals, SlotId::Translation)?;
    let rot_axis = unit(
        need_vec3(vals, SlotId::RotationAxis)?,
        "transform rotation axis",
    )?;
    let angle = need_scalar(vals, SlotId::RotationAngle)?;
    // PR 1's die convention: rotate about the axis THROUGH THE WORLD
    // ORIGIN by `angle`, then translate.
    let map = Affine3::from_parts(Mat3::rotation_about(rot_axis, angle), translation);
    let placed = transform_rigid(&body, &map).map_err(NodeErrorKind::Transform)?;
    Ok(ValuePayload::Body(Arc::new(placed)))
}

fn wire_pattern<T: Decide>(
    input: RecipeNodeId,
    kind: &PatternKind,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    let body = body_operand(results, input)?;
    let n = slots::count(vals, SlotId::Count).ok_or(NodeErrorKind::MissingSlot {
        slot: SlotId::Count,
    })?;
    if n < 1 {
        return Err(NodeErrorKind::NonPositiveCount { count: n });
    }
    let mut instances = Vec::new();
    // Instance 0 is the input body itself (identity placement, no op
    // re-run); `i as f64` is exact far beyond any representable
    // pattern (2^53).
    instances.push(Arc::clone(&body));
    for i in 1..n {
        let step = T::from_f64(i as f64);
        let map = match kind {
            PatternKind::Linear { .. } => {
                let dir = unit(need_vec3(vals, SlotId::Direction)?, "pattern direction")?;
                let spacing = need_scalar(vals, SlotId::Spacing)?;
                Affine3::translation(dir * (spacing * step))
            }
            PatternKind::Circular { axis, .. } => {
                let av = value_of(results, *axis)?;
                let ValuePayload::Datum(DatumValue::Axis { origin, dir }) = &av.payload else {
                    return Err(NodeErrorKind::WrongOperand {
                        input: *axis,
                        expected: "datum axis",
                        found: av.payload.kind_name(),
                    });
                };
                let angle = need_scalar(vals, SlotId::Step)?;
                Affine3::rotation_about_axis(*origin, *dir, angle * step)
            }
        };
        let placed = transform_rigid(&body, &map).map_err(NodeErrorKind::Transform)?;
        instances.push(Arc::new(placed));
    }
    Ok(ValuePayload::Instances(instances))
}
