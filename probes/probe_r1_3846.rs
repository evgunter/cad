//! Reviewer probes for PR 3846 (lane reach-dual3846-r1). Independent
//! oracle: closed-form areas and an analytic point-membership test,
//! never the kernel.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{Open, ProfileLoop, RawLoop, Start};
use sweep::test_support::{extruded, sketch_at};
use topo::{Body, BooleanCoincidence, BooleanDeclarations, BooleanResult, SolidContainment};

fn tol() -> Tol {
    Tol::witness()
}

fn cut(r: f64) -> f64 {
    (1.0 - core::f64::consts::FRAC_PI_4) * r * r
}

/// A w × h outline with its four corners rounded by r (r = 0: sharp).
fn rect(w: f64, h: f64, r: f64) -> ProfileLoop<f64> {
    if r == 0.0 {
        return ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(w, 0.0),
            Point2::new(w, h),
            Point2::new(0.0, h),
        ]);
    }
    let t = tol();
    Open.at(Point2::new(w / 2.0, 0.0))
        .toward(1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(w, h / 2.0), t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(w / 2.0, h), t)
        .unwrap()
        .toward(-1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(0.0, h / 2.0), t)
        .unwrap()
        .toward(0.0, -1.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .to(Start, t)
        .unwrap()
        .into()
}

/// The oracle: is (x, y) strictly inside the rounded rect, and how far
/// from its boundary (a lower bound, for skipping near-boundary samples).
fn in_rect(w: f64, h: f64, r: f64, x: f64, y: f64) -> (bool, f64) {
    let cx = x.clamp(r, w - r);
    let cy = y.clamp(r, h - r);
    let (dx, dy) = (x - cx, y - cy);
    if dx == 0.0 || dy == 0.0 {
        let d = (x).min(w - x).min(y).min(h - y);
        return (d > 0.0, d.abs());
    }
    let d = r - (dx * dx + dy * dy).sqrt();
    (d > 0.0, d.abs())
}

struct Plate {
    w: f64,
    h: f64,
    r: f64,
    z0: f64,
    t: f64,
}

impl Plate {
    fn body(&self) -> Body<f64> {
        extruded(sketch_at(self.z0), vec![rect(self.w, self.h, self.r)], self.t, tol())
    }
    fn vol(&self) -> f64 {
        (self.w * self.h - 4.0 * cut(self.r)) * self.t
    }
    fn inside(&self, p: Point3<f64>) -> (bool, f64) {
        let (i, d) = in_rect(self.w, self.h, self.r, p.x, p.y);
        let dz = (p.z - self.z0).min(self.z0 + self.t - p.z);
        (i && dz > 0.0, d.min(dz.abs()))
    }
}

fn decl(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let found = topo::flush::find_flush_candidates(a, b, tol()).expect("decides");
    let (rest, cont): (Vec<_>, Vec<_>) = found
        .into_iter()
        .partition(|f| f.class == BooleanCoincidence::REST);
    let mut d = topo::flush::declare_all(&rest);
    d.coincident_faces
        .extend(topo::flush::declare_all(&cont).coincident_faces);
    d
}

