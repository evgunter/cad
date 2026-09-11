//! **FILLET-H5 review probes (lane r2)** — rows the unit's own suite
//! does not carry, each written to falsify one claim of PR 1824 by
//! execution rather than by reading.
//!
//! - the HOSTLESS host gate ("one ring-free face whose outer cycle is
//!   EXACTLY the chain's arcs") is reached and refuses on a host face
//!   carrying one unrequested edge in its outer cycle;
//! - a CURVED single face carrying several arcs — the shape the
//!   `HostSide`-passed-not-derived argument is about — refuses at the
//!   half-band gate whether or not the plane side has been repaired,
//!   and `Struts` never carves it;
//! - two compositions the #935 row does not cover: two hostless rims
//!   of one body on a SHARED mate wall in one call, and two hostless
//!   rims sharing no wall — both against both sequential orders;
//! - a hostless rim beside a ring-hosted LADDER rim, and a rim in the
//!   outer cycle of a plane face that also carries a ring — what each
//!   refuses with, measured;
//! - the bowl's fill and the plane×sphere cut against constants derived
//!   OUTSIDE the tree (an independent Pappus derivation, not
//!   `test_support::wedge_fill`), so the row cannot agree with the
//!   oracle it is checking.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Point2, Point3, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{bowl, lantern, revolved_about_y, rim_arcs_at};
use topo::{Body, EdgeKey, FaceKey, MevSite, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn repaired(mut body: Body<f64>) -> Body<f64> {
    body.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    body
}

fn faces_of(body: &Body<f64>, e: EdgeKey) -> (FaceKey, FaceKey) {
    let ed = body.get_edge(e).unwrap();
    let f = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    (f(ed.he_plus), f(ed.he_minus))
}

fn is_plane(body: &Body<f64>, f: FaceKey) -> bool {
    matches!(
        body.get_surface(body.get_face(f).unwrap().surface).unwrap(),
        Surface::Plane { .. }
    )
}

fn census(b: &Body<f64>) -> (i64, i64, i64) {
    (
        b.vertices().count() as i64,
        b.edges().count() as i64,
        b.faces().count() as i64,
    )
}

fn detail_of(err: BlendError) -> String {
    match err {
        BlendError::UnsupportedChain { detail, .. } => detail.to_string(),
        other => panic!("expected an UnsupportedChain refusal, got {other:?}"),
    }
}

/// A pole-touching hemisphere of radius 1 on a flat base disc.
fn hemisphere_on_flat_base() -> Body<f64> {
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(
                Point2::new(1.0, 0.0),
                (core::f64::consts::FRAC_PI_2 / 4.0).tan(),
            ),
            ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// A pole-touching cylinder of radius 1 and height 1: after the repair
/// BOTH its rims are hostless (one disc each) and they share the
/// cylinder wall as their mate.
fn pole_cylinder() -> Body<f64> {
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// A cylinder of radius 1 carrying a coaxial cylindrical boss of radius
/// 0.5 on its flat top: after the repair the base disc `(1, 0)` and the
/// boss's top disc `(0.5, 1.5)` are hostless rims, the boss's root
/// `(0.5, 1)` is a RING of the annular top, and the top's outer rim
/// `(1, 1)` lies in the outer cycle of a face that also carries a ring.
fn stepped() -> Body<f64> {
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.5, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.5, 1.5), 0.0),
            ProfileVertex::new(Point2::new(0.0, 1.5), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// Merge the CURVED wall the rim's arcs rest on into ONE face by
/// killing one of its co-surface seam meridians at a rim vertex
/// (`kef`): the half-band discipline is then false of that wall.
fn merge_curved_wall(body: &mut Body<f64>, arcs: &[EdgeKey]) {
    for &a in arcs {
        let ed = body.get_edge(a).unwrap();
        for he in [ed.he_plus, ed.he_minus] {
            let v = body.get_half_edge(he).unwrap().start;
            let em = body.get_vertex(v).unwrap().emanating.unwrap();
            let orbit = body.vertex_orbit(em).unwrap();
            for h in orbit {
                let e = body.get_half_edge(h).unwrap().edge;
                if arcs.contains(&e) {
                    continue;
                }
                let (fa, fb) = faces_of(body, e);
                if fa == fb || is_plane(body, fa) || is_plane(body, fb) {
                    continue;
                }
                if body.get_face(fa).unwrap().surface != body.get_face(fb).unwrap().surface {
                    continue;
                }
                let hp = body.get_edge(e).unwrap().he_plus;
                body.kef(hp)
                    .expect("a curved wall's seam meridian kills into one face");
                return;
            }
        }
    }
    panic!("no curved co-surface seam meridian found at a rim vertex");
}

fn planar_supports(body: &Body<f64>, arcs: &[EdgeKey]) -> Vec<FaceKey> {
    let mut out = Vec::new();
    for &a in arcs {
        let (fa, fb) = faces_of(body, a);
        for f in [fa, fb] {
            if is_plane(body, f) && !out.contains(&f) {
                out.push(f);
            }
        }
    }
    out
}

fn curved_supports(body: &Body<f64>, arcs: &[EdgeKey]) -> Vec<FaceKey> {
    let mut out = Vec::new();
    for &a in arcs {
        let (fa, fb) = faces_of(body, a);
        for f in [fa, fb] {
            if !is_plane(body, f) && !out.contains(&f) {
                out.push(f);
            }
        }
    }
    out
}

// ------------------------------------------------------------------
// The host gate under `Struts`.
// ------------------------------------------------------------------

/// **A hostless host whose outer cycle carries one edge the request did
/// not name refuses at the host gate, before any mutation.** A strut
/// spur (`mev` with an empty fan run) is spliced into the repaired
/// neck's cap loop at a crossing: the cap stays one ring-free plane
/// face whose outer cycle is now the two arcs plus that spur, so the
/// rim is still routed to the annulus with hostless crossings and the
/// gate that asks for EXACTLY the chain's arcs is what fires.
#[test]
fn a_hostless_host_with_an_unrequested_outer_cycle_edge_refuses_at_the_host_gate() {
    let mut body = repaired(lantern(tol()));
    let arcs = rim_arcs_at(&body, 1.0, 0.0);
    assert_eq!(arcs.len(), 2);
    let (fa, fb) = faces_of(&body, arcs[0]);
    let host = if is_plane(&body, fa) { fa } else { fb };
    let ed = body.get_edge(arcs[0]).unwrap();
    let he = [ed.he_plus, ed.he_minus]
        .into_iter()
        .find(|&h| {
            body.get_loop(body.get_half_edge(h).unwrap().parent_loop)
                .unwrap()
                .face
                == host
        })
        .unwrap();
    let v = body.get_half_edge(he).unwrap().start;
    let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    // A spur from the crossing halfway in toward the axis, in the cap's
    // own plane `y = 0`.
    let spur_end = Point3::new(p.x * 0.5, p.y, p.z * 0.5);
    body.mev_line(MevSite::Fan { he1: he, he2: he }, spur_end, tol())
        .expect("a spur into the cap face");
    let fd = body.get_face(host).unwrap();
    assert!(fd.rings.is_empty(), "the cap is still ring-free");
    let before = census(&body);

    let err = fillet_edges(&body, &arcs, 0.05, tol())
        .expect_err("a host with an unrequested outer-cycle edge must refuse");
    let detail = detail_of(err.error);
    assert!(
        detail.contains("outside the requested chain in its outer cycle"),
        "the HOST gate is the one that fires: {detail}"
    );
    assert_eq!(census(&body), before, "refused before any mutation");
}

// ------------------------------------------------------------------
// The curved single host — the `HostSide`-passed argument's shape.
// ------------------------------------------------------------------

/// **A CURVED single face carrying both arcs refuses at the half-band
/// gate on both routes, and `Struts` never carves it.** The plane×sphere
/// hemisphere's two half-caps are merged into ONE sphere face by
/// killing a seam meridian. Unrepaired (two plane half-discs) the rim
/// takes the `Seams` route and the sphere face fails the half-band
/// gate; repaired (one disc) it takes the `Struts` route and the SAME
/// gate fires on the mate. Neither carves.
#[test]
fn a_curved_single_face_carrying_both_arcs_refuses_at_the_half_band_gate_on_both_routes() {
    // (a) Unrepaired plane side: `Seams` route.
    let mut body = hemisphere_on_flat_base();
    let arcs = rim_arcs_at(&body, 1.0, 0.0);
    assert_eq!(arcs.len(), 2);
    merge_curved_wall(&mut body, &arcs);
    assert_eq!(
        curved_supports(&body, &arcs).len(),
        1,
        "ONE sphere face carries both arcs"
    );
    assert_eq!(
        planar_supports(&body, &arcs).len(),
        2,
        "the plane is still two half-discs"
    );
    let detail = detail_of(
        fillet_edges(&body, &arcs, 0.05, tol())
            .expect_err("a curved single host refuses (Seams route)")
            .error,
    );
    assert!(
        detail.contains("does not carry exactly its own rim arc"),
        "Seams route: the half-band gate fires: {detail}"
    );

    // (b) Repaired plane side: `Struts` route, the same gate on the mate.
    // Repair FIRST (the merge refuses a body carrying the strut vertex
    // the `kef` leaves at the pole), then merge the curved wall.
    let mut body = repaired(hemisphere_on_flat_base());
    let arcs = rim_arcs_at(&body, 1.0, 0.0);
    merge_curved_wall(&mut body, &arcs);
    assert_eq!(planar_supports(&body, &arcs).len(), 1, "one plane host now");
    assert_eq!(
        curved_supports(&body, &arcs).len(),
        1,
        "still one sphere face"
    );
    let detail = detail_of(
        fillet_edges(&body, &arcs, 0.05, tol())
            .expect_err("a curved single mate refuses (Struts route)")
            .error,
    );
    assert!(
        detail.contains("does not carry exactly its own rim arc"),
        "Struts route: the half-band gate fires on the mate: {detail}"
    );
}

// ------------------------------------------------------------------
// Compositions the #935 row does not cover.
// ------------------------------------------------------------------

fn compose_two_rims(
    source: &Body<f64>,
    rims: [(f64, f64); 2],
    r: f64,
    what: &str,
) -> Result<(), String> {
    let mut both = rim_arcs_at(source, rims[0].0, rims[0].1);
    both.extend(rim_arcs_at(source, rims[1].0, rims[1].1));
    assert_eq!(both.len(), 4, "{what}: two rims of two arcs each");
    let one_call = fillet_edges(source, &both, r, tol())
        .map_err(|e| format!("{what}: one call refused: {:?}", e.error))?;
    validate_geometric(&one_call.body, tol())
        .map_err(|e| format!("{what}: one-call result not tier-3 valid: {e:?}"))?;
    assert_eq!(one_call.band_faces.len(), 2, "{what}: one band per rim");
    let one = mass_properties(&one_call.body, tol()).unwrap();
    assert_eq!(one.volume_pad, 0.0, "{what}: closed-form faces only");
    for order in [[rims[0], rims[1]], [rims[1], rims[0]]] {
        let mut body = source.clone();
        for (rr, ry) in order {
            let arcs = rim_arcs_at(&body, rr, ry);
            body = fillet_edges(&body, &arcs, r, tol())
                .map_err(|e| format!("{what}: rim ({rr}, {ry}) refused alone: {:?}", e.error))?
                .body;
        }
        validate_geometric(&body, tol())
            .map_err(|e| format!("{what}: sequential result not tier-3 valid: {e:?}"))?;
        let seq = mass_properties(&body, tol()).unwrap();
        if one.volume.to_bits() != seq.volume.to_bits() {
            return Err(format!(
                "{what}: one call {} vs sequential {order:?} {} differ in bits",
                one.volume, seq.volume
            ));
        }
        if census(&one_call.body) != census(&body) {
            return Err(format!(
                "{what}: census differs between one call and {order:?}"
            ));
        }
    }
    Ok(())
}

/// **Two hostless rims of one body on a SHARED mate wall, in one call.**
/// The repaired pole-touching cylinder's two discs are each one plane
/// face; both rims are hostless and both rest on the cylinder's two
/// half-bands, so the second rim's `refresh_annulus_seams` re-reads
/// mate seams the first band split while carrying a `Strut` through on
/// the host side. Both sequential orders must agree bit for bit.
#[test]
fn two_hostless_rims_on_a_shared_mate_wall_compose_in_one_call() {
    let source = repaired(pole_cylinder());
    for (r, y) in [(1.0, 0.0), (1.0, 1.0)] {
        let arcs = rim_arcs_at(&source, r, y);
        assert_eq!(arcs.len(), 2);
        assert_eq!(
            planar_supports(&source, &arcs).len(),
            1,
            "({r}, {y}) is hostless"
        );
    }
    compose_two_rims(
        &source,
        [(1.0, 0.0), (1.0, 1.0)],
        0.05,
        "pole cylinder, both rims",
    )
    .unwrap_or_else(|m| panic!("{m}"));
}

/// **Two hostless rims of one body sharing NO wall, in one call** — the
/// repaired lantern's neck (plane×sphere) and lip (plane×cone). Nothing
/// is refreshed; the plan's own keys carry both.
#[test]
fn two_hostless_rims_sharing_no_wall_compose_in_one_call() {
    let source = repaired(lantern(tol()));
    compose_two_rims(
        &source,
        [(1.0, 0.0), (0.2, 1.2)],
        0.05,
        "lantern neck + lip",
    )
    .unwrap_or_else(|m| panic!("{m}"));
}

/// **A hostless rim beside a ring-hosted LADDER rim, and a rim in the
/// outer cycle of a face that carries a ring — measured.** On the
/// stepped body the boss root `(0.5, 1)` is a ring of the annular top
/// (ladder) and refuses on the pre-existing false ring clearance
/// (`work/fillet/ring-clearance-refuses-a-nested-trim-circle.md`), so
/// the composition is not reachable today and the one call refuses
/// with exactly that; the top's outer rim `(1, 1)` routes to the
/// hostless annulus and refuses at its ring gate; the two disc rims
/// carve alone.
#[test]
fn a_hostless_rim_beside_a_ladder_rim_and_a_ringed_host_measured() {
    let body = repaired(stepped());
    for (r, y) in [(1.0, 0.0), (0.5, 1.5)] {
        let arcs = rim_arcs_at(&body, r, y);
        assert_eq!(arcs.len(), 2, "({r}, {y}) two arcs");
        let out = fillet_edges(&body, &arcs, 0.05, tol())
            .unwrap_or_else(|e| panic!("the disc rim ({r}, {y}) carves, got {e:?}"));
        validate_geometric(&out.body, tol()).expect("tier-3 valid");
    }
    // The ringed host: outer cycle of a face that also carries a ring.
    let outer = rim_arcs_at(&body, 1.0, 1.0);
    assert_eq!(outer.len(), 2);
    let detail = detail_of(
        fillet_edges(&body, &outer, 0.05, tol())
            .expect_err("the ringed host refuses")
            .error,
    );
    assert!(
        detail.contains("host face carries rings of its own"),
        "the hostless host gate's ring clause: {detail}"
    );
    // The ladder ring beside a hostless rim, one call.
    let mut both = rim_arcs_at(&body, 1.0, 0.0);
    both.extend(rim_arcs_at(&body, 0.5, 1.0));
    assert_eq!(both.len(), 4);
    match fillet_edges(&body, &both, 0.05, tol()).map_err(|e| e.error) {
        Err(BlendError::RingClearance { margin, .. }) => {
            assert_eq!(margin.predicate, "fillet3_ring_clearance");
        }
        other => panic!("expected the ladder's false ring-clearance refusal, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// Closed forms against constants derived outside the tree.
// ------------------------------------------------------------------

/// **The bowl's fill and the plane×sphere cut against an INDEPENDENT
/// derivation.** The constants below were computed outside the tree
/// (a separate Pappus derivation: triangle centroids, a polar-integral
/// sector moment, and for the sphere a circular-segment moment), not
/// by `test_support::wedge_fill` or `pappus`, so this row cannot agree
/// with the oracle it checks. `r = 0.05` throughout.
#[test]
fn the_hostless_closed_forms_match_an_independent_derivation() {
    const BOWL_FILL: f64 = 3.375_275_670_086_987_4e-4;
    const PLANE_SPHERE_CUT: f64 = 3.611_716_124_594_630_2e-3;
    let r = 0.05;
    /// One case: name, body, the rim's `(radius, station)`, and the
    /// volume delta an independent derivation says the carve makes.
    type Case = (&'static str, Body<f64>, (f64, f64), f64);
    let cases: [Case; 3] = [
        ("bowl floor", repaired(bowl(tol())), (1.0, 1.0), BOWL_FILL),
        (
            "lantern neck",
            repaired(lantern(tol())),
            (1.0, 0.0),
            -PLANE_SPHERE_CUT,
        ),
        (
            "hemisphere equator",
            repaired(hemisphere_on_flat_base()),
            (1.0, 0.0),
            -PLANE_SPHERE_CUT,
        ),
    ];
    for (name, body, rim, want) in cases {
        let arcs = rim_arcs_at(&body, rim.0, rim.1);
        let before = mass_properties(&body, tol()).unwrap().volume;
        let out = fillet_edges(&body, &arcs, r, tol())
            .unwrap_or_else(|e| panic!("{name} carves, got {e:?}"));
        let after = mass_properties(&out.body, tol()).unwrap().volume;
        let delta = after - before;
        assert!(
            (delta - want).abs() <= 1e-12 * want.abs(),
            "{name}: measured {delta} vs independent {want}"
        );
    }
}
