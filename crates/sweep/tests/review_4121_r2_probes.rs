//! Review probes (band-dual-4121-r2) for PR 4121: the planar cut-off.
//! Each row PRINTS its outcome (run with `--no-capture`) and asserts
//! only that nothing builds unsound or refuses untyped.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::blend::BlendError;
use sweep::blend::build::{Blended, fillet_edges};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{assert_naming_totality, block, prism, prism_on, sketch_from_axes};
use topo::{Body, EdgeKey, mass_properties, query, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, tol()).expect("props").volume
}

fn edge(body: &Body<f64>, a: [f64; 3], b: [f64; 3]) -> EdgeKey {
    let (a, b) = (Point3::new(a[0], a[1], a[2]), Point3::new(b[0], b[1], b[2]));
    let at = |p: Point3<f64>, q: Point3<f64>| (p - q).norm() < 1e-9;
    let pt = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let s = pt(body.get_half_edge(he).unwrap().start);
            let t = pt(body.half_edge_end(he).unwrap());
            (at(s, a) && at(t, b)) || (at(s, b) && at(t, a))
        })
        .unwrap_or_else(|| panic!("an edge between {a:?} and {b:?}"))
}

#[derive(Clone, Copy, Debug)]
enum Verb {
    Chamfer,
    Fillet,
}

fn run(
    verb: Verb,
    body: &Body<f64>,
    edges: &[EdgeKey],
    d: f64,
) -> Result<Blended<f64>, BlendError> {
    match verb {
        Verb::Chamfer => chamfer_edges(body, edges, d, tol()),
        Verb::Fillet => fillet_edges(body, edges, d, tol()),
    }
    .map_err(|r| r.error)
}

/// Run, and report: a build is checked tier 3 + Euler + naming and its
/// ΔV printed beside `expect` (when given); a refusal must be typed.
fn report(
    what: &str,
    verb: Verb,
    body: &Body<f64>,
    edges: &[EdgeKey],
    d: f64,
    expect: Option<f64>,
) -> Option<f64> {
    match run(verb, body, edges, d) {
        Ok(out) => {
            let t3 = validate_geometric(&out.body, tol());
            let c = topo::readback::euler_counts(&out.body);
            let dv = volume(body) - volume(&out.body);
            println!(
                "R2 {what} {verb:?} d={d}: BUILT tier3={:?} S={} genus={:?} dV={dv:.17e} expect={expect:?} err={:?}",
                t3.as_ref().map(|_| ()),
                c.s,
                c.genus(),
                expect.map(|e| dv - e)
            );
            assert!(t3.is_ok(), "{what}: built but not tier 3: {t3:?}");
            assert_naming_totality(body, &out, edges, what);
            Some(dv)
        }
        Err(e) => {
            println!("R2 {what} {verb:?} d={d}: REFUSED {e:?}");
            assert!(
                !matches!(
                    e,
                    BlendError::SurgeryInvariant { .. } | BlendError::BodyNotIntact { .. }
                ),
                "{what}: refused untyped: {e:?}"
            );
            None
        }
    }
}

/// Two parallel edges of a narrow top whose strips overlap (w < 2d),
/// and one edge whose trimline falls below a short side face (h < d).
#[test]
fn r2_overlapping_strips_and_short_faces() {
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let b = block(2.0, 0.15, 1.0, tol());
        let f = edge(&b, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        let k = edge(&b, [0.0, 0.15, 1.0], [2.0, 0.15, 1.0]);
        report("narrow-top-both", verb, &b, &[f, k], 0.1, None);
        let b = block(2.0, 1.5, 0.08, tol());
        let f = edge(&b, [0.0, 0.0, 0.08], [2.0, 0.0, 0.08]);
        report("short-front", verb, &b, &[f], 0.1, None);
    }
}

