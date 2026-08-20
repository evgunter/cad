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
//! planar face's outward normal, and the per-corner chart derivation
//! ([`octant_chart`]) the surgery reads at each trivalent corner.
//! The corner's ORIENTATION bit is not derived here at all: it is
//! whatever its admitted links carry, which the surgery reads off
//! `Corner`'s own field.
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

use super::FilletError;
use super::admit::{CornerFaces, CornerLinks};
use super::battery::{FilletRequest, Link, run_battery};
use super::surgery::{CORNER_SUPPORT_NOT_PLANAR, unbuilt_geometry};

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
/// [`FilletError::UnsupportedChain`], [`FilletError::UnsupportedRunOut`],
/// [`FilletError::UnsupportedGeometry`] or
/// [`FilletError::FilletCornerUnsupported`] when the request is outside
/// the assembly's front door ([`super::surgery`] names each case);
/// [`FilletError::BodyNotIntact`] when the body does not hold together
/// where the plan reads it;
/// [`FilletError::RingClearance`] when a carried-through ring does not
/// clear a trimline; [`FilletError::Op`], carrying the operator's own
/// typed refusal, when an Euler operator refuses;
/// [`FilletError::Certify`], carrying the pass's own typed refusal,
/// when the result's pcurve caches cannot be re-minted.
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
///
/// **The pick always yields.** A candidate needs an incident link and
/// a third support: [`CornerLinks`] carries at least one link, and
/// [`CornerFaces::third`] is total over three distinct faces, so there
/// is no "no candidate here" state left to refuse. The two refusals
/// below both read a stored `Surface` and are the genuine geometric
/// frontier.
///
/// # Errors
///
/// [`FilletError::UnsupportedGeometry`] when a support of this corner
/// is not a plane.
pub(super) fn octant_chart<T: Decide + Bounds>(
    body: &Body<T>,
    faces: &CornerFaces,
    links: &CornerLinks<'_, T>,
) -> Result<(Vec3<T>, Vec3<T>), FilletError> {
    // `(score, u_ref, axis)` for one candidate edge: the chart aimed
    // along it, scored by how nearly the third support's normal is
    // parallel to its axis.
    let candidate = |l: &Link<T>| -> Result<(f64, Vec3<T>, Vec3<T>), FilletError> {
        let planar = |f: FaceKey| {
            outward_of(body, f)
                .ok_or_else(|| unbuilt_geometry(EntityId::Face(f), CORNER_SUPPORT_NOT_PLANAR))
        };
        let (n_a, n_b) = (planar(l.face_a)?, planar(l.face_b)?);
        let axis = n_a.cross(n_b).normalize();
        // The third support of the corner — the one this edge does not
        // touch.
        let n_c = planar(faces.third(l.face_a, l.face_b))?;
        Ok((n_c.cross(axis).norm().lo().abs(), n_a, axis))
    };
    let mut best = candidate(links.first().link())?;
    for l in links.rest() {
        let next = candidate(l.link())?;
        if next.0 < best.0 {
            best = next;
        }
    }
    let (_, n_a, axis) = best;
    Ok((n_a, axis))
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
