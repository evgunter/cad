//! Reviewer probes (reach-dual3805-r1): the tilted-cut drum against balls
//! and rods, widened past the PR's poses. Oracles are closed forms and
//! pointwise membership, never the kernel.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

fn drum(s: f64, tilt: f64) -> Body<f64> {
    let tol = Tol::witness();
    let r = 0.5 * s;
    let cyl = sweep::test_support::prism(
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        s,
        tol,
    );
    let plane = topo::splitting::SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5 * s),
        normal: Vec3::new(tilt.sin(), 0.0, tilt.cos()),
    };
    let res = topo::splitting::split(&cyl, &plane, tol).expect("split");
    let topo::splitting::SplitPart::Body(b) = res.below else { panic!() };
    b
}
fn ball(r: f64, c: [f64; 3]) -> Body<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::translation(Vec3::new(c[0], c[1], c[2])), Tol::witness())
        .unwrap()
}
fn rod(r: f64, x: f64, y: f64, z0: f64, h: f64) -> Body<f64> {
    sweep::test_support::prism_at(
        vec![(Point2::new(x - r, y), 1.0), (Point2::new(x + r, y), 1.0)],
        z0,
        h,
        Tol::witness(),
    )
}
fn mv(b: &Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, m, Tol::witness()).unwrap()
}
fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<topo::BooleanResult<f64>, topo::BooleanError> {
    let t = Tol::witness();
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, t),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, t),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, t),
    }
}
fn vol(b: &Body<f64>) -> (f64, f64) {
    let p = topo::mass_properties(b, Tol::witness()).expect("mass");
    (p.volume, p.volume_pad)
}
fn check(label: &str, b: &Body<f64>, want: f64, shells: usize) {
    assert_eq!(topo::validate(b), Ok(()), "{label}");
    assert_eq!(topo::validate_closed(b), Ok(()), "{label}");
    assert_eq!(topo::validate_geometric(b, Tol::witness()), Ok(()), "{label}");
    let (v, pad) = vol(b);
    let rel = (v - want).abs() / want.abs().max(1e-300);
    eprintln!("{label}: vol {v:.10e} want {want:.10e} pad {pad:.2e} rel {rel:.2e} shells {}", b.shells().count());
    assert!((v - want).abs() <= pad + 1e-9 * want.abs(), "{label}: vol {v} want {want} pad {pad}");
    assert_eq!(b.shells().count(), shells, "{label}: shells");
}

/// Disjoint-or-nested outcomes: per op the expected volume and shell count
/// (None = empty).
fn table(label: &str, a: &Body<f64>, b: &Body<f64>, rows: [(BooleanOp, bool, Option<(f64, usize)>); 6]) {
    for (op, swap, want) in rows {
        let (x, y) = if swap { (b, a) } else { (a, b) };
        let l = format!("{label} {op:?} swap={swap}");
        let out = run(op, x, y);
        match (out, want) {
            (Ok(r), Some((v, n))) => {
                let body = &r.body().unwrap_or_else(|| panic!("{l}: empty, want {v}")).body;
                check(&l, body, v, n);
            }
            (Ok(r), None) => assert!(r.body().is_none(), "{l}: want empty"),
            (Err(e), _) => eprintln!("{l}: REFUSED {e:?}"),
        }
    }
}

fn held(label: &str, a: &Body<f64>, b: &Body<f64>, va: f64, vb: f64) {
    use BooleanOp::*;
    table(label, a, b, [
        (Union, false, Some((va, 1))),
        (Union, true, Some((va, 1))),
        (Intersect, false, Some((vb, 1))),
        (Intersect, true, Some((vb, 1))),
        (Subtract, false, Some((va - vb, 2))),
        (Subtract, true, None),
    ]);
}
fn apart(label: &str, a: &Body<f64>, b: &Body<f64>, va: f64, vb: f64) {
    use BooleanOp::*;
    table(label, a, b, [
        (Union, false, Some((va + vb, 2))),
        (Union, true, Some((va + vb, 2))),
        (Intersect, false, None),
        (Intersect, true, None),
        (Subtract, false, Some((va, 1))),
        (Subtract, true, Some((vb, 1))),
    ]);
}

#[test]
fn p1_held_inside_scaled_and_reposed() {
    for s in [1e-3, 1.0, 1e3] {
        let a = drum(s, 0.3);
        let va = PI * (0.5 * s).powi(2) * 0.5 * s;
        let (r, c) = (0.18 * s, [0.0, 0.0, 0.3 * s]);
        held(&format!("ball s={s}"), &a, &ball(r, c), va, 4.0 / 3.0 * PI * r.powi(3));
        let rr = 0.45 * s;
        held(&format!("rod s={s}"), &a, &rod(rr, 0.0, 0.0, 0.1 * s, 0.255 * s), va, PI * rr * rr * 0.255 * s);
    }
    // a rigid re-pose of the whole pair
    let m = Affine3::rotation_about_axis(Point3::new(0.3, -1.0, 2.0), Vec3::new(1.0, 2.0, 3.0).normalize(), 0.7);
    let a = mv(&drum(1.0, 0.3), &m);
    let va = PI * 0.125;
    held("ball reposed", &a, &mv(&ball(0.15, [0.3, 0.0, 0.2]), &m), va, 4.0 / 3.0 * PI * 0.15f64.powi(3));
    held("rod reposed", &a, &mv(&rod(0.3, 0.15, 0.05, 0.05, 0.3), &m), va, PI * 0.09 * 0.3);
}