/// A notch in the top face's outline reaching to y = 0.05 of the front
/// edge: a strip of d = 0.1 crosses it (must refuse), d = 0.04 clears it.
#[test]
fn r2_strip_crossing_a_notch() {
    let b = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(3.0, 0.0), 0.0),
            (Point2::new(3.0, 1.0), 0.0),
            (Point2::new(2.0, 1.0), 0.0),
            (Point2::new(2.0, 0.05), 0.0),
            (Point2::new(1.0, 0.05), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        1.0,
        tol(),
    );
    let f = edge(&b, [0.0, 0.0, 1.0], [3.0, 0.0, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let built = report("notch d=0.1", verb, &b, &[f], 0.1, None);
        assert!(
            built.is_none(),
            "{verb:?}: a strip crossing the notch must refuse"
        );
        let sec = |d: f64| match verb {
            Verb::Chamfer => d * d / 2.0,
            Verb::Fillet => (1.0 - core::f64::consts::PI / 4.0) * d * d,
        };
        report("notch d=0.04", verb, &b, &[f], 0.04, Some(sec(0.04) * 3.0));
    }
    // The notch on the END face instead: the end face x=0 is a polygon
    // in yz whose notch dips to within 0.03 of the top-front corner.
    let plane = sketch_from_axes(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let e = prism_on(
        plane,
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.5, 1.0), 0.0),
            (Point2::new(0.5, 0.5), 0.0),
            (Point2::new(0.03, 0.5), 0.0),
            (Point2::new(0.03, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        2.0,
        tol(),
    );
    // The edge along x at y = 0, z = 1 is the top of the thin fin.
    let fin = edge(&e, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    report(
        "fin d=0.1 (top 0.03 wide)",
        Verb::Chamfer,
        &e,
        &[fin],
        0.1,
        None,
    );
    report(
        "fin d=0.02",
        Verb::Chamfer,
        &e,
        &[fin],
        0.02,
        Some(0.02 * 0.02 / 2.0 * 2.0),
    );
}

/// Oblique end faces: parallelograms at several slants (parallel ends,
/// ΔV = d²/2·L), a trapezoid (non-parallel vertical ends, ΔV = d² − d³/6
/// at L = 2 − y), and walls tilted out of the vertical in both senses.
#[test]
fn r2_oblique_end_faces() {
    let d = 0.1;
    for s in [0.05, 0.5, 1.0, 3.0, -1.0] {
        let b = prism(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(2.0, 0.0), 0.0),
                (Point2::new(2.0 + s, 1.0), 0.0),
                (Point2::new(s, 1.0), 0.0),
            ],
            1.0,
            tol(),
        );
        let f = edge(&b, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        report(
            &format!("parallelogram s={s}"),
            Verb::Chamfer,
            &b,
            &[f],
            d,
            Some(d * d / 2.0 * 2.0),
        );
        report(
            &format!("parallelogram s={s}"),
            Verb::Fillet,
            &b,
            &[f],
            d,
            None,
        );
        // The concave twin is not reachable on a prism; the back edge
        // (y = 1, slanted ends the other way) instead.
        let k = edge(&b, [s, 1.0, 1.0], [2.0 + s, 1.0, 1.0]);
        report(
            &format!("parallelogram-back s={s}"),
            Verb::Chamfer,
            &b,
            &[k],
            d,
            Some(d * d / 2.0 * 2.0),
        );
    }
    let b = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(1.5, 1.0), 0.0),
            (Point2::new(0.5, 1.0), 0.0),
        ],
        1.0,
        tol(),
    );
    let f = edge(&b, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    report(
        "trapezoid",
        Verb::Chamfer,
        &b,
        &[f],
        d,
        Some(d * d - d * d * d / 6.0),
    );
    // Walls tilted out of the vertical: profile in (z, x), extruded +y.
    for (name, pts, len) in [
        // narrowing upward: x from 0.3z to 2 − 0.3z, L(z) = 2 − 0.6z
        (
            "tilt-in",
            [(0.0, 0.0), (1.0, 0.3), (1.0, 1.7), (0.0, 2.0)],
            (2.0, -0.6),
        ),
        // widening upward: x from 0.3(1−z) to 1.7 + 0.3z, L(z) = 1.4 + 0.6z
        (
            "tilt-out",
            [(0.0, 0.3), (1.0, 0.0), (1.0, 2.0), (0.0, 1.7)],
            (1.4, 0.6),
        ),
    ] {
        let plane = sketch_from_axes(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            tol(),
        );
        let mut v: Vec<(Point2<f64>, f64)> =
            pts.iter().map(|&(z, x)| (Point2::new(z, x), 0.0)).collect();
        // Keep the loop counter-clockwise in (z, x).
        let area: f64 = (0..4)
            .map(|i| {
                let (a, b) = (v[i].0, v[(i + 1) % 4].0);
                a.x * b.y - b.x * a.y
            })
            .sum();
        if area < 0.0 {
            v.reverse();
        }
        let b = prism_on(plane, v, 1.0, tol());
        let (x0, x1) = if name == "tilt-in" {
            (0.3, 1.7)
        } else {
            (0.0, 2.0)
        };
        let f = edge(&b, [x0, 0.0, 1.0], [x1, 0.0, 1.0]);
        // Section triangle (y,z): (0,1), (d,1), (0,1−d): z̄ = 1 − d/3.
        let zbar = 1.0 - d / 3.0;
        report(
            name,
            Verb::Chamfer,
            &b,
            &[f],
            d,
            Some(d * d / 2.0 * (len.0 + len.1 * zbar)),
        );
        report(name, Verb::Fillet, &b, &[f], d, None);
    }
}

