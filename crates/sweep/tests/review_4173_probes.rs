//! Review probes for PR 4173 (oblique fillet ends in an ellipse).
//! Printed measurements; run with `--nocapture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::{Curve3, Surface};
use geom_core::{Point2, Point3, Vec3};
use sweep::blend::Blended;
use sweep::test_support::{block, brick, finished, prism, prism_on, realized, sketch_from_axes};
use topo::boolean::BooleanOp;
use topo::boolean::{SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, validate_geometric};

fn try_bool(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, String> {
    boolean_op_with(
        op,
        &finished("a", a.clone(), tol()),
        &finished("b", b.clone(), tol()),
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .map_err(|e| format!("{e}"))?
    .body()
    .map(|b| b.body.clone().into_body())
    .ok_or_else(|| "no body".to_string())
}

use crate::band_planar_cut_off::{Verb, edge, tol, volume_enclosure};

/// Max residual of every ellipse end arc on the band cylinder and on its plane.
fn residuals(out: &Blended<f64>) -> (usize, f64, f64) {
    let rec = out.naming.as_ref().expect("births");
    let cyl = out
        .blend_faces
        .iter()
        .find_map(
            |f| match *out.body.get_surface(out.body.get_face(*f)?.surface)? {
                Surface::Cylinder {
                    origin,
                    axis,
                    radius,
                    ..
                } => Some((origin, axis, radius)),
                _ => None,
            },
        )
        .expect("cylinder band");
    let (mut n, mut rc, mut rp) = (0, 0.0f64, 0.0f64);
    for (arc, _, _) in &rec.arcs {
        let c = out
            .body
            .get_curve_geom(out.body.get_edge(*arc).unwrap().curve)
            .and_then(|g| g.certified())
            .expect("certified");
        if let Curve3::Ellipse { center, axis, .. } = *c.carrier() {
            n += 1;
            let (t0, t1) = c.params();
            for k in 0..=64 {
                let t = t0 + (t1 - t0) * f64::from(k) / 64.0;
                let p = c.carrier().eval(t);
                let d = p - cyl.0;
                let radial = d - cyl.1 * d.dot(cyl.1);
                rc = rc.max((radial.norm() - cyl.2).abs());
                rp = rp.max((p - center).dot(axis).abs());
            }
        }
    }
    (n, rc, rp)
}

fn parallelogram(s: f64) -> Body<f64> {
    prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0 + s, 1.0), 0.0),
            (Point2::new(s, 1.0), 0.0),
        ],
        1.0,
        tol(),
    )
}

fn trapezoid(s: f64) -> Body<f64> {
    prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0 - s, 1.0), 0.0),
            (Point2::new(s, 1.0), 0.0),
        ],
        1.0,
        tol(),
    )
}

fn report(
    what: &str,
    body: &Body<f64>,
    out: Result<Blended<f64>, sweep::blend::BlendError>,
    want: f64,
) {
    match out {
        Err(e) => println!("[probe] {what}: REFUSED {e}"),
        Ok(out) => {
            let t3 = validate_geometric(&out.body, tol())
                .map(|_| ())
                .map_err(|e| format!("{e:?}"));
            let (n, rc, rp) = residuals(&out);
            let ((v0, p0), (v1, p1)) = (volume_enclosure(body), volume_enclosure(&out.body));
            let dv = v0 - v1;
            println!(
                "[probe] {what}: tier3={:?} ellipses={n} cylres={rc:.2e} planeres={rp:.2e} dV-closed={:.3e} pad={:.2e}",
                t3.is_ok(),
                dv - want,
                p0 + p1
            );
            if let Err(e) = t3 {
                println!("[probe]    tier3 error: {e}");
            }
        }
    }
}