/// Runs one op and returns a cell, checking the body against the oracle.
fn cell(
    out: Result<BooleanResult<f64>, topo::BooleanError>,
    expect: Option<f64>,
    member: &dyn Fn(Point3<f64>) -> (bool, f64),
    bbox: (Point3<f64>, Point3<f64>),
    wrong: &mut Vec<String>,
    label: &str,
) -> String {
    match out {
        Err(e) => {
            let s = format!("{e:?}");
            format!("REFUSE {}", &s[..s.find([' ', '{', '(']).unwrap_or(s.len())])
        }
        Ok(BooleanResult::Empty) => {
            if expect.is_some() {
                wrong.push(format!("{label}: empty but expected {expect:?}"));
                "WRONG-empty".into()
            } else {
                "empty".into()
            }
        }
        Ok(BooleanResult::Body(bb)) => {
            let Some(v0) = expect else {
                wrong.push(format!("{label}: body but expected empty"));
                return "WRONG-body".into();
            };
            let v = topo::mass_properties(&bb.body, tol()).map(|m| m.volume);
            let t3 = topo::validate_geometric(&bb.body, tol()).is_ok();
            let t3p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).is_ok();
            let rel = v.as_ref().map(|v| (v - v0).abs() / v0).unwrap_or(f64::NAN);
            // Monte Carlo against the oracle: a deterministic LCG.
            let band = Band::linear(tol()).unwrap();
            let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
            let mut rnd = || {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                (seed >> 11) as f64 / (1u64 << 53) as f64
            };
            let (lo, hi) = bbox;
            let (mut n, mut bad) = (0, 0);
            for _ in 0..120 {
                let p = Point3::new(
                    lo.x + (hi.x - lo.x) * rnd(),
                    lo.y + (hi.y - lo.y) * rnd(),
                    lo.z + (hi.z - lo.z) * rnd(),
                );
                let (want, d) = member(p);
                let scale = (hi.x - lo.x).abs().max(1e-300);
                if d < 1e-6 * scale {
                    continue;
                }
                n += 1;
                match topo::point_in_solid(&bb.body, p, band, tol()) {
                    Ok(SolidContainment::In) if want => {}
                    Ok(SolidContainment::Out) if !want => {}
                    other => {
                        bad += 1;
                        if bad <= 2 {
                            wrong.push(format!("{label}: pis {p:?} want {want} got {other:?}"));
                        }
                    }
                }
            }
            let ok = rel <= 1e-9 && t3 && t3p && bad == 0;
            if !ok {
                wrong.push(format!(
                    "{label}: rel {rel:e} t3 {t3} t3' {t3p} pis-bad {bad}/{n}"
                ));
            }
            format!(
                "{} f{} rel{:.0e} pis{}/{}",
                if ok { "ok" } else { "WRONG" },
                bb.body.faces().count(),
                rel,
                n - bad,
                n
            )
        }
    }
}

/// Every op in both orders for two plates (`p` below or above `q`),
/// optionally under a rigid map applied to both.
fn matrix(name: &str, p: &Plate, q: &Plate, map: Option<Affine3<f64>>, wrong: &mut Vec<String>) {
    let place = |b: Body<f64>| match map {
        Some(m) => topo::transform_rigid(&b, &m, tol()).expect("rigid"),
        None => b,
    };
    let inv = map.map(|m| m.inverse());
    let back = move |x: Point3<f64>| match inv {
        Some(i) => i.transform_point(x),
        None => x,
    };
    let (bp, bq) = (place(p.body()), place(q.body()));
    // A world bbox of both (conservative: map the local box corners).
    let s = p.w.max(p.h).max(q.w).max(q.h);
    let (zl, zh) = (p.z0.min(q.z0), (p.z0 + p.t).max(q.z0 + q.t));
    let corners: Vec<Point3<f64>> = (0..8)
        .map(|i| {
            let c = Point3::new(
                if i & 1 == 0 { -0.1 * s } else { 1.1 * s },
                if i & 2 == 0 { -0.1 * s } else { 1.1 * s },
                if i & 4 == 0 { zl - 0.1 * (zh - zl) } else { zh + 0.1 * (zh - zl) },
            );
            match map {
                Some(m) => m.transform_point(c),
                None => c,
            }
        })
        .collect();
    let lo = corners.iter().fold(Point3::new(f64::MAX, f64::MAX, f64::MAX), |a, c| {
        Point3::new(a.x.min(c.x), a.y.min(c.y), a.z.min(c.z))
    });
    let hi = corners.iter().fold(Point3::new(f64::MIN, f64::MIN, f64::MIN), |a, c| {
        Point3::new(a.x.max(c.x), a.y.max(c.y), a.z.max(c.z))
    });
    for (order, a, b, pa, pb) in [("p=A", &bp, &bq, p, q), ("q=A", &bq, &bp, q, p)] {
        let dab = decl(a, b);
        let dba = decl(b, a);
        let la = format!("{name} {order}");
        let u = |x| {
            let x = back(x);
            let (ia, da) = pa.inside(x);
            let (ib, db) = pb.inside(x);
            (ia || ib, da.min(db))
        };
        let amb = |x| {
            let x = back(x);
            let (ia, da) = pa.inside(x);
            let (ib, db) = pb.inside(x);
            (ia && !ib, da.min(db))
        };
        let bma = |x| {
            let x = back(x);
            let (ia, da) = pa.inside(x);
            let (ib, db) = pb.inside(x);
            (ib && !ia, da.min(db))
        };
        let none = |_x| (false, 1.0);
        let c1 = cell(topo::union_with(a, b, &dab, tol()), Some(pa.vol() + pb.vol()), &u, (lo, hi), wrong, &format!("{la} ∪"));
        let c2 = cell(topo::subtract_with(a, b, &dab, tol()), Some(pa.vol()), &amb, (lo, hi), wrong, &format!("{la} A∖B"));
        let c3 = cell(topo::subtract_with(b, a, &dba, tol()), Some(pb.vol()), &bma, (lo, hi), wrong, &format!("{la} B∖A"));
        let c4 = cell(topo::intersect_with(a, b, &dab, tol()), None, &none, (lo, hi), wrong, &format!("{la} ∩"));
        println!("ROW {la:<40} | {c1} | {c2} | {c3} | {c4}");
    }
}

