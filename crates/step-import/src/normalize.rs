//! **Reported structure normalizations** for periodic faces the kernel
//! cannot represent as stated (M7-2 Leg C; the sphere's edge-free face
//! is the sibling case, minted in [`crate::entities`] where the bound
//! is read).
//!
//! # Why a shell can arrive unrepresentable
//!
//! Open CASCADE never splits a periodic face: a full cylinder wall, a
//! full cone, and a whole torus each arrive as ONE `ADVANCED_FACE`
//! whose loop uses its seam edge twice. Most of that is fine — the
//! importer's manifold precondition and the seam adoption rung were
//! built for exactly that shape, and the cylinder and the truncated
//! cone assemble and certify as stated. Two shapes do not:
//!
//! - **The apex cone.** Its lateral face is `(seam, base circle,
//!   seam)`, and the seam's far end is the apex — a vertex with ONE
//!   incident edge. `topo`'s tier-2 validity calls a valence-1 vertex
//!   construction scaffolding (`ScaffoldingStrutVertex`), because in a
//!   finished solid it is: the strut tip of an unfinished Euler
//!   sequence. A body carrying one is not a closed solid by the
//!   kernel's own definition.
//! - **The whole torus.** One face wrapping the FULL period in BOTH
//!   chart directions is the fundamental-polygon square, whose four
//!   sides are two curves each used twice. The topology closes
//!   (Euler–Poincaré gives genus 1 and tier 2 passes), but the face is
//!   not a chart iso-rectangle, and the closed-form divergence
//!   contribution reads its own `Δu` off a rim that appears with both
//!   orientations — the volume comes back with the right magnitude and
//!   the wrong SIGN, which tier 3 catches as `NegativeVolume`.
//!
//! # What this module does about it
//!
//! The same D7 stage-3 repair the edge-free sphere takes: the LOCUS is
//! fully explained by the file's own surfaces and carriers and is
//! adopted unchanged — every minted curve is either a sub-arc of a
//! carrier the file states or that carrier's half-turn rotation about
//! the face's own axis, never a new shape — and only the
//! boundary-graph tessellation is re-minted, into the splitting a
//! natively built body carries (the kernel's own cone is two lateral
//! half-faces; its own torus is two half-faces). The census mapping is
//! carried out as [`crate::StructureNormalization`] data.
//!
//! Anything that does not match these shapes exactly is left alone —
//! this is a bounded repair of two named cases, not a licence to
//! re-tessellate.
//!
//! # Orientation is derived, never minted
//!
//! A re-tessellation decides how its new faces are wound, which makes
//! it the exact place an inside-out solid could be quietly turned
//! right-side-out. The torus is where that bites: its face is the
//! fundamental-polygon square `[x, y, x', y']`, whose multiset of
//! oriented uses is INVARIANT under loop reversal, so the winding
//! lives only in the cyclic ORDER. [`full_torus`] reads that order —
//! against `u` and `v` directions sampled through the kernel's own
//! chart inverse — and cross-checks it against the face's
//! `same_sense`. The two consistent statements of a right-side-out
//! ring (`.T.` with a CCW loop, `.F.` with a CW one) both adopt to the
//! same body; the two inverted ones REFUSE typed. Nothing about the
//! file's material side is assumed, and nothing is repaired into
//! existence.

use std::collections::BTreeMap;

use geom_core::{Point3, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};
use crate::error::StepImportError;
use crate::{FaceCensus, NormalizationKind, StructureNormalization};

/// Rotates `w` a half turn about the unit direction `axis` — the exact
/// f64 identity `2(w·â)â − w`, with no trigonometry to round.
fn half_turn(w: Vec3<f64>, axis: Vec3<f64>) -> Vec3<f64> {
    // `plus_zero`: a half turn negates exact zeros into `−0.0`, and a
    // minted field carrying one would cost the adoption pass its
    // fixed point over a printed sign (`geometry::plus_zero`).
    crate::geometry::plus_zero(axis * (2.0 * w.dot(axis)) - w)
}