#[test]
fn probe_slant_sweep_convex() {
    let (a, c) = (Verb::Fillet.section(), Verb::Fillet.centroid());
    for s in [1e-3, 3e-3, 1e-2, 0.1, 0.5, 0.95, -0.5, -2.0, -8.0] {
        let body = trapezoid(s);
        let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        let want = a * (2.0 - 2.0 * s * c);
        report(
            &format!("convex s={s}"),
            &body,
            Verb::Fillet.run(&body, &[e]),
            want,
        );
        // bottom edge too (different dihedral orientation)
        let e = edge(&body, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        report(
            &format!("convex-bottom s={s}"),
            &body,
            Verb::Fillet.run(&body, &[e]),
            want,
        );
    }
}

#[test]
fn probe_parallelogram_steep() {
    let a = Verb::Fillet.section();
    for s in [0.0, 0.5, 1.0, 2.0, 4.0, 8.0, 20.0, 60.0, -1.0, -4.0, -20.0] {
        let body = parallelogram(s);
        let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        report(
            &format!("para s={s}"),
            &body,
            Verb::Fillet.run(&body, &[e]),
            2.0 * a,
        );
        let e = edge(&body, [s, 1.0, 1.0], [2.0 + s, 1.0, 1.0]);
        report(
            &format!("para-back s={s}"),
            &body,
            Verb::Fillet.run(&body, &[e]),
            2.0 * a,
        );
    }
}

/// Oblique end with an out-of-plane tilt as well: end walls tilted about x (trapezoid in xz).
#[test]
fn probe_slant_sweep_xz() {
    let (a, c) = (Verb::Fillet.section(), Verb::Fillet.centroid());
    for s in [0.01, 0.3, 0.6, -0.5, -3.0] {
        let plane = sketch_from_axes(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        );
        let body = prism_on(
            plane,
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(2.0, 0.0), 0.0),
                (Point2::new(2.0 - s, 1.0), 0.0),
                (Point2::new(s, 1.0), 0.0),
            ],
            1.0,
            tol(),
        );
        // edges along x at z=0 between faces y-side; find the bottom edge at y=0 or y=1.
        for y in [0.0, 1.0, -1.0] {
            let found = topo::query::all_edges(&body).into_iter().find(|&k| {
                let he = body.get_edge(k).unwrap().he_plus;
                let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                let (ps, pt) = (
                    p(body.get_half_edge(he).unwrap().start),
                    p(body.half_edge_end(he).unwrap()),
                );
                let ok = |q: Point3<f64>, x: f64| (q - Point3::new(x, y, 0.0)).norm() < 1e-12;
                (ok(ps, 0.0) && ok(pt, 2.0)) || (ok(ps, 2.0) && ok(pt, 0.0))
            });
            if let Some(e) = found {
                let want = a * (2.0 - 2.0 * s * c);
                report(
                    &format!("xz s={s} y={y}"),
                    &body,
                    Verb::Fillet.run(&body, &[e]),
                    want,
                );
            }
        }
    }
}

#[test]
fn probe_slant_sweep_concave() {
    let (a, c) = (Verb::Fillet.section(), Verb::Fillet.centroid());
    for s in [0.01, 0.3, 1.0, 2.5, -0.3, -0.45] {
        let plane = sketch_from_axes(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            tol(),
        );
        let (x0, x1) = (3.0, 4.0);
        let pocket = prism_on(
            plane,
            vec![
                (Point2::new(x0, 0.25), 0.0),
                (Point2::new(x1, 0.25), 0.0),
                (Point2::new(x1 + s, 1.25), 0.0),
                (Point2::new(x0 + s, 1.25), 0.0),
            ],
            1.0,
            tol(),
        );
        let body = realized(
            BooleanOp::Subtract,
            &block(8.0, 1.5, 1.0, tol()),
            &pocket,
            tol(),
        );
        let e = edge(&body, [x0, 0.25, 0.5], [x1, 0.25, 0.5]);
        // concave: band adds section; slanted walls: length at centroid = 1 + ... walls x = x0 + s(y-0.25)
        // the band lies at y in [0.25, 0.25+r], centroid c from the wall y=0.25: L = 1 (parallel walls).
        let _ = c;
        report(
            &format!("concave s={s}"),
            &body,
            Verb::Fillet.run(&body, &[e]),
            -a * 1.0,
        );
        // a trapezoidal pocket (non-parallel walls) on the concave side
        let pocket = prism_on(
            sketch_from_axes(
                Point3::new(0.0, 0.0, 0.5),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                tol(),
            ),
            vec![
                (Point2::new(x0, 0.25), 0.0),
                (Point2::new(x1, 0.25), 0.0),
                (Point2::new(x1 + s, 1.25), 0.0),
                (Point2::new(x0 - s, 1.25), 0.0),
            ],
            1.0,
            tol(),
        );
        let body = realized(
            BooleanOp::Subtract,
            &block(8.0, 1.5, 1.0, tol()),
            &pocket,
            tol(),
        );
        let e = edge(&body, [x0, 0.25, 0.5], [x1, 0.25, 0.5]);
        // band region in pocket: x from x0 - s*(y') to x1 + s*y', y' = y - 0.25 in [0, r]
        let want = -a * (1.0 + 2.0 * s * c);
        report(
            &format!("concave-trap s={s}"),
            &body,
            Verb::Fillet.run(&body, &[e]),
            want,
        );
    }
}

