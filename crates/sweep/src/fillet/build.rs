//! **The assembly half of the fillet unit**: turning a
//! [`BatteryVerdict`](super::BatteryVerdict) into a rounded solid.
//!
//! # The ordering contract, structurally
//!
//! [`fillet_edges`] calls [`run_battery`] as its FIRST statement and
//! propagates the refusal unchanged. Nothing here mints a surface, a
//! point, or a topology entity before a verdict exists — the C8 claim
//! ("if the battery returns `Ok`, construction cannot fail for a
//! geometric reason") is kept by construction order, not by hope.
//!
//! # The assembly front door
//!
//! The kernel has no face-soup constructor: bodies come only from
//! Euler operators. Every admissible request routes to the in-place
//! edge-blend **composition surgery** ([`super::surgery`]): split the
//! supports along the stored trimlines, excise the strips, graft the
//! blends — which is what carries a face's RINGS through and what
//! replaces a pip rim with a torus band. That module's docs state the
//! door as a predicate per chain kind, and what falls outside it
//! refuses typed through the `Unsupported*` frontier vocabulary, naming
//! itself.
//!
//! What lives here besides the entry point is the part of the
//! derivation that is about a SOURCE body rather than about the
//! mutation: a face's boundary cycle, a vertex's face orbit, a
//! planar face's outward normal, and the two per-corner derivations
//! ([`octant_chart`], [`corner_convexity`]) the surgery reads at each
//! trivalent corner.
//!
//! # Naming
//!
//! The surgery emits per-entity birth records
//! ([`super::naming::FilletNaming`]) as it mutates, from the plan that
//! decided each mint — never recovered afterwards by matching
//! geometry, which is what N4 forbids.
//!
//! # KNOWN LIMITATION — the octant's chart, and oblique trihedra
//!
//! The result is a tier-1/2 valid closed shell for every input the
//! front door admits. Tier **3** additionally needs the sphere
//! octant's three contact circles to be an iso-parameter rectangle in
//! its chart, because check 7's `+V` invariant runs the closed-form
//! mass properties, whose curved-face inventory is rims and meridians
//! (`geom_brep::props::curved_face`). The chart [`octant_chart`]
//! picks — aimed along one incident edge — achieves that exactly when
//! the THIRD support's normal is parallel to that edge, i.e. at a
//! RIGHT trihedron (every vertex of a box). At an oblique trihedron no
//! chart makes a spherical triangle an iso-rectangle, so
//! `topo::mass_properties` refuses `NotIsoRectangle` and tier 3
//! reports `VolumeUncomputable` — a gap in the props inventory (a
//! spherical-triangle closed form, or the certified-quadrature lane
//! extended to sphere faces), not in the body. The assembly does not
//! pretend otherwise: it neither gates on it nor asserts tier 3.

use geom::Surface;
use geom_core::{Band, Bounds, Decide, Real, Vec3};
use topo::{
    Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary, ShellKey, SolidKey, VertexKey,
};

use super::battery::{Convexity, FilletRequest, Link, run_battery};
use super::surgery::CORNER_SUPPORT_NOT_PLANAR;
use super::{CornerConfig, FilletError, RunOutPolicy};

/// A filleted body: the rounded solid plus the keys of the faces the
/// blend introduced.
#[derive(Clone, Debug)]
pub struct Filleted<T: Real> {
    /// The rounded solid.
    pub body: Body<T>,
    /// Its (only) solid.
    pub solid: SolidKey,
    /// Its (only) shell.
    pub shell: ShellKey,
    /// The blend faces — the quarter-cylinder patches, one per
    /// original edge, in original-edge order.
    pub blend_faces: Vec<FaceKey>,
    /// The corner faces — the sphere patches, one per original
    /// vertex, in original-vertex order.
    pub corner_faces: Vec<FaceKey>,
    /// The torus band faces — one per CLOSED chain (the rim blends),
    /// in first-link-edge order. Empty when no requested chain closes.
    pub band_faces: Vec<FaceKey>,
    /// **Per-entity birth records**: what the fillet minted and which
    /// source entity each mint was made for.
    ///
    /// The surgery writes the rows as it mutates, and it is the only
    /// producer, so `None` is never constructed. It is a **permanent
    /// `Option` over a value that is always `Some`**, and deliberately
    /// so: `Filleted` is public, and the alternative — a bare
    /// `FilletNaming` with `Default` — would let a caller or a future
    /// assembly ship an EMPTY table indistinguishable from a full one.
    /// `None` is the state that says "this body has no birth records",
    /// which `editor-core` refuses as a kernel bug rather than falling
    /// back to unnamed geometry; an empty struct would be refused by
    /// nothing.
    pub naming: Option<super::naming::FilletNaming>,
}