/// A carrier rotated a half turn about `(origin, axis)` — the copy of
/// a seam curve half a period away. Only the two kinds the normalized
/// shapes use exist here; anything else answers `None` and the caller
/// leaves the face alone.
fn half_turn_curve(
    carrier: &Curve3<f64>,
    origin: Point3<f64>,
    axis: Vec3<f64>,
) -> Option<Curve3<f64>> {
    let point =
        |p: Point3<f64>| crate::geometry::plus_zero_point(origin + half_turn(p - origin, axis));
    match *carrier {
        Curve3::Line { origin: o, dir } => Some(Curve3::Line {
            origin: point(o),
            dir: half_turn(dir, axis),
        }),
        Curve3::Circle {
            center,
            axis: n,
            radius,
            u_ref,
        } => Some(Curve3::Circle {
            center: point(center),
            axis: half_turn(n, axis),
            radius,
            u_ref: half_turn(u_ref, axis),
        }),
        _ => None,
    }
}

/// The shell's vertex valences: how many edge ENDS meet each vertex (a
/// self-loop contributes two).
fn valences(solid: &SolidSpec) -> BTreeMap<u64, usize> {
    let mut out = BTreeMap::new();
    for spec in solid.edges.values() {
        *out.entry(spec.start).or_insert(0) += 1;
        *out.entry(spec.end).or_insert(0) += 1;
    }
    out
}

/// Replaces every use of `edge` in the shell, EXCEPT inside
/// `skip_faces`, by the two halves it was split into.
///
/// `first` runs from the split edge's own start to the new midpoint
/// vertex; `second` runs from the midpoint back to the end. A forward
/// use therefore walks `first` then `second`; a reversed use walks
/// `second` then `first`, each reversed.
fn expand_split_uses(
    solid: &mut SolidSpec,
    edge: u64,
    first: u64,
    second: u64,
    skip_faces: &[usize],
) {
    for (fi, face) in solid.faces.iter_mut().enumerate() {
        if skip_faces.contains(&fi) {
            continue;
        }
        for lp in &mut face.loops {
            let mut out = Vec::with_capacity(lp.uses.len() + 1);
            for u in &lp.uses {
                if u.edge == edge {
                    let (a, b) = if u.forward {
                        (first, second)
                    } else {
                        (second, first)
                    };
                    out.push(EdgeUse {
                        edge: a,
                        forward: u.forward,
                    });
                    out.push(EdgeUse {
                        edge: b,
                        forward: u.forward,
                    });
                } else {
                    out.push(*u);
                }
            }
            lp.uses = out;
        }
    }
}

/// Splits `edge` at its parameter midpoint, minting the midpoint
/// vertex and the two half-edges. Answers `(first, second, midpoint)`.
fn split_at_midpoint(
    solid: &mut SolidSpec,
    edge: u64,
    mint: &mut dyn FnMut() -> u64,
) -> (u64, u64, u64) {
    let spec = &solid.edges[&edge];
    let (t0, t1, start, end) = (spec.t0, spec.t1, spec.start, spec.end);
    let carrier = spec.carrier.clone();
    let tm = (t0 + t1) / 2.0;
    let mid_v = mint();
    solid.vertices.insert(mid_v, carrier.eval(tm));
    let (first, second) = (mint(), mint());
    solid.edges.insert(
        first,
        EdgeSpec {
            start,
            end: mid_v,
            carrier: carrier.clone(),
            t0,
            t1: tm,
            // A minted edge: the importer states its own
            // start → end, so there is no sense to compose.
            reversed: false,
        },
    );
    solid.edges.insert(
        second,
        EdgeSpec {
            start: mid_v,
            end,
            carrier,
            t0: tm,
            t1,
            // A minted edge: the importer states its own
            // start → end, so there is no sense to compose.
            reversed: false,
        },
    );
    solid.edges.remove(&edge);
    (first, second, mid_v)
}

