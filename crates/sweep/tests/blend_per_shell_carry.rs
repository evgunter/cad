//! **A shell a blend is not asked about comes out byte for byte the
//! shell that went in**, and the shells it is asked about carve as they
//! would alone: each row snapshots the carried shell's every record
//! (faces, surfaces, provenance, loops, half-edges, edges, curves,
//! points, pcurve rows, joints) before and after, or measures the
//! carved one against a closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::PI;
use std::fmt::Write as _;

use crate::common::cavity::{brick, edges_with_corners, rod, vented_cavity};
use crate::common::oracles::rounded_box_volume;
use geom_core::{Point2, Point3, Tol};
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::finished;
use topo::{AtRestBody, Body, EdgeKey, EntityId, ShellKey};

fn tol() -> Tol {
    Tol::witness()
}

fn union(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    combine(a, b, true)
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    combine(a, b, false)
}

fn combine(a: &Body<f64>, b: &Body<f64>, union: bool) -> Body<f64> {
    let a = finished("a", a.clone(), tol());
    let b = finished("b", b.clone(), tol());
    let r = if union {
        topo::union(&a, &b, tol())
    } else {
        topo::subtract(&a, &b, tol())
    };
    r.unwrap_or_else(|e| panic!("{e:?}"))
        .body()
        .expect("material")
        .body
        .clone()
        .into_body()
}

fn shell_of_edge(body: &Body<f64>, e: EdgeKey) -> ShellKey {
    let (f, _) = topo::readback::edge_sides(body, e).unwrap().faces();
    body.get_face(f).unwrap().shell
}

/// Every record of `shell`, in key order: each face with its surface
/// and provenance, and each loop's half-edges with their edge, curve,
/// start vertex and point, pcurve row, joint and edge provenance.
fn snapshot(body: &Body<f64>, shell: ShellKey) -> String {
    let mut s = String::new();
    let sh = body.get_shell(shell).unwrap();
    writeln!(
        s,
        "shell {shell:?} {sh:?} solid {:?}",
        body.get_solid(sh.solid)
    )
    .unwrap();
    let mut faces = sh.faces.clone();
    faces.sort_unstable();
    for f in faces {
        let face = body.get_face(f).unwrap();
        writeln!(
            s,
            "face {f:?} {face:?} surf {:?} prov {:?}",
            body.get_surface(face.surface),
            body.provenance(EntityId::Face(f))
        )
        .unwrap();
        for lp in std::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let l = body.get_loop(lp).unwrap();
            writeln!(s, " loop {lp:?} {l:?}").unwrap();
            let topo::LoopBoundary::Cycle { first: start } = l.boundary else {
                continue;
            };
            let mut he = start;
            loop {
                let h = body.get_half_edge(he).unwrap();
                let e = body.get_edge(h.edge).unwrap();
                let v = body.get_vertex(h.start).unwrap();
                writeln!(
                    s,
                    "  he {he:?} {h:?} edge {e:?} curve {:?} v {v:?} p {:?} pc {:?} j {:?} eprov {:?}",
                    body.get_curve_geom(e.curve),
                    body.get_point(v.point),
                    body.pcurve(he),
                    body.joint(he),
                    body.provenance(EntityId::Edge(h.edge)),
                )
                .unwrap();
                he = h.next;
                if he == start {
                    break;
                }
            }
        }
    }
    s
}

