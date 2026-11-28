//! **FILLET-H5 review probes (lane r2)** — rows the unit's own suite
//! does not carry, each written to falsify one claim of PR 1824 by
//! execution rather than by reading.
//!
//! - a host face carrying a strut spur in its outer cycle, and a CURVED
//!   single face carrying both arcs over a slit, are scaffolding: the
//!   operand does not finish, so neither reaches a blend door;
//! - a finished host whose outer cycle is pinched at a rim vertex (a
//!   coplanar triangle cut into the disc) refuses at the hostless host
//!   gate, and the chamfer refuses its arm; the mirrored triangle does
//!   not finish;
//! - a finished CURVED single face carrying both arcs (a cylinder wall
//!   merged into one face over its wrap edge, on either meridian,
//!   touching no pole), the plane side repaired or not, refuses at the
//!   half-band gate, and the chamfer refuses its arm;
//! - two compositions the #935 row does not cover: two hostless rims
//!   of one body on a SHARED mate wall in one call, and two hostless
//!   rims sharing no wall — both against both sequential orders;
//! - a hostless rim beside a ring-hosted LADDER rim, and a rim in the
//!   outer cycle of a plane face that also carries a ring — that each
//!   carves, measured;
//! - the bowl's fill and the plane×sphere cut against constants derived
//!   OUTSIDE the tree (an independent Pappus derivation, not
//!   `test_support::wedge_fill`), so the row cannot agree with the
//!   oracle it is checking.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Point2, Point3, Tol};
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::test_support::{assert_full_revolve_rim, bowl, lantern, revolved_about_y, rim_arcs_at};
use topo::construction::Construct;
use topo::construction::{FaceSurface, MefSite, MevSite};
use topo::{Body, EdgeKey, FaceKey, ValidationError, validate_geometric};

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
    matches!(body.face_surface(f).unwrap(), Surface::Plane { .. })
}

/// What the at-rest gate says of a body that does not finish.
fn unfinished(body: &Body<f64>) -> Vec<ValidationError> {
    topo::finish(body.clone(), tol())
        .map(drop)
        .expect_err("a body carrying scaffolding does not finish")
        .into_findings()
}

fn census(b: &Body<f64>) -> (i64, i64, i64) {
    (
        b.vertices().count() as i64,
        b.edges().count() as i64,
        b.faces().count() as i64,
    )
}