/// Downstream verbs on an oblique fillet.
#[test]
fn probe_downstream() {
    for s in [0.0, 0.5, -1.0, 3.0] {
        let body = parallelogram(s);
        let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        let out = Verb::Fillet.run(&body, &[e]).expect("builds");
        let b = out.body;
        println!(
            "[probe] s={s} tess: {:?}",
            mesh::tessellate(&b, 2e-3, tol())
                .map(|m| mesh::validate::check_mesh(&m).is_ok())
                .map_err(|e| e.to_string())
        );
        let st = step_export::step_string(&b, &step_export::StepOptions::default(), tol());
        println!(
            "[probe] s={s} step: {:?}",
            st.as_ref()
                .map(|t| (t.len(), t.matches("ELLIPSE").count()))
                .map_err(|e| e.to_string())
        );
        let fin = finished("filleted", b.clone(), tol());
        match topo::shell(&fin, 0.05, tol()) {
            Ok(sh) => println!(
                "[probe] s={s} shell: ok tier3={:?}",
                validate_geometric(&sh.body, tol()).is_ok()
            ),
            Err(e) => println!("[probe] s={s} shell: REFUSED {e:?}"),
        }
        // boolean: cutter slicing the elliptic end region (crosses the end arc)
        let cutter = brick((-0.5, 0.15), (-0.5, 0.05), (0.92, 1.5), tol());
        for op in [BooleanOp::Subtract, BooleanOp::Union, BooleanOp::Intersect] {
            let r = try_bool(op, &b, &cutter);
            match r {
                Ok(r) => println!(
                    "[probe] s={s} bool-at-end {op:?}: tier3={:?} V={:?}",
                    validate_geometric(&r, tol()).is_ok(),
                    topo::mass_properties(&r, tol())
                        .map(|p| (p.volume, p.volume_pad))
                        .map_err(|e| format!("{e:?}"))
                ),
                Err(e) => println!("[probe] s={s} bool-at-end {op:?}: REFUSED {e:?}"),
            }
        }
        // a second fillet: fillet another edge of the filleted body (re-blend adjacency)
        let e2 = edge(&b, [2.0, 0.0, 0.0], [2.0 + s, 1.0, 0.0]);
        match Verb::Fillet.run(&b, &[e2]) {
            Ok(o) => println!(
                "[probe] s={s} second fillet: ok tier3={:?}",
                validate_geometric(&o.body, tol()).is_ok()
            ),
            Err(e) => println!("[probe] s={s} second fillet: REFUSED {e}"),
        }
        let _ = PI;
    }
}

/// A round pin of radius `rad` whose axis runs along `a × b` from `origin`.
fn pin(origin: Point3<f64>, a: Vec3<f64>, b: Vec3<f64>, rad: f64, h: f64) -> Body<f64> {
    prism_on(
        sketch_from_axes(origin, a, b, tol()),
        vec![(Point2::new(-rad, 0.0), 1.0), (Point2::new(rad, 0.0), 1.0)],
        h,
        tol(),
    )
}

