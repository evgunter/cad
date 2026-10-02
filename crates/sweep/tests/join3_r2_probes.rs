//! JOIN-3 review lane r2: differential probes for the ring lane's
//! closing curve. `#[ignore]`d; each prints one line per pose so two
//! trees can be diffed.
//!
//! - `j3r2_pocket_battery`: the block `[−1, 1]² × [0, 1]` against an
//!   upright prism whose profile mixes lines and arcs (D shapes with
//!   major, half, minor and sliver arcs; a stadium; a lens; a
//!   tombstone), turned and offset in the block's top, entering from
//!   the top, the bottom, through, and flush with either cap, under ∪,
//!   ∖ and ∩ in both orders. Oracle: the profile's own area (bulge
//!   closed form, cross-checked against the operand's volume) times the
//!   z-overlap.
//! - `j3r2_tilted_battery`: the same profiles on tilted sketch planes,
//!   so the block's caps cut the cutter's cylinder walls in ELLIPSE
//!   arcs. Oracle: inclusion–exclusion across the three ops, and the
//!   oblique-prism closed form `S / cos θ` where the cutter crosses the
//!   slab whole.
//! - `j3r2_rand`: R2's random z-prism battery with the certificate and
//!   a legal-operand column added.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, extrude};
use topo::Body;

fn tol() -> Tol {
    Tol::witness()
}

type Shape = Vec<((f64, f64), f64)>;

fn body_of(plane: SketchPlane<f64>, shape: &Shape, h: f64) -> Body<f64> {
    let lp = bulge_loop(
        shape
            .iter()
            .map(|&((x, y), b)| (Point2::new(x, y), b))
            .collect(),
    );
    let p = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(&p, Extrusion::Distance(h), tol()).unwrap().body
}

/// Signed area of a bulge loop: the chord polygon plus each arc's
/// segment (bulge `b = tan(θ/4)`).
fn bulge_area(shape: &Shape) -> f64 {
    let n = shape.len();
    let mut a = 0.0;
    for i in 0..n {
        let ((x0, y0), b) = shape[i];
        let ((x1, y1), _) = shape[(i + 1) % n];
        a += 0.5 * (x0 * y1 - x1 * y0);
        if b != 0.0 {
            let th = 4.0 * b.atan();
            let l = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
            let r = l / (2.0 * (th / 2.0).sin()).abs();
            a += 0.5 * r * r * (th - th.sin());
        }
    }
    a
}

fn turn(shape: &Shape, phi: f64, (dx, dy): (f64, f64)) -> Shape {
    let (s, c) = phi.sin_cos();
    shape
        .iter()
        .map(|&((x, y), b)| ((c * x - s * y + dx, s * x + c * y + dy), b))
        .collect()
}

fn d_shape(flat: f64) -> Shape {
    let c = sweep::test_support::rod_chord_at(flat);
    vec![((flat, c.half), c.wall_bulge), ((flat, -c.half), 0.0)]
}

fn shapes() -> Vec<(String, Shape)> {
    let v = shapes_raw();
    v.into_iter()
        .filter(|(name, sh)| {
            let lp = bulge_loop(sh.iter().map(|&((x, y), b)| (Point2::new(x, y), b)).collect());
            let ok = Profile::new(SketchPlane::xy(), vec![lp]).validate(tol());
            if let Err(e) = &ok {
                println!("J3R2 SHAPE-REFUSED {name}: {e:?}");
            }
            ok.is_ok()
        })
        .collect()
}