fn validate(what: &str, body: &Body<f64>) {
    AtRestBody::validate(body.clone(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3 refuses: {e:?}"));
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, tol()).unwrap().volume
}

fn close(what: &str, got: f64, want: f64, rel: f64) {
    assert!(
        (got - want).abs() <= rel * want,
        "{what}: V = {got}, closed form {want}"
    );
}

/// All edges whose both ends have every coordinate at `lo` or `hi`: a
/// box's twelve.
fn box_edges(body: &Body<f64>, lo: f64, hi: f64) -> Vec<EdgeKey> {
    edges_with_corners(body, |p| {
        p.to_array()
            .iter()
            .all(|c| (c - lo).abs() < 1e-9 || (c - hi).abs() < 1e-9)
    })
}

fn two_boxes() -> Body<f64> {
    union(
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        &brick(Point3::new(2.0, 0.0, 0.0), Point3::new(3.0, 1.0, 1.0)),
    )
}

/// A planar box, a box already filleted (cylinders, spheres, pcurve
/// rows, which the whole-body pcurve pass re-mints) and a curved rod
/// each ride through another shell's blend byte for byte.
#[test]
fn a_carried_shell_is_byte_identical_down_to_its_pcurves() {
    let body = two_boxes();
    let a = edges_with_corners(&body, |p| p.x < 1.5);
    let b = edges_with_corners(&body, |p| p.x > 1.5);
    let (sa, sb) = (shell_of_edge(&body, a[0]), shell_of_edge(&body, b[0]));
    let out = fillet_edges(&sweep::test_support::at_rest(&body, tol()), &a, 0.1, tol()).unwrap();
    assert_eq!(snapshot(&body, sb), snapshot(&out.body, sb), "planar box b");

    assert!(
        out.body.pcurves().count() > 0,
        "the filleted shell carries pcurve rows"
    );
    let out2 = fillet_edges(
        &sweep::test_support::at_rest(&out.body, tol()),
        &b,
        0.1,
        tol(),
    )
    .unwrap();
    assert_eq!(
        snapshot(&out.body, sa),
        snapshot(&out2.body, sa),
        "a filleted box carried through a second blend"
    );
    validate("second blend", &out2.body);
    close(
        "both boxes, one at a time",
        volume(&out2.body),
        2.0 * rounded_box_volume(0.8, 0.1),
        1e-12,
    );

    let body = union(
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        &rod(Point2::new(3.0, 0.5), 0.4, 0.0, 1.0),
    );
    let a = edges_with_corners(&body, |p| p.x < 1.5);
    assert_eq!(a.len(), 12);
    let sr = shell_of_edge(&body, edges_with_corners(&body, |p| p.x > 1.5)[0]);
    let out = chamfer_edges(&sweep::test_support::at_rest(&body, tol()), &a, 0.1, tol()).unwrap();
    assert_eq!(snapshot(&body, sr), snapshot(&out.body, sr), "rod");
}

/// Filleting one box of a union equals filleting that box alone and
/// unioning: the same volume and the same multiset of carriers.
#[test]
fn a_shell_carved_in_place_matches_it_carved_alone_then_unioned() {
    let lo = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0));
    let hi = brick(Point3::new(2.0, 0.0, 0.0), Point3::new(3.0, 1.0, 1.0));
    let body = two_boxes();
    let a = edges_with_corners(&body, |p| p.x < 1.5);
    let together =
        fillet_edges(&sweep::test_support::at_rest(&body, tol()), &a, 0.1, tol()).unwrap();
    let alone = fillet_edges(
        &sweep::test_support::at_rest(&lo, tol()),
        &topo::query::all_edges(&lo),
        0.1,
        tol(),
    )
    .unwrap();
    let recombined = union(&alone.body, &hi);
    close(
        "in place vs alone",
        volume(&together.body),
        volume(&recombined),
        1e-12,
    );
    let surfaces = |b: &Body<f64>| {
        let mut v: Vec<String> = b
            .faces()
            .map(|(_, f)| format!("{:?}", b.get_surface(f.surface)))
            .collect();
        v.sort();
        v
    };
    assert_eq!(
        surfaces(&together.body),
        surfaces(&recombined),
        "same carriers"
    );
}

/// A request that fails in one shell names a face of that shell, and
/// no edge of the other, whether or not the other shell is requested
/// too: box b is too small for the radius.
#[test]
fn a_refusal_in_one_shell_names_a_face_of_that_shell() {
    let body = sweep::test_support::finished(
        "body",
        union(
            &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
            &brick(Point3::new(2.0, 0.0, 0.0), Point3::new(2.15, 0.15, 0.15)),
        ),
        tol(),
    );
    let a = edges_with_corners(&body, |p| p.x < 1.5);
    let b = edges_with_corners(&body, |p| p.x > 1.5);
    let sb = shell_of_edge(&body, b[0]);
    let both: Vec<EdgeKey> = a.iter().chain(&b).copied().collect();
    for (what, req) in [("a and b", both), ("b only", b)] {
        let err = fillet_edges(&body, &req, 0.1, tol()).expect_err("box b is too small");
        let dbg = format!("{:?}", err.error);
        assert!(
            !a.iter().any(|e| dbg.contains(&format!("{e:?}"))),
            "{what}: names an edge of box a: {dbg}"
        );
        match err.error {
            BlendError::FaceClearanceUncertified { face, .. } => {
                assert_eq!(
                    body.get_face(face).unwrap().shell,
                    sb,
                    "{what}: names box b's face"
                );
            }
            other => panic!("{what}: {other:?}"),
        }
    }
}

