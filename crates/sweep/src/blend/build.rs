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
/// verb-neutral — around: [`BlendError::Band`] when the committed
/// tolerance admits no ambiguity band; any refusal the battery
/// produces;
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
    tol: Tol,
) -> Result<Filleted<T>, BlendRefusal> {
    fillet_edges_inner(body, edges, radius, tol).map_err(|error| BlendRefusal {
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
    tol: Tol,
) -> Result<Filleted<T>, BlendError> {
    let band = Band::linear(tol)?;
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
/// none does. Returns `(u_ref, axis)`.
///
/// **The chart follows the corner's convexity, and the invariant it
/// keeps is about the EQUATOR**: the seam meridian (`u_ref`) and its
/// quarter-turn (`axis × u_ref`) each pass exactly through a foot, on
/// either material side — a convex octant's feet lie along `+n_i`
/// from the centre, so `(u_ref, axis) = (n_a, n_a × n_b)`; a concave
/// one's lie along `−n_i`, so `(−n_b, n_b × n_a)`. The POLE is
/// parallel to the third foot only UP TO SIGN, inherently: the score
/// `|n_c × axis|` is even in `axis`, so the pick cannot constrain the
/// sign, which falls out of the winning link's stored face order
/// (measured split: 7:1 over a cube's convex octants, 6:2 over the
/// cavity's concave ones —
/// `review_blend4_r2_probes::r2_the_octant_chart_pole_is_the_third_foot_only_up_to_sign`,
/// beside the seam/quarter-turn pin that holds both halves of the
/// concave fold).
///
/// **The pick always yields, over one corner.** [`CornerLinks`] carries
/// at least one link and [`CornerFaces::third`] is total over the
/// corner's own three faces, so there is no "no candidate" state left.
///
/// **That the two tokens are ONE corner's is checked, not assumed** —
/// no type says it, and [`CornerLinks`] proves only that a link
/// terminates at its vertex. Today's one call site pairs them by
/// construction ([`super::surgery`]'s corner plan), so the check
/// cannot fire; it guards future mis-wiring, and this value is worth
/// guarding because it fails silently — a wrong `u_ref`/`axis` still
/// closes and still passes tiers 1 and 2.
///
/// # Errors
///
/// [`BlendError::BodyNotIntact`] when `faces` and `links` are not the
/// same corner's, or when a link's supports are not two of this
/// corner's three; [`BlendError::UnsupportedGeometry`] when a support
/// of this corner is not a plane.
pub(super) fn octant_chart<T: Decide + Bounds>(
    body: &Body<T>,
    faces: &CornerFaces,
    links: &CornerLinks<'_, T>,
    convexity: super::battery::Convexity,
) -> Result<(Vec3<T>, Vec3<T>), BlendError> {
    // The two tokens describe ONE corner. This is the total check:
    // the ends of a single edge share both of its supports and differ
    // only in the third, so a face-membership test alone cannot tell
    // adjacent corners apart, and the third support is exactly what
    // the score below turns on.
    if faces.vertex() != links.vertex() {
        return Err(not_intact(
            EntityId::Vertex(links.vertex()),
            "a corner's chart was offered the face orbit of a different vertex",
        ));
    }
    let convex = matches!(convexity, super::battery::Convexity::Convex);
    // `(score, u_ref, axis)` for one candidate edge: the chart aimed
    // along it, scored by how nearly the third support's normal is
    // parallel to its axis. The score is side-blind (`|n_c × axis|`
    // is even in `axis`), so the pick and the fold commute.
    let candidate = |l: &Link<T>| -> Result<(f64, Vec3<T>, Vec3<T>), BlendError> {
        // Before a normal is read: this link's two supports are two of
        // THIS corner's three. Necessary, not sufficient — the vertex
        // check above is what identifies the corner.
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
        // The third support of the corner — the one this edge does not
        // touch.
        let n_c = planar(third)?;
        let (u_ref, axis) = if convex {
            (n_a, n_a.cross(n_b).normalize())
        } else {
            (-n_b, n_b.cross(n_a).normalize())
        };
        Ok((n_c.cross(axis).norm().lo().abs(), u_ref, axis))
    };
    let mut best = candidate(links.first().link())?;
    for l in links.rest() {
        let next = candidate(l.link())?;
        if next.0 < best.0 {
            best = next;
        }
    }
    let (_, u_ref, axis) = best;
    Ok((u_ref, axis))
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
/// verb-neutral — around: [`BlendError::Band`] when the committed
/// tolerance admits no ambiguity band;
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
    tol: Tol,
) -> Result<Chamfered<T>, BlendRefusal> {
    chamfer_edges_inner(body, edges, distance, tol).map_err(|error| BlendRefusal {
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
    tol: Tol,
) -> Result<Chamfered<T>, BlendError> {
    let band = Band::linear(tol)?;
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

    use super::super::BlendError;
    use super::super::admit::{AdmittedOpen, CornerFaces, CornerLinks};
    use super::super::battery::{Chain, ChainClosure, Convexity, Link};
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

    /// The cross-corner refusal drill, shared by the two convexity
    /// arms below: the corner's OWN faces chart, a far corner (no
    /// shared support) refuses, and the SAME EDGE's other end (both
    /// supports shared, a different third — the face the score reads)
    /// refuses too.
    fn cross_corner_drill(convexity: Convexity, chains: &[Chain<f64>], body: &topo::Body<f64>) {
        let admitted: Vec<AdmittedOpen<'_, f64>> = chains
            .iter()
            .map(|c| AdmittedOpen::admit(c).expect("a cube's links are plane–plane"))
            .collect();
        let here = *admitted.first().expect("a cube has requested links");
        let vertex = here.link().start;
        let corner = CornerLinks::seed(vertex, here).expect("the seed link terminates here");

        // The corner's OWN faces still chart, so the refusals below are
        // about the pairing and not about the corner.
        let mine = CornerFaces::admit(body, vertex).expect("a cube corner is trivalent");
        assert!(
            super::octant_chart(body, &mine, &corner, convexity).is_ok(),
            "a corner charted against its own three supports must still yield"
        );

        // (1) A corner sharing NEITHER support of this link — on a
        // cube, the diagonally opposite one. Face membership alone
        // catches this one.
        let elsewhere = body
            .vertices()
            .map(|(v, _)| v)
            .filter_map(|v| CornerFaces::admit(body, v).ok())
            .find(|f| !f.contains(here.link().face_a) && !f.contains(here.link().face_b))
            .expect("a cube has a corner sharing neither of a given edge's supports");
        assert!(
            matches!(
                super::octant_chart(body, &elsewhere, &corner, convexity),
                Err(BlendError::BodyNotIntact { .. })
            ),
            "a chart derived from a link whose supports are not the corner's own must \
             refuse, not score"
        );

        // (2) The OTHER END of this very edge: both supports shared, a
        // different third — the mismatch every face-membership test
        // admits, and the one that changes the score.
        let adjacent = CornerFaces::admit(body, here.link().end)
            .expect("the far end of a cube edge is a trivalent corner too");
        assert!(
            adjacent.contains(here.link().face_a) && adjacent.contains(here.link().face_b),
            "the adjacent corner must share BOTH supports, or it is case (1) again"
        );
        assert_ne!(
            adjacent
                .third(here.link().face_a, here.link().face_b)
                .expect("both supports are this corner's"),
            mine.third(here.link().face_a, here.link().face_b)
                .expect("and this corner's"),
            "the two ends must differ in the third support, or the case is vacuous"
        );
        assert!(
            matches!(
                super::octant_chart(body, &adjacent, &corner, convexity),
                Err(BlendError::BodyNotIntact { .. })
            ),
            "a chart offered the face orbit of the edge's OTHER end must refuse: both \
             supports match and the third — the one the score reads — does not"
        );
    }

    /// **A chart may not be scored off a corner the links do not
    /// belong to.**
    ///
    /// [`super::octant_chart`] takes two tokens that are derived
    /// independently — the corner's three faces off the source body's
    /// vertex orbit, the incident links off the battery's resolved
    /// arms — and neither type says they describe the same corner.
    /// Handed a mismatched pair it would score one anyway, because
    /// excluding two faces from three still names a face.
    ///
    /// **Today's one production call site pairs them by construction**
    /// (`surgery`'s corner plan derives both from one vertex), so this
    /// is not a live defect being pinned — it is the check that keeps
    /// a future caller from assembling them further apart and getting
    /// no complaint. That direction matters here because the value is
    /// a sphere face's `u_ref`/`axis`: a wrong one closes the same
    /// shell, passes tiers 1 and 2 unchanged, and would surface at
    /// most as a tier-3 `NotIsoRectangle` at a corner whose geometry
    /// is right.
    ///
    /// **Two mismatches, and the second is why the check is on the
    /// VERTEX.** A far corner shares no support, so a face-membership
    /// test catches it; the OTHER END OF THE SAME EDGE shares both
    /// supports and differs only in the third — which is precisely the
    /// face the score reads — so membership alone answers it happily.
    /// The positive half is asserted in the same row so the refusal
    /// cannot pass by refusing everything.
    #[test]
    fn a_chart_scored_off_another_corner_s_faces_is_refused() {
        let body = cube(L, Tol::witness());
        let links = all_links(&body, Tol::witness());
        let chains: Vec<Chain<f64>> = links.iter().cloned().map(open_chain).collect();
        cross_corner_drill(Convexity::Convex, &chains, &body);
    }

    /// **The concave arm carries the cross-corner refusal too.** The
    /// guard above resolves and refuses BEFORE the convexity fold, so
    /// on paper the arm cannot matter — and that is exactly the kind
    /// of claim the fold's own review showed needs a row rather than
    /// an argument (half of the fold was once pinned by nothing). The
    /// verdicts are FALSIFIED to concave on a cube whose geometry is
    /// untouched, the established shape for exercising the concave arm
    /// of a plan-level derivation: what is pinned is the guard's
    /// behaviour as a function of the tokens and the arm, not the
    /// cube.
    #[test]
    fn a_chart_scored_off_another_corner_s_faces_is_refused_on_the_concave_arm_too() {
        let body = cube(L, Tol::witness());
        let links = all_links(&body, Tol::witness());
        let chains: Vec<Chain<f64>> = links
            .iter()
            .cloned()
            .map(|mut l| {
                l.convexity = Convexity::Concave;
                open_chain(l)
            })
            .collect();
        cross_corner_drill(Convexity::Concave, &chains, &body);
    }
}