fn shapes_raw() -> Vec<(String, Shape)> {
    let mut v = Vec::new();
    for flat in [0.3, 0.0, -0.3, 0.45, 0.49, 0.4999, -0.45, -0.49, -0.4999, -0.49999, -0.4999999] {
        v.push((format!("D{flat}"), d_shape(flat)));
    }
    v.push((
        "stadium".into(),
        vec![
            ((-0.3, -0.2), 0.0),
            ((0.3, -0.2), 0.6),
            ((0.3, 0.2), 0.0),
            ((-0.3, 0.2), 0.6),
        ],
    ));
    v.push((
        "lens".into(),
        vec![((0.0, -0.4), 0.3), ((0.0, 0.4), 0.3)],
    ));
    v.push((
        "tomb".into(),
        vec![
            ((-0.3, -0.3), 0.0),
            ((0.3, -0.3), 0.0),
            ((0.3, 0.1), 0.6),
            ((-0.3, 0.1), 0.0),
        ],
    ));
    v.push((
        "notch".into(),
        vec![
            ((-0.4, -0.3), 0.0),
            ((0.4, -0.3), 0.0),
            ((0.4, 0.3), 0.0),
            ((0.1, 0.3), -1.0),
            ((-0.1, 0.3), 0.0),
            ((-0.4, 0.3), 0.0),
        ],
    ));
    v
}

fn outcome(r: Result<topo::BooleanResult<f64>, topo::BooleanError>, want: Option<f64>) -> (String, Option<f64>) {
    match r {
        Err(e) => {
            let s = format!("{e:?}");
            let cut: String = s.chars().take(110).collect();
            (format!("ERR {cut}"), None)
        }
        Ok(r) => match r.body() {
            None => (
                match want {
                    Some(w) if w.abs() > 1e-9 => format!("EMPTY WRONG want={w}"),
                    _ => "EMPTY ok".into(),
                },
                Some(0.0),
            ),
            Some(bb) => {
                let t2 = topo::validate_closed(&bb.body).is_ok();
                let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).is_ok();
                let cert = topo::validate_geometric_certificate(&bb.body, tol()).is_ok();
                let far =
                    sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol());
                let legal = match topo::union(&bb.body, &far, tol()) {
                    Ok(_) => "legal".to_string(),
                    Err(e) => format!("NONOPERAND {:?}", e.kind()),
                };
                match topo::mass_properties(&bb.body, tol()).map(|m| m.volume) {
                    Ok(v) => {
                        let good = want.is_none_or(|w| (v - w).abs() < 1e-7);
                        (
                            format!(
                                "OK {} t2={t2} t3p={t3} cert={cert} {legal} v={v:.9} want={want:?}",
                                if good && t2 && t3 && cert { "SOUND" } else { "BAD" }
                            ),
                            Some(v),
                        )
                    }
                    Err(e) => (
                        format!("OK-UNMEASURED t2={t2} t3p={t3} cert={cert} {legal} {e:?}"),
                        None,
                    ),
                }
            }
        },
    }
}

fn block() -> Body<f64> {
    body_of(
        SketchPlane::xy(),
        &vec![
            ((-1.0, -1.0), 0.0),
            ((1.0, -1.0), 0.0),
            ((1.0, 1.0), 0.0),
            ((-1.0, 1.0), 0.0),
        ],
        1.0,
    )
}