/// A sealed cylindrical void's two rims are concave closed chains whose
/// supports face into the void. Filleted, the void shrinks by two
/// spandrel tori (Pappus), and the outer shell rides through.
#[test]
fn a_sealed_cylindrical_void_fillets_its_rims_at_the_pappus_form() {
    let (a, h, r) = (0.5, 2.0, 0.2);
    let body = subtract(
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0)),
        &rod(Point2::new(2.0, 2.0), a, 1.0, 1.0 + h),
    );
    assert_eq!(body.shells().count(), 2);
    let mut rims = edges_with_corners(&body, |p| (p.z - 1.0).abs() < 1e-9);
    rims.extend(edges_with_corners(&body, |p| (p.z - 1.0 - h).abs() < 1e-9));
    let outer = edges_with_corners(&body, |p| p.x.abs() < 1e-9 || p.x > 3.99);
    let so = shell_of_edge(&body, outer[0]);
    let out = fillet_edges(&sweep::test_support::at_rest(&body, tol()), &rims, r, tol()).unwrap();
    assert_eq!(snapshot(&body, so), snapshot(&out.body, so), "outer");
    validate("cylindrical void", &out.body);
    // The spandrel's area is r²(1 − π/4), its centroid
    // (10 − 3π) r / (12 − 3π) off the wall.
    let area = r * r * (1.0 - PI / 4.0);
    let rho = a - (10.0 - 3.0 * PI) * r / (12.0 - 3.0 * PI);
    let want = 64.0 - PI * a * a * h + 2.0 * 2.0 * PI * rho * area;
    close("cylindrical void", volume(&out.body), want, 1e-9);
}

/// A sealed cavity's outer twelve edges fillet with its void shell
/// carried byte for byte.
#[test]
fn an_outer_blend_carries_the_void_shell() {
    let body = subtract(
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0)),
        &brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0)),
    );
    let sv = shell_of_edge(&body, box_edges(&body, 1.0, 3.0)[0]);
    let out = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &box_edges(&body, 0.0, 4.0),
        0.25,
        tol(),
    )
    .unwrap();
    assert_eq!(snapshot(&body, sv), snapshot(&out.body, sv), "void carried");
    validate("outer filleted", &out.body);
    close(
        "outer filleted",
        volume(&out.body),
        rounded_box_volume(3.5, 0.25) - 8.0,
        1e-12,
    );
}

/// Valid bodies with a second solid inside a blend's reach build, and
/// at their closed forms: an island box in a vented cavity under a
/// fillet of the block's outer edges, and an island in a sealed cavity
/// filleted in one request with the void, its convex round receding
/// from the void's concave one. Whatever check keeps a band clear of
/// faces that are not its supports must still admit both.
#[test]
fn a_blend_beside_another_solid_builds_when_it_clears_it() {
    let island = brick(Point3::new(1.5, 1.5, 1.2), Point3::new(2.5, 2.5, 1.8));
    let body = union(&vented_cavity(), &island);
    let top = edges_with_corners(&body, |p| {
        p.to_array()
            .iter()
            .all(|c| c.abs() < 1e-9 || (c - 4.0).abs() < 1e-9)
    });
    let before = volume(&body);
    let out = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &top,
        0.1,
        tol(),
    )
    .unwrap();
    validate("vented, outer edges filleted", &out.body);
    close(
        "vented, outer edges filleted",
        volume(&out.body),
        before - (64.0 - rounded_box_volume(3.8, 0.1)),
        1e-12,
    );

    let sealed = subtract(
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0)),
        &brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0)),
    );
    let body = union(
        &sealed,
        &brick(Point3::new(1.05, 1.05, 1.05), Point3::new(2.95, 2.95, 2.95)),
    );
    let mut req = box_edges(&body, 1.0, 3.0);
    req.extend(box_edges(&body, 1.05, 2.95));
    let out = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &req,
        0.25,
        tol(),
    )
    .unwrap();
    validate("void and island filleted", &out.body);
    let want = 64.0 - rounded_box_volume(1.5, 0.25) + rounded_box_volume(1.4, 0.25);
    close("void and island filleted", volume(&out.body), want, 1e-12);
}

/// An empty request carves nothing and carries everything, whatever
/// the body holds: two boxes come back shell for shell, and an empty
/// body comes back empty rather than as a surgery invariant.
#[test]
fn an_empty_request_returns_the_body_whatever_it_holds() {
    let empty = fillet_edges(
        &sweep::test_support::at_rest(&Body::<f64>::new(), tol()),
        &[],
        0.1,
        tol(),
    )
    .expect("nothing to carve");
    assert_eq!(empty.body.faces().count(), 0);
    assert!(empty.shells.is_empty());

    let body = two_boxes();
    let out = fillet_edges(&sweep::test_support::at_rest(&body, tol()), &[], 0.1, tol())
        .expect("nothing to carve");
    assert!(out.shells.is_empty());
    for (shell, _) in body.shells() {
        assert_eq!(snapshot(&body, shell), snapshot(&out.body, shell));
    }
}