#[test]
fn p2_steep_tilt_held_and_apart() {
    for tilt in [0.6, 0.9] {
        let a = drum(1.0, tilt);
        let va = PI * 0.125;
        // below the cut everywhere: cut height at x is 0.5 - x tan(tilt)
        let c = [-0.1, 0.0, 0.25];
        let r = 0.12;
        held(&format!("tilt {tilt} ball"), &a, &ball(r, c), va, 4.0 / 3.0 * PI * r.powi(3));
    }
    let a = drum(1.0, 0.3);
    let va = PI * 0.125;
    // outside the wall, beside the rim, inside the rim's box reach
    apart("ball beside rim", &a, &ball(0.1, [0.66, 0.0, 0.35]), va, 4.0 / 3.0 * PI * 1e-3);
    // above the cut, inside the cylinder carrier (nested arm), near the rim
    apart("ball above cut", &a, &ball(0.06, [0.4, 0.0, 0.5]), va, 4.0 / 3.0 * PI * 0.06f64.powi(3));
    // above the whole drum on the axis: inside the carrier, outside every face
    apart("ball over drum", &a, &ball(0.2, [0.0, 0.0, 1.5]), va, 4.0 / 3.0 * PI * 0.008);
    // below the floor, wider than... no: inside carrier below z = 0
    apart("ball under drum", &a, &ball(0.2, [0.1, 0.0, -0.3]), va, 4.0 / 3.0 * PI * 0.008);
}

/// A ball that barely crosses the rim must never come back as two
/// disjoint shells or a held-inside verdict.
#[test]
fn p3_near_rim_crossings_never_ship_disjoint() {
    let a = drum(1.0, 0.3);
    let t = 0.3f64;
    // rim point at azimuth 0: (0.5, 0, 0.5 - 0.5 tan t)
    let rim = [0.5, 0.0, 0.5 - 0.5 * t.tan()];
    // outward bisector of the wall normal (1,0,0) and the cut normal
    let n = Vec3::new(1.0 + t.sin(), 0.0, t.cos()).normalize();
    for (r, depth) in [(0.05, 1e-3), (0.05, 1e-5), (0.02, 1e-7), (0.05, -1e-5), (0.05, -1e-3)] {
        let d = r - depth; // centre distance from rim point
        let c = [rim[0] + n.x * d, rim[1], rim[2] + n.z * d];
        let b = ball(r, c);
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            match run(op, &a, &b) {
                Err(e) => eprintln!("rim depth {depth} r {r} {op:?}: refused {e:?}"),
                Ok(res) => {
                    let got = res.body().map(|b| (b.body.shells().count(), vol(&b.body)));
                    eprintln!("rim depth {depth} r {r} {op:?}: BUILT {got:?}");
                    if depth > 0.0 {
                        // crossing: union is one shell, intersection non-empty
                        match op {
                            BooleanOp::Union => assert_eq!(got.unwrap().0, 1, "crossing union two shells"),
                            BooleanOp::Intersect => assert!(got.is_some(), "crossing ∩ empty"),
                            BooleanOp::Subtract => {}
                        }
                    }
                }
            }
        }
    }
}

fn in_drum(p: [f64; 3], tilt: f64) -> f64 {
    // signed clearance, positive inside
    let wall = 0.5 - (p[0] * p[0] + p[1] * p[1]).sqrt();
    let floor = p[2];
    let cut = -((p[0]) * tilt.sin() + (p[2] - 0.5) * tilt.cos());
    wall.min(floor).min(cut)
}

