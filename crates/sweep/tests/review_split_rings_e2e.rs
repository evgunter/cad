//! Reviewer rows for the split's section rings (PR 3658 review,
//! `cleave-rings-review-r1`): split poses beyond the PR's repros,
//! through the public doors only. `survey_*` rows print a table
//! (`REVIEW ...` lines) and assert nothing; the `*_passes_*` rows pin
//! what holds on the reviewed head.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::test_support::{bored_cylinder, brick, prism_at};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, FaceKey};

fn tol() -> Tol {
    Tol::witness()
}

fn rod(cx: f64, cy: f64, r: f64, z0: f64, h: f64) -> Body<f64> {
    prism_at(
        vec![
            (Point2::new(cx - r, cy), 1.0),
            (Point2::new(cx + r, cy), 1.0),
        ],
        z0,
        h,
        tol(),
    )
}

fn sub(a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, String> {
    match topo::subtract(a, b, tol()) {
        Ok(topo::BooleanResult::Body(b)) => Ok(b.body),
        Ok(_) => Err("empty".into()),
        Err(e) => Err(format!("{e:?}").chars().take(160).collect()),
    }
}

fn uni(a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, String> {
    match topo::union(a, b, tol()) {
        Ok(topo::BooleanResult::Body(b)) => Ok(b.body),
        Ok(_) => Err("empty".into()),
        Err(e) => Err(format!("{e:?}").chars().take(160).collect()),
    }
}

fn cavity_at(cx: f64, cy: f64, r: f64) -> Result<Body<f64>, String> {
    let block: Body<f64> = brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.5), tol());
    sub(&block, &rod(cx, cy, r, -0.5, 3.5))
}

/// Tube inside a tube: the brick less an annular pocket (r 1.5 outside,
/// an island of r 1.0 standing on the pocket floor), the island bored
/// through at r 0.5.
fn nested() -> Result<Body<f64>, String> {
    let block: Body<f64> = brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.5), tol());
    let pocket = sub(&block, &rod(0.0, 0.0, 1.5, 0.5, 3.0))?;
    let island = uni(&pocket, &rod(0.0, 0.0, 1.0, 0.3, 1.9))?;
    sub(&island, &rod(0.0, 0.0, 0.5, -0.5, 3.5))
}

/// Two bores, the second at `x2` with radius `r2` (touching or breaking
/// the `x = 2` wall when `x2 + r2 >= 2`).
fn two_bores(x2: f64, r2: f64) -> Result<Body<f64>, String> {
    let a = cavity_at(-0.9, 0.0, 0.6)?;
    sub(&a, &rod(x2, 0.0, r2, -0.5, 3.5))
}

/// An L of two overlapping bricks, then two rods off it.
fn l_bored() -> Result<Body<f64>, String> {
    let a: Body<f64> = brick((-2.0, 2.0), (-2.0, 0.5), (0.0, 2.5), tol());
    let b: Body<f64> = brick((-2.0, 0.3), (-1.0, 2.0), (0.0, 2.5), tol());
    let l = uni(&a, &b)?;
    let l = sub(&l, &rod(-1.0, -1.0, 0.4, -0.5, 3.5))?;
    sub(&l, &rod(-1.0, 1.2, 0.35, -0.5, 3.5))
}

fn tilted(z: f64, t: f64, flip: bool) -> SplitPlane<f64> {
    let s = if flip { -1.0 } else { 1.0 };
    SplitPlane {
        origin: Point3::new(0.0, 0.0, z),
        normal: Vec3::new(s * t.sin(), 0.0, s * t.cos()),
    }
}

/// A plane through `(0,0,z)` tilted `t` about an axis in xy at angle `a`.
fn skew(z: f64, t: f64, a: f64) -> SplitPlane<f64> {
    let axis = Vec3::new(a.cos(), a.sin(), 0.0);
    // normal = rotate z about axis by t
    let perp = Vec3::new(-a.sin(), a.cos(), 0.0);
    let n = Vec3::new(0.0, 0.0, t.cos()) + perp * (-t.sin());
    let _ = axis;
    SplitPlane {
        origin: Point3::new(0.0, 0.0, z),
        normal: n,
    }
}

fn section_faces(half: &Body<f64>, plane: &SplitPlane<f64>) -> Vec<FaceKey> {
    half.faces()
        .filter(|(_, f)| {
            matches!(
                half.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.cross(plane.normal).norm() < 1e-12
                        && (*origin - plane.origin).dot(plane.normal).abs() < 1e-12
            )
        })
        .map(|(k, _)| k)
        .collect()
}

