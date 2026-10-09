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
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Point2, Point3, Tol};
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::test_support::{assert_full_revolve_rim, bowl, lantern, revolved_about_y, rim_arcs_at};
use topo::{
    AtRestBody, Body, EdgeKey, FaceKey, FaceSurface, MefSite, MevSite, ValidationError,
    mass_properties, validate_geometric,
};

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

/// What the at-rest gate says of a body that does not finish.
fn unfinished(body: &Body<f64>) -> Vec<ValidationError> {
    AtRestBody::validate(body.clone(), tol())
        .map(drop)
        .expect_err("a body carrying scaffolding does not finish")
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

/// **A finished host pinched at a rim vertex refuses at the hostless
/// host gate.** A coplanar triangle is cut into the repaired cylinder's
/// base disc at a rim vertex (`mev`, `mev`, `mef`, each edge resting in
/// the disc's chart), so the disc's one outer cycle is both rim arcs
/// plus the triangle's path, visiting the vertex twice. That body
/// finishes, and the fillet refuses at the gate while the chamfer
/// refuses its arm. The mirrored triangle winds against the plane,
/// which tier 3 names.
#[test]
fn a_finished_pinched_host_refuses_at_the_hostless_gate() {
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

        let verdict = AtRestBody::validate(body, tol());
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
                    if detail.contains("host face carries edges outside the requested chain"))
            ),
            "the hostless host gate fires on the fillet: {fillet:?}"
        );
        let chamfer = chamfer_edges(&operand, &arcs, 0.05, tol()).map(drop);
        assert!(
            matches!(
                &chamfer,
                Err(r) if matches!(r.error, BlendError::ChamferArmUnsupported { .. })
            ),
            "the chamfer refuses its arm: {chamfer:?}"
        );
    }
}

// ------------------------------------------------------------------
// The curved single host — the `HostSide`-passed argument's shape.
// ------------------------------------------------------------------

/// **A CURVED single face carrying both arcs over a slit does not
/// finish.** The plane×sphere hemisphere's two half-caps are merged into
/// ONE sphere face by killing a seam meridian, which leaves the pole a
/// strut tip; tier 2 names it, so the operand never reaches the blend
/// door.
#[test]
fn a_curved_single_face_carrying_both_arcs_over_a_slit_does_not_finish() {
    let mut body = hemisphere_on_flat_base();
    let arcs = rim_arcs_at(&body, 1.0, 0.0);
    assert_full_revolve_rim(&arcs, "the hemisphere base");
    merge_curved_wall(&mut body, &arcs);
    assert_eq!(planar_supports(&body, &arcs).len(), 1, "one plane host");
    assert_eq!(
        curved_supports(&body, &arcs).len(),
        1,
        "ONE sphere face carries both arcs"
    );
    let errors = unfinished(&body);
    assert!(
        errors.len() == 1 && matches!(errors[0], ValidationError::ScaffoldingStrutVertex { .. }),
        "tier 2 names the slit's pole: {errors:?}"
    );
}

/// The two seam meridians of the cylinder wall the rim's arcs rest on.
fn wall_meridians(body: &Body<f64>) -> [EdgeKey; 2] {
    let found: Vec<EdgeKey> = body
        .edges()
        .map(|(e, _)| e)
        .filter(|&e| {
            let (fa, fb) = faces_of(body, e);
            fa != fb
                && !is_plane(body, fa)
                && !is_plane(body, fb)
                && body.get_face(fa).unwrap().surface == body.get_face(fb).unwrap().surface
        })
        .collect();
    found
        .try_into()
        .unwrap_or_else(|f: Vec<_>| panic!("a full revolve's wall has two meridians, got {f:?}"))
}