fn run_ops(tag: &str, a: &Body<f64>, b: &Body<f64>, va: f64, vb: f64, vi: Option<f64>) {
    let mut got = std::collections::BTreeMap::new();
    for (op, wab, wba) in [
        ("U", vi.map(|i| va + vb - i), vi.map(|i| va + vb - i)),
        ("S", vi.map(|i| va - i), vi.map(|i| vb - i)),
        ("I", vi, vi),
    ] {
        for (order, want) in [("AB", wab), ("BA", wba)] {
            let (l, r) = if order == "AB" { (a, b) } else { (b, a) };
            let res = match op {
                "U" => topo::union(l, r, tol()),
                "S" => topo::subtract(l, r, tol()),
                _ => topo::intersect(l, r, tol()),
            };
            let (line, v) = outcome(res, want);
            got.insert((op, order), v);
            println!("J3R2 {tag} {op} {order} => {line}");
        }
    }
    // Inclusion–exclusion across whatever built.
    let g = |k| got.get(&k).copied().flatten();
    if let (Some(u), Some(i)) = (g(("U", "AB")), g(("I", "AB"))) {
        if (u + i - va - vb).abs() > 1e-7 {
            println!("J3R2 {tag} INCL-EXCL BAD U+I={} want {}", u + i, va + vb);
        }
    }
    for (order, own) in [("AB", va), ("BA", vb)] {
        if let (Some(s), Some(i)) = (g(("S", order)), g(("I", "AB"))) {
            if (s + i - own).abs() > 1e-7 {
                println!("J3R2 {tag} INCL-EXCL BAD S{order}+I={} want {own}", s + i);
            }
        }
    }
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn j3r2_pocket_battery() {
    let blk = block();
    let vb = 4.0;
    let entries = [
        ("top", 0.5, 1.0),
        ("bottom", -0.5, 1.0),
        ("through", -0.5, 2.0),
        ("flushtop", 0.5, 0.5),
        ("flushbot", 0.0, 0.5),
    ];
    let only = std::env::var("J3_SHAPE").ok();
    for (name, base) in shapes() {
        if only.as_deref().is_some_and(|o| o != name) {
            continue;
        }
        let s = bulge_area(&base);
        for phi in [0.0, 0.7, std::f64::consts::FRAC_PI_2, std::f64::consts::PI, 2.3, 4.0] {
            for off in [(0.0f64, 0.0f64), (0.37, -0.21), (0.8, 0.1), (0.75, 0.75)] {
                let inside = off.0.abs() + 0.5 < 1.0 && off.1.abs() + 0.5 < 1.0;
                let shape = turn(&base, phi, off);
                for (entry, z0, h) in entries {
                    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
                    let c = body_of(plane, &shape, h);
                    let vc = topo::mass_properties(&c, tol()).unwrap().volume;
                    if (vc - s * h).abs() > 1e-9 {
                        println!("J3R2 ORACLE-MISMATCH {name} {vc} vs {}", s * h);
                    }
                    let ov = (1.0f64.min(z0 + h) - 0.0f64.max(z0)).max(0.0);
                    let tag = format!("pocket {name} phi={phi:.3} off={off:?} {entry}");
                    run_ops(&tag, &blk, &c, vb, vc, inside.then_some(s * ov));
                }
            }
        }
    }
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn j3r2_tilted_battery() {
    let blk = block();
    let vb = 4.0;
    let only = std::env::var("J3_SHAPE").ok();
    for (name, base) in shapes() {
        if only.as_deref().is_some_and(|o| o != name) {
            continue;
        }
        let s = bulge_area(&base);
        for (axis, ax_name) in [
            (Vec3::new(1.0, 0.0, 0.0), "x"),
            (Vec3::new(1.0, 1.0, 0.0), "xy"),
        ] {
            for theta in [0.15, 0.3] {
                for phi in [0.0, 1.9, 3.5] {
                    let shape = turn(&base, phi, (0.0, 0.0));
                    // Blind from the top: the sketch plane through
                    // (0, 0, 0.5), extruded 1.0 along its tilted normal.
                    // Through: from (0, −0.3, −0.6), extruded 2.6, the
                    // slab crossed whole when the tilt keeps both caps
                    // outside it.
                    let rot = Affine3::rotation_about_axis(Point3::origin(), axis, theta);
                    let n0 = rot.transform_vec(Vec3::new(0.0, 0.0, 1.0));
                    // Through: the axis passes (0, 0, 0.5), so the
                    // cutter stays clear of the block's side walls.
                    let through = Point3::new(0.0, 0.0, 0.5) - n0 * 1.3;
                    for (entry, p, h) in [
                        ("top", Point3::new(0.0, 0.0, 0.5), 1.0),
                        ("through", through, 2.6),
                    ] {
                        let place = Affine3::translation(p - Point3::origin()) * rot;
                        let o = place.transform_point(Point3::origin());
                        assert!((o - p).norm() < 1e-12, "placement composes as expected");
                        let c = body_of(SketchPlane::new(place), &shape, h);
                        let vc = topo::mass_properties(&c, tol()).unwrap().volume;
                        let n = place.transform_vec(Vec3::new(0.0, 0.0, 1.0));
                        // The oblique closed form, only for the through
                        // pose and only once the caps clear the slab.
                        let vi = if entry == "through" {
                            let tilt = n.z / n.norm();
                            Some(s / tilt)
                        } else {
                            None
                        };
                        let tag =
                            format!("tilt {name} ax={ax_name} th={theta} phi={phi:.2} {entry}");
                        run_ops(&tag, &blk, &c, vb, vc, vi);
                    }
                }
            }
        }
    }
}

fn hull(mut p: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    p.sort_by(|a, b| a.partial_cmp(b).unwrap());
    p.dedup();
    if p.len() < 3 {
        return p;
    }
    let cross = |o: (f64, f64), a: (f64, f64), b: (f64, f64)| {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    };
    let mut lower: Vec<(f64, f64)> = Vec::new();
    for &q in &p {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], q) <= 0.0 {
            lower.pop();
        }
        lower.push(q);
    }
    let mut upper: Vec<(f64, f64)> = Vec::new();
    for &q in p.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], q) <= 0.0 {
            upper.pop();
        }
        upper.push(q);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn clip_area(subject: &[(f64, f64)], clipper: &[(f64, f64)]) -> f64 {
    let mut out: Vec<(f64, f64)> = subject.to_vec();
    let m = clipper.len();
    for i in 0..m {
        let (a, b) = (clipper[i], clipper[(i + 1) % m]);
        let side = |p: (f64, f64)| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        let input = std::mem::take(&mut out);
        let k = input.len();
        for j in 0..k {
            let (p, q) = (input[j], input[(j + 1) % k]);
            let (sp, sq) = (side(p), side(q));
            if sp >= 0.0 {
                out.push(p);
            }
            if (sp >= 0.0) != (sq >= 0.0) {
                let t = sp / (sp - sq);
                out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
            }
        }
        if out.is_empty() {
            return 0.0;
        }
    }
    let n = out.len();
    0.5 * (0..n)
        .map(|i| out[i].0 * out[(i + 1) % n].1 - out[(i + 1) % n].0 * out[i].1)
        .sum::<f64>()
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn j3r2_rand() {
    let mut s: u64 = std::env::var("R2_SEED")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(31337);
    let n: usize = std::env::var("R2_N")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(400);
    let mut rnd = |m: u64| {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (s >> 33) % m
    };
    for case in 0..n {
        let mut poly = || loop {
            let k = 3 + rnd(3) as usize;
            let pts: Vec<(f64, f64)> = (0..k)
                .map(|_| (rnd(5) as f64 * 0.5, rnd(5) as f64 * 0.5))
                .collect();
            let h = hull(pts);
            if h.len() >= 3 {
                return h;
            }
        };
        let (p, q) = (poly(), poly());
        let mut zr = || loop {
            let (a, b) = (rnd(4) as f64, rnd(4) as f64);
            if a < b {
                return (a, b);
            }
        };
        let (zp, zq) = (zr(), zr());
        let ov = clip_area(&p, &q) * (zp.1.min(zq.1) - zp.0.max(zq.0)).max(0.0);
        let (vp, vq) = (clip_area(&p, &p) * (zp.1 - zp.0), clip_area(&q, &q) * (zq.1 - zq.0));
        let shape = |pts: &[(f64, f64)]| -> Shape { pts.iter().map(|&p| (p, 0.0)).collect() };
        let zb = |sh: &Shape, z: (f64, f64)| {
            body_of(
                SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z.0))),
                sh,
                z.1 - z.0,
            )
        };
        let (bp, bq) = (zb(&shape(&p), zp), zb(&shape(&q), zq));
        let tag = format!("case {case} P={p:?} z{zp:?} Q={q:?} z{zq:?}");
        run_ops(&tag, &bp, &bq, vp, vq, Some(ov));
    }
}

