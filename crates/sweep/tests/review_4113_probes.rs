//! Review probes for PR 4113 (a blend carves each chain inside its own
//! shell). Each row prints what it measured; the asserts pin the
//! reviewer's reading of the head `fda9974d`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::PI;
use std::fmt::Write as _;
use std::time::Instant;

use crate::common::cavity::{brick, edges_with_corners, rod};
use crate::common::oracles::rounded_box_volume;
use geom_core::{Point2, Point3, Tol};
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::finished;
use topo::{AtRestBody, Body, EdgeKey, EntityId, ShellKey};

fn tol() -> Tol {
    Tol::witness()
}

fn op(kind: &str, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let a = finished("a", a.clone(), tol());
    let b = finished("b", b.clone(), tol());
    let r = match kind {
        "union" => topo::union(&a, &b, tol()),
        _ => topo::subtract(&a, &b, tol()),
    };
    r.unwrap_or_else(|e| panic!("{kind}: {e:?}"))
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

/// **Every byte of a shell**: each face (sorted by key), its surface,
/// its provenance, and each loop's half-edges with their edge, curve,
/// start vertex and point, pcurve row and joint element.
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

fn tier3p(what: &str, body: &Body<f64>) -> Result<(), Vec<topo::ValidationError>> {
    let t = Instant::now();
    let r = topo::validate_pseudomanifold(body, &topo::boolean::ContactRecords::default(), tol());
    eprintln!(
        "[tier3' {what}] {:?} in {:?} ({} faces)",
        r.as_ref().map_err(|e| {
            let mut k: Vec<String> = e
                .iter()
                .map(|x| {
                    format!("{x:?}")
                        .split([' ', '('])
                        .next()
                        .unwrap()
                        .to_owned()
                })
                .collect();
            k.dedup();
            (e.len(), k)
        }),
        t.elapsed(),
        body.faces().count()
    );
    r
}

fn two_boxes() -> Body<f64> {
    op(
        "union",
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        &brick(Point3::new(2.0, 0.0, 0.0), Point3::new(3.0, 1.0, 1.0)),
    )
}

/// Claim 1, strict: a carried shell is byte-identical (faces, loops,
/// half-edges, edges, curves, surfaces, points, pcurves, joints,
/// provenance) — a planar box, a curved rod, and a box filleted in an
/// earlier step (so it carries cylinders, spheres and pcurve rows).
#[test]
fn carried_shells_are_byte_identical() {
    // (a) two planar boxes.
    let body = two_boxes();
    let a = edges_with_corners(&body, |p| p.x < 1.5);
    let b = edges_with_corners(&body, |p| p.x > 1.5);
    let (sa, sb) = (shell_of_edge(&body, a[0]), shell_of_edge(&body, b[0]));
    let out = fillet_edges(&body, &a, 0.1, tol()).unwrap();
    assert_eq!(snapshot(&body, sb), snapshot(&out.body, sb), "planar box b");

    // (b) the filleted box a carried while box b fillets: a curved,
    // pcurve-carrying shell re-minted by the whole-body pass.
    let pc_a = out.body.pcurves().count();
    let out2 = fillet_edges(&out.body, &b, 0.1, tol()).unwrap();
    assert!(pc_a > 0, "the filleted shell carries pcurve rows");
    assert_eq!(
        snapshot(&out.body, sa),
        snapshot(&out2.body, sa),
        "a filleted box carried through a second blend"
    );
    validate("second blend", &out2.body);
    let want = 2.0 * rounded_box_volume(0.8, 0.1);
    let got = volume(&out2.body);
    eprintln!("[sequential] V = {got}, closed form {want}");
    assert!((got - want).abs() <= 1e-12 * want);

    // (c) a box beside a rod: the rod's cylinder rides through.
    let body = op(
        "union",
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        &rod(Point2::new(3.0, 0.5), 0.4, 0.0, 1.0),
    );
    let a = edges_with_corners(&body, |p| p.x < 1.5);
    assert_eq!(a.len(), 12);
    let rod_edge = edges_with_corners(&body, |p| p.x > 1.5)[0];
    let sr = shell_of_edge(&body, rod_edge);
    let out = chamfer_edges(&body, &a, 0.1, tol()).unwrap();
    assert_eq!(snapshot(&body, sr), snapshot(&out.body, sr), "rod");
}

/// Claim 2: the per-shell result equals blending that shell alone and
/// recombining — volume and the multiset of new surfaces.
#[test]
fn per_shell_equals_blend_alone_then_union() {
    let lo = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0));
    let hi = brick(Point3::new(2.0, 0.0, 0.0), Point3::new(3.0, 1.0, 1.0));
    let body = two_boxes();
    let a = edges_with_corners(&body, |p| p.x < 1.5);
    let together = fillet_edges(&body, &a, 0.1, tol()).unwrap();
    let alone_a = fillet_edges(&lo, &topo::query::all_edges(&lo), 0.1, tol()).unwrap();
    let recombined = op("union", &alone_a.body, &hi);
    let (v1, v2) = (volume(&together.body), volume(&recombined));
    eprintln!("[claim 2] per-shell V = {v1}, alone+union V = {v2}");
    assert!((v1 - v2).abs() <= 1e-12 * v1);
    let surfs = |b: &Body<f64>| {
        let mut v: Vec<String> = b
            .faces()
            .map(|(_, f)| format!("{:?}", b.get_surface(f.surface)))
            .collect();
        v.sort();
        v
    };
    assert_eq!(surfs(&together.body), surfs(&recombined), "same carriers");
}

