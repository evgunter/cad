//! PR 3814 delta-review probe. To run: copy to crates/sweep/tests/, add
//! `#[path = "delta3814_probe.rs"] mod delta3814_probe;` to all.rs, then
//! `cargo nextest run -p sweep -E 'test(delta3814)' --no-capture`.
//!
//! Delta review of PR 3814's fix pass: both operand orders over
//! azimuths, spans, scales and poses, each body against the closed
//! form, tier 3, the census, a twin-survival scan and an independent
//! `point_in_solid` membership oracle. Reports; asserts only that no
//! body is WRONG (a typed refusal is counted, not failed).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::{extruded, sketch_at};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{
    Body, BooleanDeclarations, BooleanOp, BooleanResult, ContactClass, FacePairDeclaration,
    SolidContainment, mass_properties,
};

const R_IN: f64 = 0.5;
const R_OUT: f64 = 1.5;

fn collar(s: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(R_IN * s, s),
        Point2::new(R_OUT * s, s),
        Point2::new(R_OUT * s, 2.0 * s),
        Point2::new(R_IN * s, 2.0 * s),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

fn shaft(s: f64, deg: f64, y0: f64, h: f64) -> Body<f64> {
    let peg = extruded(
        sketch_at(y0 * s),
        vec![crate::common::three_arc(Point2::new(0.0, 0.0), R_IN * s, deg)],
        h * s,
        Tol::witness(),
    );
    let up = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), -FRAC_PI_2);
    topo::transform_rigid(&peg, &up, Tol::witness()).unwrap()
}

fn walls(b: &Body<f64>, r: f64) -> Vec<topo::FaceKey> {
    b.faces()
        .filter(|(_, f)| {
            matches!(b.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - r).abs() < 1e-9 * r.max(1.0))
        })
        .map(|(k, _)| k)
        .collect()
}