fn seg_area(r: f64, d: f64) -> f64 {
    r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
}

/// A rod of radius 0.5 along `x` over `[−1.5, 1.5]`, its axis at
/// `(y0, z0)`, against the block: the block's edges along `y` pierce the
/// rod's wall (the pierce-ring lane JOIN-3 moved).
#[test]
#[ignore = "differential battery; run with --ignored"]
fn j3r2_groove_battery() {
    let blk = block();
    let r = 0.5;
    for z0 in [0.6, 0.8, 1.0, 1.2, 1.45, 0.3] {
        for y0 in [0.0, 0.3, -0.45] {
            let rot = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), std::f64::consts::FRAC_PI_2);
            let place = Affine3::translation(Vec3::new(-1.5, 0.0, 0.0)) * rot;
            let probe = place.transform_vec(Vec3::new(0.0, 0.0, 1.0));
            assert!((probe - Vec3::new(1.0, 0.0, 0.0)).norm() < 1e-12, "{probe:?}");
            // Sketch (u, v) lands at world (z = −u, y = v).
            let disc = profile::circle(Point2::new(-z0, y0), r, tol()).unwrap();
            let p = Profile::new(SketchPlane::new(place), vec![disc.into()])
                .validate(tol())
                .unwrap();
            let rod = extrude(&p, Extrusion::Distance(3.0), tol()).unwrap().body;
            let vr = topo::mass_properties(&rod, tol()).unwrap().volume;
            // The disc's part inside z ∈ [0, 1] (y is inside the block).
            let below = |h: f64| {
                // area of the disc below the plane z = h
                let d = h - z0;
                if d >= r {
                    std::f64::consts::PI * r * r
                } else if d <= -r {
                    0.0
                } else if d >= 0.0 {
                    std::f64::consts::PI * r * r - seg_area(r, d)
                } else {
                    seg_area(r, -d)
                }
            };
            let vi = (below(1.0) - below(0.0)) * 2.0;
            let tag = format!("groove z0={z0} y0={y0}");
            run_ops(&tag, &blk, &rod, 4.0, vr, Some(vi));
        }
    }
}