/// Claim 4: a request spanning two shells that fails in one names an
/// entity of that shell, and of no other.
#[test]
fn a_refusal_in_one_shell_names_that_shell() {
    let body = op(
        "union",
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        &brick(Point3::new(2.0, 0.0, 0.0), Point3::new(2.15, 0.15, 0.15)),
    );
    let a = edges_with_corners(&body, |p| p.x < 1.5);
    let b = edges_with_corners(&body, |p| p.x > 1.5);
    let mut both = a.clone();
    both.extend(&b);
    for (what, req) in [("a then b", both.clone()), ("b only", b.clone())] {
        let err = fillet_edges(&body, &req, 0.1, tol()).expect_err("box b is too small");
        let dbg = format!("{:?}", err.error);
        let named = |es: &[EdgeKey]| es.iter().any(|e| dbg.contains(&format!("{e:?}")));
        eprintln!("[claim 4 {what}] DBG {dbg}");
        assert!(!named(&a), "{what}: names no edge of box a");
        let sb = shell_of_edge(&body, b[0]);
        match err.error {
            sweep::blend::BlendError::FaceClearanceUncertified { face, .. } => {
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

/// A SEALED cylindrical void: its two rims are concave closed chains
/// whose supports carry `sense = false` (a void shell). Fillet them;
/// the void shrinks by two spandrel tori (Pappus).
#[test]
fn a_sealed_cylindrical_void_fillets_its_rims() {
    let (a, h, r) = (0.5, 2.0, 0.2);
    let body = op(
        "subtract",
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0)),
        &rod(Point2::new(2.0, 2.0), a, 1.0, 1.0 + h),
    );
    assert_eq!(body.shells().count(), 2);
    let mut rims = edges_with_corners(&body, |p| (p.z - 1.0).abs() < 1e-9);
    rims.extend(edges_with_corners(&body, |p| (p.z - 1.0 - h).abs() < 1e-9));
    eprintln!("[cyl void] {} rim edges", rims.len());
    let outer = edges_with_corners(&body, |p| p.x.abs() < 1e-9 || p.x > 3.99);
    let so = shell_of_edge(&body, outer[0]);
    let out = fillet_edges(&body, &rims, r, tol());
    match out {
        Ok(out) => {
            assert_eq!(snapshot(&body, so), snapshot(&out.body, so), "outer");
            validate("cyl void", &out.body);
            // Spandrel: area r²(1 − π/4), centroid (10 − 3π) r / (12 − 3π)
            // off the wall.
            let area = r * r * (1.0 - PI / 4.0);
            let rho = a - (10.0 - 3.0 * PI) * r / (12.0 - 3.0 * PI);
            let want = 64.0 - PI * a * a * h + 2.0 * 2.0 * PI * rho * area;
            let got = volume(&out.body);
            eprintln!("[cyl void] V = {got}, closed form {want}");
            assert!((got - want).abs() <= 1e-9 * want);
        }
        Err(e) => eprintln!("[cyl void] refuses: {e}"),
    }
}

/// Claim 6: tier 3′ as a backstop on multi-shell results — its verdict
/// and its cost beside the blend's own.
#[test]
fn tier3_prime_on_multi_shell_results() {
    // (1) two filleted boxes a unit apart: valid.
    let body = two_boxes();
    let all = topo::query::all_edges(&body);
    let t = Instant::now();
    let out = fillet_edges(&body, &all, 0.1, tol()).unwrap();
    eprintln!("[blend two boxes] {:?}", t.elapsed());
    assert!(tier3p("two filleted boxes", &out.body).is_ok());

    // (2) a VALID filleted part with a second solid inside its reach: an
    // island box in a vented cavity, the block's top outer edges
    // filleted far from it.
    let cavity = crate::common::cavity::vented_cavity();
    let island = brick(Point3::new(1.5, 1.5, 1.2), Point3::new(2.5, 2.5, 1.8));
    let body = op("union", &cavity, &island);
    eprintln!(
        "[island] solids {} shells {}",
        body.solids().count(),
        body.shells().count()
    );
    let _ = tier3p("island, unblended", &body);
    let top = edges_with_corners(&body, |p| {
        p.to_array()
            .iter()
            .all(|c| c.abs() < 1e-9 || (c - 4.0).abs() < 1e-9)
    });
    eprintln!("[island] {} top edges", top.len());
    let t = Instant::now();
    let out = fillet_edges(&body, &top, 0.1, tol()).unwrap();
    eprintln!("[blend island top] {:?}", t.elapsed());
    validate("island top", &out.body);
    let _ = tier3p("island, top filleted (valid)", &out.body);

    // (3) the filed witness: sealed cavity + island solid, void filleted
    // into the island (an overlap).
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let sealed = op(
        "subtract",
        &block,
        &brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0)),
    );
    let body = op(
        "union",
        &sealed,
        &brick(Point3::new(1.05, 1.05, 1.05), Point3::new(2.95, 2.95, 2.95)),
    );
    let void = edges_with_corners(&body, |p| {
        p.to_array().iter().all(|c| (0.99..=3.01).contains(c))
            && p.to_array()
                .iter()
                .all(|c| (c - 1.0).abs() < 1e-9 || (c - 3.0).abs() < 1e-9)
    });
    let t = Instant::now();
    let out = fillet_edges(&body, &void, 0.25, tol()).unwrap();
    eprintln!(
        "[blend sealed+island] {:?} ({} edges)",
        t.elapsed(),
        void.len()
    );
    assert!(tier3p("sealed + island, void filleted (overlap)", &out.body).is_err());

    // (4) same, the island ALSO filleted at 0.25 in one request: it
    // shrinks away from the band, so no overlap where the box corners
    // were; is the bodies' overlap still there and still caught?
    let island_edges = edges_with_corners(&body, |p| {
        p.to_array()
            .iter()
            .all(|c| (c - 1.05).abs() < 1e-9 || (c - 2.95).abs() < 1e-9)
    });
    let mut req = void.clone();
    req.extend(&island_edges);
    let t = Instant::now();
    match fillet_edges(&body, &req, 0.25, tol()) {
        Ok(out) => {
            eprintln!("[blend both] {:?}", t.elapsed());
            validate("both", &out.body);
            eprintln!("[both] V = {}", volume(&out.body));
            let _ = tier3p("sealed + island, both filleted", &out.body);
        }
        Err(e) => eprintln!("[both] refuses: {e}"),
    }
}

