//! Review probes for PR 5516 (the seam region's cover census). Each row
//! prints what tier 3 says, so the same file runs on the merge base.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::seam_region_common::{flipped, sphere_faces, sphere_of, split_at_meridian, v};
use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::{ball_poled, brick, finished};
use topo::{Body, FaceKey, FaceSurface, MefSite};

fn report(label: &str, body: &Body<f64>, tol: Tol) -> String {
    let verdict = match topo::validate_geometric(body, tol) {
        Ok(_) => "PASSES".to_string(),
        Err(e) => format!("refused {e:?}"),
    };
    let vol = topo::mass_properties(body, tol).map(|m| m.volume);
    let line = format!(
        "[eps={:e}] {label}: {} sphere faces, {verdict}; door volume {vol:?}",
        tol.eps(),
        sphere_faces(body).len()
    );
    println!("{line}");
    line
}

fn volume(body: &Body<f64>, tol: Tol) -> Option<f64> {
    topo::mass_properties(body, tol).ok().map(|m| m.volume)
}

/// Splits the face of `body` that a meridian in direction `dir` passes
/// through INSIDE it (the split that keeps the volume), returning
/// whether one did.
fn split_inside(body: &mut Body<f64>, dir: Vec3<f64>, tol: Tol) -> bool {
    let before = volume(body, tol).unwrap();
    for f in sphere_faces(body) {
        let mut b = body.clone();
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            split_at_meridian(&mut b, f, dir, tol)
        }))
        .is_ok();
        if ok && volume(&b, tol).is_some_and(|w| (w - before).abs() <= 1e-9 * before.abs()) {
            *body = b;
            return true;
        }
    }
    false
}

/// Splits the face of `body` that a meridian in direction `dir` passes
/// OUTSIDE (the split that adds a cover), returning whether one did.
fn split_outside(body: &mut Body<f64>, dir: Vec3<f64>, tol: Tol) -> bool {
    let before = volume(body, tol).unwrap();
    for f in sphere_faces(body) {
        let mut b = body.clone();
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            split_at_meridian(&mut b, f, dir, tol)
        }))
        .is_ok();
        if ok && volume(&b, tol).is_some_and(|w| (w - before).abs() > 1e-6 * before.abs()) {
            *body = b;
            return true;
        }
    }
    false
}

fn points(body: &Body<f64>) -> Vec<(topo::VertexKey, Point3<f64>)> {
    body.vertex_points().collect()
}

/// Splits every sphere edge running pole to pole at the equator.
fn split_meridians_at_equator(body: &mut Body<f64>, tol: Tol) {
    let f0 = sphere_faces(body)[0];
    let (center, radius, pole) = sphere_of(body, f0);
    let pts = points(body);
    let at = |k: topo::VertexKey| pts.iter().find(|(q, _)| *q == k).unwrap().1;
    let mut todo = Vec::new();
    for (ek, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let carrier = c.carrier().clone();
        if !matches!(carrier, Curve3::Circle { .. }) {
            continue;
        }
        let (t0, t1) = c.params();
        let h = |t: f64| (carrier.eval(t) - center).dot(pole);
        let (a, b) = (h(t0), h(t1));
        if (a.abs() - radius).abs() > 1e-9 || (b.abs() - radius).abs() > 1e-9 || a * b > 0.0 {
            continue;
        }
        let _ = at;
        let (mut lo, mut hi) = (t0, t1);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if (h(mid) > 0.0) == (a > 0.0) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        todo.push((ek, 0.5 * (lo + hi)));
    }
    for (ek, t) in todo {
        body.split_edge(ek, t, tol).unwrap();
    }
}