fn snow_ball(r: f64, y: f64) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, y - r), 1.0),
            (Point2::new(0.0, y + r), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

/// The spun snowman (JOIN-3 re-pinned it from the pierce-ring door to a
/// build): more spins, more offsets, both orders.
#[test]
#[ignore = "differential battery; run with --ignored"]
fn j3r2_snowman_battery() {
    let pi = std::f64::consts::PI;
    let cap = |r: f64, h: f64| pi * h * h * (3.0 * r - h) / 3.0;
    for (r1, r2, d) in [(1.0, 0.8, 1.4), (1.0, 0.8, 0.5), (1.0, 1.0, 1.9), (0.7, 1.0, 1.2)] {
        let a = snow_ball(r1, 0.0);
        let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
        let lens = cap(r1, r1 - x) + cap(r2, r2 - (d - x));
        let (va, vb) = (4.0 / 3.0 * pi * r1.powi(3), 4.0 / 3.0 * pi * r2.powi(3));
        for angle in [1e-3, 0.3, 0.9, std::f64::consts::FRAC_PI_2, 2.0, 3.0, 4.4, 5.9] {
            let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), angle);
            let b = topo::transform_rigid(&snow_ball(r2, d), &spin, tol()).unwrap();
            let tag = format!("snow r1={r1} r2={r2} d={d} spin={angle}");
            run_ops(&tag, &a, &b, va, vb, Some(lens));
        }
    }
}