/// Clearance on the 45° slanted wall (s = 1): wall coords `t` along the
/// wall from the old vertex, `h` from the crease's support plane toward
/// the band. Ellipse centre (0.1414, 0.1), major 0.1414 along t, minor 0.1.
#[test]
fn probe_clearance() {
    let rt2 = 2f64.sqrt();
    let u = Vec3::new(1.0, 1.0, 0.0) / rt2;
    let z = Vec3::new(0.0, 0.0, 1.0);
    let ell = |t: f64, h: f64| ((t - 0.1 * rt2) / (0.1 * rt2)).powi(2) + ((h - 0.1) / 0.1).powi(2);
    let cases = [
        ("sliver mid", 0.04, 0.015, 0.004, "refuse"),
        (
            "sliver cusp at major vertex (vertical foot)",
            0.004,
            0.05,
            0.0018,
            "refuse",
        ),
        ("sliver cusp at top foot", 0.10, 0.003, 0.0015, "refuse"),
        (
            "kept: inside ellipse, outside disc",
            0.03,
            0.07,
            0.005,
            "build",
        ),
        (
            "kept: inside ellipse near major vertex",
            0.012,
            0.10,
            0.004,
            "build",
        ),
        ("kept: far", 0.4, 0.4, 0.02, "build"),
    ];
    for (what, t, h, rad, want) in cases {
        println!(
            "[probe] clearance case {what}: ellipse value {:.3} (<1 kept, >1 sliver)",
            ell(t, h)
        );
        // convex: parallelogram s=1, vertex (0,0,1); h measured down from z=1.
        let body = parallelogram(1.0);
        let n_out = Vec3::new(-1.0, 1.0, 0.0) / rt2;
        let p = Point3::new(0.0, 0.0, 1.0 - h) + u * t + n_out * 0.1;
        let drilled = try_bool(
            BooleanOp::Subtract,
            &body,
            &pin(p, u, z, rad, 0.1 + 0.4 * t),
        );
        match drilled {
            Err(e) => println!("[probe] clearance convex {what}: fixture refused {e}"),
            Ok(d) => {
                let e = edge(&d, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
                match Verb::Fillet.run(&d, &[e]) {
                    Ok(o) => println!(
                        "[probe] clearance convex {what} (want {want}): BUILT tier3={:?}",
                        validate_geometric(&o.body, tol()).map_err(|e| format!("{e:?}"))
                    ),
                    Err(e) => {
                        println!("[probe] clearance convex {what} (want {want}): refused {e}")
                    }
                }
            }
        }
        // concave: parallelogram pocket s=1, vertex (0.5,0.25,0.5); h up from the floor.
        let pocket = prism_on(
            sketch_from_axes(
                Point3::new(0.0, 0.0, 0.5),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                tol(),
            ),
            vec![
                (Point2::new(0.5, 0.25), 0.0),
                (Point2::new(1.5, 0.25), 0.0),
                (Point2::new(2.5, 1.25), 0.0),
                (Point2::new(1.5, 1.25), 0.0),
            ],
            1.0,
            tol(),
        );
        let body = realized(
            BooleanOp::Subtract,
            &block(3.0, 1.5, 1.0, tol()),
            &pocket,
            tol(),
        );
        let n_void = Vec3::new(1.0, -1.0, 0.0) / rt2;
        let p = Point3::new(0.5, 0.25, 0.5 + h) + u * t + n_void * 0.1;
        let drilled = try_bool(
            BooleanOp::Subtract,
            &body,
            &pin(p, z, u, rad, 0.1 + 0.4 * t),
        );
        match drilled {
            Err(e) => println!("[probe] clearance concave {what}: fixture refused {e}"),
            Ok(d) => {
                let e = edge(&d, [0.5, 0.25, 0.5], [1.5, 0.25, 0.5]);
                match Verb::Fillet.run(&d, &[e]) {
                    Ok(o) => println!(
                        "[probe] clearance concave {what} (want {want}): BUILT tier3={:?}",
                        validate_geometric(&o.body, tol()).map_err(|e| format!("{e:?}"))
                    ),
                    Err(e) => {
                        println!("[probe] clearance concave {what} (want {want}): refused {e}")
                    }
                }
            }
        }
    }
    let _ = brick::<f64>;
}

/// The same holes on the perpendicular wall (s = 0) and a 45° wall, convex: which screen answers.
#[test]
fn probe_clearance_baseline() {
    for sl in [0.0f64, 0.3, 1.0] {
        let len = (1.0 + sl * sl).sqrt();
        let u = Vec3::new(sl, 1.0, 0.0) / len;
        let n_out = Vec3::new(-1.0, sl, 0.0) / len;
        let z = Vec3::new(0.0, 0.0, 1.0);
        for (t, h, rad) in [
            (0.03, 0.03, 0.01),
            (0.04, 0.015, 0.004),
            (0.03, 0.07, 0.005),
            (0.10, 0.003, 0.0015),
            (0.2, 0.2, 0.01),
        ] {
            let body = parallelogram(sl);
            let p = Point3::new(0.0, 0.0, 1.0 - h) + u * t + n_out * 0.1;
            let Ok(d) = try_bool(
                BooleanOp::Subtract,
                &body,
                &pin(p, u, z, rad, 0.1 + 0.4 * t),
            ) else {
                println!("[probe] base fixture refused");
                continue;
            };
            let e = edge(&d, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
            let r = Verb::Fillet
                .run(&d, &[e])
                .map(|o| validate_geometric(&o.body, tol()).is_ok())
                .map_err(|e| e.to_string().chars().take(90).collect::<String>());
            println!("[probe] base s={sl} t={t} h={h} r={rad}: {r:?}");
        }
    }
}

#[test]
fn probe_clearance_concave_baseline() {
    for sl in [0.0f64, 0.3, 1.0] {
        let len = (1.0 + sl * sl).sqrt();
        let u = Vec3::new(sl, 1.0, 0.0) / len;
        let n_void = Vec3::new(1.0, -sl, 0.0) / len;
        let z = Vec3::new(0.0, 0.0, 1.0);
        let pocket = prism_on(
            sketch_from_axes(
                Point3::new(0.0, 0.0, 0.5),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                tol(),
            ),
            vec![
                (Point2::new(0.5, 0.25), 0.0),
                (Point2::new(1.5, 0.25), 0.0),
                (Point2::new(1.5 + sl, 1.25), 0.0),
                (Point2::new(0.5 + sl, 1.25), 0.0),
            ],
            1.0,
            tol(),
        );
        let body = realized(
            BooleanOp::Subtract,
            &block(3.0, 1.5, 1.0, tol()),
            &pocket,
            tol(),
        );
        for (t, h, rad) in [
            (0.03, 0.03, 0.01),
            (0.04, 0.015, 0.004),
            (0.03, 0.07, 0.005),
            (0.10, 0.003, 0.0015),
            (0.2, 0.2, 0.01),
        ] {
            let p = Point3::new(0.5, 0.25, 0.5 + h) + u * t + n_void * 0.1;
            let Ok(d) = try_bool(BooleanOp::Subtract, &body, &pin(p, z, u, rad, 0.15)) else {
                println!("[probe] cbase fixture refused");
                continue;
            };
            let e = edge(&d, [0.5, 0.25, 0.5], [1.5, 0.25, 0.5]);
            let r = Verb::Fillet
                .run(&d, &[e])
                .map(|o| validate_geometric(&o.body, tol()).is_ok())
                .map_err(|e| format!("{e:?}").chars().take(160).collect::<String>());
            println!("[probe] cbase s={sl} t={t} h={h} r={rad}: {r:?}");
        }
    }
}

/// The second-order escalation window in user terms: a small fillet at a draft-angle end wall.
#[test]
fn probe_draft_window() {
    for r in [0.1f64, 0.01, 0.001] {
        for deg in [0.05f64, 0.25, 0.5, 1.0, 2.0] {
            let s = deg.to_radians().tan();
            let body = trapezoid(s);
            let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
            let res = sweep::blend::build::fillet_edges(&body, &[e], r, tol());
            let what = match res {
                Ok(o) => format!("BUILT tier3={}", validate_geometric(&o.body, tol()).is_ok()),
                Err(e) => match e.error {
                    ref esc @ sweep::blend::BlendError::Escalated { ref source, .. } => {
                        format!("ESCALATED via {:?} :: {esc}", source.predicate)
                    }
                    other => format!("refused {other}"),
                },
            };
            println!(
                "[probe] draft r={r} θ={deg}°: r(secθ−1)={:.2e}: {what}",
                r * (1.0 / deg.to_radians().cos() - 1.0)
            );
        }
    }
}

/// The ruled band at an oblique cap: residuals of its ellipse arcs, tier 3, naming totality,
/// tilts about y and about x, and every crease subset.
#[test]
fn probe_ruled_oblique() {
    use sweep::test_support::{ROD_FILLET, assert_naming_totality, rod_creases, rod_d_profile_at};
    let rod = finished("rod", rod_d_profile_at::<f64>(tol()), tol());
    for (axis, phi) in [
        ("y", 0.05f64),
        ("y", 0.3),
        ("y", 0.6),
        ("y", -0.55),
        ("x", 0.3),
        ("x", -0.6),
        ("xy", 0.4),
    ] {
        let n = match axis {
            "y" => Vec3::new(phi.sin(), 0.0, phi.cos()),
            "x" => Vec3::new(0.0, phi.sin(), phi.cos()),
            _ => Vec3::new(phi.sin() * 0.6, phi.sin() * 0.8, phi.cos()),
        };
        let plane = topo::test_support::split_plane(Point3::new(0.0, 0.0, 0.7), n, tol());
        let Ok(result) = topo::splitting::split(&rod, &plane, tol()) else {
            println!("[probe] ruled {axis} {phi}: split refused");
            continue;
        };
        let topo::splitting::SplitPart::Body(below) = &result.below else {
            continue;
        };
        let creases = rod_creases(below);
        for req in [vec![creases[0]], vec![creases[1]], creases.clone()] {
            match sweep::blend::build::fillet_edges(below, &req, ROD_FILLET, tol()) {
                Ok(out) => {
                    let t3 = validate_geometric(&out.body, tol()).is_ok();
                    assert_naming_totality(below, &out, &req, "ruled probe");
                    let (ne, rc, rp) = residuals(&out);
                    let tess = mesh::tessellate(&out.body, 5e-3, tol())
                        .map(|m| mesh::validate::check_mesh(&m).is_ok())
                        .is_ok();
                    println!(
                        "[probe] ruled tilt-{axis} φ={phi} n={}: tier3={t3} ellipses={ne} cylres={rc:.1e} planeres={rp:.1e} tess={tess}",
                        req.len()
                    );
                }
                Err(e) => println!(
                    "[probe] ruled tilt-{axis} φ={phi} n={}: refused {}",
                    req.len(),
                    e.error
                ),
            }
        }
    }
}

#[test]
fn probe_ruled_tilt_x_detail() {
    use sweep::test_support::{ROD_FILLET, rod_creases, rod_d_profile_at};
    let rod = finished("rod", rod_d_profile_at::<f64>(tol()), tol());
    for phi in [-0.2f64, -0.4, -0.5, -0.6, 0.6] {
        let n = Vec3::new(0.0, phi.sin(), phi.cos());
        let plane = topo::test_support::split_plane(Point3::new(0.0, 0.0, 0.7), n, tol());
        let result = topo::splitting::split(&rod, &plane, tol()).unwrap();
        let topo::splitting::SplitPart::Body(below) = &result.below else {
            continue;
        };
        for (i, c) in rod_creases(below).into_iter().enumerate() {
            let he = below.get_edge(c).unwrap().he_plus;
            let p = |v| *below.get_point(below.get_vertex(v).unwrap().point).unwrap();
            let (a, b) = (
                p(below.get_half_edge(he).unwrap().start),
                p(below.half_edge_end(he).unwrap()),
            );
            let r = sweep::blend::build::fillet_edges(below, &[c], ROD_FILLET, tol());
            println!(
                "[probe] tiltx φ={phi} crease{i} {a:?}->{b:?}: {:?}",
                r.map(|o| validate_geometric(&o.body, tol()).is_ok())
                    .map_err(|e| format!("{:?}", e.error)
                        .chars()
                        .take(200)
                        .collect::<String>())
            );
        }
        // the cap's edges
        for e in topo::query::all_edges(below) {
            let g = below
                .get_curve_geom(below.get_edge(e).unwrap().curve)
                .and_then(|g| g.certified());
            if let Some(g) = g {
                let he = below.get_edge(e).unwrap().he_plus;
                let p = |v| *below.get_point(below.get_vertex(v).unwrap().point).unwrap();
                let (a, b) = (
                    p(below.get_half_edge(he).unwrap().start),
                    p(below.half_edge_end(he).unwrap()),
                );
                if a.z > 0.3 && b.z > 0.3 {
                    println!(
                        "[probe]    cap-ish edge {:?}: {a:?} -> {b:?}",
                        std::mem::discriminant(g.carrier())
                    );
                }
            }
        }
    }
}