fn plates(s: f64, r_low: f64, r_up: f64) -> (Plate, Plate) {
    (
        Plate { w: 6.0 * s, h: 4.0 * s, r: r_low * s, z0: 0.0, t: s },
        Plate { w: 6.0 * s, h: 4.0 * s, r: r_up * s, z0: s, t: s },
    )
}

#[test]
fn probe_radii_and_mismatch() {
    let mut wrong = Vec::new();
    for (rl, ru) in [
        (0.5, 0.0),
        (0.25, 0.0),
        (1.0, 0.0),
        (0.01, 0.0),
        (1.5, 0.0),
        (1.99, 0.0),
        (0.3, 0.5),
        (0.5, 0.3),
        (0.25, 1.0),
        (1.0, 0.25),
        (0.5, 0.5 + 1e-6),
        (0.5, 0.5 + 1e-10),
        (1.9, 0.2),
    ] {
        let (p, q) = plates(1.0, rl, ru);
        matrix(&format!("r{rl}/{ru}"), &p, &q, None, &mut wrong);
    }
    for w in &wrong {
        println!("WRONGLINE {w}");
    }
    assert!(wrong.is_empty(), "{} wrong", wrong.len());
}

#[test]
fn probe_scales() {
    let mut wrong = Vec::new();
    for s in [1e-3, 1e3] {
        for (rl, ru) in [(0.5, 0.0), (0.3, 0.5)] {
            let (p, q) = plates(s, rl, ru);
            matrix(&format!("s{s} r{rl}/{ru}"), &p, &q, None, &mut wrong);
        }
    }
    for w in &wrong {
        println!("WRONGLINE {w}");
    }
    assert!(wrong.is_empty(), "{} wrong", wrong.len());
}

#[test]
fn probe_rotated() {
    let mut wrong = Vec::new();
    let axis = Vec3::new(1.0, 2.0, 3.0);
    let n = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
    let axis = Vec3::new(axis.x / n, axis.y / n, axis.z / n);
    for (k, m) in [
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.5236),
        Affine3::rotation_about_axis(Point3::new(1.0, -2.0, 0.5), axis, 0.7),
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), core::f64::consts::FRAC_PI_2),
    ]
    .into_iter()
    .enumerate()
    {
        for (rl, ru) in [(0.5, 0.0), (0.3, 0.5)] {
            let (p, q) = plates(1.0, rl, ru);
            matrix(&format!("rot{k} r{rl}/{ru}"), &p, &q, Some(m), &mut wrong);
        }
    }
    for w in &wrong {
        println!("WRONGLINE {w}");
    }
    assert!(wrong.is_empty(), "{} wrong", wrong.len());
}