/// **Fillet a set of a body's edges** at constant radius `radius`.
///
/// The battery runs FIRST (module docs): a refusal from
/// [`run_battery`] propagates unchanged, and no entity is minted
/// before its verdict. Only then is the verdict handed to the
/// assembly, and only then is anything built.
///
/// # Errors
///
/// Any [`FilletError`] the battery produces;
/// [`FilletError::RepeatedEdge`] when the request names one edge
/// twice; [`FilletError::UnsupportedBody`],
/// [`FilletError::UnsupportedChain`], [`FilletError::UnsupportedCorner`]
/// or [`FilletError::UnsupportedGeometry`] when the request is outside
/// the assembly's front door ([`super::surgery`] names each case);
/// [`FilletError::BodyNotIntact`] when the body does not hold together
/// where the plan reads it;
/// [`FilletError::RingClearance`] when a carried-through ring does not
/// clear a trimline; [`FilletError::Op`] when an Euler operator
/// refuses; [`FilletError::Certify`] when a blend description does not
/// certify as stored.
pub fn fillet_edges<T: Decide + Bounds>(
    body: &Body<T>,
    edges: &[EdgeKey],
    radius: T,
    band: Band,
) -> Result<Filleted<T>, FilletError> {
    // A repeated edge is malformed for the chain walk (it would
    // double a link), so it refuses before the battery samples
    // anything.
    let mut requested = edges.to_vec();
    requested.sort_unstable();
    let repeated = requested.windows(2).find(|w| w[0] == w[1]).map(|w| w[0]);
    if let Some(edge) = repeated {
        return Err(FilletError::RepeatedEdge { edge });
    }

    // ---- The ordering contract: verdict first, unchanged. ----
    let request = FilletRequest {
        body,
        edges: edges.to_vec(),
        radius,
    };
    let verdict = run_battery(&request, band)?;

    // ---- Then the assembly, which is the composition surgery. ----
    super::surgery::fillet_surgery(body, &verdict, band)
}

/// A face's boundary cycle (outer loop, cycle order).
pub(super) fn face_cycle<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<Vec<HalfEdgeKey>> {
    let f = body.get_face(face)?;
    let LoopBoundary::Cycle { first } = body.get_loop(f.outer)?.boundary else {
        return None;
    };
    body.loop_cycle(first)
}

/// The distinct faces around a vertex, in orbit order.
pub(super) fn vertex_faces<T: Decide>(body: &Body<T>, vertex: VertexKey) -> Option<Vec<FaceKey>> {
    let he = body.get_vertex(vertex)?.emanating?;
    let mut faces = Vec::new();
    for h in body.vertex_orbit(he)? {
        let f = body.get_loop(body.get_half_edge(h)?.parent_loop)?.face;
        if !faces.contains(&f) {
            faces.push(f);
        }
    }
    Some(faces)
}