fn kinds<E: core::fmt::Debug>(errs: &[E]) -> Vec<String> {
    let mut out: Vec<String> = errs
        .iter()
        .map(|e| {
            format!("{e:?}")
                .split([' ', '(', '{'])
                .next()
                .unwrap()
                .to_owned()
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

/// One row's verdict: `None` when everything holds, else the trouble.
struct Row {
    line: String,
    invalid: bool,
    cancelling: bool,
    refused: bool,
}

fn run(what: &str, body: &Body<f64>, plane: &SplitPlane<f64>) -> Row {
    let whole = topo::mass_properties(body, tol()).ok().map(|m| m.volume);
    match split(body, plane, tol()) {
        Err(e) => {
            let s: String = format!("{e:?}").chars().take(140).collect();
            Row {
                line: format!("REVIEW {what}: REFUSED {s}"),
                invalid: false,
                cancelling: false,
                refused: true,
            }
        }
        Ok(r) => {
            let mut line = format!("REVIEW {what}: ok");
            let (mut invalid, mut cancelling) = (false, false);
            let mut vsum = Some(0.0);
            for (side, part) in [("below", r.below), ("above", r.above)] {
                let SplitPart::Body(h) = part else {
                    line += &format!(" | {side} EMPTY");
                    vsum = None;
                    continue;
                };
                let t1 = topo::validate(&h).err().map(|e| kinds(&e));
                let t3 = topo::validate_geometric(&h, tol()).err().map(|e| kinds(&e));
                if t1.is_some() || t3.is_some() {
                    invalid = true;
                }
                let secs = section_faces(&h, plane);
                let desc: Vec<String> = secs
                    .iter()
                    .map(|&f| {
                        let fd = h.get_face(f).unwrap();
                        if !fd.sense {
                            cancelling = true;
                        }
                        format!("{}{}", if fd.sense { "+" } else { "-" }, fd.rings.len())
                    })
                    .collect();
                let rings: usize = h.faces().map(|(_, f)| f.rings.len()).sum();
                let v = topo::mass_properties(&h, tol()).ok().map(|m| m.volume);
                vsum = match (vsum, v) {
                    (Some(a), Some(b)) => Some(a + b),
                    _ => None,
                };
                line +=
                    &format!(" | {side}: t1 {t1:?} t3 {t3:?} sec {desc:?} rings {rings} vol {v:?}");
            }
            if let (Some(w), Some(s)) = (whole, vsum) {
                line += &format!(" | dV {:.3e}", s - w);
                if (s - w).abs() > 1e-7 {
                    invalid = true;
                    line += " VOLUME-MISMATCH";
                }
            }
            Row {
                line,
                invalid,
                cancelling,
                refused: false,
            }
        }
    }
}

fn report(rows: &[Row]) {
    for r in rows {
        eprintln!("{}", r.line);
    }
    eprintln!(
        "REVIEW SUMMARY: {} rows, {} refused, {} invalid, {} with a clockwise section face",
        rows.len(),
        rows.iter().filter(|r| r.refused).count(),
        rows.iter().filter(|r| r.invalid).count(),
        rows.iter().filter(|r| r.cancelling).count()
    );
}

#[test]
fn survey_bored_brick_tilts_and_offsets() {
    let mut rows = Vec::new();
    for (cx, cy) in [
        (0.0, 0.0),
        (0.8, 0.0),
        (0.4, -0.3),
        (0.5, 0.0),
        (-0.7, 0.6),
        (1.1, 0.3),
    ] {
        let body = match cavity_at(cx, cy, 0.7) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("REVIEW cavity ({cx},{cy}) r0.7: subtract refused {e}");
                continue;
            }
        };
        for t in [0.0, 0.1, 0.3, 0.6, 0.9, 1.2, 1.4, 1.5] {
            for flip in [false, true] {
                rows.push(run(
                    &format!("cavity ({cx},{cy}) r0.7 tilt {t} flip {flip}"),
                    &body,
                    &tilted(1.25, t, flip),
                ));
            }
        }
        for (t, a) in [(0.3, 0.7), (0.9, 1.1), (1.3, 2.3)] {
            rows.push(run(
                &format!("cavity ({cx},{cy}) r0.7 skew {t} about {a}"),
                &body,
                &skew(1.25, t, a),
            ));
        }
    }
    let bc = bored_cylinder(0.3, 0.2, 0.37, tol());
    for t in [0.0, 0.2, 0.5, 0.8, 1.0, 1.2, 1.45] {
        for flip in [false, true] {
            rows.push(run(
                &format!("bored cylinder tilt {t} flip {flip}"),
                &bc,
                &tilted(0.5, t, flip),
            ));
        }
    }
    report(&rows);
}

#[test]
fn survey_nested_two_holes_and_boolean_then_split() {
    let mut rows = Vec::new();
    match nested() {
        Ok(b) => {
            for (z, t) in [
                (1.25, 0.0),
                (1.25, 0.1),
                (1.25, 0.2),
                (1.0, 0.0),
                (1.9, 0.0),
                (0.4, 0.0),
            ] {
                rows.push(run(
                    &format!("nested tube z {z} tilt {t}"),
                    &b,
                    &tilted(z, t, false),
                ));
                rows.push(run(
                    &format!("nested tube z {z} tilt {t} flip"),
                    &b,
                    &tilted(z, t, true),
                ));
            }
            rows.push(run(
                "nested tube skew 0.15 about 0.4",
                &b,
                &skew(1.25, 0.15, 0.4),
            ));
            rows.push(run("nested tube steep 1.3", &b, &tilted(1.25, 1.3, false)));
        }
        Err(e) => eprintln!("REVIEW nested: build refused {e}"),
    }
    for (x2, r2, what) in [
        (1.0, 0.5, "interior"),
        (1.5, 0.5, "tangent to x=2"),
        (1.7, 0.5, "breaking x=2"),
    ] {
        match two_bores(x2, r2) {
            Ok(b) => {
                for t in [0.0, 0.3, 1.2] {
                    rows.push(run(
                        &format!("two bores {what} tilt {t}"),
                        &b,
                        &tilted(1.25, t, false),
                    ));
                }
            }
            Err(e) => eprintln!("REVIEW two bores {what}: build refused {e}"),
        }
    }
    match l_bored() {
        Ok(b) => {
            for t in [0.0, 0.25, 0.7, 1.3] {
                rows.push(run(
                    &format!("L bored tilt {t}"),
                    &b,
                    &tilted(1.25, t, false),
                ));
                rows.push(run(&format!("L bored skew {t}"), &b, &skew(1.25, t, 0.9)));
            }
        }
        Err(e) => eprintln!("REVIEW L: build refused {e}"),
    }
    report(&rows);
}

/// A cap line a hair off parallel to the join frame's `v`: the cavity
/// with a rod of r 0.25 at `(x0, 1.72)`, where `x0` is the steep
/// plane's trace on the `z = 0` cap, so the cap's four crossings sit at
/// `y = −2, 1.47, 1.97, 2` along the line (gaps 3.47, 0.5, 0.03). The
/// body is turned `δ` about `x`, which tilts the cap line's `u` by
/// about `δ / sin t` per unit of `v`: at |δ| ≈ 2.6e-8 the 0.03 gap is a
/// column (< ε) and the 0.5 gap is not (> K·ε).
#[test]
fn survey_cap_line_a_hair_off_v() {
    let t: f64 = 1.4;
    let x0 = 1.25 * t.cos() / t.sin();
    let mut rows = Vec::new();
    let block: Body<f64> = brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.5), tol());
    let body = match sub(&block, &rod(x0, 1.72, 0.25, -0.5, 3.5)) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("REVIEW hair: build refused {e}");
            return;
        }
    };
    for d in [
        0.0, 1e-12, -1e-12, 1e-9, -1e-9, 2.6e-8, -2.6e-8, 3e-8, -3e-8, 1e-7, -1e-7, 1e-5, -1e-5,
    ] {
        let map = geom_core::Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            d,
        );
        let posed = match topo::transform_rigid(&body, &map, tol()) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("REVIEW hair δ {d}: transform refused {e:?}");
                continue;
            }
        };
        for flip in [true, false] {
            rows.push(run(
                &format!("hair δ {d} tilt {t} flip {flip}"),
                &posed,
                &tilted(1.25, t, flip),
            ));
        }
    }
    report(&rows);
}