/// Results reused as operands: (rounded ∪ sharp) then a third plate on top,
/// in both orders, sharp-first and rounded-first.
#[test]
fn probe_reuse() {
    let mut wrong = Vec::new();
    for (r0, r1, r2) in [(0.5, 0.0, 0.5), (0.0, 0.5, 0.0), (0.5, 0.0, 0.3)] {
        let p0 = Plate { w: 6.0, h: 4.0, r: r0, z0: 0.0, t: 1.0 };
        let p1 = Plate { w: 6.0, h: 4.0, r: r1, z0: 1.0, t: 1.0 };
        let p2 = Plate { w: 6.0, h: 4.0, r: r2, z0: 2.0, t: 1.0 };
        let (b0, b1, b2) = (p0.body(), p1.body(), p2.body());
        // build the first union sharp/upper-first
        let Ok(BooleanResult::Body(s01)) = topo::union_with(&b1, &b0, &decl(&b1, &b0), tol()) else {
            wrong.push(format!("reuse {r0}/{r1}: first union refuses"));
            continue;
        };
        let s = s01.body;
        let v01 = p0.vol() + p1.vol();
        let member = |x: Point3<f64>| {
            let (a, da) = p0.inside(x);
            let (b, db) = p1.inside(x);
            let (c, dc) = p2.inside(x);
            (a || b || c, da.min(db).min(dc))
        };
        let bbox = (Point3::new(-0.6, -0.6, -0.3), Point3::new(6.6, 4.6, 3.3));
        for (order, a, b) in [("stack=A", &s, &b2), ("third=A", &b2, &s)] {
            let c = cell(
                topo::union_with(a, b, &decl(a, b), tol()),
                Some(v01 + p2.vol()),
                &member,
                bbox,
                &mut wrong,
                &format!("reuse {r0}/{r1}/{r2} {order} ∪"),
            );
            let d = cell(
                topo::subtract_with(a, b, &decl(a, b), tol()),
                Some(if order == "stack=A" { v01 } else { p2.vol() }),
                &|x| {
                    let (a0, d0) = p0.inside(x);
                    let (a1, d1) = p1.inside(x);
                    let (a2, d2) = p2.inside(x);
                    let st = a0 || a1;
                    (if order == "stack=A" { st && !a2 } else { a2 && !st }, d0.min(d1).min(d2))
                },
                bbox,
                &mut wrong,
                &format!("reuse {r0}/{r1}/{r2} {order} A∖B"),
            );
            println!("ROW reuse {r0}/{r1}/{r2} {order:<10} | {c} | {d}");
        }
    }
    for w in &wrong {
        println!("WRONGLINE {w}");
    }
    assert!(wrong.is_empty(), "{} wrong", wrong.len());
}

/// Claim 3: a sharp plate shifted in y by δ over a rounded plate. The
/// continuation is gone, so the cover is not structural; a graze in or
/// near the band must refuse (or build correctly), never silently drop.
#[test]
fn probe_graze_shift() {
    for d in [1e-12, 1e-10, -1e-10, 1e-7, -1e-7, 1e-3, -1e-3] {
        let low = Plate { w: 6.0, h: 4.0, r: 0.5, z0: 0.0, t: 1.0 };
        let up = Plate { w: 6.0, h: 4.0, r: 0.0, z0: 1.0, t: 1.0 };
        let m = Affine3::translation(Vec3::new(0.0, d, 0.0));
        let bl = low.body();
        let bu = topo::transform_rigid(&up.body(), &m, tol()).unwrap();
        for (order, a, b) in [("sharp=A", &bu, &bl), ("rounded=A", &bl, &bu)] {
            let de = decl(a, b);
            let r = match topo::union_with(a, b, &de, tol()) {
                Ok(BooleanResult::Body(bb)) => {
                    let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
                    let want = low.vol() + up.vol();
                    format!("body rel {:e} decl {}", (v - want).abs() / want, de.coincident_faces.len())
                }
                Ok(BooleanResult::Empty) => "EMPTY".into(),
                Err(e) => {
                    let s = format!("{e:?}");
                    format!("refuse {} decl {}", &s[..s.len().min(70)], de.coincident_faces.len())
                }
            };
            println!("ROW graze δ={d:e} {order}: {r}");
        }
    }
}