/// Whether traversing `carrier` in its increasing parameter raises the
/// chart coordinate selected by `pick` — the file's own winding, read
/// off the geometry rather than assumed from a flag.
///
/// Sampled a quarter and a fifth of the way along the interval: far
/// enough from both ends that the sample sits mid-quadrant, and clear
/// of the chart's own wrap (the seam at |u| = π and the inner equator
/// at |v| = π), where a finite difference would read a jump as a
/// direction. `None` when the samples do not invert onto the chart, or
/// when the step is too small to have a sign — an undecidable winding
/// is refused by the caller, never guessed.
fn chart_direction(
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    t0: f64,
    t1: f64,
    pick: fn(geom_core::Point2<f64>) -> f64,
) -> Option<bool> {
    let span = t1 - t0;
    let at = |f: f64| crate::chart::uv_of(surface, carrier.eval(t0 + span * f)).map(pick);
    let (lo, hi) = (at(0.25)?, at(0.30)?);
    // The step covers 5% of a full period, i.e. ~0.31 rad; anything
    // materially smaller than that is not a reading.
    ((hi - lo).abs() > 0.1).then_some(hi > lo)
}

/// Runs both normalizations over one shell (module docs).
pub(crate) fn normalize_shell(
    solid: &mut SolidSpec,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    apex_cone(solid, mint, sink)?;
    full_torus(solid, mint, sink)
}

/// **The degenerate-apex cone** (module docs): a conical face whose
/// loop is `(seam, base circle, seam)` with the seam's far vertex of
/// valence 1. Re-minted as the kernel's own two lateral half-faces,
/// joined by a second generator half a turn round the axis.
///
/// Census: 1 lateral face / 2 edges / (apex + 1 base vertex) becomes 2
/// lateral faces / 4 edges / (apex + 2 base vertices). The base cap
/// face keeps its identity; only its single circular bound becomes two
/// half-circles.
fn apex_cone(
    solid: &mut SolidSpec,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let valence = valences(solid);
    // The one candidate shape, found by the defect it has: a valence-1
    // vertex. Anything else this scan meets is left alone.
    let Some((&apex_v, _)) = valence.iter().find(|&(_, &n)| n == 1) else {
        return Ok(());
    };
    let mut found = None;
    for (fi, face) in solid.faces.iter().enumerate() {
        let Surface::Cone { apex, axis, .. } = face.surface else {
            continue;
        };
        let [lp] = face.loops.as_slice() else {
            continue;
        };
        let [a, b, c] = lp.uses.as_slice() else {
            continue;
        };
        // (seam, circle, seam): the seam edge twice, the base circle
        // once, in either rotation of the cycle.
        let (seam, circle) = if a.edge == c.edge {
            (*a, *b)
        } else if a.edge == b.edge {
            (*a, *c)
        } else if b.edge == c.edge {
            (*b, *a)
        } else {
            continue;
        };
        let seam_spec = &solid.edges[&seam.edge];
        if (seam_spec.start != apex_v && seam_spec.end != apex_v)
            || !matches!(seam_spec.carrier, Curve3::Line { .. })
        {
            continue;
        }
        found = Some((
            fi,
            apex,
            axis,
            seam,
            circle,
            *lp.uses
                .iter()
                .find(|u| u.edge == seam.edge && u.forward != seam.forward)
                .unwrap_or(&seam),
        ));
        break;
    }
    let Some((fi, apex, axis, seam_a, circle_use, seam_b)) = found else {
        // A valence-1 vertex that is NOT this shape is a defect the
        // kernel's own validity ladder must report, not something to
        // repair blind.
        return Ok(());
    };
    let face_id = fi;
    let (base_v, seam_carrier) = {
        let seam_spec = &solid.edges[&seam_a.edge];
        let base_v = if seam_spec.start == apex_v {
            seam_spec.end
        } else {
            seam_spec.start
        };
        (base_v, seam_spec.carrier.clone())
    };
    let Some(second_seam) = half_turn_curve(&seam_carrier, apex, axis) else {
        return Ok(());
    };
    // The half of the base circle each new face carries.
    let (first_c, second_c, mid_v) = split_at_midpoint(solid, circle_use.edge, mint);
    // The minted generator, from the new base vertex to the apex.
    let gen_id = mint();
    let (t0, t1) = crate::geometry::endpoint_params(
        gen_id,
        &second_seam,
        solid.vertices[&mid_v],
        solid.vertices[&apex_v],
        false,
    )?;
    solid.edges.insert(
        gen_id,
        EdgeSpec {
            start: mid_v,
            end: apex_v,
            carrier: second_seam,
            t0,
            t1,
            // A minted edge: the importer states its own
            // start → end, so there is no sense to compose.
            reversed: false,
        },
    );
    // Every OTHER loop's use of the base circle becomes its two halves
    // (the cap face's single bound is the one that exists).
    expand_split_uses(solid, circle_use.edge, first_c, second_c, &[face_id]);

    // The two lateral half-faces. `seam_a` is the seam use that leaves
    // the apex (it precedes the circle in the cycle); `seam_b` is the
    // one that returns to it. Each new face walks out along one
    // generator, round its half of the base, and back along the other.
    let out_use = if solid_edge_leaves_apex(solid, seam_a, apex_v, base_v) {
        seam_a
    } else {
        seam_b
    };
    let in_use = EdgeUse {
        edge: out_use.edge,
        forward: !out_use.forward,
    };
    let template = &solid.faces[face_id];
    let (surface, sense, face_entity) = (template.surface.clone(), template.sense, template.id);
    let g = circle_use.forward;
    // Traversal order: under orientation `g` the base circle walks its
    // two halves in this order, which is the order the two new faces
    // take them in.
    let (h1, h2) = if g {
        (first_c, second_c)
    } else {
        (second_c, first_c)
    };
    let face_a = FaceSpec {
        id: face_entity,
        surface: surface.clone(),
        sense,
        loops: vec![LoopSpec {
            outer: true,
            uses: vec![
                out_use,
                EdgeUse {
                    edge: h1,
                    forward: g,
                },
                EdgeUse {
                    edge: gen_id,
                    forward: true,
                },
            ],
        }],
    };
    let face_b = FaceSpec {
        id: face_entity,
        surface,
        sense,
        loops: vec![LoopSpec {
            outer: true,
            uses: vec![
                EdgeUse {
                    edge: gen_id,
                    forward: false,
                },
                EdgeUse {
                    edge: h2,
                    forward: g,
                },
                in_use,
            ],
        }],
    };
    solid.faces[face_id] = face_a;
    solid.faces.insert(face_id + 1, face_b);
    sink.push(StructureNormalization {
        face: face_entity,
        kind: NormalizationKind::DegenerateApexCone,
        file_census: FaceCensus {
            faces: 1,
            edges: 2,
            vertices: 2,
        },
        kernel_census: FaceCensus {
            faces: 2,
            edges: 4,
            vertices: 3,
        },
    });
    Ok(())
}

