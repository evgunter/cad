//! Reviewer probes for PR 4394 (the split's tangent section chord asks
//! the must-carry rule). Scratch branch only: these print tables and
//! assert the soundness invariants an oracle independent of the kernel
//! can check — tier 3, the closed-form volume, and the seam lying on
//! the true ruling within ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::RawLoop;
use profile::test_support::bulge_loop;
use sweep::test_support::{extruded, finished, sketch_from_axes};

fn tol() -> Tol {
    Tol::witness()
}

/// An orthonormal pose: origin, in-plane `u`, `v`, normal `n = u × v`.
#[derive(Clone, Copy)]
struct Pose {
    name: &'static str,
    o: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
}

impl Pose {
    fn n(&self) -> Vec3<f64> {
        self.u.cross(self.v)
    }
}

fn poses(s: f64) -> Vec<Pose> {
    let (a, b) = (0.7_f64, 0.3_f64);
    // A rotation: Rz(a)·Rx(b).
    let rz = |p: Vec3<f64>| {
        Vec3::new(
            a.cos() * p.x - a.sin() * p.y,
            a.sin() * p.x + a.cos() * p.y,
            p.z,
        )
    };
    let rx = |p: Vec3<f64>| {
        Vec3::new(
            p.x,
            b.cos() * p.y - b.sin() * p.z,
            b.sin() * p.y + b.cos() * p.z,
        )
    };
    let rot = |p: Vec3<f64>| rz(rx(p));
    let off = Point3::new(3.7 * s, -2.9 * s, 1.3 * s);
    vec![
        Pose {
            name: "identity",
            o: Point3::new(0.0, 0.0, 0.0),
            u: Vec3::new(1.0, 0.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
        },
        Pose {
            name: "offset",
            o: off,
            u: Vec3::new(1.0, 0.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
        },
        Pose {
            name: "tilted+offset",
            o: off,
            u: rot(Vec3::new(1.0, 0.0, 0.0)),
            v: rot(Vec3::new(0.0, 1.0, 0.0)),
        },
    ]
}

/// The shoulder at scale `s` (radius `s`), extruded `h` along the pose's
/// normal.
fn shoulder(pose: Pose, s: f64, h: f64) -> topo::AtRestBody<f64> {
    let bulge = (std::f64::consts::PI / 8.0).tan();
    let p = |x: f64, y: f64| Point2::new(x * s, y * s);
    let outline = bulge_loop(vec![
        (p(-1.0, 0.0), 0.0),
        (p(2.0, 0.0), 0.0),
        (p(2.0, 2.0), 0.0),
        (p(0.0, 2.0), 0.0),
        (p(0.0, 1.0), bulge),
    ]);
    let plane = sketch_from_axes(pose.o, pose.u, pose.v, tol());
    finished("shoulder", extruded(plane, vec![outline], h, tol()), tol())
}

/// The cut `y = y_c·s` in the pose, normal `−v`.
fn cut(pose: Pose, s: f64, y_c: f64) -> topo::SplitPlane<f64> {
    topo::test_support::split_plane(pose.o + pose.v * (y_c * s), -pose.v, tol())
}

/// The closed-form section areas (unit scale) of the `normal` side
/// (y < y_c) and the far side, for y_c ≤ 1.
fn areas(y_c: f64) -> (f64, f64) {
    let total = 4.0 + std::f64::consts::FRAC_PI_4;
    let half_segment = if y_c < 1.0 {
        ((y_c).acos() - y_c * (1.0 - y_c * y_c).sqrt()) / 2.0
    } else {
        0.0
    };
    let far = 2.0 * (2.0 - y_c) + half_segment;
    (total - far, far)
}

fn dist_to_line(p: Point3<f64>, a: Point3<f64>, d: Vec3<f64>) -> f64 {
    let w = p - a;
    (w - d * w.dot(d)).norm()
}

#[derive(Debug, Default)]
struct Seam {
    kind: String,
    /// Max carrier distance from the true ruling over 11 samples.
    carrier_off: f64,
    /// Max distance of the carrier from the cylinder (radial − R).
    off_cylinder: f64,
    /// For a chart: max distance of the image (eval on its surface)
    /// from the carrier at the same parameter.
    image_off: Option<f64>,
}

/// Checks a piece; returns its seams (edges on the ruling between the
/// wall and a plane).
fn audit(piece: &topo::Body<f64>, pose: Pose, s: f64, eps: f64) -> Vec<Seam> {
    let ruling_at = pose.o + pose.v * s;
    let n = pose.n();
    let axis = pose.o;
    let mut seams = vec![];
    let surface_of = |he| {
        let face = piece.face_of_half_edge(he).unwrap();
        piece.get_face(face).unwrap().surface
    };
    for (_, e) in piece.edges() {
        let pair = [surface_of(e.he_plus), surface_of(e.he_minus)];
        let kinds = pair.map(|k| piece.get_surface(k).unwrap().kind());
        if !kinds.contains(&geom::SurfaceKind::Cylinder)
            || !kinds.contains(&geom::SurfaceKind::Plane)
        {
            continue;
        }
        let Some(cg) = piece.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let carrier = cg.carrier();
        if !matches!(carrier, geom::Curve3::Line { .. }) {
            continue;
        }
        let (t0, t1) = cg.params();
        let samples: Vec<f64> = (0..=10)
            .map(|i| t0 + (t1 - t0) * f64::from(i) / 10.0)
            .collect();
        let carrier_off = samples
            .iter()
            .map(|&t| dist_to_line(carrier.eval(t), ruling_at, n))
            .fold(0.0, f64::max);
        if carrier_off > 1e3 * eps.max(1e-12) * s.max(1.0) {
            // Another wall–plane line edge (the corner ruling at (−s, 0)).
            continue;
        }
        let off_cylinder = samples
            .iter()
            .map(|&t| (dist_to_line(carrier.eval(t), axis, n) - s).abs())
            .fold(0.0, f64::max);
        let (kind, image_off) = match cg.description() {
            geom_brep::EdgeDescription::Chart(chart) => {
                let surf = piece.get_surface(chart.surface).unwrap();
                let off = samples
                    .iter()
                    .map(|&t| {
                        let uv = chart.pcurve.eval(t);
                        surf.eval(uv.x, uv.y).distance(carrier.eval(t))
                    })
                    .fold(0.0, f64::max);
                (format!("Chart({:?})", surf.kind()), Some(off))
            }
            geom_brep::EdgeDescription::TangentIntersection { .. } => ("Tangent".into(), None),
            other => (format!("{other:?}").chars().take(30).collect(), None),
        };
        seams.push(Seam {
            kind,
            carrier_off,
            off_cylinder,
            image_off,
        });
    }
    seams
}

/// Runs one cut and returns (outcome label, failures).
fn run(pose: Pose, s: f64, h: f64, y_c: f64, eps: f64) -> (String, Vec<String>) {
    let mut bad = vec![];
    let Ok(body) = std::panic::catch_unwind(|| shoulder(pose, s, h)) else {
        return ("FIXTURE refuses (extrude)".into(), bad);
    };
    let result = topo::split(&body, &cut(pose, s, y_c), tol());
    let halves = match result {
        Ok(h) => h,
        Err(e) => {
            let text = format!("{e:?}");
            let short: String = text.chars().take(110).collect();
            return (format!("ERR {short}"), bad);
        }
    };
    let (a_near, a_far) = areas(y_c);
    let mut label = String::from("OK");
    for (side, area, part) in [
        ("near", a_near, &halves.above),
        ("far", a_far, &halves.below),
    ] {
        let Some(piece) = part.body() else {
            bad.push(format!("{side}: no material"));
            continue;
        };
        if let Err(e) = topo::validate_geometric(piece, tol()) {
            bad.push(format!("{side}: tier3 {e:?}"));
        }
        let want = area * s * s * h;
        match topo::mass_properties(piece, tol()) {
            Ok(m) => {
                let rel = (m.volume - want).abs() / want;
                if rel > 1e-8 {
                    bad.push(format!(
                        "{side}: volume {:e} want {want:e} rel {rel:e}",
                        m.volume
                    ));
                }
            }
            Err(e) => bad.push(format!("{side}: mass {e:?}")),
        }
        for seam in audit(piece, pose, s, eps) {
            label.push_str(&format!(
                " [{side} {} carrier_off={:.1e} off_cyl={:.1e} img={:?}]",
                seam.kind,
                seam.carrier_off,
                seam.off_cylinder,
                seam.image_off.map(|x| format!("{x:.1e}"))
            ));
            // D4: the stored locus within ε of both surfaces (the
            // true ruling is on both when y_c = 1).
            if seam.off_cylinder > eps * s.max(1.0) {
                bad.push(format!(
                    "{side}: seam off the cylinder by {:e}",
                    seam.off_cylinder
                ));
            }
            if let Some(off) = seam.image_off
                && off > eps * s.max(1.0)
            {
                bad.push(format!("{side}: chart image off its carrier by {off:e}"));
            }
        }
    }
    (label, bad)
}

/// **The grid**: scale × pose × c (h = c·√(2εs), the sagitta h²/(2s)
/// metered against ε), tangent cut exactly at y = s.
#[test]
fn probe_grid_scale_pose_depth() {
    let eps = tol().eps();
    let k = tol().k();
    let mut failures = vec![];
    for s in [1e-3, 1e-1, 1.0, 10.0, 1e3] {
        for pose in poses(s) {
            for c in [
                0.25,
                0.5,
                0.9,
                0.99,
                1.01,
                1.1,
                2.0,
                k.sqrt() * 0.99,
                k.sqrt() * 1.01,
                4.0,
                8.0,
            ] {
                let h = c * (2.0 * eps * s).sqrt();
                let (label, bad) = run(pose, s, h, 1.0, eps);
                // The rule's expectation, away from its two boundaries.
                let expect = if c < 0.99 {
                    "Chart"
                } else if c > 1.01 && c * c < k * 0.98 {
                    "TangentChordBendEscalated"
                } else if c * c > k * 1.02 {
                    "Tangent"
                } else {
                    "?"
                };
                let agrees =
                    expect == "?" || label.contains(expect) || label.starts_with("FIXTURE");
                println!(
                    "[grid] s={s:e} {} c={c:.3} h={h:.3e} expect={expect} agrees={agrees} :: {label}",
                    pose.name
                );
                for b in bad {
                    failures.push(format!("s={s:e} {} c={c:.3}: {b}", pose.name));
                }
                if !agrees {
                    failures.push(format!(
                        "s={s:e} {} c={c:.3}: expected {expect}, got {label}",
                        pose.name
                    ));
                }
            }
        }
    }
    for f in &failures {
        println!("[grid FAIL] {f}");
    }
    assert!(failures.is_empty(), "{} failures", failures.len());
}

/// **Near-tangent cuts**: the plane at y = 1 − δ (δ > 0 cuts the
/// cylinder in two rulings 2√(2δ) apart), across the zero band.
#[test]
fn probe_near_tangent_offsets() {
    let eps = tol().eps();
    let mut failures = vec![];
    let pose = poses(1.0)[0];
    for c in [0.5, 8.0] {
        let h = c * (2.0 * eps).sqrt();
        for m in [
            -3.0, -1.0, -0.5, -0.1, 0.0, 0.1, 0.5, 0.9, 1.0, 1.5, 3.0, 20.0,
        ] {
            let delta = m * eps;
            let (label, bad) = run(pose, 1.0, h, 1.0 - delta, eps);
            println!("[near] c={c} δ={m}ε :: {label}");
            for b in bad {
                // A δ > 0 seam lies off the cylinder by about δ by
                // construction; flag beyond ε only.
                failures.push(format!("c={c} δ={m}ε: {b}"));
            }
        }
    }
    for f in &failures {
        println!("[near FAIL] {f}");
    }
    assert!(failures.is_empty(), "{} failures", failures.len());
}

/// The conventional seam's downstream consumers: mesh, a later split
/// across the seam, a boolean notch through the seam, STEP round trip.
#[test]
fn probe_downstream_of_a_chart_seam() {
    let eps = tol().eps();
    let h = 0.5 * (2.0 * eps).sqrt();
    let pose = poses(1.0)[0];
    let halves = topo::split(&shoulder(pose, 1.0, h), &cut(pose, 1.0, 1.0), tol()).expect("cuts");
    let near = halves.above.body().expect("near").clone();
    let seams = audit(&near, pose, 1.0, eps);
    println!("[down] seams {seams:?}");
    assert!(
        seams.iter().any(|s| s.kind.starts_with("Chart")),
        "the conventional seam"
    );
    let near = finished("near piece", near, tol());

    // mesh
    match mesh::tessellate(&near, 1e-3, tol()) {
        Ok(m) => println!(
            "[down] mesh: check_mesh {:?}",
            mesh::validate::check_mesh(&m)
        ),
        Err(e) => println!("[down] mesh refuses: {e:?}"),
    }

    // A second split across the seam at z = h/2 (the pose normal).
    let (a_near, _) = areas(1.0);
    let z_cut = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, h / 2.0),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    match topo::split(&near, &z_cut, tol()) {
        Ok(r) => {
            for (side, p) in [("above", &r.above), ("below", &r.below)] {
                let b = p.body().unwrap();
                let v = topo::mass_properties(b, tol()).map(|m| m.volume);
                println!(
                    "[down] split across the seam {side}: tier3 {:?} volume {v:?} want {:e}; seams {:?}",
                    topo::validate_geometric(b, tol()),
                    a_near * h / 2.0,
                    audit(b, pose, 1.0, eps)
                );
            }
        }
        Err(e) => println!("[down] split across the seam refuses: {e:?}"),
    }

    // A boolean notch through the seam's middle.
    let notch = finished(
        "notch",
        sweep::test_support::brick((-0.3, 0.3), (0.9, 1.1), (h / 3.0, 2.0 * h / 3.0), tol()),
        tol(),
    );
    let removed_area = {
        let x = 0.3_f64;
        0.3 * 0.1 + (x / 2.0 * (1.0 - x * x).sqrt() + x.asin() / 2.0 - 0.9 * x)
    };
    match topo::subtract(&near, &notch, tol()) {
        Ok(r) => {
            let b = &r.body().expect("a body").body;
            let v = topo::mass_properties(b, tol()).map(|m| m.volume);
            println!(
                "[down] notch: tier3 {:?} volume {v:?} want {:e}",
                topo::validate_geometric(b, tol()),
                (a_near - removed_area / 3.0) * h
            );
        }
        Err(e) => println!("[down] notch refuses: {e:?}"),
    }
    match topo::union(&near, &notch, tol()) {
        Ok(r) => {
            let b = &r.body().expect("a body").body;
            println!(
                "[down] union notch: tier3 {:?} volume {:?}",
                topo::validate_geometric(b, tol()),
                topo::mass_properties(b, tol()).map(|m| m.volume)
            );
        }
        Err(e) => println!("[down] union notch refuses: {e:?}"),
    }

    // STEP round trip.
    let text = step_export::step_string(&near, &step_export::StepOptions::default(), tol());
    match text {
        Ok(text) => {
            match step_import::import_step(&text, &step_import::ImportOptions::default(), tol()) {
                Ok(_) => println!("[down] STEP round trip imports"),
                Err(e) => println!(
                    "[down] STEP re-import refuses: {}",
                    format!("{e:?}").chars().take(300).collect::<String>()
                ),
            }
        }
        Err(e) => println!("[down] STEP export refuses: {e:?}"),
    }

    // The transform of a chart-seam body.
    let moved = topo::transform_rigid(
        &near,
        &geom_core::Affine3::translation(Vec3::new(5.0, -3.0, 2.0)),
        tol(),
    );
    match moved {
        Ok(m) => println!(
            "[down] transform: tier3 {:?}",
            topo::validate_geometric(&m, tol())
        ),
        Err(e) => println!("[down] transform refuses: {e:?}"),
    }
}

/// Pre-existing on main? A thin extrusion of a profile with a tangent
/// line–arc joint (a stadium) — the sweep's own conventional smooth
/// join between distinct surfaces — through the STEP round trip.
#[test]
fn probe_step_round_trip_of_a_thin_tangent_joint_extrusion() {
    let eps = tol().eps();
    for h in [1.0, 1e-3, 0.5 * (2.0 * eps).sqrt()] {
        let stadium = bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 1.0),
            (Point2::new(2.0, 2.0), 0.0),
            (Point2::new(0.0, 2.0), 1.0),
        ])
        .with_tangent_joints(vec![0, 1, 2, 3]);
        let body = finished(
            "stadium",
            extruded(sweep::test_support::sketch_at(0.0), vec![stadium], h, tol()),
            tol(),
        );
        let kinds: Vec<String> = body
            .edges()
            .filter_map(|(_, e)| body.get_curve_geom(e.curve).and_then(|g| g.certified()))
            .filter(|cg| matches!(cg.carrier(), geom::Curve3::Line { .. }))
            .map(|cg| format!("{:?}", cg.description()).chars().take(20).collect())
            .collect();
        let text =
            step_export::step_string(&body, &step_export::StepOptions::default(), tol()).unwrap();
        let got = step_import::import_step(&text, &step_import::ImportOptions::default(), tol());
        println!(
            "[stadium] h={h:e} line edges {kinds:?} → import {}",
            match got {
                Ok(_) => "ok".to_string(),
                Err(e) => format!("{e:?}").chars().take(200).collect(),
            }
        );
    }
}