fn decls(a: &Body<f64>, b: &Body<f64>, r: f64) -> BooleanDeclarations {
    let mut d = BooleanDeclarations::none();
    for &fa in &walls(a, r) {
        for &fb in &walls(b, r) {
            d.coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    d
}

/// The oracle in the unposed frame.
fn inside(p: Point3<f64>, s: f64, y0: f64, h: f64) -> bool {
    let r = (p.x * p.x + p.z * p.z).sqrt() / s;
    let y = p.y / s;
    (r > R_IN && r < R_OUT && y > 1.0 && y < 2.0) || (r < R_IN && y > y0 && y < y0 + h)
}

/// Far (relative margin `m`) from every boundary of either operand.
fn clear(p: Point3<f64>, s: f64, y0: f64, h: f64, m: f64) -> bool {
    let r = (p.x * p.x + p.z * p.z).sqrt() / s;
    let y = p.y / s;
    [r - R_IN, r - R_OUT, y - 1.0, y - 2.0, y - y0, y - y0 - h]
        .iter()
        .all(|d| d.abs() > m)
}

#[derive(Default, Debug)]
struct Tally {
    ok: usize,
    refused: std::collections::BTreeMap<String, usize>,
    wrong: Vec<String>,
    max_rel: f64,
}

#[allow(clippy::too_many_arguments)]
fn one(
    t: &mut Tally,
    tag: &str,
    a: &Body<f64>,
    b: &Body<f64>,
    s: f64,
    y0: f64,
    h: f64,
    pose: &Affine3<f64>,
) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let d = decls(a, b, R_IN * s);
    let bb = match topo::union_with(a, b, &d, tol) {
        Ok(BooleanResult::Body(bb)) => bb,
        Ok(BooleanResult::Empty) => {
            t.wrong.push(format!("{tag}: EMPTY"));
            return;
        }
        Err(e) => {
            let k = format!("{e:?}");
            let k: String = k.chars().take(90).collect();
            let k = format!("{} :: {k}", tag.split(" pose").next().unwrap_or("") .to_string() + &tag[tag.find(" az").unwrap_or(0)..]);
            *t.refused.entry(k).or_default() += 1;
            return;
        }
    };
    let want = (PI * (R_OUT * R_OUT - R_IN * R_IN) * 1.0 + PI * R_IN * R_IN * h) * s.powi(3);
    let got = mass_properties(&bb.body, tol).unwrap().volume;
    let rel = (got - want).abs() / want;
    t.max_rel = t.max_rel.max(rel);
    let mut bad = Vec::new();
    if rel > 1e-11 {
        bad.push(format!("volume {got} vs {want} (rel {rel:e})"));
    }
    if bb.body.shells().count() != 1 {
        bad.push(format!("shells {}", bb.body.shells().count()));
    }
    if let Err(e) = topo::validate_geometric(&bb.body, tol) {
        bad.push(format!("tier3 {e:?}"));
    }
    if let Err(e) = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol) {
        bad.push(format!("census {e:?}"));
    }
    // Twin survival: no edge of the result may run through the open
    // contact band (r = R_IN, 1 < y < 2, and inside the shaft's span).
    let inv = pose.inverse();
    for (_, e) in bb.body.edges() {
        let Some(c) = bb
            .body
            .get_curve_geom(e.curve)
            .and_then(topo::null::CurveGeom::certified)
        else {
            continue;
        };
        let (t0, t1) = c.params();
        for f in [0.25, 0.5, 0.75] {
            let q = inv.transform_point(c.carrier().eval(t0 + (t1 - t0) * f));
            let r = (q.x * q.x + q.z * q.z).sqrt() / s;
            let y = q.y / s;
            let (lo, hi) = (1.0f64.max(y0), 2.0f64.min(y0 + h));
            if (r - R_IN).abs() < 1e-6 && y > lo + 1e-6 && y < hi - 1e-6 {
                bad.push(format!("edge survives in the contact band at r {r} y {y}"));
            }
        }
    }
    // Membership oracle.
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut rnd = || {
        seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((seed >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let mut checked = 0;
    let mut pis_err = 0;
    while checked < 40 {
        let q = Point3::new(
            (rnd() * 3.4 - 1.7) * s,
            (rnd() * 2.4 + 0.3) * s,
            (rnd() * 3.4 - 1.7) * s,
        );
        if !clear(q, s, y0, h, 2e-3) {
            continue;
        }
        checked += 1;
        let want_in = inside(q, s, y0, h);
        match topo::point_in_solid(&bb.body, pose.transform_point(q), band, tol) {
            Ok(SolidContainment::In) if want_in => {}
            Ok(SolidContainment::Out) if !want_in => {}
            Ok(other) => bad.push(format!("membership at {q:?}: {other:?}, want in={want_in}")),
            Err(_) => pis_err += 1,
        }
    }
    if pis_err > 0 {
        *t.refused
            .entry("point_in_solid errored on the result (oracle unread)".into())
            .or_default() += 1;
    }
    if bad.is_empty() {
        t.ok += 1;
    } else {
        t.wrong.push(format!("{tag}: {}", bad.join("; ")));
    }
}

const AZ: [f64; 7] = [0.0, 1e-7, 17.0, 60.0, 119.99, 180.0, 300.0];
const SPANS: [(&str, f64, f64); 4] = [
    ("through", 0.5, 2.0),
    ("flush", 1.0, 1.0),
    ("flush-low proud-high", 1.0, 1.5),
    ("proud-low flush-high", 0.5, 1.5),
];

/// `full_turn_bore_mate::split_collar`, at scale `s`.
fn split_collar(s: f64) -> Body<f64> {
    let tol = Tol::witness();
    let mut c = collar(s);
    let bore = walls(&c, R_IN * s)[0];
    let skey = c.get_face(bore).unwrap().surface;
    let carrier = |c: &Body<f64>, e: &topo::Edge| {
        c.get_curve_geom(e.curve)
            .and_then(topo::null::CurveGeom::certified)
            .map(|g| g.carrier().clone())
    };
    let (seam, origin, dir) = c
        .edges()
        .find_map(|(k, e)| match carrier(&c, e) {
            Some(geom::Curve3::Line { origin, dir })
                if c.face_of_half_edge(e.he_plus) == Some(bore)
                    && c.face_of_half_edge(e.he_minus) == Some(bore) =>
            {
                Some((k, origin, dir))
            }
            _ => None,
        })
        .unwrap();
    let w = c.split_edge(seam, (1.5 * s - origin.y) / dir.y, tol).unwrap().vertex;
    let rim = c
        .edges()
        .find_map(|(_, e)| match carrier(&c, e) {
            Some(geom::Curve3::Circle { center, axis, radius, u_ref })
                if (radius - R_IN * s).abs() < 1e-12 * s.max(1.0) && (center.y - s).abs() < 1e-12 * s.max(1.0) =>
            {
                Some(geom::Curve3::Circle { center: Point3::new(center.x, 1.5 * s, center.z), axis, radius, u_ref })
            }
            _ => None,
        })
        .unwrap();
    let pw = *c.get_point(c.get_vertex(w).unwrap().point).unwrap();
    let t0 = rim.param_near(pw, 0.0).unwrap();
    let spec = geom_brep::EdgeCurveSpec::arc_of_circle(rim, t0, t0 + core::f64::consts::TAU)
        .unwrap()
        .at_rest_in_chart(skey, false);
    let visits: Vec<_> = c
        .half_edges()
        .filter(|&(k, h)| h.start == w && c.face_of_half_edge(k) == Some(bore))
        .map(|(k, _)| k)
        .collect();
    let [down, up] = visits[..] else { panic!() };
    c.mef(topo::MefSite::Chords { he1: up, he2: down }, spec, topo::FaceSurface::Inherit, tol)
        .unwrap();
    c
}

fn run(scales: &[f64], all_poses: bool) -> Tally {
    run_with(scales, all_poses, collar)
}

fn run_with(scales: &[f64], all_poses: bool, collar: fn(f64) -> Body<f64>) -> Tally {
    let mut t = Tally::default();
    let poses: Vec<(&str, Affine3<f64>)> = if all_poses {
        crate::common::poses::poses()
    } else {
        vec![("identity", Affine3::identity())]
    };
    for &s in scales {
        for (pn, pose) in &poses {
            let c = topo::transform_rigid(&collar(s), pose, Tol::witness()).unwrap();
            for deg in AZ {
                for (span, y0, h) in SPANS {
                    let p =
                        topo::transform_rigid(&shaft(s, deg, y0, h), pose, Tol::witness()).unwrap();
                    for (ord, a, b) in [("c∪s", &c, &p), ("s∪c", &p, &c)] {
                        let tag = format!("s {s} pose {pn} az {deg} {span} {ord}");
                        one(&mut t, &tag, a, b, s, y0, h, pose);
                    }
                }
            }
        }
    }
    t
}

fn report(name: &str, t: &Tally) {
    println!("DELTA {name}: ok {} max_rel {:e}", t.ok, t.max_rel);
    for (k, n) in &t.refused {
        println!("DELTA {name}: refused x{n}: {k}");
    }
    for w in &t.wrong {
        println!("DELTA {name}: WRONG {w}");
    }
}

#[test]
fn delta_scales_identity() {
    let t = run(&[1e-3, 1.0, 1e3], false);
    report("scales", &t);
    assert!(t.wrong.is_empty(), "{} wrong bodies", t.wrong.len());
}

#[test]
fn delta_poses_unit_scale() {
    let t = run(&[1.0], true);
    report("poses", &t);
    assert!(t.wrong.is_empty(), "{} wrong bodies", t.wrong.len());
}

/// ∩ and both differences at every azimuth/span, both orders' decls:
/// what comes back, and never a body that disagrees with the closed
/// form (∩ empty, each difference its minuend whole).
#[test]
fn delta_other_ops() {
    let tol = Tol::witness();
    let mut seen = std::collections::BTreeMap::<String, usize>::new();
    for deg in AZ {
        for (_, y0, h) in SPANS {
            let c = collar(1.0);
            let p = shaft(1.0, deg, y0, h);
            for (op, a, b) in [
                (BooleanOp::Intersect, &c, &p),
                (BooleanOp::Intersect, &p, &c),
                (BooleanOp::Subtract, &c, &p),
                (BooleanOp::Subtract, &p, &c),
            ] {
                let d = decls(a, b, R_IN);
                let out = topo::boolean_op_with(op, a, b, &d, topo::SweepStrategy::Realized, tol);
                let k = match out {
                    Err(e) => format!("{op:?} Err {}", format!("{e:?}").chars().take(60).collect::<String>()),
                    Ok(BooleanResult::Empty) => format!("{op:?} Empty"),
                    Ok(BooleanResult::Body(bb)) => {
                        let v = mass_properties(&bb.body, tol).unwrap().volume;
                        let va = mass_properties(a, tol).unwrap().volume;
                        format!("{op:?} Body v {v:.12} (minuend {va:.12})")
                    }
                };
                *seen.entry(k).or_default() += 1;
            }
        }
    }
    for (k, n) in seen {
        println!("DELTA ops x{n}: {k}");
    }
}

#[test]
fn delta_split_bore() {
    let t = run_with(&[1e-3, 1.0, 1e3], true, split_collar);
    report("split", &t);
    assert!(t.wrong.is_empty(), "{} wrong bodies", t.wrong.len());
}

/// The blind-shaft item's table, re-measured.
#[test]
fn delta_blind_table() {
    let tol = Tol::witness();
    for (span, y0, h) in [("below", 0.5, 1.0), ("above", 1.5, 1.0), ("inside", 1.2, 0.6)] {
        for deg in [0.0, 60.0, 90.0] {
            let c = collar(1.0);
            let p = shaft(1.0, deg, y0, h);
            for (ord, a, b) in [("c∪s", &c, &p), ("s∪c", &p, &c)] {
                let out = topo::union_with(a, b, &decls(a, b, R_IN), tol);
                let k = match out {
                    Err(e) => format!("{e:?}").chars().take(80).collect::<String>(),
                    Ok(_) => "BODY".into(),
                };
                println!("DELTA blind {span} {deg} {ord}: {k}");
            }
        }
    }
}