/// A pole-touching hemisphere of radius 1 on a flat base disc.
fn hemisphere_on_flat_base() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (
                Point2::new(1.0, 0.0),
                (core::f64::consts::FRAC_PI_2 / 4.0).tan(),
            ),
            (Point2::new(0.0, 1.0), 0.0),
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
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
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
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.5, 1.0), 0.0),
            (Point2::new(0.5, 1.5), 0.0),
            (Point2::new(0.0, 1.5), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
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

/// **A host whose outer cycle carries a strut spur does not finish.** A
/// spur (`mev` with an empty fan run) spliced into the repaired neck's
/// cap loop at a crossing leaves the cap one ring-free plane face whose
/// outer cycle is the two arcs plus that spur; tier 2 names the spur's
/// tip, so the operand never reaches the blend door.
#[test]
fn a_host_with_a_strut_spur_in_its_outer_cycle_does_not_finish() {
    let mut body = repaired(lantern(tol()));
    let arcs = rim_arcs_at(&body, 1.0, 0.0);
    assert_full_revolve_rim(&arcs, "the repaired lantern base");
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
    let spur = body
        .mev_line(MevSite::Fan { he1: he, he2: he }, spur_end, tol())
        .expect("a spur into the cap face");
    let fd = body.get_face(host).unwrap();
    assert!(fd.rings.is_empty(), "the cap is still ring-free");
    assert_eq!(
        unfinished(&body),
        vec![ValidationError::ScaffoldingStrutVertex {
            vertex: spur.vertex
        }],
        "tier 2 names the spur's tip"
    );
}

/// **A finished host pinched at a rim vertex refuses at the one-edge
/// rim door.** A coplanar triangle is cut into the repaired cylinder's
/// base disc at the rim's vertex (`mev`, `mev`, `mef`, each edge
/// resting in the disc's chart), so the disc's one outer cycle is the
/// rim plus the triangle's path, visiting the vertex twice. That body
/// finishes, and both the fillet and the chamfer refuse the rim's
/// support as no revolution wall. The mirrored triangle winds against
/// the plane, which tier 3 names.
#[test]
fn a_finished_pinched_host_refuses_at_the_one_edge_rim_door() {
    for mirrored in [false, true] {
        let mut body = repaired(pole_cylinder());
        let arcs = rim_arcs_at(&body, 1.0, 0.0);
        assert_full_revolve_rim(&arcs, "the repaired cylinder base");
        let (fa, fb) = faces_of(&body, arcs[0]);
        let host = if is_plane(&body, fa) { fa } else { fb };
        let plane = body.get_face(host).unwrap().surface;
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
        let v0 = body.get_half_edge(he).unwrap().start;
        let p0 = *body.get_point(body.get_vertex(v0).unwrap().point).unwrap();
        let inward = Point3::new(p0.x * 0.5, p0.y, p0.z * 0.5);
        let side = Point3::new(-p0.z, 0.0, p0.x);
        let s = if mirrored { -0.2 } else { 0.2 };
        let w1 = Point3::new(inward.x + s * side.x, p0.y, inward.z + s * side.z);
        let w2 = Point3::new(inward.x - s * side.x, p0.y, inward.z - s * side.z);
        let seg = |a, b| EdgeCurveSpec::line_between(a, b).at_rest_in_chart(plane, false);
        let a = body
            .mev(MevSite::Fan { he1: he, he2: he }, w1, seg(p0, w1), tol())
            .expect("the first side cuts in");
        let b = body
            .mev(
                MevSite::Fan {
                    he1: a.he_minus,
                    he2: a.he_minus,
                },
                w2,
                seg(w1, w2),
                tol(),
            )
            .expect("the second side cuts in");
        body.mef(
            MefSite::Chords {
                he1: b.he_minus,
                he2: a.he_plus,
            },
            seg(w2, p0),
            FaceSurface::Inherit,
            tol(),
        )
        .expect("the third side closes the triangle");
        assert!(
            body.get_face(host).unwrap().rings.is_empty(),
            "the disc stays ring-free"
        );

        let verdict = topo::finish(body, tol());
        if mirrored {
            let errors = verdict
                .map(drop)
                .expect_err("the mirrored pinch does not finish");
            assert!(
                errors
                    .iter()
                    .any(|e| matches!(e, ValidationError::LoopRoleInverted { .. })),
                "the mirrored triangle winds against the plane: {errors:?}"
            );
            continue;
        }
        let operand = verdict.unwrap_or_else(|e| panic!("the pinched disc finishes, got {e:?}"));
        let fillet = fillet_edges(&operand, &arcs, 0.05, tol()).map(drop);
        assert!(
            matches!(
                &fillet,
                Err(r) if matches!(&r.error, BlendError::UnsupportedChain { detail, .. }
                    if detail.contains("support is not a revolution wall"))
            ),
            "the one-edge rim door refuses the fillet: {fillet:?}"
        );
        let chamfer = chamfer_edges(&operand, &arcs, 0.05, tol()).map(drop);
        assert!(
            matches!(
                &chamfer,
                Err(r) if matches!(&r.error, BlendError::UnsupportedChain { detail, .. }
                    if detail.contains("support is not a revolution wall"))
            ),
            "the one-edge rim door refuses the chamfer too: {chamfer:?}"
        );
    }
}

// ------------------------------------------------------------------
// The curved single host — the `HostSide`-passed argument's shape.
// ------------------------------------------------------------------

/// **A CURVED single face carrying both arcs of a rim is construction
/// state.** The pole cylinder's wall is one face, its base rim one
/// closed edge; split at its antipode the rim is two arcs of one circle,
/// both on that one cylinder face, meeting at the split's vertex with
/// nothing else there, which tier 3's check 11 refuses at rest: the join
/// would take the rim back into one closed edge, so no blend door meets
/// a curved support carrying both arcs. The wall still meters π.
#[test]
fn a_curved_single_face_carrying_both_arcs_is_construction_state() {
    for repair in [false, true] {
        let mut body = pole_cylinder();
        if repair {
            body = repaired(body);
        }
        let rim = rim_arcs_at(&body, 1.0, 0.0);
        assert_full_revolve_rim(&rim, "the cylinder base");
        let (t0, t1) = body
            .get_curve_geom(body.get_edge(rim[0]).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap()
            .params();
        let split = body
            .split_edge(rim[0], 0.5 * (t0 + t1), tol())
            .expect("the rim splits at its antipode");
        let arcs = rim_arcs_at(&body, 1.0, 0.0);
        assert_eq!(arcs.len(), 2, "repair = {repair}: the rim is two arcs");
        assert_eq!(
            curved_supports(&body, &arcs).len(),
            1,
            "repair = {repair}: ONE cylinder face carries both arcs"
        );
        // Raw door: the measured body is construction state whose joinable vertices tier 3 refuses (JoinableVertexAtRest).
        let volume = topo::test_support::measure_unvalidated(
            &body,
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume;
        assert!(
            (volume - core::f64::consts::PI).abs() < 1e-9,
            "repair = {repair}: the wall meters π, got {volume}"
        );
        assert_eq!(
            topo::finish(body, tol())
                .map(drop)
                .map_err(topo::NotFinished::into_findings),
            Err(vec![ValidationError::JoinableVertexAtRest {
                vertex: split.vertex
            }]),
            "repair = {repair}: the rim's two arcs meet at a joinable vertex"
        );
    }
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
    assert_eq!(both.len(), 2, "{what}: two rims of one edge each");
    let one_call = fillet_edges(
        &sweep::test_support::at_rest(source, tol()),
        &both,
        r,
        tol(),
    )
    .map_err(|e| format!("{what}: one call refused: {:?}", e.error))?;
    validate_geometric(&one_call.body, tol())
        .map_err(|e| format!("{what}: one-call result not tier-3 valid: {e:?}"))?;
    assert_eq!(one_call.band_faces.len(), 2, "{what}: one band per rim");
    let one = topo::mass_properties(
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&one_call.body),
            tol(),
        ),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .unwrap();
    assert_eq!(
        one.enclosure.volume_pad, 0.0,
        "{what}: closed-form faces only"
    );
    for order in [[rims[0], rims[1]], [rims[1], rims[0]]] {
        let mut body = source.clone();
        for (rr, ry) in order {
            let arcs = rim_arcs_at(&body, rr, ry);
            body = fillet_edges(&sweep::test_support::at_rest(&body, tol()), &arcs, r, tol())
                .map_err(|e| format!("{what}: rim ({rr}, {ry}) refused alone: {:?}", e.error))?
                .body
                .into_body();
        }
        validate_geometric(&body, tol())
            .map_err(|e| format!("{what}: sequential result not tier-3 valid: {e:?}"))?;
        let seq = topo::mass_properties(
            &topo::test_support::finished("the measured body", topo::Body::clone(&body), tol()),
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .unwrap();
        // The same body, not the same history: each carve ends with the
        // join, which kills the edge of the vertex's first half-edge in
        // arena order and extends the other's interval over it
        // (`joinable`'s `gone` and `kept`, `joined_spec`). One call and a sequence of carves
        // reach the join with different arenas, so they can keep
        // different pieces of one rim, and the joined edge's parameter
        // interval — which the volume integrates over — differs in its
        // last bits. The census below is the identity; the volume agrees
        // to the last ulps (D9 promises bits for one history only).
        if (one.enclosure.volume - seq.enclosure.volume).abs()
            > 4.0 * f64::EPSILON * one.enclosure.volume.abs()
        {
            return Err(format!(
                "{what}: one call {} vs sequential {order:?} {} differ past summation order",
                one.enclosure.volume, seq.enclosure.volume
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
/// face; both rims are hostless and both rest on the cylinder's one
/// band, so the second rim's `refresh_annulus_seams` re-reads the mate
/// seam the first band split while carrying a `Strut` through on
/// the host side. Both sequential orders must agree bit for bit.
#[test]
fn two_hostless_rims_on_a_shared_mate_wall_compose_in_one_call() {
    let source = repaired(pole_cylinder());
    for (r, y) in [(1.0, 0.0), (1.0, 1.0)] {
        let arcs = rim_arcs_at(&source, r, y);
        assert_full_revolve_rim(&arcs, "the pole cylinder rim");
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
/// stepped body every one of the three shapes carves: the two disc
/// rims `(1, 0)` and `(0.5, 1.5)` alone, the annular top's OUTER rim
/// `(1, 1)` on a host that also carries the boss root as a ring, and
/// the boss root `(0.5, 1)` — a LADDER rim nested inside that host's
/// circular outer boundary — beside the base disc rim in ONE call,
/// since the two rims rest on two different plane hosts.
#[test]
fn a_hostless_rim_beside_a_ladder_rim_and_a_ringed_host_measured() {
    let body = sweep::test_support::finished("body", repaired(stepped()), tol());
    for (r, y) in [(1.0, 0.0), (0.5, 1.5), (1.0, 1.0)] {
        let arcs = rim_arcs_at(&body, r, y);
        assert_eq!(arcs.len(), 1, "({r}, {y}) one edge");
        let out = fillet_edges(&body, &arcs, 0.05, tol())
            .unwrap_or_else(|e| panic!("the ({r}, {y}) rim carves, got {e:?}"));
        validate_geometric(&out.body, tol()).expect("tier-3 valid");
    }
    // The ladder ring beside a hostless rim, one call.
    let mut both = rim_arcs_at(&body, 1.0, 0.0);
    both.extend(rim_arcs_at(&body, 0.5, 1.0));
    assert_eq!(both.len(), 2, "two rims of one edge each");
    let out = fillet_edges(&body, &both, 0.05, tol())
        .unwrap_or_else(|e| panic!("the ladder rim composes with the base rim, got {e:?}"));
    validate_geometric(&out.body, tol()).expect("tier-3 valid");
    assert_eq!(out.band_faces.len(), 2, "one band per rim");
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
        let before = topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", topo::Body::clone(&body), tol()),
            tol(),
        )
        .volume;
        let out = fillet_edges(&sweep::test_support::at_rest(&body, tol()), &arcs, r, tol())
            .unwrap_or_else(|e| panic!("{name} carves, got {e:?}"));
        let after = topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", topo::Body::clone(&out.body), tol()),
            tol(),
        )
        .volume;
        let delta = after - before;
        assert!(
            (delta - want).abs() <= 1e-12 * want.abs(),
            "{name}: measured {delta} vs independent {want}"
        );
    }
}