/// The cut turned about the tangent ruling by θ: the section table
/// reads it tangent while R(1 − cos θ) ≤ ε, and the rule's first-order
/// dihedral meters θ over the extent. Reaches the in-band first-order
/// arm and `Refuted`?
#[test]
fn probe_cut_turned_about_the_ruling() {
    let eps = tol().eps();
    let mut failures = vec![];
    for h in [1.0, 0.5 * (2.0 * eps).sqrt()] {
        for theta in [
            1e-10_f64, 1e-9, 3e-9, 1e-8, 3e-8, 1e-7, 1e-6, 1e-5, 4e-5, 1e-4,
        ] {
            let body = shoulder(poses(1.0)[0], 1.0, h);
            let normal = Vec3::new(theta.sin(), -theta.cos(), 0.0);
            let plane = topo::test_support::split_plane(Point3::new(0.0, 1.0, 0.0), normal, tol());
            let out = match topo::split(&body, &plane, tol()) {
                Ok(r) => {
                    let mut s = String::from("OK");
                    for (side, p) in [("near", &r.above), ("far", &r.below)] {
                        if let Some(b) = p.body() {
                            let t3 = topo::validate_geometric(b, tol());
                            if t3.is_err() {
                                failures.push(format!("h={h:e} θ={theta:e} {side}: {t3:?}"));
                            }
                            for seam in audit(b, poses(1.0)[0], 1.0, eps) {
                                s.push_str(&format!(
                                    " [{side} {} off_cyl={:.1e}]",
                                    seam.kind, seam.off_cylinder
                                ));
                            }
                        }
                    }
                    s
                }
                Err(e) => format!(
                    "ERR {}",
                    format!("{e:?}").chars().take(140).collect::<String>()
                ),
            };
            println!("[turn] h={h:.3e} θ={theta:e} :: {out}");
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

/// The split_edge filing's measurement, reproduced.
#[test]
fn probe_split_edge_filing() {
    let pose = poses(1.0)[0];
    let halves = topo::split(&shoulder(pose, 1.0, 1.0), &cut(pose, 1.0, 1.0), tol()).expect("cuts");
    let near = finished("near", halves.above.body().unwrap().clone(), tol());
    for d in [1e-5, 3e-5, 5e-5, 1e-4, 2e-4] {
        for (label, z, n) in [
            ("z=1-d,+z", 1.0 - d, 1.0),
            ("z=1-d,-z", 1.0 - d, -1.0),
            ("z=d,+z", d, 1.0),
        ] {
            let plane = topo::test_support::split_plane(
                Point3::new(0.0, 0.0, z),
                Vec3::new(0.0, 0.0, n),
                tol(),
            );
            let out = match topo::split(&near, &plane, tol()) {
                Ok(_) => "cuts".to_string(),
                Err(e) => format!("{e:?}").chars().take(160).collect(),
            };
            println!("[split_edge] d={d:e} {label} :: {out}");
        }
    }
}

/// The band edge: δ close to ±ε, on both routes (c = 0.5 chart, c = 8
/// intrinsic). Ok-but-not-tier-3 is the failure.
#[test]
fn probe_band_edge_offsets() {
    let eps = tol().eps();
    let pose = poses(1.0)[0];
    let mut failures = vec![];
    for c in [0.5, 0.9, 8.0] {
        let h = c * (2.0 * eps).sqrt();
        for m in [
            -1.0001, -1.0, -0.9999, -0.999, -0.99, -0.95, 0.95, 0.99, 0.999, 0.9999, 1.0,
        ] {
            let y_c = 1.0 - m * eps;
            let body = shoulder(pose, 1.0, h);
            let out = match topo::split(&body, &cut(pose, 1.0, y_c), tol()) {
                Ok(r) => {
                    let mut s = String::from("OK");
                    for (side, p) in [("near", &r.above), ("far", &r.below)] {
                        let b = p.body().unwrap();
                        match topo::validate_geometric(b, tol()) {
                            Ok(()) => {}
                            Err(e) => {
                                let faces: Vec<String> = b
                                    .faces()
                                    .map(|(k, f)| {
                                        format!(
                                            "{k:?}:{:?}",
                                            b.get_surface(f.surface).unwrap().kind()
                                        )
                                    })
                                    .collect();
                                let text = format!(
                                    "{side} tier3 {}  faces {faces:?}",
                                    format!("{e:?}").chars().take(400).collect::<String>()
                                );
                                s.push_str(&format!(" [{text}]"));
                                failures.push(format!("c={c} δ={m}ε {text}"));
                            }
                        }
                        for seam in audit(b, pose, 1.0, eps) {
                            s.push_str(&format!(
                                " [{side} {} off_cyl={:.3e}]",
                                seam.kind, seam.off_cylinder
                            ));
                        }
                    }
                    s
                }
                Err(e) => format!(
                    "ERR {}",
                    format!("{e:?}").chars().take(120).collect::<String>()
                ),
            };
            println!("[edge] c={c} δ={m}ε :: {out}");
        }
    }
    for f in &failures {
        println!("[edge FAIL] {f}");
    }
    assert!(failures.is_empty(), "{} failures", failures.len());
}