/// Whether traversing `use_` walks from the apex towards the base.
fn solid_edge_leaves_apex(solid: &SolidSpec, use_: EdgeUse, apex_v: u64, _base_v: u64) -> bool {
    let spec = &solid.edges[&use_.edge];
    let from = if use_.forward { spec.start } else { spec.end };
    from == apex_v
}

/// **The whole torus in one face** (module docs): a toroidal face whose
/// single loop is the fundamental-polygon square — four uses over two
/// self-loop edges at one vertex. Re-minted as the kernel's own two
/// half-faces.
///
/// The split runs in **v**, the minor direction, because that is the
/// splitting a natively revolved torus carries: each half-face is a
/// full turn in u (its two rims are whole circles, one at the profile
/// vertex's latitude and one at the antipodal latitude) and half a turn
/// in v (the u-seam meridian, cut in two and used twice per face —
/// STEP's ordinary seam encoding). Splitting in u instead leaves each
/// half wrapping the full v-period, which is the same both-ways
/// ambiguity in the transpose.
///
/// Census: 1 face / 2 edges / 1 vertex becomes 2 / 4 / 2.
fn full_torus(
    solid: &mut SolidSpec,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let mut found = None;
    for (fi, face) in solid.faces.iter().enumerate() {
        let Surface::Torus { center, axis, .. } = face.surface else {
            continue;
        };
        let [lp] = face.loops.as_slice() else {
            continue;
        };
        let [a, b, c, d] = lp.uses.as_slice() else {
            continue;
        };
        // The fundamental polygon: A B A' B' with each edge used once
        // each way, both self-loops at the same vertex.
        if a.edge != c.edge || b.edge != d.edge || a.edge == b.edge {
            continue;
        }
        if a.forward == c.forward || b.forward == d.forward {
            continue;
        }
        let (ea, eb) = (&solid.edges[&a.edge], &solid.edges[&b.edge]);
        if ea.start != ea.end || eb.start != eb.end || ea.start != eb.start {
            continue;
        }
        // The RIM runs round the major direction: its own carrier axis
        // IS the torus axis. The other is the meridian, and cutting the
        // meridian is what halves the face.
        let axis_of = |c: &Curve3<f64>| match *c {
            Curve3::Circle { axis: n, .. } => Some(n),
            _ => None,
        };
        let (Some(na), Some(nb)) = (axis_of(&ea.carrier), axis_of(&eb.carrier)) else {
            continue;
        };
        let (rim, meridian, rim_axis) = if na.dot(axis).abs() > 0.5 && nb.dot(axis).abs() <= 0.5 {
            (*a, *b, na)
        } else if nb.dot(axis).abs() > 0.5 && na.dot(axis).abs() <= 0.5 {
            (*b, *a, nb)
        } else {
            continue;
        };
        found = Some((
            fi, face.id, face.sense, center, axis, rim, meridian, rim_axis,
        ));
        break;
    }
    let Some((fi, face_entity, sense, center, axis, rim, meridian, rim_axis)) = found else {
        return Ok(());
    };
    // ---- The file's winding, derived rather than assumed ----------
    //
    // The fundamental polygon `[x, y, x', y']` maps to ITSELF under
    // loop reversal — reversing the cycle and flipping every flag
    // permutes the four uses back into the same multiset. So the
    // per-use `forward` flags alone cannot say which way this face
    // turns; the winding lives ONLY in the cyclic ORDER, and a
    // reconstruction that reads the flags and not the order rebuilds an
    // inside-out torus as a right-side-out one. (That is exactly what
    // the first cut of this repair did, and why the order is now what
    // gets read.)
    //
    // `p` and `q` are geometric: which traversal of the rim raises u,
    // which traversal of the meridian raises v — sampled through the
    // kernel's own chart inverse. The chart's CCW boundary is, in
    // order, `(rim, +u) (meridian, +v) (rim, −u) (meridian, −v)`, so
    // the file's cycle is CCW exactly when the use FOLLOWING its `+u`
    // rim use is the meridian's `+v` one. Every reading that the
    // geometry does not answer refuses; none is guessed.
    let surface = solid.faces[fi].surface.clone();
    let undecidable = || StepImportError::Topology {
        id: face_entity,
        what: "a whole-torus face whose winding its own geometry cannot state (a rim or \
               meridian carrier that does not invert onto the torus chart, or a loop \
               whose uses do not alternate): the orientation of the re-tessellation \
               would be a guess, and D7 forbids guessing it",
    };
    let (p, q) = {
        let rim_spec = &solid.edges[&rim.edge];
        let p = chart_direction(
            &surface,
            &rim_spec.carrier,
            rim_spec.t0,
            rim_spec.t1,
            |uv| uv.x,
        )
        .ok_or_else(undecidable)?;
        let mer_spec = &solid.edges[&meridian.edge];
        let q = chart_direction(
            &surface,
            &mer_spec.carrier,
            mer_spec.t0,
            mer_spec.t1,
            |uv| uv.y,
        )
        .ok_or_else(undecidable)?;
        (p, q)
    };
    // The cyclic successor of the `+u` rim use.
    let ccw = {
        let uses = &solid.faces[fi].loops[0].uses;
        let plus_u = uses
            .iter()
            .position(|u| u.edge == rim.edge && u.forward == p)
            .ok_or_else(undecidable)?;
        let successor = uses[(plus_u + 1) % uses.len()];
        if successor.edge != meridian.edge {
            return Err(undecidable());
        }
        successor.forward == q
    };
    // The face's material side: the torus chart normal (∂u × ∂v) points
    // outward, so the face's own normal is that normal for
    // `same_sense` and its negation otherwise, and the loop must run
    // CCW about the FACE normal. Right-side-out is therefore
    // `sense == ccw`; the two consistent encodings of the same solid
    // are (`.T.`, CCW) and (`.F.`, CW), and both adopt.
    if sense != ccw {
        return Err(StepImportError::Topology {
            id: face_entity,
            what: "an ORIENTATION-INVERTED whole-torus face: its stated same_sense and the \
                   winding of its own fundamental-polygon loop disagree, so the solid it \
                   describes is inside out. This importer will not re-tessellate it \
                   right-side-out — that would launder the inversion into a positive \
                   volume — and it cannot adopt it as stated either: the kernel's \
                   closed-form torus contribution derives its sign from the face's own \
                   traversal and never consumes same_sense, so an inverted torus would \
                   certify green with a positive volume. Refused until the kernel can \
                   hold an inverted rim-bearing face honestly",
        });
    }

    // Halving the meridian mints the antipodal profile vertex.
    let (first_m, second_m, mid_v) = split_at_midpoint(solid, meridian.edge, mint);
    // The second rim: the u-circle through that vertex — same axis
    // direction as the stated rim (so u runs the same way), centred on
    // the torus axis at the vertex's own height, and anchored so the
    // vertex sits at its angle 0. Built from the vertex the file's own
    // meridian produced, so it cannot drift off the locus.
    let v1 = solid.vertices[&mid_v];
    let c1 = crate::geometry::plus_zero_point(center + axis * ((v1 - center).dot(axis)));
    let spoke = v1 - c1;
    let radius = spoke.norm();
    if !(radius.is_finite() && radius > 0.0) {
        return Ok(());
    }
    let rim_id = mint();
    let carrier = Curve3::Circle {
        center: c1,
        axis: rim_axis,
        radius,
        u_ref: crate::geometry::plus_zero(spoke * radius.recip()),
    };
    let Ok((t0, t1)) = crate::geometry::endpoint_params(rim_id, &carrier, v1, v1, true) else {
        return Ok(());
    };
    solid.edges.insert(
        rim_id,
        EdgeSpec {
            start: mid_v,
            end: mid_v,
            carrier,
            t0,
            t1,
            // A minted edge: the importer states its own
            // start → end, so there is no sense to compose.
            reversed: false,
        },
    );
    // The minted halves are wound `+u` first — CCW about the chart
    // normal, which for a torus points outward — and described with
    // `same_sense` true to match. Neither is copied from the file's
    // face, and both are DERIVED from the material side the check
    // above established:
    //
    // ISO 10303-42 lets one solid be stated two ways, `(.T., CCW)` and
    // `(.F., CW)`; they are the same material side and both reach here.
    // The kernel, however, derives a torus face's flux sign from its
    // TRAVERSAL alone (its closed-form contribution never consumes
    // `same_sense` — the fenced kernel gap this refusal is written
    // against), so a `(.F., CW)` re-tessellation would come back with a
    // negative volume for a right-side-out ring. Minting the outward
    // convention explicitly is therefore the honest description of the
    // faces being minted, not a healing of the file's: the file's face
    // does not survive the re-tessellation, and its replacements get a
    // description that means what it says. The inverted encodings never
    // get here at all — they refused above.
    let rs = p;
    let sense = true;
    // Traversal order: under `q` the meridian walks its halves in this
    // order, so `h1` is the half touching the stated vertex and `h2`
    // the half touching the minted one.
    let (h1, h2) = if q {
        (first_m, second_m)
    } else {
        (second_m, first_m)
    };
    let u = |edge: u64, forward: bool| EdgeUse { edge, forward };
    // Each half-face is the chart rectangle between the two rims: the
    // stated rim below, its half of the meridian up one side and back
    // down the other (the seam, used twice), the minted rim above —
    // and the mirror image for the other half.
    let face_a = FaceSpec {
        id: face_entity,
        surface: surface.clone(),
        sense,
        loops: vec![LoopSpec {
            outer: true,
            uses: vec![u(rim.edge, rs), u(h1, q), u(rim_id, !rs), u(h1, !q)],
        }],
    };
    let face_b = FaceSpec {
        id: face_entity,
        surface,
        sense,
        loops: vec![LoopSpec {
            outer: true,
            uses: vec![u(rim_id, rs), u(h2, q), u(rim.edge, !rs), u(h2, !q)],
        }],
    };
    solid.faces[fi] = face_a;
    solid.faces.insert(fi + 1, face_b);
    sink.push(StructureNormalization {
        face: face_entity,
        kind: NormalizationKind::FullPeriodTorus,
        file_census: FaceCensus {
            faces: 1,
            edges: 2,
            vertices: 1,
        },
        kernel_census: FaceCensus {
            faces: 2,
            edges: 4,
            vertices: 2,
        },
    });
    Ok(())
}