/// Whether the tilted battery's CUTTERS are themselves legal operands
/// (a disjoint union with a far brick): the results' non-operand column
/// is read against this.
#[test]
#[ignore = "probe"]
fn j3r2_tilted_operands() {
    let far = sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol());
    println!("block: {:?}", topo::union(&block(), &far, tol()).map(|_| ()).map_err(|e| e.kind()));
    for (name, base) in shapes() {
        for (axis, ax_name) in [
            (Vec3::new(1.0, 0.0, 0.0), "x"),
            (Vec3::new(1.0, 1.0, 0.0), "xy"),
        ] {
            for theta in [0.0, 0.15, 0.3] {
                let rot = Affine3::rotation_about_axis(Point3::origin(), axis, theta);
                let place = Affine3::translation(Vec3::new(0.0, 0.0, 0.5)) * rot;
                let c = body_of(SketchPlane::new(place), &base, 1.0);
                let r = topo::union(&c, &far, tol()).map(|_| ()).map_err(|e| e.kind());
                println!("J3R2OP {name} ax={ax_name} th={theta} => {r:?}");
            }
        }
    }
}

/// The non-operand column of the tilted battery, read against a tilted
/// ROUND rod (a full-ellipse ring, no straight side) and with the full
/// refusal printed.
#[test]
#[ignore = "probe"]
fn j3r2_tilted_rod_operand() {
    let far = sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol());
    let disc: Shape = vec![((0.5, 0.0), 1.0), ((-0.5, 0.0), 1.0)];
    for (name, shape) in [("disc", disc), ("D0.3", d_shape(0.3)), ("square", vec![((-0.3, -0.3), 0.0), ((0.3, -0.3), 0.0), ((0.3, 0.3), 0.0), ((-0.3, 0.3), 0.0)])] {
        for theta in [0.0, 0.15] {
            let rot = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), theta);
            let place = Affine3::translation(Vec3::new(0.0, 0.0, 0.5)) * rot;
            let c = body_of(SketchPlane::new(place), &shape, 1.0);
            match topo::subtract(&block(), &c, tol()) {
                Err(e) => println!("J3R2ROD {name} th={theta}: subtract refuses {e:?}"),
                Ok(r) => {
                    let bb = r.body().unwrap();
                    let cert = topo::validate_geometric_certificate(&bb.body, tol()).is_ok();
                    let op = topo::union(&bb.body, &far, tol()).map(|_| ());
                    println!("J3R2ROD {name} th={theta}: builds cert={cert}; operand {op:?}");
                }
            }
        }
    }
}

/// `reach_wall_chord_rows`' c = 0.9 deep poses, re-pinned by JOIN-3
/// from the ring door to ∩ and B∖A building: that row reads only
/// `validate_geometric` and the volume; this adds tiers 2 and 3′ and the
/// legal-operand column.
#[test]
#[ignore = "probe"]
fn j3r2_reach_wall_c09() {
    use crate::common::operands::{framed_bar, three_arc_cylinder};
    let (w, t0, t1) = (
        0.404_001_346_965_559_2,
        -0.688_250_512_587_890_8,
        1.738_194_969_430_420_4,
    );
    let s = std::f64::consts::FRAC_1_SQRT_2;
    let a = three_arc_cylinder(Point2::new(0.0, 0.0), 1.0, 0.0, 2.0, 0.0);
    let vbar = w * w * (t1 - t0);
    let c = 0.9_f64;
    let lo = c - w / 2.0;
    let half = (1.0 - lo * lo).sqrt();
    let f = |x: f64| 0.5 * (x * (1.0 - x * x).sqrt() + x.asin()) - lo * x;
    let area = f(half.min(t1)) - f((-half).max(t0));
    let va = topo::mass_properties(&a, tol()).unwrap().volume;
    for depth in [0.03, 0.1, 0.3] {
        let o = Point3::new(c * s, -c * s, 2.0 - depth + w / 2.0);
        let b = framed_bar(o, Vec3::new(s, s, 0.0), t0, t1, w);
        run_ops(&format!("reach c=0.9 depth={depth}"), &a, &b, va, vbar, Some(area * depth));
    }
}
