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
use topo::{
    Body, BooleanDeclarations, BooleanResult, CarriedContacts, GeomSource, VfContact, VvContact,
};

use super::slots::{self, SlotValues};
use super::{BooleanValue, DatumValue, NodeErrorKind, NodeResult, SplitSide, ValuePayload};
use crate::names::{self, NameTable};
use crate::node::{Axis3, BooleanOp, Datum, Node, PatternKind, RecipeNodeId, SlotId};
use crate::profile_desc::ProfileDesc;

type Results<T> = BTreeMap<RecipeNodeId, NodeResult<T>>;
/// An op's product: the payload plus its eagerly-emitted name table
/// (N4 — emission lives HERE in the wire layer, spec D4).
type OpResult<T> = Result<(ValuePayload<T>, Arc<NameTable>), NodeErrorKind>;
/// A bare payload (datum/profile lanes — empty tables).
type PayloadResult<T> = Result<ValuePayload<T>, NodeErrorKind>;

/// Runs one node's op against its (already Ok) inputs and evaluated
/// slots, emitting the node's name table alongside the payload.
pub(crate) fn run_op<T>(
    id: RecipeNodeId,
    node: &Node<ProfileDesc>,
    doc: &crate::doc::Doc<ProfileDesc>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    boolean_sweep: topo::SweepStrategy,
) -> OpResult<T>
where
    T: Decide + super::ContentBits + geom_core::Bounds,
{
    match node {
        Node::Datum(d) => Ok((wire_datum(d, vals)?, names::empty())),
        Node::Profile(desc) => Ok((wire_profile(desc)?, names::empty())),
        Node::Extrude { profile, .. } => wire_extrude(id, *profile, results, vals),
        Node::Revolve { profile, axis, .. } => wire_revolve(id, *profile, *axis, results, vals),
        Node::Loft { profiles, .. } => wire_loft(id, profiles, doc, vals),
        Node::Sweep { profile, path, .. } => wire_sweep(*profile, *path, doc, vals),
        Node::Fillet { target, .. } => wire_fillet(id, *target, results, vals),
        Node::Split { target, tool } => wire_split(id, *target, *tool, results),
        Node::Boolean { op, a, b, declare } => {
            wire_boolean(id, *op, *a, *b, *declare, doc, results, boolean_sweep)
        }
        Node::Transform { input, .. } => wire_transform(id, *input, results, vals),
        Node::Pattern { input, kind, .. } => wire_pattern(id, *input, kind, results, vals),
        Node::Declare { pairs } => Ok((ValuePayload::Declarations(pairs.clone()), names::empty())),
    }
}

/// Stamps every UNSOURCED description of `body` with this node's
/// minted [`GeomSource`]s, one shared index space in deterministic
/// arena order (D1/N6: every description minted by evaluation carries
/// its recipe source; pass-through descriptions keep the source they
/// arrived with). Per-evaluation identity — exactly the scope N6's
/// binding caveat allows.
fn stamp_minted<T: Decide>(body: &mut Body<T>, node: RecipeNodeId) {
    let mut idx: u32 = 0;
    let surfaces: Vec<_> = body
        .surfaces()
        .map(|(k, _)| k)
        .filter(|&k| body.surface_source(k).is_none())
        .collect();
    for k in surfaces {
        // Stamping a just-enumerated live key cannot fail.
        let _ = body.set_surface_source(k, GeomSource::minted(node.0, idx));
        idx += 1;
    }
    let curves: Vec<_> = body
        .curves()
        .map(|(k, _)| k)
        .filter(|&k| body.curve_source(k).is_none())
        .collect();
    for k in curves {
        let _ = body.set_curve_source(k, GeomSource::minted(node.0, idx));
        idx += 1;
    }
    let points: Vec<_> = body
        .points()
        .map(|(k, _)| k)
        .filter(|&k| body.point_source(k).is_none())
        .collect();
    for k in points {
        let _ = body.set_point_source(k, GeomSource::minted(node.0, idx));
        idx += 1;
    }
}