/// The outer shell of a sealed cavity filleted (convex), the VOID shell
/// carried byte-identical; and one solid of TWO OUTER shells (tier-3
/// invalid, `SolidOuterShells`) — what the door does with it now that
/// the inventory gate is gone.
#[test]
fn outer_blend_carries_the_void_and_a_merged_solid_is_carved() {
    let body = op(
        "subtract",
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0)),
        &brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0)),
    );
    let corner = |p: Point3<f64>, lo: f64, hi: f64| {
        p.to_array()
            .iter()
            .all(|c| (c - lo).abs() < 1e-9 || (c - hi).abs() < 1e-9)
    };
    let outer = edges_with_corners(&body, |p| corner(p, 0.0, 4.0));
    let void = edges_with_corners(&body, |p| corner(p, 1.0, 3.0));
    let sv = shell_of_edge(&body, void[0]);
    let out = fillet_edges(&body, &outer, 0.25, tol()).unwrap();
    assert_eq!(snapshot(&body, sv), snapshot(&out.body, sv), "void carried");
    validate("outer filleted", &out.body);
    let want = 64.0 - (64.0 - rounded_box_volume(3.5, 0.25)) - 8.0;
    let got = volume(&out.body);
    eprintln!("[outer] V = {got}, closed form {want}");
    assert!((got - want).abs() <= 1e-12 * want);

    let merged = two_boxes().with_solids_merged_for_tests();
    assert_eq!((merged.solids().count(), merged.shells().count()), (1, 2));
    eprintln!(
        "[merged] tier 3 on the input: {:?}",
        AtRestBody::validate(merged.clone(), tol())
            .err()
            .map(|e| format!("{e:?}").chars().take(200).collect::<String>())
    );
    let a = edges_with_corners(&merged, |p| p.x < 1.5);
    match fillet_edges(&merged, &a, 0.1, tol()) {
        Ok(out) => eprintln!(
            "[merged] BUILDS: solids {} shells {}, tier 3 on the result: {:?}",
            out.body.solids().count(),
            out.body.shells().count(),
            AtRestBody::validate(out.body.clone(), tol())
                .err()
                .map(|e| format!("{e:?}").chars().take(200).collect::<String>())
        ),
        Err(e) => eprintln!("[merged] refuses: {e}"),
    }
}