/// Two bands whose cut-offs split ONE rim from opposite ends: the top
/// front edge (end face x = 2) and the bottom right edge (end face
/// y = 0) both cut the vertical edge x = 2, y = 0. At h = 0.25 the feet
/// are in order; at h = 0.15 they cross (the two bands' regions overlap
/// near the corner) and the carve must refuse.
#[test]
fn r2_two_cut_offs_split_one_rim_from_both_ends() {
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let sec = match verb {
            Verb::Chamfer => 0.005,
            Verb::Fillet => (1.0 - core::f64::consts::PI / 4.0) * 0.01,
        };
        for h in [0.25, 0.15] {
            let b = block(2.0, 1.5, h, tol());
            let tf = edge(&b, [0.0, 0.0, h], [2.0, 0.0, h]);
            let br = edge(&b, [2.0, 0.0, 0.0], [2.0, 1.5, 0.0]);
            report(
                &format!("split-twice h={h}"),
                verb,
                &b,
                &[tf, br],
                0.1,
                Some(sec * 3.5),
            );
        }
    }
}

/// The turn classifier on a top outline bent by δ at x = 1: both top
/// front edges requested (2 of the junction's 3 edges).
#[test]
fn r2_turn_classifier_sweep() {
    for delta in [
        0.1, 1e-3, 1e-6, 1e-8, 1e-9, 1e-10, 1e-11, 1e-13, -1e-3, -1e-10,
    ] {
        let b = prism(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(2.0, delta), 0.0),
                (Point2::new(2.0, 1.0), 0.0),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            1.0,
            tol(),
        );
        let a = edge(&b, [0.0, 0.0, 1.0], [1.0, 0.0, 1.0]);
        let c = edge(&b, [1.0, 0.0, 1.0], [2.0, delta, 1.0]);
        for verb in [Verb::Chamfer, Verb::Fillet] {
            report(&format!("bent δ={delta}"), verb, &b, &[a, c], 0.1, None);
            report(&format!("bent-one δ={delta}"), verb, &b, &[a], 0.1, None);
        }
    }
}