/// Re-stamps `placed`'s descriptions with `input`'s sources wrapped
/// by placing node `by` at `instance` (N6: the transform node
/// composes into `expr`). Keys are stable across `transform_rigid`,
/// so the input's rows map key-for-key.
fn compose_placed<T: Decide>(
    input: &Body<T>,
    placed: &mut Body<T>,
    by: RecipeNodeId,
    instance: u32,
) {
    let surfaces: Vec<_> = input
        .surfaces()
        .filter_map(|(k, _)| {
            input
                .surface_source(k)
                .map(|s| (k, s.placed(by.0, instance)))
        })
        .collect();
    for (k, src) in surfaces {
        let _ = placed.set_surface_source(k, src);
    }
    let curves: Vec<_> = input
        .curves()
        .filter_map(|(k, _)| input.curve_source(k).map(|s| (k, s.placed(by.0, instance))))
        .collect();
    for (k, src) in curves {
        let _ = placed.set_curve_source(k, src);
    }
    let points: Vec<_> = input
        .points()
        .filter_map(|(k, _)| input.point_source(k).map(|s| (k, s.placed(by.0, instance))))
        .collect();
    for (k, src) in points {
        let _ = placed.set_point_source(k, src);
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

fn wire_datum<T: Decide>(d: &Datum, vals: &SlotValues<T>) -> PayloadResult<T> {
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

fn wire_profile<T: Decide>(desc: &ProfileDesc) -> PayloadResult<T> {
    let validated = desc
        .embed::<T>()
        .validate(Tolerance::get())
        .map_err(NodeErrorKind::Profile)?;
    Ok(ValuePayload::Profile(Arc::new(validated)))
}

fn wire_extrude<T: Decide>(
    id: RecipeNodeId,
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
    let mut built = extrude(vp, Extrusion::Distance(distance)).map_err(NodeErrorKind::Extrude)?;
    // Eager N4 emission from the emitter's own maps, BEFORE the
    // structural handoff is dropped.
    let table = names::name_extrude(id, &built).map_err(NodeErrorKind::Naming)?;
    stamp_minted(&mut built.body, id);
    Ok((ValuePayload::Body(Arc::new(built.body)), table))
}

fn wire_revolve<T: Decide>(
    id: RecipeNodeId,
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
    let mut built = revolve(vp, axis2, revolution).map_err(NodeErrorKind::Revolve)?;
    let table = names::name_revolve(id, &built).map_err(NodeErrorKind::Naming)?;
    stamp_minted(&mut built.body, id);
    Ok((ValuePayload::Body(Arc::new(built.body)), table))
}

/// The constant-radius fillet node (M5 PR 12).
///
/// The recipe carries NO edge selection (see [`Node::Fillet`]): the op's
/// front door is "every edge of a convex, planar-faced,
/// trivalent-vertex polyhedron", so the request is the target body's
/// whole edge arena, enumerated in arena order — the deterministic
/// order every derived list in this kernel inherits (D9).
///
/// Failure is a TYPED refusal ([`NodeErrorKind::Fillet`]) carrying the
/// kernel's own error unaltered, exactly as the split/boolean arms
/// carry theirs. The input body is never passed through: a fillet that
/// did not happen must read as a failed node, not as a silently sharp
/// solid.
///
/// # Naming
///
/// The node emits the EMPTY table. The blend introduces faces, edges
/// and vertices that no [`RoleSeg`](crate::names::RoleSeg) vocabulary
/// describes yet, and `Filleted` carries no birth map to name them
/// from — so there is nothing to emit honestly, and inventing names by
/// matching is exactly what N4 forbids. The consequence is loud rather
/// than silent: every downstream reference into a filleted body fails
/// to resolve, and an appearance record targeting one is a typed loss.
/// A fillet naming emitter is banked with the in-place edge-blend
/// surgery it would name.
fn wire_fillet<T: Decide + geom_core::Bounds>(
    id: RecipeNodeId,
    target: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    let body = body_operand(results, target)?;
    let radius = need_scalar(vals, SlotId::Radius)?;
    let edges: Vec<_> = body.edges().map(|(k, _)| k).collect();
    let filleted = sweep::fillet::build::fillet_edges(&body, &edges, radius, band()?)
        .map_err(NodeErrorKind::Fillet)?;
    let mut out = filleted.body;
    // The blend's own surfaces/curves/points are minted HERE (D1/N6);
    // the supports' pass-through descriptions keep the source they
    // arrived with.
    stamp_minted(&mut out, id);
    Ok((ValuePayload::Body(Arc::new(out)), names::empty()))
}

fn wire_split<T: Decide>(
    id: RecipeNodeId,
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
    // Pass-through descriptions keep their sources (the clone carried
    // them); the split's fresh section planes get THIS node's (D1).
    let side = |part: SplitPart<T>| match part {
        SplitPart::Body(mut b) => {
            stamp_minted(&mut b, id);
            SplitSide::Body(Arc::new(b))
        }
        SplitPart::Empty => SplitSide::Empty,
    };
    let above = side(result.above);
    let below = side(result.below);
    let as_body = |s: &SplitSide<T>| match s {
        SplitSide::Body(b) => Some(Arc::clone(b)),
        SplitSide::Empty => None,
    };
    let target_table = Arc::clone(&value_of(results, target)?.name_table);
    let (ab, bb) = (as_body(&above), as_body(&below));
    let table = names::name_split(
        id,
        ab.as_deref(),
        bb.as_deref(),
        &result.naming,
        target,
        &target_table,
        &body,
        *normal,
    )
    .map_err(NodeErrorKind::Naming)?;
    Ok((ValuePayload::Split { above, below }, table))
}

// `Bounds` rides along for the boolean lane only (M5 PR 8): the sweep's
// BVH candidate generation reads coordinate brackets — the L7 driver-code
// allowance, threaded from `run_op`'s service bound.
#[allow(clippy::too_many_arguments)] // one parameter per named input; strategy is the §4.4 door
fn wire_boolean<T: Decide + geom_core::Bounds>(
    id: RecipeNodeId,
    op: BooleanOp,
    a: RecipeNodeId,
    b: RecipeNodeId,
    declare: Option<RecipeNodeId>,
    doc: &crate::doc::Doc<ProfileDesc>,
    results: &Results<T>,
    boolean_sweep: topo::SweepStrategy,
) -> OpResult<T> {
    // F5 threading (M4 PR 5): the Declare input's name pairs resolve
    // through the OPERANDS' name tables into the kernel's declared
    // coincidence data. Resolution failures are the N5 typed errors —
    // no silent drop, no best-effort gluing.
    let mut kernel_decls = BooleanDeclarations::none();
    if let Some(d) = declare {
        let dv = value_of(results, d)?;
        let ValuePayload::Declarations(pairs) = &dv.payload else {
            return Err(NodeErrorKind::WrongOperand {
                input: d,
                expected: "declarations",
                found: dv.payload.kind_name(),
            });
        };
        let a_table = Arc::clone(&value_of(results, a)?.name_table);
        let b_table = Arc::clone(&value_of(results, b)?.name_table);
        kernel_decls = resolve_declarations(pairs, doc, &a_table, &b_table)?;
    }
    let body_a = body_operand(results, a)?;
    let body_b = body_operand(results, b)?;
    let kernel_op = match op {
        BooleanOp::Union => topo::BooleanOp::Union,
        BooleanOp::Intersect => topo::BooleanOp::Intersect,
        BooleanOp::Subtract => topo::BooleanOp::Subtract,
    };
    match topo::boolean_op_with(kernel_op, &body_a, &body_b, &kernel_decls, boolean_sweep)
        .map_err(NodeErrorKind::Boolean)?
    {
        BooleanResult::Empty => Ok((ValuePayload::Boolean(BooleanValue::Empty), names::empty())),
        BooleanResult::Body(bb) => {
            let a_table = Arc::clone(&value_of(results, a)?.name_table);
            let b_table = Arc::clone(&value_of(results, b)?.name_table);
            let table = names::name_boolean(
                id,
                &bb.body,
                &bb.naming,
                &names::OperandCtx {
                    node: a,
                    table: &a_table,
                    body: &body_a,
                },
                &names::OperandCtx {
                    node: b,
                    table: &b_table,
                    body: &body_b,
                },
            )
            .map_err(NodeErrorKind::Naming)?;
            let mut body = bb.body;
            // Seam chords / minted descriptions get THIS node's
            // sources; everything carried keeps its own (D1).
            stamp_minted(&mut body, id);
            Ok((
                ValuePayload::Boolean(BooleanValue::Body {
                    body: Arc::new(body),
                    kind: bb.kind,
                    contacts: Arc::new(bb.contacts),
                }),
                table,
            ))
        }
    }
}

/// Resolves one Declare payload's name pairs against the two operand
/// tables into the kernel's [`BooleanDeclarations`] (F5, M4 PR 5).
///
/// v1 vocabulary: cross-operand Face–Face pairs (coincident-plane
/// glue intents) and same-operand Vertex–Vertex / Vertex–Face pairs
/// (carried 3′ contacts). Everything else refuses typed. Resolution
/// scope is deliberately the OPERANDS' tables (spec D4: "resolve
/// through the operands' name tables") — a name minted elsewhere in
/// the document is Vanished HERE even if some other node still
/// carries it.
fn resolve_declarations(
    pairs: &[(names::StableName, names::StableName)],
    doc: &crate::doc::Doc<ProfileDesc>,
    a_table: &NameTable,
    b_table: &NameTable,
) -> Result<BooleanDeclarations, NodeErrorKind> {
    use crate::names::{EntityKey, Entry};
    use crate::resolve::{Diagnosis, RecipeEditRef, ResolveError, TieWitness};
    use topo::Operand;

    let resolve_one = |name: &names::StableName| -> Result<(Operand, EntityKey), NodeErrorKind> {
        // NodeGone first (ids are never reused; below the mint
        // counter ⇒ deleted, at/above ⇒ foreign).
        if doc.node(name.node).is_none() {
            let edit = if name.node.0 < doc.next_id {
                RecipeEditRef::NodeDeleted { node: name.node }
            } else {
                RecipeEditRef::ForeignNode { node: name.node }
            };
            return Err(NodeErrorKind::DeclareResolve {
                error: Box::new(ResolveError::NodeGone {
                    name: name.clone(),
                    edit,
                }),
            });
        }
        let hit = |table: &NameTable,
                   op: Operand|
         -> Option<Result<(Operand, EntityKey), NodeErrorKind>> {
            match table.lookup(name) {
                Some(Entry::Unique(ent)) => Some(Ok((op, ent.key))),
                Some(Entry::Tied(ents)) => Some(Err(NodeErrorKind::DeclareResolve {
                    error: Box::new(ResolveError::Ambiguous {
                        name: name.clone(),
                        candidates: vec![name.clone()],
                        tie: TieWitness {
                            node: name.node,
                            at: name.clone(),
                            width: ents.len() as u32,
                        },
                    }),
                })),
                None => None,
            }
        };
        match (hit(a_table, Operand::A), hit(b_table, Operand::B)) {
            (Some(_), Some(_)) => Err(NodeErrorKind::DeclareBothOperands {
                name: Box::new(name.clone()),
            }),
            (Some(r), None) | (None, Some(r)) => r,
            // Absent from both operand tables: Vanished, with the
            // honest single-run fallback diagnosis (no prior run
            // is consultable mid-evaluation; `NodeChanged` names
            // the minting node as the disagreement site, not a
            // claim an edit happened — same posture as the
            // resolve ladder's total fallback).
            (None, None) => Err(NodeErrorKind::DeclareResolve {
                error: Box::new(ResolveError::Vanished {
                    name: name.clone(),
                    diagnosis: Diagnosis::RecipeEdit {
                        edit: RecipeEditRef::NodeChanged { node: name.node },
                    },
                    last_good: None,
                }),
            }),
        }
    };

    let mut out = BooleanDeclarations::none();
    for (n1, n2) in pairs {
        let (o1, k1) = resolve_one(n1)?;
        let (o2, k2) = resolve_one(n2)?;
        let unsupported = || NodeErrorKind::DeclareUnsupportedPair {
            kinds: (n1.kind, n2.kind),
            cross_operand: o1 != o2,
        };
        match ((o1, k1), (o2, k2)) {
            // Cross-operand face pair: the coincident-plane intent.
            ((Operand::A, EntityKey::Face(fa)), (Operand::B, EntityKey::Face(fb)))
            | ((Operand::B, EntityKey::Face(fb)), (Operand::A, EntityKey::Face(fa))) => {
                out.coincident_faces.push((fa, fb));
            }
            // Same-operand carried contacts.
            ((oa, EntityKey::Vertex(va)), (ob, EntityKey::Vertex(vb))) if oa == ob => {
                let c: &mut CarriedContacts = match oa {
                    Operand::A => &mut out.carried_a,
                    Operand::B => &mut out.carried_b,
                };
                c.vv.push(VvContact { a: va, b: vb });
            }
            ((oa, EntityKey::Vertex(v)), (ob, EntityKey::Face(f)))
            | ((ob, EntityKey::Face(f)), (oa, EntityKey::Vertex(v)))
                if oa == ob =>
            {
                let c: &mut CarriedContacts = match oa {
                    Operand::A => &mut out.carried_a,
                    Operand::B => &mut out.carried_b,
                };
                c.vf.push(VfContact { vertex: v, face: f });
            }
            _ => return Err(unsupported()),
        }
    }
    Ok(out)
}

fn wire_transform<T: Decide>(
    id: RecipeNodeId,
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
    let mut placed = transform_rigid(&body, &map).map_err(NodeErrorKind::Transform)?;
    // N6 composition: `transform_rigid` cleared the source records
    // (its geometric rewrite invalidates the bit-identity claim); the
    // recipe layer re-stamps each description with the INPUT's source
    // wrapped by this placing node (keys are stable across the op).
    // Unsourced input descriptions stay unsourced — never invented.
    compose_placed(&body, &mut placed, id, 0);
    // Identity-preserving pass-through (spec D2): the transform
    // contributes NO RolePath segment — `transform_rigid` is
    // key-stable (arenas rewritten in place of a clone), so the
    // input's table rows hold verbatim: same names, same keys, the
    // N1 derivation-path semantics (the name still points at the
    // MINTING node; the placement is recipe context, not identity).
    let table = Arc::clone(&value_of(results, input)?.name_table);
    Ok((ValuePayload::Body(Arc::new(placed)), table))
}

fn wire_pattern<T: Decide>(
    id: RecipeNodeId,
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
        let mut placed = transform_rigid(&body, &map).map_err(NodeErrorKind::Transform)?;
        // N6 composition, per structural instance (`Placed { node,
        // instance: i, .. }`): distinct instances are distinct
        // sources — their descriptions genuinely differ.
        compose_placed(&body, &mut placed, id, i as u32);
        instances.push(Arc::new(placed));
    }
    // Instance(i) wrapping (A8/N1): every master entity name wraps
    // per structural index; `transform_rigid` key-stability means
    // instance keys equal master keys.
    let master = Arc::clone(&value_of(results, input)?.name_table);
    let table = names::name_pattern(id, &master, n, &instances).map_err(NodeErrorKind::Naming)?;
    Ok((ValuePayload::Instances(instances), table))
}

// ---------------------------------------------------------------------
// M5 PR 10: the definitional §10.3/§10.4 nodes
// ---------------------------------------------------------------------

/// The Sweep node's frontier — the ONE remaining
/// [`NodeErrorKind::CurvedSolidFrontier`] door (M5 PR 10 fix pass,
/// review MAJOR-1; narrowed at M6-3 when the loft body landed and the
/// former `LOFT_FRONTIER` text retired with its frontier). Kept as a
/// constant so the acceptance rows assert the SAME text the node
/// produces.
///
/// §10.4's rigid-profile sweep needs the path as ONE curve. The recipe
/// layer cannot supply one: a `Node::Sweep`'s `path` operand is a
/// profile, a validated profile's loop is a CLOSED chain, and a closed
/// chain has two or more segments — even the minimal two-vertex loop is
/// two half-turn arcs. So there is no recipe-expressible path, and the
/// honest node-layer answer is a single refusal naming what is
/// missing: a joined-path composition lane (banked past M6 — the
/// PR 10 MAJ ruling, reaffirmed by the M6-3 spec §1).
///
/// `sweep::sweep_geometry` AND `sweep::sweep_body` are live and
/// exercised through the library API; it is only this NODE lane that
/// is gated, at one door, so the message cannot imply an expressible
/// case that does not exist.
pub(crate) const SWEEP_FRONTIER: &str = "a swept solid: the recipe's path operand is a profile LOOP — always \
     a closed chain of two or more segments, even at the minimal \
     two-vertex circle — while §10.4's rigid-profile sweep needs the \
     path as ONE curve, so every recipe-expressible sweep waits on a \
     joined-path composition lane (banked past M6); the swept BODY \
     machinery itself is live since M6-3 — sweep::sweep_body at the \
     library API";

/// One section of a loft, taken from the RECIPE's own `f64`
/// description rather than from the evaluated `T` payload.
///
/// Structure selection is `f64` (C6/D9): the skinned surface's knots,
/// degrees, and control bits must be identical in every scalar lane,
/// and every lane's profile is the same stored `f64` description
/// embedded through `from_f64`. Taking the description directly is
/// therefore not a shortcut — it is the only way the Interval lane
/// encloses the SAME surface the `f64` lane defines.
fn section_of(
    doc: &crate::doc::Doc<ProfileDesc>,
    id: RecipeNodeId,
) -> Result<(sweep::SectionSegments, Affine3<f64>), NodeErrorKind> {
    let Some(Node::Profile(desc)) = doc.nodes.get(&id) else {
        return Err(NodeErrorKind::WrongOperand {
            input: id,
            expected: "profile node",
            found: "not a profile node",
        });
    };
    // The profile's own validation door still runs: a section that
    // would not extrude does not skin either.
    let validated = desc
        .embed::<f64>()
        .validate(Tolerance::get())
        .map_err(NodeErrorKind::Profile)?;
    let place = validated.plane().placement;
    let chains = desc
        .0
        .loops
        .iter()
        .map(|lp| {
            let n = lp.vertices.len();
            (0..n)
                .map(|j| {
                    let a = lp.vertices[j];
                    let b = lp.vertices[(j + 1) % n];
                    if a.bulge == 0.0 {
                        sweep::SketchSegment::Line { a: a.pos, b: b.pos }
                    } else {
                        sweep::SketchSegment::Arc {
                            a: a.pos,
                            b: b.pos,
                            bulge: a.bulge,
                        }
                    }
                })
                .collect()
        })
        .collect();
    Ok((chains, place))
}

/// A structural (Count) slot, refused typed when absent or unusable.
fn need_count(vals: &SlotValues<impl Decide>, slot: SlotId) -> Result<usize, NodeErrorKind> {
    let n = slots::count(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })?;
    usize::try_from(n).map_err(|_| NodeErrorKind::NonPositiveCount { count: n })
}

/// The Loft node (M6-3: the frontier flipped to the BUILDER — the
/// §10.3 walls plus the M5-LOG item-6 assembly, tiers 1–3 green at
/// rest).
fn wire_loft<T: Decide>(
    id: RecipeNodeId,
    profiles: &[RecipeNodeId],
    doc: &crate::doc::Doc<ProfileDesc>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    let v_degree = need_count(vals, SlotId::VDegree)?;
    let mut sections = Vec::with_capacity(profiles.len());
    let mut places = Vec::with_capacity(profiles.len());
    for pid in profiles {
        let (chain, place) = section_of(doc, *pid)?;
        sections.push(chain);
        places.push(place);
    }
    // The geometry/profile doors keep their historical node-error
    // shapes (the §2 compatibility contract predates the builder);
    // assembly-proper refusals arrive as the M6-3 `Loft` kind.
    let mut built = sweep::loft_body::<T>(&sections, &places, v_degree).map_err(|e| match e {
        sweep::LoftError::Skin(s) => NodeErrorKind::Skin(s),
        sweep::LoftError::Profile(p) => NodeErrorKind::Profile(p),
        other => NodeErrorKind::Loft(other),
    })?;
    // Eager N4 emission from the builder's own maps, BEFORE the
    // structural handoff is dropped (the extrude idiom).
    let table = names::name_loft(id, &built).map_err(NodeErrorKind::Naming)?;
    stamp_minted(&mut built.body, id);
    Ok((ValuePayload::Body(Arc::new(built.body)), table))
}

/// The Sweep node (M5 PR 10 fix pass, review MAJOR-1: ONE honest
/// arm).
///
/// Every RECIPE door still runs first — the structural slots, and both
/// operands through [`section_of`] — because a Sweep on a datum, or
/// with a bad Count, is a recipe error and must read as one. What the
/// node cannot do is reach the geometry: see [`SWEEP_FRONTIER`] for
/// why no recipe-expressible path exists to sweep.
fn wire_sweep<T: Decide>(
    profile: RecipeNodeId,
    path: RecipeNodeId,
    doc: &crate::doc::Doc<ProfileDesc>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    let _stations = need_count(vals, SlotId::Stations)?;
    let _v_degree = need_count(vals, SlotId::VDegree)?;
    let _ = section_of(doc, profile)?;
    let _ = section_of(doc, path)?;
    Err(NodeErrorKind::CurvedSolidFrontier {
        what: SWEEP_FRONTIER,
    })
}