/// As [`survey_cap_line_a_hair_off_v`], with the near pair made by a
/// rod the cap line cuts near tangency: crossings at `y = −2, 0.275,
/// 0.325, 2` (gaps 2.275, 0.05, 1.675). At |δ′| ∈ (6.7e-9, 2e-8) the
/// ring pair is one column and the outer crossings are columns of
/// their own.
#[test]
fn survey_cap_line_a_hair_off_v_near_tangent_ring() {
    let t: f64 = 1.4;
    let x0 = 1.25 * t.cos() / t.sin();
    let r: f64 = 0.25;
    let d = (r * r - 0.025f64 * 0.025).sqrt();
    let mut rows = Vec::new();
    let block: Body<f64> = brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.5), tol());
    let body = match sub(&block, &rod(x0 + d, 0.3, r, -0.5, 3.5)) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("REVIEW hair2: build refused {e}");
            return;
        }
    };
    for dl in [
        0.0, 1e-9, -1e-9, 8e-9, -8e-9, 1.2e-8, -1.2e-8, 1.6e-8, -1.6e-8, 1e-6, -1e-6,
    ] {
        let map = geom_core::Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            dl,
        );
        let posed = match topo::transform_rigid(&body, &map, tol()) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("REVIEW hair2 δ {dl}: transform refused {e:?}");
                continue;
            }
        };
        for flip in [true, false] {
            rows.push(run(
                &format!("hair2 δ {dl} tilt {t} flip {flip}"),
                &posed,
                &tilted(1.25, t, flip),
            ));
        }
    }
    report(&rows);
}