/// Mixed roles on one face: the top-front band's end face (x = 2) is
/// a support of the bottom-right band, with clearance (h = 1).
#[test]
fn r2_end_face_is_another_bands_support() {
    let b = block(2.0, 1.5, 1.0, tol());
    let tf = edge(&b, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let br = edge(&b, [2.0, 0.0, 0.0], [2.0, 1.5, 0.0]);
    let bl = edge(&b, [0.0, 0.0, 0.0], [0.0, 1.5, 0.0]);
    report(
        "tf+br+bl",
        Verb::Chamfer,
        &b,
        &[tf, br, bl],
        0.1,
        Some(0.005 * 5.0),
    );
    report(
        "tf+br+bl",
        Verb::Fillet,
        &b,
        &[tf, br, bl],
        0.1,
        Some((1.0 - core::f64::consts::PI / 4.0) * 0.01 * 5.0),
    );
}

/// The twice-split rim at the coincidence: h = 2d ± tiny, so the two
/// feet on the vertical edge x = 2, y = 0 are tiny apart (or equal).
#[test]
fn r2_twice_split_rim_at_coincidence() {
    for verb in [Verb::Chamfer, Verb::Fillet] {
        for dh in [1e-6, 1e-8, 1e-10, 1e-12, 1e-14, 0.0, -1e-12] {
            let h = 0.2 + dh;
            let b = block(2.0, 1.5, h, tol());
            let tf = edge(&b, [0.0, 0.0, h], [2.0, 0.0, h]);
            let br = edge(&b, [2.0, 0.0, 0.0], [2.0, 1.5, 0.0]);
            if let Some(_) = report(
                &format!("coincident-feet dh={dh:e}"),
                verb,
                &b,
                &[tf, br],
                0.1,
                None,
            ) {
                // Shortest edge of the result.
                let out = run(verb, &b, &[tf, br], 0.1).unwrap();
                let mut short = f64::INFINITY;
                for e in query::all_edges(&out.body) {
                    let he = out.body.get_edge(e).unwrap().he_plus;
                    let p = |v| {
                        *out.body
                            .get_point(out.body.get_vertex(v).unwrap().point)
                            .unwrap()
                    };
                    let l = (p(out.body.get_half_edge(he).unwrap().start)
                        - p(out.body.half_edge_end(he).unwrap()))
                    .norm();
                    short = short.min(l);
                }
                println!("R2   shortest edge chord {short:e}");
            }
        }
    }
}

/// Arm (d) witness hunt: notches predicate 2 may not read.
#[test]
fn r2_arm_d_witness_hunt() {
    let shapes: Vec<(&str, Vec<(f64, f64)>)> = vec![
        (
            "v-notch",
            vec![(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (1.5, 0.05), (0.0, 1.0)],
        ),
        (
            "spike",
            vec![
                (0.0, 0.0),
                (3.0, 0.0),
                (3.0, 1.0),
                (2.95, 1.0),
                (2.9, 0.05),
                (2.85, 1.0),
                (0.0, 1.0),
            ],
        ),
        (
            "slot",
            vec![
                (0.0, 0.0),
                (3.0, 0.0),
                (3.0, 1.0),
                (2.0, 1.0),
                (2.0, 0.05),
                (1.9, 0.05),
                (1.9, 1.0),
                (0.0, 1.0),
            ],
        ),
    ];
    for (name, pts) in shapes {
        let b = prism(
            pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect(),
            1.0,
            tol(),
        );
        let f = edge(&b, [0.0, 0.0, 1.0], [3.0, 0.0, 1.0]);
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let r = run(verb, &b, &[f], 0.1);
            let tier3 = r
                .as_ref()
                .map(|o| validate_geometric(&o.body, tol()).is_ok());
            println!(
                "R2D {name} {verb:?} mut={}: {:?}",
                std::env::var("R2MUT").is_ok(),
                tier3.map_err(|e| format!("{e:?}").chars().take(160).collect::<String>())
            );
        }
    }
}