/// The PR's box-beside-fillet, the box spanning z ∈ [z0, z1] while the
/// plate spans [0, 1]: where z0 > 0 the box's lower wall edge touches
/// the fillet's ruling mid-span with no plate vertex under it.
#[test]
fn probe_short_box_beside_fillet() {
    let (w, h, r) = (6.0, 4.0, 0.5);
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let touch = Point2::new(w - r + r * s2, r - r * s2);
    let at = |along: f64, out: f64| {
        Point2::new(touch.x + (along + out) * s2, touch.y + (along - out) * s2)
    };
    for (z0, z1) in [(0.0, 1.0), (0.25, 0.75), (0.5, 1.5), (-0.5, 0.5), (0.25, 1.0)] {
        let boxed = extruded(
            sketch_at(z0),
            vec![ProfileLoop::polygon([at(-0.5, 0.0), at(-0.5, 1.0), at(0.5, 1.0), at(0.5, 0.0)])],
            z1 - z0,
            tol(),
        );
        let p = Plate { w, h, r, z0: 0.0, t: 1.0 }.body();
        let wall = boxed
            .faces()
            .map(|(k, _)| k)
            .find(|&f| matches!(boxed.get_face(f).and_then(|x| boxed.get_surface(x.surface)),
                Some(geom::Surface::Plane { normal, .. }) if (normal.x + s2).abs() < 1e-9 && (normal.y - s2).abs() < 1e-9))
            .unwrap();
        let fillet = p
            .faces()
            .map(|(k, _)| k)
            .find(|&f| matches!(p.get_face(f).and_then(|x| p.get_surface(x.surface)),
                Some(geom::Surface::Cylinder { origin, .. }) if origin.x > 3.0 && origin.y < 2.0))
            .unwrap();
        let tangent = |fa, fb| BooleanDeclarations {
            coincident_faces: vec![topo::FacePairDeclaration::new(fa, fb, topo::ContactClass::Tangent)],
            ..BooleanDeclarations::default()
        };
        let vb = z1 - z0;
        let vp = w * h - 4.0 * cut(r);
        for (order, a, b, fa, fb, va, vbb) in
            [("box=A", &boxed, &p, wall, fillet, vb, vp), ("plate=A", &p, &boxed, fillet, wall, vp, vb)]
        {
            let d = tangent(fa, fb);
            let dd = tangent(fb, fa);
            let f = |o: Result<BooleanResult<f64>, topo::BooleanError>, want: Option<f64>| match o {
                Ok(BooleanResult::Body(bb)) => {
                    let v = topo::mass_properties(&bb.body, tol()).map(|m| m.volume);
                    let t3 = topo::validate_geometric(&bb.body, tol()).is_ok();
                    let t3p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).is_ok();
                    format!("body v={v:?} want={want:?} f{} t3={t3} t3'={t3p}", bb.body.faces().count())
                }
                Ok(BooleanResult::Empty) => format!("empty want={want:?}"),
                Err(e) => {
                    let s = format!("{e:?}");
                    format!("refuse {}", &s[..s.len().min(60)])
                }
            };
            println!("ROW sbox z[{z0},{z1}] {order} ∪  {}", f(topo::union_with(a, b, &d, tol()), Some(va + vbb)));
            println!("ROW sbox z[{z0},{z1}] {order} A∖B {}", f(topo::subtract_with(a, b, &d, tol()), Some(va)));
            println!("ROW sbox z[{z0},{z1}] {order} B∖A {}", f(topo::subtract_with(b, a, &dd, tol()), Some(vbb)));
            println!("ROW sbox z[{z0},{z1}] {order} ∩  {}", f(topo::intersect_with(a, b, &d, tol()), None));
        }
    }
}