/// **A CURVED single face carrying both arcs is construction state.**
/// One of a cylinder wall's two seam meridians is
/// killed and the other, either one, restated as the wall's wrap edge
/// (`kef_describing`), which merges the wall into ONE face that
/// finishes: the wall touches no pole, so no strut tip is left. A wrap
/// edge sits wherever the construction cut the wall, so either meridian
/// carries the wrap flag and meters the cylinder's volume, π. The kill
/// leaves each rim two arcs of one circle meeting at the killed
/// meridian's end with nothing else there, which tier 3's check 11
/// refuses at rest: the join would take each rim back into one closed
/// edge, so no blend door meets a support carrying both arcs. The same
/// kill through plain `kef` leaves the survivor a slit as well, which
/// tier 3 refuses beside them.
#[test]
fn a_curved_single_face_carrying_both_arcs_is_construction_state() {
    for (repair, order) in [(false, 0), (false, 1), (true, 0), (true, 1)] {
        let mut body = pole_cylinder();
        if repair {
            body = repaired(body);
        }
        let arcs = rim_arcs_at(&body, 1.0, 0.0);
        assert_full_revolve_rim(&arcs, "the cylinder base");
        let meridians = wall_meridians(&body);
        let (dies, wraps) = (meridians[order], meridians[1 - order]);
        let (wall, _) = faces_of(&body, wraps);
        let wall = body.get_face(wall).unwrap().surface;
        let dying = body.get_edge(dies).unwrap().he_plus;
        // The killed meridian's two ends are left between two arcs of
        // one rim circle each: joinable vertices (tier 3's check 11).
        let mut ends = [
            body.get_half_edge(dying).unwrap().start,
            body.half_edge_end(dying).unwrap(),
        ];
        ends.sort();
        let joinable = || {
            ends.iter()
                .map(|&vertex| ValidationError::JoinableVertexAtRest { vertex })
        };

        let mut slit = body.clone();
        slit.kef(dying).expect("the meridian kills");
        assert_eq!(
            AtRestBody::validate(slit, tol()).map(drop),
            Err(
                std::iter::once(ValidationError::DescriptionNotAdjacent { edge: wraps })
                    .chain(joinable())
                    .collect()
            ),
            "repair = {repair}: plain kef leaves a slit, not at rest"
        );

        body.kef_describing(dying, &[(wraps, EdgeDescriptionSpec::wrap(wall))], tol())
            .expect("the survivor restates as the wall's wrap edge");
        assert_eq!(
            curved_supports(&body, &arcs).len(),
            1,
            "repair = {repair}: ONE cylinder face carries both arcs"
        );
        let image = body
            .get_curve_geom(body.get_edge(wraps).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap()
            .description()
            .chart()
            .expect("the wrap edge carries a chart image")
            .wrap;
        assert!(
            image,
            "repair = {repair}, order = {order}: the survivor wraps"
        );
        let volume = mass_properties(&body, tol()).unwrap().volume;
        assert!(
            (volume - core::f64::consts::PI).abs() < 1e-9,
            "repair = {repair}, order = {order}: the wall meters π, got {volume}"
        );
        // Each rim's two arcs meet at the killed meridian's end with
        // nothing else there, so the merged wall is construction state
        // (tier 3's check 11) and no blend door takes it: the join would
        // take each rim back into one closed edge first.
        assert_eq!(
            AtRestBody::validate(body, tol()).map(drop),
            Err(joinable().collect()),
            "repair = {repair}, order = {order}: the merged wall's rims are each two arcs"
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
    assert_eq!(both.len(), 4, "{what}: two rims of two arcs each");
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
    let one = mass_properties(&one_call.body, tol()).unwrap();
    assert_eq!(one.volume_pad, 0.0, "{what}: closed-form faces only");
    for order in [[rims[0], rims[1]], [rims[1], rims[0]]] {
        let mut body = source.clone();
        for (rr, ry) in order {
            let arcs = rim_arcs_at(&body, rr, ry);
            body = fillet_edges(&sweep::test_support::at_rest(&body, tol()), &arcs, r, tol())
                .map_err(|e| format!("{what}: rim ({rr}, {ry}) refused alone: {:?}", e.error))?
                .body;
        }
        validate_geometric(&body, tol())
            .map_err(|e| format!("{what}: sequential result not tier-3 valid: {e:?}"))?;
        let seq = mass_properties(&body, tol()).unwrap();
        // The same body, not the same history: each carve ends with the
        // join, which kills the edge of the vertex's first half-edge in
        // arena order and extends the other's interval over it
        // (`joinable`'s `gone` and `kept`, `joined_spec`). One call and a sequence of carves
        // reach the join with different arenas, so they can keep
        // different pieces of one rim, and the joined edge's parameter
        // interval — which the volume integrates over — differs in its
        // last bits. The census below is the identity; the volume agrees
        // to the last ulps (D9 promises bits for one history only).
        if (one.volume - seq.volume).abs() > 4.0 * f64::EPSILON * one.volume.abs() {
            return Err(format!(
                "{what}: one call {} vs sequential {order:?} {} differ past summation order",
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
        assert_eq!(arcs.len(), 2, "({r}, {y}) two arcs");
        let out = fillet_edges(&body, &arcs, 0.05, tol())
            .unwrap_or_else(|e| panic!("the ({r}, {y}) rim carves, got {e:?}"));
        validate_geometric(&out.body, tol()).expect("tier-3 valid");
    }
    // The ladder ring beside a hostless rim, one call.
    let mut both = rim_arcs_at(&body, 1.0, 0.0);
    both.extend(rim_arcs_at(&body, 0.5, 1.0));
    assert_eq!(both.len(), 4);
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
        let before = mass_properties(&body, tol()).unwrap().volume;
        let out = fillet_edges(&sweep::test_support::at_rest(&body, tol()), &arcs, r, tol())
            .unwrap_or_else(|e| panic!("{name} carves, got {e:?}"));
        let after = mass_properties(&out.body, tol()).unwrap().volume;
        let delta = after - before;
        assert!(
            (delta - want).abs() <= 1e-12 * want.abs(),
            "{name}: measured {delta} vs independent {want}"
        );
    }
}
