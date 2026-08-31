//! **The assembly half of the edge-blend unit** — both verbs' front
//! doors, turning a [`BatteryVerdict`](super::BatteryVerdict) into a
//! blended solid.
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
//! ([`super::naming::BlendNaming`]) as it mutates, from the plan that
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

use super::admit::{CornerFaces, CornerLinks};
use super::battery::{BlendRequest, Link, run_battery};
use super::surgery::{CORNER_SUPPORT_NOT_PLANAR, not_intact, unbuilt_geometry};
use super::{BlendError, BlendKind, BlendRefusal};
use geom_core::Tol;

/// A blended body — the one result type both verbs return: the
/// carved solid plus the keys of the faces the blend introduced.
/// [`Filleted`] and [`Chamfered`] alias it for call-site readability.
#[derive(Clone, Debug)]
pub struct Blended<T: Real> {
    /// The blended solid.
    pub body: Body<T>,
    /// Its (only) solid.
    pub solid: SolidKey,
    /// Its (only) shell.
    pub shell: ShellKey,
    /// The blend faces — the fillet's quarter-cylinder patches or
    /// the chamfer's flat strips, one per original edge, in
    /// original-edge order.
    pub blend_faces: Vec<FaceKey>,
    /// The corner faces — the fillet's sphere patches or the
    /// chamfer's planar patches, one per original vertex, in
    /// original-vertex order.
    pub corner_faces: Vec<FaceKey>,
    /// The torus band faces — one per CLOSED chain (the fillet's rim
    /// blends; a chamfer has no closed-chain band), in
    /// first-link-edge order. Empty when no requested chain closes.
    pub band_faces: Vec<FaceKey>,
    /// **Per-entity birth records**: what the blend minted and which
    /// source entity each mint was made for.
    ///
    /// The surgery writes the rows as it mutates, and it is the only
    /// producer, so `None` is never constructed. It is a **permanent
    /// `Option` over a value that is always `Some`**, and deliberately
    /// so: `Filleted` is public, and the alternative — a bare
    /// `BlendNaming` with `Default` — would let a caller or a future
    /// assembly ship an EMPTY table indistinguishable from a full one.
    /// `None` is the state that says "this body has no birth records",
    /// which `editor-core` refuses as a kernel bug rather than falling
    /// back to unnamed geometry; an empty struct would be refused by
    /// nothing.
    pub naming: Option<super::naming::BlendNaming>,
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
/// A [`BlendRefusal`] carrying [`BlendKind::Fillet`] — the verb
/// crosses HERE, once, and the inner [`BlendError`] stays
/// verb-neutral — around: any refusal the battery produces;
/// [`BlendError::RepeatedEdge`] when the request names one edge
/// twice; [`BlendError::UnsupportedBody`],
/// [`BlendError::UnsupportedChain`], [`BlendError::UnsupportedRunOut`],
/// [`BlendError::UnsupportedGeometry`] or
/// [`BlendError::UnsupportedCorner`] when the request is outside
/// the assembly's front door ([`super::surgery`] names each case);
/// [`BlendError::BodyNotIntact`] when the body does not hold together
/// where the plan reads it;
/// [`BlendError::RingClearance`] when a carried-through ring does not
/// clear a trimline; [`BlendError::Op`], carrying the operator's own
/// typed refusal, when an Euler operator refuses;
/// [`BlendError::Certify`], carrying the pass's own typed refusal,
/// when the result's pcurve caches cannot be re-minted.
pub fn fillet_edges<T: Decide + Bounds + geom_brep::PcurveFittedLane>(
    body: &Body<T>,
    edges: &[EdgeKey],
    radius: T,
    band: Band,
    tol: Tol,
) -> Result<Filleted<T>, BlendRefusal> {
    fillet_edges_inner(body, edges, radius, band, tol).map_err(|error| BlendRefusal {
        verb: BlendKind::Fillet,
        error,
    })
}

/// [`fillet_edges`] behind the door: the whole request, refusing
/// through the shared verb-neutral vocabulary. The door above is the
/// one place the fillet's verb is attached.
fn fillet_edges_inner<T: Decide + Bounds + geom_brep::PcurveFittedLane>(
    body: &Body<T>,
    edges: &[EdgeKey],
    radius: T,
    band: Band,
    tol: Tol,
) -> Result<Filleted<T>, BlendError> {
    repeated_edge_gate(edges)?;
    // NOTE the door asymmetry: this door has no NonpositiveSize check,
    // so a zero radius reaches predicate 1 and refuses RadiusHeadroom
    // with an unfollowable sentence (pinned as a characterization in
    // `tests/review_blend6_r1_probes.rs`). Issue #1336 (the
    // door-asymmetric size validation) owns closing it.

    // ---- The ordering contract: verdict first, unchanged. ----
    let request = BlendRequest {
        body,
        edges: edges.to_vec(),
        size: radius,
    };
    let verdict = run_battery(&request, band)?;

    // ---- Then the assembly, which is the composition surgery. ----
    super::surgery::blend_surgery(body, &verdict, band, tol)
}

/// **Both doors' shared request preamble**: a repeated edge is
/// malformed for the chain walk (it would double a link), so it
/// refuses before the battery samples anything. One home rather than
/// a stanza per door — the duplicated stanza is where the doors'
/// validation already drifted once (the size check grew on one side
/// only).
fn repeated_edge_gate(edges: &[EdgeKey]) -> Result<(), BlendError> {
    let mut requested = edges.to_vec();
    requested.sort_unstable();
    match requested.windows(2).find(|w| w[0] == w[1]).map(|w| w[0]) {
        Some(edge) => Err(BlendError::RepeatedEdge { edge }),
        None => Ok(()),
    }
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
/// **The pick always yields, over candidates that are this corner's.**
/// A candidate needs an incident link and a third support:
/// [`CornerLinks`] carries at least one link, and
/// [`CornerFaces::third`] is total over the corner's own three
/// distinct faces — so there is no "no candidate here" state left to
/// refuse, and what remains to refuse is a candidate that is not this
/// corner's at all.
///
/// **Why that is checked and not assumed.** The two tokens are derived
/// independently: `faces` from the source body's vertex orbit
/// ([`vertex_faces`]), `links` from the battery's resolved arms, whose
/// supports are the blended edge's two half-edge faces. On a body that
/// holds together they name the same three faces, and
/// [`CornerLinks`]'s own incidence check does not say so — it proves
/// each link TERMINATES at the corner, not that its supports are the
/// corner's. Scoring a pair that is not this corner's yields a chart
/// aimed at nothing in particular, and **no downstream check can see
/// it**: the chart is a sphere face's `u_ref`/`axis`, so a wrong one
/// still closes, still passes tier 1 and 2, and shows up only as a
/// tier-3 `NotIsoRectangle` at a corner where the geometry was right.
/// So the disagreement refuses as the body-integrity fault it is.
///
/// # Errors
///
/// [`BlendError::BodyNotIntact`] when an incident link's supports are
/// not two of this corner's three faces;
/// [`BlendError::UnsupportedGeometry`] when a support of this corner
/// is not a plane.
pub(super) fn octant_chart<T: Decide + Bounds>(
    body: &Body<T>,
    faces: &CornerFaces,
    links: &CornerLinks<'_, T>,
) -> Result<(Vec3<T>, Vec3<T>), BlendError> {
    // `(score, u_ref, axis)` for one candidate edge: the chart aimed
    // along it, scored by how nearly the third support's normal is
    // parallel to its axis.
    let candidate = |l: &Link<T>| -> Result<(f64, Vec3<T>, Vec3<T>), BlendError> {
        // FIRST, before a normal is read: this link's two supports are
        // two of THIS corner's three. A stranger pair would be scored
        // exactly as readily as the right one (rustdoc above).
        let third = faces.third(l.face_a, l.face_b).ok_or_else(|| {
            not_intact(
                EntityId::Vertex(links.vertex()),
                "a corner's chart candidate carries supports that are not two of this \
                 corner's own three faces",
            )
        })?;
        let planar = |f: FaceKey| {
            outward_of(body, f)
                .ok_or_else(|| unbuilt_geometry(EntityId::Face(f), CORNER_SUPPORT_NOT_PLANAR))
        };
        let (n_a, n_b) = (planar(l.face_a)?, planar(l.face_b)?);
        let axis = n_a.cross(n_b).normalize();
        // The third support of the corner — the one this edge does not
        // touch.
        let n_c = planar(third)?;
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

// ------------------------------------------------------------------
// The chamfer's front door.
//
// It lives beside `fillet_edges` rather than in `crate::chamfer`
// because the two are one seam: the same battery, the same admission
// tokens, the same composition surgery, and the same certified metric
// payloads that the `Decide + Bounds` scope rule ratifies for exactly
// these three files (`geom-core/src/real.rs`). `crate::chamfer` is
// the verb's documented module and re-exports what is written here.
// ------------------------------------------------------------------

/// A filleted body: [`Blended`] under the fillet's own noun.
pub type Filleted<T> = Blended<T>;

/// A chamfered body. The same record the fillet's assembly returns —
/// `blend_faces` are the strips, `corner_faces` the flat patches, and
/// `band_faces` is empty, since a chamfer has no closed-chain band.
pub type Chamfered<T> = Blended<T>;

/// **Chamfer a set of a body's edges** at equal setback `distance`
/// along both supports.
///
/// The battery runs FIRST and its refusal propagates unchanged — the
/// same ordering contract [`fillet_edges`] keeps,
/// for the same reason: nothing is minted before a verdict exists.
///
/// # Errors
///
/// A [`BlendRefusal`] carrying [`BlendKind::Chamfer`] — the verb
/// crosses HERE, once, and the inner [`BlendError`] stays
/// verb-neutral — around:
/// [`BlendError::NonpositiveSize`] when `distance` is not definitely
/// positive; [`BlendError::RepeatedEdge`] when the request names one
/// edge twice; [`BlendError::ChamferArmUnsupported`] when a requested
/// edge's supports are not both planes; any predicate refusal the
/// battery raises, or [`BlendError::Escalated`] carrying the margin;
/// [`BlendError::UnsupportedBody`], [`BlendError::UnsupportedChain`],
/// [`BlendError::UnsupportedRunOut`],
/// [`BlendError::UnsupportedGeometry`] or
/// [`BlendError::UnsupportedCorner`] when the request is outside
/// the assembly's front door; [`BlendError::BodyNotIntact`] when the
/// body does not hold together where the plan reads it;
/// [`BlendError::RingClearance`] when a carried-through ring does not
/// clear a trimline; [`BlendError::Op`] / [`BlendError::Certify`]
/// carrying an operator's or the pcurve pass's own typed refusal.
pub fn chamfer_edges<T: Decide + Bounds + geom_brep::PcurveFittedLane>(
    body: &Body<T>,
    edges: &[EdgeKey],
    distance: T,
    band: Band,
    tol: Tol,
) -> Result<Chamfered<T>, BlendRefusal> {
    chamfer_edges_inner(body, edges, distance, band, tol).map_err(|error| BlendRefusal {
        verb: BlendKind::Chamfer,
        error,
    })
}

/// [`chamfer_edges`] behind the door: the whole request, refusing
/// through the shared verb-neutral vocabulary. The door above is the
/// one place the chamfer's verb is attached.
fn chamfer_edges_inner<T: Decide + Bounds + geom_brep::PcurveFittedLane>(
    body: &Body<T>,
    edges: &[EdgeKey],
    distance: T,
    band: Band,
    tol: Tol,
) -> Result<Chamfered<T>, BlendError> {
    // The setback must be definitely positive, and that is a fact
    // about the REQUEST, so it is read off the bracket's low end
    // rather than metered: a `Zero`/`Negative` here is not a geometric
    // verdict about the body, and quoting one downstream would lever
    // the corner and clearance margins by the very number that is
    // wrong. Written through `partial_cmp` rather than `<= 0` so the
    // INCOMPARABLE case is an arm and not an accident: a poisoned size
    // is not definitely positive either, and it refuses here with the
    // other two.
    // NOTE the door asymmetry: only THIS door refuses a nonpositive
    // size; the fillet door lets a zero radius reach predicate 1 and
    // report a false fact. Issue #1336 (the door-asymmetric size validation)
    // owns closing it.
    if !matches!(
        distance.lo().partial_cmp(&0.0),
        Some(core::cmp::Ordering::Greater)
    ) {
        return Err(BlendError::NonpositiveSize {
            size: distance.lo(),
        });
    }
    repeated_edge_gate(edges)?;

    let request = BlendRequest {
        body,
        edges: edges.to_vec(),
        size: distance,
    };
    let verdict = super::battery::run_battery_for(&request, band, BlendKind::Chamfer)?;
    super::surgery::blend_surgery(body, &verdict, band, tol)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use geom_core::Tol;

    use super::super::admit::{AdmittedOpen, CornerFaces, CornerLinks};
    use super::super::battery::{Chain, ChainClosure, Link};
    use super::super::{BlendError, BlendKind};
    use crate::test_support::{L, all_links, cube};

    /// One open chain per link, so the door has something to admit.
    fn open_chain(link: Link<f64>) -> Chain<f64> {
        let (head, tail) = (link.start, link.end);
        Chain::new(
            link,
            Vec::new(),
            Vec::new(),
            ChainClosure::Open { head, tail },
        )
    }

    /// **A chart may not be scored off a corner the links do not
    /// belong to.**
    ///
    /// [`super::octant_chart`] takes two tokens that are derived
    /// independently — the corner's three faces off the source body's
    /// vertex orbit, the incident links off the battery's resolved
    /// arms — and neither type says they describe the same corner.
    /// Handed a mismatched pair it used to score one anyway, because
    /// excluding two strangers from three faces still names a face;
    /// the chart that came back was aimed at a trihedron somewhere
    /// else on the body.
    ///
    /// **Nothing downstream could have caught it**, which is why this
    /// row exists rather than an acceptance one: the chart is a sphere
    /// face's `u_ref` and `axis`, so a wrong one closes the same
    /// shell, passes tiers 1 and 2 unchanged, and surfaces at most as
    /// a tier-3 `NotIsoRectangle` at a corner whose geometry is right.
    /// The positive half is asserted in the same row so the refusal
    /// cannot pass by refusing everything.
    #[test]
    fn a_chart_scored_off_another_corner_s_faces_is_refused() {
        let body = cube(L, Tol::witness());
        let links = all_links(&body, Tol::witness());
        let chains: Vec<Chain<f64>> = links.iter().cloned().map(open_chain).collect();
        let admitted: Vec<AdmittedOpen<'_, f64>> = chains
            .iter()
            .map(|c| {
                AdmittedOpen::admit(c, BlendKind::Fillet)
                    .expect("a cube's links are convex plane–plane")
            })
            .collect();
        let here = *admitted.first().expect("a cube has requested links");
        let vertex = here.link().start;
        let corner = CornerLinks::seed(vertex, here).expect("the seed link terminates here");

        // The corner's OWN faces still chart, so the refusal below is
        // about the pairing and not about the corner.
        let mine = CornerFaces::admit(&body, vertex).expect("a cube corner is trivalent");
        assert!(
            super::octant_chart(&body, &mine, &corner).is_ok(),
            "a corner charted against its own three supports must still yield"
        );

        // A corner sharing NEITHER support of this link — on a cube,
        // the diagonally opposite one.
        let elsewhere = body
            .vertices()
            .map(|(v, _)| v)
            .filter_map(|v| CornerFaces::admit(&body, v).ok())
            .find(|f| !f.contains(here.link().face_a) && !f.contains(here.link().face_b))
            .expect("a cube has a corner sharing neither of a given edge's supports");
        assert!(
            matches!(
                super::octant_chart(&body, &elsewhere, &corner),
                Err(BlendError::BodyNotIntact { .. })
            ),
            "a chart derived from a link whose supports are not the corner's own must \
             refuse, not score"
        );
    }
}