/// Splits each sphere face whose loop holds both poles along the
/// equator, through its own interior.
fn split_faces_at_equator(body: &mut Body<f64>, tol: Tol) {
    let f0 = sphere_faces(body)[0];
    let (center, radius, pole) = sphere_of(body, f0);
    for f in sphere_faces(body) {
        let face = body.faces().find(|(k, _)| *k == f).unwrap().1.clone();
        if !face.rings.is_empty() {
            continue;
        }
        let (_, hes) = topo::props::loop_edges(body, face.outer).unwrap();
        let pts = points(body);
        let at = |h: topo::HalfEdgeKey| {
            let v = body.get_half_edge(h).unwrap().start;
            pts.iter().find(|(k, _)| *k == v).unwrap().1
        };
        let on_eq: Vec<_> = hes
            .iter()
            .copied()
            .filter(|&h| (at(h) - center).dot(pole).abs() < 1e-9)
            .collect();
        if on_eq.len() != 2 {
            continue;
        }
        // The half-edge leaving the equator downward starts the face's
        // CCW (about the pole, under sense) sweep.
        let next_z = |h: topo::HalfEdgeKey| {
            let i = hes.iter().position(|&x| x == h).unwrap();
            let n = hes[(i + 1) % hes.len()];
            (at(n) - center).dot(pole)
        };
        let (down, up) = if next_z(on_eq[0]) < 0.0 {
            (on_eq[0], on_eq[1])
        } else {
            (on_eq[1], on_eq[0])
        };
        let a = (at(down) - center) / radius;
        let b = (at(up) - center) / radius;
        let p = pole;
        let w = p.cross(a);
        let mut ang = b.dot(w).atan2(b.dot(a));
        if !face.sense {
            ang = -ang;
        }
        if ang <= 0.0 {
            ang += std::f64::consts::TAU;
        }
        let axis = if face.sense { p } else { -p };
        let circle = Curve3::Circle {
            center,
            axis,
            radius,
            u_ref: a,
        };
        let spec = EdgeCurveSpec::arc_of_circle(circle, 0.0, ang)
            .unwrap()
            .at_rest_in_chart(face.surface, false);
        let r = body.mef(
            MefSite::Chords { he1: down, he2: up },
            spec,
            FaceSurface::Inherit,
            tol,
        );
        if let Err(e) = r {
            println!("equator mef on {f:?} refused: {e:?}");
        }
    }
}

fn tols() -> Tol {
    Tol::witness()
}

/// A z-poled ball whose lune is split twice along exterior meridians:
/// four faces cover the sphere three times over.
#[test]
fn p1_triple_cover() {
    let tol = tols();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 0.0, 1.0]), tol),
        tol,
    );
    let mut body = Body::clone(&ball);
    report("ball", &body, tol);
    assert!(split_outside(&mut body, v([0.0, 1.0, 0.0]), tol));
    report("double cover", &body, tol);
    let mut triple = None;
    for d in [
        [1.0, 1.0, 0.0],
        [-1.0, 1.0, 0.0],
        [1.0, -1.0, 0.0],
        [-1.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
    ] {
        let mut b = body.clone();
        let n = v(d).normalize();
        if split_outside(&mut b, n, tol) {
            let r = report(&format!("triple cover via {d:?}"), &b, tol);
            triple = Some(r);
            break;
        }
    }
    let r = triple.expect("a triple cover was built");
    assert!(r.contains("times: 3"), "{r}");
}

/// The double cover, then every face split at the equator: no face's
/// loop holds both poles, so every face reads its own side and none
/// enters a seam region read.
#[test]
fn p2_double_cover_split_at_the_equator() {
    let tol = tols();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 0.0, 1.0]), tol),
        tol,
    );
    // Control: the valid ball split so.
    let mut ctl = Body::clone(&ball);
    split_meridians_at_equator(&mut ctl, tol);
    split_faces_at_equator(&mut ctl, tol);
    let c = report("valid ball, equator-split", &ctl, tol);
    let mut body = Body::clone(&ball);
    assert!(split_outside(&mut body, v([0.0, 1.0, 0.0]), tol));
    report("double cover", &body, tol);
    split_meridians_at_equator(&mut body, tol);
    report("double cover, meridians split", &body, tol);
    split_faces_at_equator(&mut body, tol);
    let r = report("double cover, faces split at the equator", &body, tol);
    assert!(c.contains("PASSES"), "{c}");
    assert!(!r.contains("PASSES"), "an over-cover passes tier 3: {r}");
}

/// Witness A's two halves, one split again: three members whose region
/// carries both poles. Valid as built; all three flipped.
#[test]
fn p3_three_members_carrying_both_poles() {
    let tol = tols();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 1.0, 0.0]), tol),
        tol,
    );
    let block = finished(
        "block",
        brick((-3.0, 0.0), (-2.0, 2.0), (-2.0, 2.0), tol),
        tol,
    );
    let mut body = Body::clone(
        &topo::boolean::union(&ball, &block, tol)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    let mut n = 1;
    for a in [0.0f64, 40.0, -40.0, 70.0, -70.0, 20.0, -20.0, 85.0] {
        let r = a.to_radians();
        if split_inside(&mut body, v([r.cos(), 0.0, r.sin()]), tol) {
            n += 1;
        }
        let faces = sphere_faces(&body);
        let ok = report(&format!("{n} members, valid"), &body, tol);
        assert!(ok.contains("PASSES"), "{ok}");
        let fl = report(
            &format!("{n} members, all flipped"),
            &flipped(&body, &faces),
            tol,
        );
        if n >= 2 {
            assert!(fl.contains(&format!("times: {n}")), "{fl}");
        }
    }
}