/// Two bores far apart in `y` whose seams sit `dx` apart in `x`: a flat
/// cut's crossings on the two bores' seams differ in `u` by `dx` and
/// lie on different faces, so no face's crossings are ambiguous.
#[test]
fn survey_unrelated_crossings_a_few_eps_apart_in_u() {
    let mut rows = Vec::new();
    for dx in [0.0, 1e-12, 5e-10, 3e-9, 5e-9, 9e-9, 2e-8] {
        let block: Body<f64> = brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.5), tol());
        let body = sub(&block, &rod(-1.0, -1.0, 0.5, -0.5, 3.5))
            .and_then(|b| sub(&b, &rod(-1.0 + dx, 1.0, 0.5, -0.5, 3.5)));
        let body = match body {
            Ok(b) => b,
            Err(e) => {
                eprintln!("REVIEW unrelated dx {dx}: build refused {e}");
                continue;
            }
        };
        for (what, plane) in [
            ("flat", tilted(1.25, 0.0, false)),
            ("tilt 0.2 about x", skew(1.25, 0.2, 0.0)),
        ] {
            rows.push(run(&format!("unrelated dx {dx} {what}"), &body, &plane));
        }
    }
    report(&rows);
}

/// **A hole in an island in a hole goes to the island.** The nested
/// tube cut flat and at small tilts: each half's section is two faces,
/// the square holed by the pocket and the island holed by its bore,
/// each one ring, and both halves pass tiers 1 and 3.
#[test]
fn a_nested_tube_split_passes_with_each_hole_on_its_immediate_encloser() {
    let body = nested().expect("the nested tube builds");
    for (z, t, flip) in [(1.25, 0.0, false), (1.25, 0.2, true), (1.9, 0.0, false)] {
        let plane = tilted(z, t, flip);
        let row = run(&format!("nested z {z} tilt {t}"), &body, &plane);
        assert!(
            !row.refused && !row.invalid && !row.cancelling,
            "{}",
            row.line
        );
        let r = split(&body, &plane, tol()).unwrap();
        for part in [r.below, r.above] {
            let SplitPart::Body(h) = part else {
                panic!("material both sides")
            };
            let mut rings: Vec<usize> = section_faces(&h, &plane)
                .iter()
                .map(|&f| h.get_face(f).unwrap().rings.len())
                .collect();
            rings.sort_unstable();
            assert_eq!(rings, vec![1, 1], "{}", row.line);
        }
    }
}

/// **Crossings on different faces a few ε apart in `u` do not refuse.**
/// Two bores whose seams sit `dx ∈ (ε, K·ε)` apart in `x`, cut flat:
/// no face holds both crossings, so nothing the join orders is
/// ambiguous. Main answers these; 6ceb56ffb refuses
/// `Join(OrderEscalated)` on `split_join_order_column`.
#[test]
#[ignore = "reds on 6ceb56ffb: the column gap refuses across faces (review M1)"]
fn crossings_on_different_faces_a_few_eps_apart_in_u_do_not_refuse() {
    let eps = tol().eps();
    for dx in [3.0 * eps, 5.0 * eps] {
        let block: Body<f64> = brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.5), tol());
        let body = sub(&block, &rod(-1.0, -1.0, 0.5, -0.5, 3.5))
            .and_then(|b| sub(&b, &rod(-1.0 + dx, 1.0, 0.5, -0.5, 3.5)))
            .expect("two bores build");
        let row = run(&format!("dx {dx}"), &body, &tilted(1.25, 0.0, false));
        assert!(!row.refused && !row.invalid, "{}", row.line);
    }
}
