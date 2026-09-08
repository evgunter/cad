//! **Ring clearance takes the relation the trim circle stands in.**
//!
//! Two circles that do not cross are clear of each other in one of two
//! ways, and which one is admissible is fixed by what the blend's trim
//! circle REPLACES on the face it lies in — never by which margin reads
//! better. These rows carve the four bodies where the relation is not
//! the plain external separation, and refuse the two where the right
//! relation is violated.
//!
//! The fixture is `test_support::boss` after `merge_coplanar_faces`: one
//! plane ANNULUS for the flat top, carrying the dome rim as a RING and
//! the top rim as its OUTER cycle.
//!
//! - the DOME rim `(0.5, 1)` is a LADDER rim (it IS the ring), and its
//!   widened trim circle sits CONCENTRIC inside the host's circular
//!   outer boundary. The external form reads minus the sum of the two
//!   radii there; the boundary must CONTAIN the trim circle, and does,
//!   by `1 − √0.35 ≈ 0.408`.
//! - the TOP OUTER rim `(1, 1)` is a hostless ANNULUS rim on that same
//!   ringed host. Its trim circle BECOMES the face's outer boundary, so
//!   the dome ring is admissible exactly when the trim contains it, by
//!   `0.9 − 0.5 = 0.4`; the external form reads `−1.4` on the same pair
//!   and would refuse a carve that is geometrically fine.
//!
//! Both material sides are covered by the boss's dimple twin, which is
//! the same two doors with the dome dug in instead of raised.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Sign, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    boss, plane_sphere_external_cut, revolved_about_y, rim_arcs_at, wedge_fill,
};
use topo::{Body, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// The boss (or its dimple twin) after the repair every boolean
/// consumer runs.
fn repaired(up: bool) -> Body<f64> {
    let mut b = boss(up, tol());
    b.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    b
}

fn census(b: &Body<f64>) -> (usize, usize, usize) {
    (b.vertices().count(), b.edges().count(), b.faces().count())
}

/// The volume delta of one carve, tier-3 valid and closed-form, against
/// the hand oracle: `(measured, census before, census after)`.
fn carve(name: &str, body: &Body<f64>, rims: &[(f64, f64)], r: f64) -> (f64, Body<f64>) {
    let mut arcs = Vec::new();
    for (rr, ry) in rims {
        let a = rim_arcs_at(body, *rr, *ry);
        assert_eq!(a.len(), 2, "{name}: the ({rr}, {ry}) rim is two arcs");
        arcs.extend(a);
    }
    let before = mass_properties(body, tol()).unwrap();
    let out =
        fillet_edges(body, &arcs, r, tol()).unwrap_or_else(|e| panic!("{name} carves, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{name} is tier-3 valid, got {e:?}"));
    let after = mass_properties(&out.body, tol()).unwrap();
    assert_eq!(after.volume_pad, 0.0, "{name}: closed-form faces only");
    assert_eq!(out.band_faces.len(), rims.len(), "{name}: one band per rim");
    (after.volume - before.volume, out.body)
}

/// The bar every closed form below is held to: an absolute `1e-12`,
/// scaled by the form's own magnitude once that exceeds one meter³.
fn agrees(name: &str, measured: f64, want: f64) {
    assert!(
        (measured - want).abs() <= 1e-12 * want.abs().max(1.0),
        "{name}: the carve's volume delta is the hand form \
         (measured {measured}, closed form {want})"
    );
}

// ------------------------------------------------------------------
// The four carves, at their closed forms.
// ------------------------------------------------------------------

/// **The LADDER rim inside a circular outer boundary, both material
/// sides.** The dome rim's host is the merged flat top; its outer
/// boundary is the cylinder's top rim at radius 1, a circle CONTAINING
/// the widened trim circle at `√0.35 ≈ 0.5916` with `0.408` to spare.
///
/// **The closed form, derived here.** In the meridian half-plane the rim
/// is `K = (0.5, 1)`; the flat top runs OUT along `(+1, 0)` and the
/// dome's tangent runs UP along `(0, +1)`, with the material filling the
/// other 270 degrees. So the boss's dome rim is CONCAVE — the ball rests
/// in the 90-degree VOID, EXTERNALLY tangent to the dome sphere at
/// `‖C − (0, 1)‖ = 0.5 + r`, and the band ADDS the region between the
/// plane, the sphere and the ball's arc. That region is the kite
/// `K, F_a, C, F_b` minus the sphere's segment on the chord `K`–`F_b`
/// (the arc bulges away from the sphere's centre, INTO the kite, and the
/// void is on the outside of the sphere) minus the ball's sector; by
/// Pappus, `test_support::plane_sphere_external_cut`.
///
/// The DIMPLE's rim at the same station is that configuration's mirror
/// through the plane `y = 1`: the material wedge is the 90-degree one,
/// the rim is CONVEX and the band REMOVES the same region. One form,
/// two signs — which is what makes the sign a measurement rather than a
/// fit.
#[test]
fn the_bosss_dome_rim_carves_inside_its_hosts_circular_boundary() {
    let form = plane_sphere_external_cut(0.5, 0.1);
    for (up, adds) in [(true, true), (false, false)] {
        let name = if up { "boss dome rim" } else { "dimple rim" };
        let body = repaired(up);
        assert_eq!(census(&body), (7, 10, 6), "{name}: the repaired census");
        let (measured, out) = carve(name, &body, &[(0.5, 1.0)], 0.1);
        assert_eq!(census(&out), (9, 13, 7), "{name}: the band's census delta");
        agrees(name, measured, if adds { form } else { -form });
    }
}

/// **The hostless ANNULUS rim on a host that carries a RING, both
/// material sides.** The top outer rim is the merged flat top's whole
/// OUTER cycle, so the band's host trim at radius `1 − r` becomes that
/// face's new outer boundary and the dome ring at radius 0.5 must lie
/// INSIDE it — which it does, by `0.4`.
///
/// **The closed form, derived here.** At `K = (1, 1)` the flat top runs
/// IN along `(−1, 0)` and the cylinder wall runs DOWN along `(0, −1)`,
/// with the material between them: a 90-degree material wedge, so the
/// rim is CONVEX on both fixtures (the dimple changes only the inner
/// part of the top) and the band REMOVES the corner square minus the
/// ball's quadrant. By Pappus, `test_support::wedge_fill` at those two
/// generators.
#[test]
fn the_bosss_top_outer_rim_carves_on_a_ringed_host() {
    let form = wedge_fill((1.0, 1.0), (-1.0, 0.0), (0.0, -1.0), 0.1);
    for up in [true, false] {
        let name = if up { "boss top rim" } else { "dimple top rim" };
        let body = repaired(up);
        // The shape: ONE plane host, the rim its whole outer cycle, and
        // that host also carrying a ring.
        let host = {
            let arcs = rim_arcs_at(&body, 1.0, 1.0);
            let ed = body.get_edge(arcs[0]).unwrap();
            [ed.he_plus, ed.he_minus]
                .into_iter()
                .map(|he| {
                    body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
                        .unwrap()
                        .face
                })
                .find(|&f| !body.get_face(f).unwrap().rings.is_empty())
                .expect("the merged flat top carries the dome ring")
        };
        assert_eq!(
            body.get_face(host).unwrap().rings.len(),
            1,
            "{name}: the host is an annulus"
        );
        let (measured, out) = carve(name, &body, &[(1.0, 1.0)], 0.1);
        assert_eq!(census(&out), (9, 13, 7), "{name}: the band's census delta");
        assert_eq!(
            body.get_face(host).unwrap().rings.len(),
            out.get_face(host).map_or(0, |fd| fd.rings.len()),
            "{name}: the host keeps its ring across the annulus surgery"
        );
        agrees(name, measured, -form);
    }
}

/// **The two rims of one host, together and in sequence.** Requesting
/// the dome rim (a LADDER) and the top outer rim (an ANNULUS) in ONE
/// call reaches `shared_support_gate`'s mixed arm — an annulus carve
/// consumes structure of the shared face beyond its own rim — and takes
/// that arm's SEQUENTIAL recourse. Following the recourse builds: the
/// two calls in order give one tier-3-valid solid whose volume delta is
/// the SUM of the two closed forms, on both material sides.
#[test]
fn the_bosss_two_rims_refuse_together_and_compose_sequentially() {
    let ladder = plane_sphere_external_cut(0.5, 0.1);
    let annulus = wedge_fill((1.0, 1.0), (-1.0, 0.0), (0.0, -1.0), 0.1);
    for up in [true, false] {
        let name = if up { "boss" } else { "dimple" };
        let body = repaired(up);
        let mut both = rim_arcs_at(&body, 0.5, 1.0);
        both.extend(rim_arcs_at(&body, 1.0, 1.0));
        match fillet_edges(&body, &both, 0.1, tol()).map_err(|e| e.error) {
            Err(BlendError::UnsupportedChain { detail, .. }) => assert!(
                detail.contains("SEQUENTIAL calls"),
                "{name}: the mixed ladder/annulus arm and its recourse: {detail}"
            ),
            other => panic!("{name}: expected the mixed shared-support arm, got {other:?}"),
        }
        let before = mass_properties(&body, tol()).unwrap().volume;
        let (_, after_ladder) = carve(name, &body, &[(0.5, 1.0)], 0.1);
        let (_, out) = carve(name, &after_ladder, &[(1.0, 1.0)], 0.1);
        assert_eq!(census(&out), (11, 16, 8), "{name}: two bands' census");
        let measured = mass_properties(&out, tol()).unwrap().volume - before;
        agrees(name, measured, if up { ladder } else { -ladder } - annulus);
    }
}

// ------------------------------------------------------------------
// The refusing rows, one per relation — and where the refusal lands.
// ------------------------------------------------------------------

/// A revolve of `(0,0) (rr,0) (rr,1) (0.5,1)[dome] (0,1.5)`, repaired:
/// the boss with its flat top narrowed to outer radius `rr`.
fn narrowed(rr: f64) -> Body<f64> {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let mut b = revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(rr, 0.0), 0.0),
            ProfileVertex::new(Point2::new(rr, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.5, 1.0), q),
            ProfileVertex::new(Point2::new(0.0, 1.5), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    b.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    b
}

/// A revolve of `(0,0) (1,0) (1,1) (a,1)[dome] (0,1+a)`, repaired: the
/// boss with its dome grown to radius `a`, so the dome RING sits `1 − a`
/// inside the top rim.
fn domed(a: f64) -> Body<f64> {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let mut b = revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(a, 1.0), q),
            ProfileVertex::new(Point2::new(0.0, 1.0 + a), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    b.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    b
}

/// The definite reading of whichever clearance predicate refuses a
/// request, with its predicate name.
fn refusal_reading(err: BlendError) -> (String, f64) {
    let margin = match err {
        BlendError::RingClearance { margin, .. }
        | BlendError::FaceClearanceUncertified { margin, .. } => margin,
        other => panic!("expected a clearance refusal, got {other:?}"),
    };
    assert_eq!(margin.sign, Sign::Negative, "a definite refusal");
    (
        margin.predicate.to_string(),
        margin.value().expect("a definite reading"),
    )
}

/// **A LADDER rim whose widened trim circle CROSSES its host's circular
/// outer boundary is refused, at exactly the containment margin's zero.**
///
/// The radii are derived, not searched. The dome rim's ladder trim sits
/// at `√((0.5 + r)² − r²)` — the ball rests in the void at height `r`
/// and EXTERNALLY tangent to the dome of radius 0.5 — and the host's
/// outer boundary is the cylinder's top rim at `rr`, concentric with it.
/// So the containment margin is `rr − √((0.5 + r)² − r²)`, which at
/// `r = 0.1` is `rr − √0.35`: `−0.0416` at `rr = 0.55` and `+0.0084` at
/// `rr = 0.6`. The door refuses the first and carves the second, so the
/// margin's own zero is the boundary between them and the row pins both
/// sides of it.
///
/// **Where the refusal lands, measured.** Predicate 2's sampled screen
/// answers first, and its reading is the containment margin to the bit:
/// on a coaxial pair the screen's `gap − setback` — here
/// `(rr − 0.5) − (√0.35 − 0.5)` — IS `rr − √0.35`, and nine samples on
/// each of two arcs of a circle put a sample pair at a shared azimuth,
/// so the sampled gap is the true one. The closed form in
/// `ring_clearance_pass` is the exact backstop of that screen and agrees
/// with it wherever both are defined; what changed is that the pass no
/// longer CONTRADICTS the screen by reading external separation on a
/// nested pair.
#[test]
fn a_ladder_trim_circle_crossing_its_boundary_refuses() {
    let want = 0.55 - ((0.5 + 0.1f64).powi(2) - 0.01).sqrt();
    assert!(want < -0.04, "the derived margin is definitely negative");
    let body = narrowed(0.55);
    let arcs = rim_arcs_at(&body, 0.5, 1.0);
    let (predicate, read) = refusal_reading(
        fillet_edges(&body, &arcs, 0.1, tol())
            .expect_err("a trim circle crossing its host's boundary refuses")
            .error,
    );
    assert_eq!(
        predicate, "fillet3_face_clearance",
        "the sampled screen answers first on this coaxial pair"
    );
    assert!(
        (read - want).abs() <= 1e-15,
        "the reading is the containment margin `rr − √((0.5+r)² − r²)` \
         (read {read}, derived {want})"
    );
    // The other side of the same zero: widen the flat top by 0.05 and
    // the containment margin turns positive and the carve goes through.
    let wide = narrowed(0.6);
    let arcs = rim_arcs_at(&wide, 0.5, 1.0);
    assert!(
        0.6 - ((0.5 + 0.1f64).powi(2) - 0.01).sqrt() > 0.008,
        "and the derived margin there is positive"
    );
    let out = fillet_edges(&wide, &arcs, 0.1, tol()).expect("the nested trim circle carves");
    validate_geometric(&out.body, tol()).expect("tier-3 valid");
}

/// **A hostless ANNULUS rim whose host RING lies in the strip the carve
/// excises is refused, at exactly the containment margin's zero.**
///
/// Derived the same way. A boss whose dome radius is `a` puts the ring
/// at `a` and the top rim's annulus trim at `1 − r`, concentric, so the
/// containment margin is `(1 − r) − a` and the ring sits inside the
/// excised strip once `a > 1 − r`. At `r = 0.1` that is `0.9 − a`:
/// `−0.02` at `a = 0.92` and `+0.05` at `a = 0.85`. The row pins both
/// sides.
///
/// **The relation is what decides, not the sign.** The EXTERNAL form on
/// the same pair reads `0 − 0.9 − a`, negative at every `a`, because
/// concentric circles are never externally separated — so metering this
/// relation externally refuses every hostless annulus rim on its own
/// host's rings, which is what the routing gate on a ringed host used to
/// stand in for. The carving half of this row is what goes red if the
/// forms are swapped.
#[test]
fn a_hostless_annulus_ring_in_the_excised_strip_refuses() {
    let body = domed(0.92);
    let arcs = rim_arcs_at(&body, 1.0, 1.0);
    let (predicate, read) = refusal_reading(
        fillet_edges(&body, &arcs, 0.1, tol())
            .expect_err("a ring in the excised strip refuses")
            .error,
    );
    assert_eq!(
        predicate, "fillet3_face_clearance",
        "the sampled screen answers first on this coaxial pair"
    );
    assert!(
        (read - (0.9 - 0.92)).abs() <= 1e-15,
        "the reading is the containment margin `(1 − r) − a` (read {read})"
    );
    // The other side of the same zero.
    let wide = domed(0.85);
    let arcs = rim_arcs_at(&wide, 1.0, 1.0);
    let out = fillet_edges(&wide, &arcs, 0.1, tol())
        .expect("a ring the trim circle contains carves through");
    validate_geometric(&out.body, tol()).expect("tier-3 valid");
}