/// Valid balls, tilted and off-origin, big and small, split many times
/// inside: every one passes.
#[test]
fn p4_valid_split_balls() {
    let tol = tols();
    for (r, c, pole) in [
        (1.0, [0.0; 3], [0.0, 0.0, 1.0]),
        (1e3, [5e3, -2e3, 7e2], [0.3, 0.4, 0.866]),
        (1e-4, [0.5, 0.25, -0.125], [1.0, 2.0, 3.0]),
        (2.5, [1e2, 1e2, 1e2], [0.0, 1.0, 1e-7]),
        (1e-6, [0.0; 3], [0.0, 1.0, 0.0]),
    ] {
        let pole = v(pole).normalize();
        let Ok(ball) =
            std::panic::catch_unwind(|| finished("ball", ball_poled(r, v(c), pole, tol), tol))
        else {
            println!("R={r}: the ball does not build at this eps (not the census)");
            continue;
        };
        let mut body = Body::clone(&ball);
        let x = if pole.x.abs() < 0.9 {
            v([1.0, 0.0, 0.0])
        } else {
            v([0.0, 1.0, 0.0])
        };
        let e1 = (x - pole * x.dot(pole)).normalize();
        let e2 = pole.cross(e1);
        let mut n = 0;
        for k in 0..12 {
            let a = (k as f64) * 31.0_f64.to_radians() + 0.1;
            if split_inside(&mut body, e1 * a.cos() + e2 * a.sin(), tol) {
                n += 1;
            }
        }
        let l = report(&format!("R={r} c={c:?} split {n} times"), &body, tol);
        assert!(l.contains("PASSES"), "{l}");
        let faces = sphere_faces(&body);
        let fl = report(&format!("R={r} all flipped"), &flipped(&body, &faces), tol);
        assert!(!fl.contains("PASSES"), "{fl}");
    }
}

/// A bar through a ball (the PR's ringed row), its unringed lune split
/// along an exterior meridian: a double cover one of whose members
/// carries rings.
#[test]
fn p5_ringed_double_cover() {
    let tol = tols();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 1.0, 0.0]), tol),
        tol,
    );
    let bar = finished(
        "bar",
        brick((-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4), tol),
        tol,
    );
    let body = Body::clone(
        &topo::boolean::subtract(&ball, &bar, tol)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    report("bar through ball", &body, tol);
    let mut found = false;
    for d in [
        [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
        [1.0, 0.0, 1.0],
        [-1.0, 0.0, -1.0],
    ] {
        let mut b = body.clone();
        if split_outside(&mut b, v(d).normalize(), tol) {
            let r = report(&format!("ringed double cover via {d:?}"), &b, tol);
            let faces = sphere_faces(&b);
            report(
                "ringed double cover, all flipped",
                &flipped(&b, &faces),
                tol,
            );
            for &f in &faces {
                report(
                    &format!("ringed double cover, {f:?} flipped"),
                    &flipped(&b, &[f]),
                    tol,
                );
            }
            assert!(!r.contains("PASSES"), "{r}");
            found = true;
            break;
        }
    }
    assert!(found);
}

/// The 3-face double cover with one member flipped (mixed sense).
#[test]
fn p6_mixed_double_cover() {
    let tol = tols();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 0.0, 1.0]), tol),
        tol,
    );
    let mut body = Body::clone(&ball);
    assert!(split_outside(&mut body, v([0.0, 1.0, 0.0]), tol));
    let faces: Vec<FaceKey> = sphere_faces(&body);
    for &f in &faces {
        let r = report(
            &format!("double cover, {f:?} flipped"),
            &flipped(&body, &[f]),
            tol,
        );
        assert!(!r.contains("PASSES"), "{r}");
    }
    for (i, &f) in faces.iter().enumerate() {
        let rest: Vec<_> = faces.iter().copied().filter(|&g| g != f).collect();
        let r = report(
            &format!("double cover, all but #{i} flipped"),
            &flipped(&body, &rest),
            tol,
        );
        assert!(!r.contains("PASSES"), "{r}");
    }
}

#[allow(dead_code)]
fn unused(_: Surface<f64>) {}