#[test]
fn p4_point_in_solid_and_reuse() {
    let a = drum(1.0, 0.3);
    let (r, c) = (0.15, [0.3, 0.0, 0.2]);
    let b = ball(r, c);
    let diff = run(BooleanOp::Subtract, &a, &b).expect("A-B").body().unwrap().body.clone();
    let band = Band::linear(Tol::witness()).unwrap();
    let mut seed = 0x1234_5678_9abc_def0u64;
    let mut rnd = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    let (mut n, mut bad, mut refused) = (0, 0, 0);
    for _ in 0..600 {
        let p = [rnd() * 1.2 - 0.6, rnd() * 1.2 - 0.6, rnd() * 1.2 - 0.1];
        let dball = ((p[0] - c[0]).powi(2) + (p[1] - c[1]).powi(2) + (p[2] - c[2]).powi(2)).sqrt() - r;
        let s = in_drum(p, 0.3).min(dball);
        if s.abs() < 1e-6 {
            continue;
        }
        n += 1;
        match topo::point_in_solid(&diff, Point3::from_array(p), band, Tol::witness()) {
            Ok(topo::SolidContainment::In) if s > 0.0 => {}
            Ok(topo::SolidContainment::Out) if s < 0.0 => {}
            Ok(v) => {
                bad += 1;
                eprintln!("MISMATCH at {p:?}: {v:?}, oracle {s}");
            }
            Err(e) => { refused += 1; if refused < 4 { eprintln!("PIS refusal at {p:?}: {e:?}"); } }
        }
    }
    eprintln!("pis: {n} sampled, {bad} wrong, {refused} refused");
    assert_eq!(bad, 0);
    // reuse: (A - B) ∪ B = A ; (A - B) ∩ B = ∅ ; A - (A - B) = B
    let va = PI * 0.125;
    let vb = 4.0 / 3.0 * PI * r.powi(3);
    for (l, op, x, y, want) in [
        ("(A-B)∪B", BooleanOp::Union, &diff, &b, Some(va)),
        ("(A-B)∩B", BooleanOp::Intersect, &diff, &b, None),
        ("A-(A-B)", BooleanOp::Subtract, &a, &diff, Some(vb)),
    ] {
        match run(op, x, y) {
            Err(e) => eprintln!("{l}: REFUSED {e:?}"),
            Ok(res) => match (res.body(), want) {
                (Some(bb), Some(v)) => check(l, &bb.body, v, 1),
                (None, None) => eprintln!("{l}: empty ok"),
                (g, w) => panic!("{l}: got {} want {w:?}", g.is_some()),
            },
        }
    }
}

fn cut(body: &Body<f64>, o: [f64; 3], n: Vec3<f64>, keep_below: bool) -> Body<f64> {
    let plane = topo::splitting::SplitPlane { origin: Point3::from_array(o), normal: n.normalize() };
    let res = topo::splitting::split(body, &plane, Tol::witness()).expect("split");
    let part = if keep_below { res.below } else { res.above };
    let topo::splitting::SplitPart::Body(b) = part else { panic!("empty part") };
    b
}

/// Wall placement on walls bounded by planar sections that cross the
/// seams, twice-cut walls, and points within 1e-3 / 1e-6 of a section.
#[test]
fn p5_wall_placement_against_sections() {
    let band = Band::linear(Tol::witness()).unwrap();
    let r = 0.5;
    let base = sweep::test_support::prism(
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        1.0,
        Tol::witness(),
    );
    let mut stats = [0usize; 4];
    for (tilt, rot, second) in [(0.3f64, 0.4f64, false), (1.1, 0.4, false), (0.3, 1.9, true), (0.7, -0.8, true)] {
        let n1 = Vec3::new(tilt.sin() * f64::cos(rot), tilt.sin() * f64::sin(rot), tilt.cos());
        let o1 = [0.0, 0.0, 0.6];
        let mut body = cut(&base, o1, n1, true);
        let n2 = Vec3::new(-0.5f64.sin(), 0.2, 0.5f64.cos()).normalize();
        let o2 = [0.0, 0.0, 0.15];
        if second {
            body = cut(&body, o2, n2, false);
        }
        let n1u = n1.normalize();
        let inside = |p: Point3<f64>| {
            let s1 = -(p - Point3::from_array(o1)).dot(n1u);
            let s2 = if second { (p - Point3::from_array(o2)).dot(n2).min(p.z) } else { p.z };
            s1.min(s2).min(1.0 - p.z)
        };
        let walls: Vec<_> = body
            .faces()
            .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(geom::Surface::Cylinder { .. })))
            .map(|(k, _)| k)
            .collect();
        for k in 0..97 {
            let az = f64::from(k) * core::f64::consts::TAU / 97.0 + 1e-3;
            let (x, y) = (r * az.cos(), r * az.sin());
            // heights: a grid plus points straddling each section
            let mut zs: Vec<f64> = (0..20).map(|j| -0.05 + 1.1 * f64::from(j) / 19.0).collect();
            let z1 = 0.6 - (x * n1u.x + y * n1u.y) / n1u.z;
            for d in [1e-3, 1e-6, -1e-3, -1e-6] {
                zs.push(z1 + d);
                if second {
                    zs.push(0.15 - (x * n2.x + y * n2.y) / n2.z + d);
                }
            }
            for z in zs {
                let q = Point3::new(x, y, z);
                let s = inside(q);
                let mut ins = 0;
                let mut undecided = false;
                for &f in &walls {
                    match topo::curved_face_containment(&body, f, q, band) {
                        Ok(Some(topo::FaceContainment::In)) => ins += 1,
                        Ok(Some(topo::FaceContainment::Out)) => {}
                        _ => undecided = true,
                    }
                }
                if undecided {
                    stats[2] += 1;
                    continue;
                }
                if s.abs() < 1e-9 {
                    continue;
                }
                let want = usize::from(s > 0.0);
                if ins != want {
                    stats[3] += 1;
                    eprintln!("WRONG tilt {tilt} rot {rot} second {second} ({x:.4},{y:.4},{z:.7}) s {s:e}: ins {ins}");
                } else {
                    stats[want] += 1;
                }
            }
        }
    }
    eprintln!("placement: out {} in {} undecided {} wrong {}", stats[0], stats[1], stats[2], stats[3]);
    assert_eq!(stats[3], 0);
}