fn short(e: Result<BooleanResult<f64>, topo::BooleanError>) -> String {
    match e {
        Ok(BooleanResult::Body(bb)) => format!(
            "body v={:?}",
            topo::mass_properties(&bb.body, tol()).map(|m| m.volume)
        ),
        Ok(BooleanResult::Empty) => "empty".into(),
        Err(e) => {
            let s = format!("{e:?}");
            s[..s.len().min(90)].to_string()
        }
    }
}

/// Filed item 1: the short-armed L, r = 1, sharp over rounded.
#[test]
fn probe_filed_short_l() {
    let t = tol();
    let mut path = Open.at(Point2::new(3.0, 0.0)).toward(1.0, 0.0, t).unwrap();
    for (corner, (dx, dy)) in [
        (Point2::new(6.0, 1.0), (0.0, 1.0)),
        (Point2::new(4.5, 2.0), (-1.0, 0.0)),
        (Point2::new(3.0, 3.0), (0.0, 1.0)),
        (Point2::new(1.5, 4.0), (-1.0, 0.0)),
        (Point2::new(0.0, 2.0), (0.0, -1.0)),
    ] {
        path = path.fillet(1.0, t).unwrap().at(corner, t).unwrap().toward(dx, dy, t).unwrap();
    }
    let rounded: ProfileLoop<f64> = path.fillet(1.0, t).unwrap().to(Start, t).unwrap().into();
    let sharp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(6.0, 0.0),
        Point2::new(6.0, 2.0),
        Point2::new(3.0, 2.0),
        Point2::new(3.0, 4.0),
        Point2::new(0.0, 4.0),
    ]);
    let lo = extruded(sketch_at(0.0), vec![rounded], 1.0, t);
    let up = extruded(sketch_at(1.0), vec![sharp], 1.0, t);
    let want = 18.0 - 4.0 * cut(1.0) + 18.0;
    for (o, a, b) in [("sharp=A", &up, &lo), ("rounded=A", &lo, &up)] {
        println!("ROW filedL {o} ∪ {} (want {want})", short(topo::union_with(a, b, &decl(a, b), t)));
    }
}

/// Filed item 2: a box corner on the declared tangent ruling.
#[test]
fn probe_filed_box_corner() {
    let (w, h, r) = (6.0, 4.0, 0.5);
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let touch = Point2::new(w - r + r * s2, r - r * s2);
    let at = |along: f64, out: f64| {
        Point2::new(touch.x + (along + out) * s2, touch.y + (along - out) * s2)
    };
    for (lab, a0, a1) in [("+", 0.0, 1.0), ("-", -1.0, 0.0)] {
        let boxed = extruded(
            sketch_at(0.0),
            vec![ProfileLoop::polygon([at(a0, 0.0), at(a0, 1.0), at(a1, 1.0), at(a1, 0.0)])],
            1.0,
            tol(),
        );
        let p = Plate { w, h, r, z0: 0.0, t: 1.0 }.body();
        let wall = boxed.faces().map(|(k, _)| k).find(|&f| matches!(boxed.get_face(f).and_then(|x| boxed.get_surface(x.surface)),
            Some(geom::Surface::Plane { normal, .. }) if (normal.x + s2).abs() < 1e-9 && (normal.y - s2).abs() < 1e-9)).unwrap();
        let fillet = p.faces().map(|(k, _)| k).find(|&f| matches!(p.get_face(f).and_then(|x| p.get_surface(x.surface)),
            Some(geom::Surface::Cylinder { origin, .. }) if origin.x > 3.0 && origin.y < 2.0)).unwrap();
        let tg = |fa, fb| BooleanDeclarations {
            coincident_faces: vec![topo::FacePairDeclaration::new(fa, fb, topo::ContactClass::Tangent)],
            ..BooleanDeclarations::default()
        };
        println!("ROW boxcorner {lab} box=A ∪ {}", short(topo::union_with(&boxed, &p, &tg(wall, fillet), tol())));
        println!("ROW boxcorner {lab} plate=A ∪ {}", short(topo::union_with(&p, &boxed, &tg(fillet, wall), tol())));
        println!("ROW boxcorner {lab} box=A ∩ {}", short(topo::intersect_with(&boxed, &p, &tg(wall, fillet), tol())));
    }
}