/// The octant's chart pick at one trivalent corner. The criterion:
/// the octant is an iso-parameter
/// rectangle exactly when the chart is aimed along an incident edge
/// whose axis `n_a × n_b` the THIRD support's normal is parallel to,
/// so the pick minimizes `|n_c × axis|` over the incident requested
/// links, ORDER-FREE — it finds the admitting edge whenever one
/// exists and degrades to "no chart admits this trihedron" (the
/// genuinely oblique case, tier-3 `VolumeUncomputable`) only when
/// none does. Returns `(u_ref, axis)` — the seam is the picked
/// link's first support normal.
pub(super) fn octant_chart<T: Decide + Bounds>(
    body: &Body<T>,
    vertex: VertexKey,
    faces: &[FaceKey],
    links: &[&Link<T>],
) -> Result<(Vec3<T>, Vec3<T>), FilletError> {
    let mut best: Option<(f64, Vec3<T>, Vec3<T>)> = None;
    for l in links
        .iter()
        .filter(|l| l.start == vertex || l.end == vertex)
    {
        let (Some(n_a), Some(n_b)) = (outward_of(body, l.face_a), outward_of(body, l.face_b))
        else {
            let bad = if outward_of(body, l.face_a).is_none() {
                l.face_a
            } else {
                l.face_b
            };
            return Err(FilletError::UnsupportedGeometry {
                at: EntityId::Face(bad),
                detail: CORNER_SUPPORT_NOT_PLANAR,
            });
        };
        let axis = n_a.cross(n_b).normalize();
        // The third support of the corner — the one this edge does
        // not touch. `faces` is the vertex's three, so the complement
        // is a single face.
        let Some(&f_c) = faces.iter().find(|f| **f != l.face_a && **f != l.face_b) else {
            continue;
        };
        let Some(n_c) = outward_of(body, f_c) else {
            return Err(FilletError::UnsupportedGeometry {
                at: EntityId::Face(f_c),
                detail: CORNER_SUPPORT_NOT_PLANAR,
            });
        };
        let score = n_c.cross(axis).norm().lo().abs();
        if best.as_ref().is_none_or(|(s, _, _)| score < *s) {
            best = Some((score, n_a, axis));
        }
    }
    // `best` stays `None` when no incident requested link contributed a
    // candidate: either none touches this vertex, or every one of them
    // has a support outside the corner's own three faces. Both are the
    // same unbuilt configuration, and neither is the run-out door above.
    let (_, n_a, axis) = best.ok_or(FilletError::UnsupportedRunOut {
        at: EntityId::Vertex(vertex),
        detail: "no requested link at this corner is supported by two of its three faces; \
                 corners assembled from other links are not implemented",
    })?;
    Ok((n_a, axis))
}

/// The octant's ORIENTATION BIT at one trivalent corner (the shape
/// [`octant_chart`] already has).
///
/// A corner patch is a sphere about the rolling ball's rest centre and
/// its chart normal is the outward radial, exactly as a blend's
/// cylinder chart normal is. The centre lies on the material side
/// precisely when the corner is convex, so the octant's sense is the
/// same bit its blends take, off the same stored verdict
/// ([`Convexity::blend_sense`]) — never a sampled normal.
///
/// `links` are the requested links already filtered to this corner.
/// **Neither refusal below is reachable through the front door**: it
/// admits only convex chains, and a corner exists only where an open
/// link terminates, so the links are always present and always agree.
///
/// They stay TYPED rather than becoming the addendum's row 4 because
/// **this function cannot observe the fact that would make them
/// impossible**: `links` is a parameter, and the door that establishes
/// convexity is two frames up in [`super::surgery::fillet_surgery`],
/// which carries its verdict as prose rather than as a type. An
/// `unreachable!` here would rest on a caller's behaviour, which is the
/// inheritance the D2 addendum's row 4 forbids; doing it honestly needs
/// a type that says "convex link".
pub(super) fn corner_convexity<T: Real>(
    vertex: VertexKey,
    links: &[&Link<T>],
) -> Result<Convexity, FilletError> {
    let mut convexity: Option<Convexity> = None;
    for l in links {
        if *convexity.get_or_insert(l.convexity) != l.convexity {
            // The corner's own configuration, in the vocabulary the
            // battery's classifier already speaks.
            return Err(FilletError::FilletCornerUnsupported {
                vertex,
                corner: CornerConfig::MixedConvexity {
                    convex: links
                        .iter()
                        .filter(|l| matches!(l.convexity, Convexity::Convex))
                        .count(),
                },
                policy: RunOutPolicy::RunOutStopAtVertex,
            });
        }
    }
    convexity.ok_or(FilletError::UnsupportedRunOut {
        at: EntityId::Vertex(vertex),
        detail: "a corner has no requested incident link, so no octant sense is derivable",
    })
}

/// A planar face's OUTWARD normal: the stored plane normal folded
/// through the stored sense bit (S10 category A — never sampled).
pub(super) fn outward_of<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<Vec3<T>> {
    let f = body.get_face(face)?;
    match body.get_surface(f.surface)? {
        Surface::Plane { normal, .. } => Some(*normal * f.sense_sign::<T>()),
        _ => None,
    }
}
