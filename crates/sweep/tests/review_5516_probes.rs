//! Reviewer probes for PR 5516 (the seam region's cover census). They
//! name no new variant, so the same file runs on the merge base for a
//! differential; outcomes are read from the errors' `Debug` text.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::seam_region_common::{flipped, sphere_faces, sphere_of, split_at_meridian, v};
use geom::Surface;
use geom_core::{Tol, Vec3};
use std::f64::consts::PI;
use sweep::test_support::{ball_poled, brick, finished};
use topo::{Body, FaceKey};

fn outcome(body: &Body<f64>, tol: Tol) -> String {
    let vol = topo::mass_properties(body, tol)
        .map(|m| format!("{:.9}", m.volume))
        .unwrap_or_else(|e| format!("door refuses {e:?}"));
    match topo::validate_geometric(body, tol) {
        Ok(_) => format!("PASS vol={vol}"),
        Err(errs) => {
            let names: Vec<String> = errs
                .iter()
                .map(|e| {
                    let s = format!("{e:?}");
                    s.split([' ', '{', '(']).next().unwrap().to_string()
                        + if s.contains("times: 3") {
                            "{3}"
                        } else if s.contains("times: 2") {
                            "{2}"
                        } else {
                            ""
                        }
                })
                .collect();
            format!("REFUSED {names:?} vol={vol}")
        }
    }
}

fn vol(body: &Body<f64>, tol: Tol) -> f64 {
    topo::mass_properties(body, tol).unwrap().volume
}

/// Splits some sphere face of `body` along the meridian `dir` from its
/// north pole such that the volume is unchanged (an interior split);
/// returns whether a face took it.
fn split_interior(body: &mut Body<f64>, dir: Vec3<f64>, tol: Tol) -> bool {
    let before = vol(body, tol);
    for f in sphere_faces(body) {
        let mut trial = body.clone();
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            split_at_meridian(&mut trial, f, dir, tol);
        }))
        .is_ok();
        if ok && (vol(&trial, tol) - before).abs() < 1e-6 {
            *body = trial;
            return true;
        }
    }
    false
}

/// Splits some face along `dir` such that the volume rises by one ball
/// (an exterior split).
fn split_exterior(body: &mut Body<f64>, dir: Vec3<f64>, ball: f64, tol: Tol) -> bool {
    let before = vol(body, tol);
    for f in sphere_faces(body) {
        let mut trial = body.clone();
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            split_at_meridian(&mut trial, f, dir, tol);
        }))
        .is_ok();
        if ok && (vol(&trial, tol) - before - ball).abs() < 1e-6 {
            *body = trial;
            return true;
        }
    }
    false
}

fn dir(theta: f64) -> Vec3<f64> {
    v([theta.cos(), theta.sin(), 0.0])
}

/// A z-poled unit ball cut into `n` lunes by interior meridians.
fn lunes(n: usize, tol: Tol) -> Body<f64> {
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 0.0, 1.0]), tol),
        tol,
    );
    let mut body = Body::clone(&ball);
    for k in 0..n {
        split_interior(&mut body, dir(2.0 * PI * k as f64 / n as f64 + 0.1), tol);
    }
    body
}

/// Re-charts every sphere face of `body` (all on one entity) onto the
/// same sphere with its polar axis along `axis`.
fn recharted(body: &Body<f64>, axis: Vec3<f64>, tol: Tol) -> Result<Body<f64>, String> {
    let faces = sphere_faces(body);
    let (center, radius, _) = sphere_of(body, faces[0]);
    let axis = axis.normalize();
    let u_ref = axis.cross(v([0.3, 0.5, 0.7])).normalize();
    let surface = Surface::Sphere {
        center,
        radius,
        axis,
        u_ref,
    };
    let sense = |k: FaceKey| body.get_face(k).unwrap().sense;
    let chart = faces[1..].iter().fold(
        topo::Rechart::new(surface, faces[0], sense(faces[0])),
        |c, &k| c.with(k, sense(k)),
    );
    let mut out = body.clone();
    let charts = vec![chart];
    let specs = out
        .carried_redescriptions(&charts)
        .map_err(|e| format!("carry {e:?}"))?;
    out.set_face_surfaces_describing(charts, &specs, tol)
        .map_err(|e| format!("rechart {e:?}"))?;
    Ok(out)
}

/// CLAIM 1/2 table: covers built of many lunes, read at this run's ε.
#[test]
fn p_lune_covers_table() {
    let tol = Tol::witness();
    let ball = 4.0 * PI / 3.0;
    for n in [2usize, 3, 5, 8, 13] {
        let body = lunes(n, tol);
        let k = sphere_faces(&body).len();
        eprintln!("ROW lunes{n} faces={k}: {}", outcome(&body, tol));
        // one exterior split: a double cover of n+1 faces
        let mut two = body.clone();
        if split_exterior(&mut two, dir(0.1 + PI / n as f64), ball, tol) {
            eprintln!(
                "ROW lunes{n}+ext faces={}: {}",
                sphere_faces(&two).len(),
                outcome(&two, tol)
            );
            let mut three = two.clone();
            let mut done = false;
            for j in 0..32 {
                if split_exterior(&mut three, dir(0.05 + j as f64 * PI / 16.0), ball, tol) {
                    done = true;
                    break;
                }
            }
            if done {
                eprintln!(
                    "ROW lunes{n}+ext+ext faces={}: {}",
                    sphere_faces(&three).len(),
                    outcome(&three, tol)
                );
            }
            // flip every face of the double cover
            let all = sphere_faces(&two);
            eprintln!(
                "ROW lunes{n}+ext flipped-all: {}",
                outcome(&flipped(&two, &all), tol)
            );
        } else {
            eprintln!("ROW lunes{n}+ext: no exterior split found");
        }
        let all = sphere_faces(&body);
        eprintln!(
            "ROW lunes{n} flipped-all: {}",
            outcome(&flipped(&body, &all), tol)
        );
    }
}

/// A double cover whose lunes are each under a half turn, re-charted so
/// no face's loop carries both chart poles: no member reads Unencoded,
/// so check 6 never reaches the region and the census never runs.
#[test]
fn p_recharted_double_cover() {
    let tol = Tol::witness();
    let ball = 4.0 * PI / 3.0;
    let mut body = lunes(8, tol);
    assert!(split_exterior(&mut body, dir(0.1 + PI / 8.0), ball, tol));
    // cut every face to under a half turn
    for j in 0..64 {
        split_interior(
            &mut body,
            dir(0.1 + PI / 8.0 + j as f64 * PI / 32.0 + 0.013),
            tol,
        );
    }
    eprintln!(
        "ROW dc faces={} z-charted: {}",
        sphere_faces(&body).len(),
        outcome(&body, tol)
    );
    let single = lunes(8, tol);
    for axis in [v([1.0, 0.2, 0.3]), v([0.3, 1.0, 0.1]), v([0.6, 0.0, 0.8])] {
        match recharted(&body, axis, tol) {
            Ok(b) => eprintln!("ROW dc recharted {axis:?}: {}", outcome(&b, tol)),
            Err(e) => eprintln!("ROW dc recharted {axis:?}: {e}"),
        }
        match recharted(&single, axis, tol) {
            Ok(b) => eprintln!("ROW single8 recharted {axis:?}: {}", outcome(&b, tol)),
            Err(e) => eprintln!("ROW single8 recharted {axis:?}: {e}"),
        }
    }
}

/// Both-poles regions of 3 and 4 members (ball ∪ brick), flipped
/// together, and in part.
#[test]
fn p_both_poles_many_members() {
    let tol = Tol::witness();
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
    let base = Body::clone(
        &topo::boolean::union(&ball, &block, tol)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    eprintln!("ROW bp base: {}", outcome(&base, tol));
    let mut body = base.clone();
    for (i, theta) in [0.0f64, 0.8, -0.7, 0.4].iter().enumerate() {
        // meridians from the +y pole, in the x>0 half
        let d = v([theta.cos(), 0.0, theta.sin()]);
        let before = sphere_faces(&body).len();
        split_interior(&mut body, d, tol);
        let faces = sphere_faces(&body);
        eprintln!(
            "ROW bp split{i} faces={} (was {before}): {}",
            faces.len(),
            outcome(&body, tol)
        );
        eprintln!(
            "ROW bp split{i} flipped-all: {}",
            outcome(&flipped(&body, &faces), tol)
        );
        if faces.len() >= 3 {
            eprintln!(
                "ROW bp split{i} flipped-all-but-one: {}",
                outcome(&flipped(&body, &faces[1..]), tol)
            );
        }
    }
}

/// A corpus of legal sphere bodies; outcomes printed for a merge-base
/// differential.
#[test]
fn p_corpus() {
    let tol = Tol::witness();
    let poles = [v([0.0, 0.0, 1.0]), v([0.0, 1.0, 0.0]), v([1.0, 0.0, 0.0])];
    for &r in &[1e-3, 1.0, 1e3] {
        for (pi, &pole) in poles.iter().enumerate() {
            let c = v([0.31 * r, -0.17 * r, 0.23 * r]);
            let ball = finished("ball", ball_poled(r, c, pole, tol), tol);
            eprintln!("ROW corpus r={r} pole{pi} ball: {}", outcome(&ball, tol));
            let bricks = [
                // a face plane through the centre, normal to x
                (
                    (c.x - 3.0 * r, c.x),
                    (c.y - 2.0 * r, c.y + 2.0 * r),
                    (c.z - 2.0 * r, c.z + 2.0 * r),
                ),
                // a corner cutting the ball
                (
                    (c.x + 0.3 * r, c.x + 3.0 * r),
                    (c.y + 0.2 * r, c.y + 3.0 * r),
                    (c.z - 3.0 * r, c.z + 3.0 * r),
                ),
                // a bar through it
                (
                    (c.x - 2.0 * r, c.x + 2.0 * r),
                    (c.y - 0.2 * r, c.y + 0.25 * r),
                    (c.z + 0.1 * r, c.z + 0.4 * r),
                ),
                // a slab through the equator band
                (
                    (c.x - 2.0 * r, c.x + 2.0 * r),
                    (c.y - 2.0 * r, c.y + 2.0 * r),
                    (c.z - 0.3 * r, c.z + 0.2 * r),
                ),
            ];
            for (bi, (x, y, z)) in bricks.iter().enumerate() {
                let block = finished("block", brick(*x, *y, *z, tol), tol);
                for (op, res) in [
                    ("union", topo::boolean::union(&ball, &block, tol)),
                    ("subtract", topo::boolean::subtract(&ball, &block, tol)),
                    ("brick-ball", topo::boolean::subtract(&block, &ball, tol)),
                    ("intersect", topo::boolean::intersect(&ball, &block, tol)),
                ] {
                    let row = match res {
                        Ok(o) => match o.body() {
                            Some(b) => {
                                let body = Body::clone(&b.body);
                                let n = sphere_faces(&body).len();
                                format!("sphere_faces={n} {}", outcome(&body, tol))
                            }
                            None => "empty".to_string(),
                        },
                        Err(e) => format!(
                            "op refuses {}",
                            format!("{e:?}").chars().take(80).collect::<String>()
                        ),
                    };
                    eprintln!("ROW corpus r={r} pole{pi} brick{bi} {op}: {row}");
                }
            }
            // ball with ball
            let other = finished(
                "ball2",
                ball_poled(0.7 * r, v([c.x + 0.9 * r, c.y, c.z]), pole, tol),
                tol,
            );
            for (op, res) in [
                ("union", topo::boolean::union(&ball, &other, tol)),
                ("subtract", topo::boolean::subtract(&ball, &other, tol)),
            ] {
                let row = match res {
                    Ok(o) => match o.body() {
                        Some(b) => outcome(&Body::clone(&b.body), tol),
                        None => "empty".to_string(),
                    },
                    Err(e) => format!(
                        "op refuses {}",
                        format!("{e:?}").chars().take(80).collect::<String>()
                    ),
                };
                eprintln!("ROW corpus r={r} pole{pi} ball2 {op}: {row}");
            }
        }
    }
}

/// A spherical VOID: a brick less a ball wholly inside it. The void's
/// faces carry the flipped bit, its region has no boundary. A ball of
/// 2 faces and of more (lune splits) are both legal operands.
#[test]
fn p_spherical_void() {
    let tol = Tol::witness();
    let block = finished(
        "block",
        brick((-3.0, 3.0), (-3.0, 3.0), (-3.0, 3.0), tol),
        tol,
    );
    for n in [0usize, 2, 3, 5] {
        let ball = lunes(n, tol);
        let k = sphere_faces(&ball).len();
        let ball = finished("lunes", ball, tol);
        eprintln!(
            "ROW void operand lunes{n} faces={k}: {}",
            outcome(&ball, tol)
        );
        match topo::boolean::subtract(&block, &ball, tol) {
            Ok(o) => match o.body() {
                Some(b) => {
                    let body = Body::clone(&b.body);
                    let senses: Vec<bool> = sphere_faces(&body)
                        .iter()
                        .map(|&f| body.get_face(f).unwrap().sense)
                        .collect();
                    eprintln!(
                        "ROW void brick-lunes{n} sphere_faces={} senses={senses:?}: {}",
                        senses.len(),
                        outcome(&body, tol)
                    );
                }
                None => eprintln!("ROW void brick-lunes{n}: empty"),
            },
            Err(e) => eprintln!("ROW void brick-lunes{n}: op refuses {e:?}"),
        }
    }
}

#[test]
fn p_recharted_unencoded_census() {
    let tol = Tol::witness();
    let ball = 4.0 * PI / 3.0;
    let mut body = lunes(8, tol);
    assert!(split_exterior(&mut body, dir(0.1 + PI / 8.0), ball, tol));
    for j in 0..64 {
        split_interior(
            &mut body,
            dir(0.1 + PI / 8.0 + j as f64 * PI / 32.0 + 0.013),
            tol,
        );
    }
    for axis in [v([1.0, 0.2, 0.3]), v([0.6, 0.0, 0.8])] {
        let b = recharted(&body, axis, tol).unwrap();
        let faces = sphere_faces(&b);
        let un: Vec<_> = faces
            .iter()
            .filter(|&&f| crate::seam_region_common::unencoded(&b, f, tol))
            .collect();
        let band = geom_core::Band::linear(tol).unwrap();
        let reads: Vec<String> = faces
            .iter()
            .map(|&f| {
                let face = b.get_face(f).unwrap();
                let (outer, _) = topo::props::loop_edges(&b, face.outer).unwrap();
                format!(
                    "{:?}",
                    geom_brep::props::boundary_material_sign(
                        b.get_surface(face.surface).unwrap(),
                        &outer,
                        band
                    )
                )
                .chars()
                .take(40)
                .collect()
            })
            .collect();
        let mut counts = std::collections::BTreeMap::new();
        for r in &reads {
            *counts.entry(r.clone()).or_insert(0) += 1;
        }
        eprintln!(
            "ROW rechart {axis:?}: faces={} unencoded={} reads={counts:?}",
            faces.len(),
            un.len()
        );
    }
}

mod interval_rows {
    use geom::{Curve3, Surface};
    use geom_brep::EdgeCurveSpec;
    use geom_core::{Bounds, Interval, Point3, Real, Tol, Vec3};
    use topo::{Body, FaceKey, FaceSurface, MefSite};

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }
    fn sphere_faces(b: &Body<Interval>) -> Vec<FaceKey> {
        b.faces()
            .filter(|(_, f)| matches!(b.get_surface(f.surface), Some(Surface::Sphere { .. })))
            .map(|(k, _)| k)
            .collect()
    }
    fn vol(b: &Body<Interval>, tol: Tol) -> f64 {
        topo::mass_properties(b, tol).unwrap().volume.hi()
    }
    fn split(b: &mut Body<Interval>, key: FaceKey, theta: f64, tol: Tol) -> bool {
        let f = b.get_face(key).unwrap().clone();
        let Some(&Surface::Sphere {
            center,
            radius,
            axis,
            ..
        }) = b.get_surface(f.surface)
        else {
            return false;
        };
        let pole = axis.normalize();
        let Ok((_, hes)) = topo::props::loop_edges(b, f.outer) else {
            return false;
        };
        let points: Vec<_> = b.vertex_points().collect();
        let at = |h: topo::HalfEdgeKey| {
            let v = b.get_half_edge(h).unwrap().start;
            points.iter().find(|(k, _)| *k == v).unwrap().1
        };
        let find = |p: Point3<Interval>| {
            hes.iter()
                .copied()
                .find(|&h| (at(h) - p).norm().hi() < 1e-9)
        };
        let (Some(he1), Some(he2)) = (find(center + pole * radius), find(center - pole * radius))
        else {
            return false;
        };
        // pole is +y: directions in the xz plane
        let d = Vec3::new(iv(theta.cos()), iv(0.0), iv(theta.sin()));
        let circle = Curve3::Circle {
            center,
            axis: pole.cross(d),
            radius,
            u_ref: pole,
        };
        let Some(spec) = EdgeCurveSpec::arc_of_circle(circle, iv(0.0), Interval::pi()) else {
            return false;
        };
        b.mef(
            MefSite::Chords { he1, he2 },
            spec.at_rest_in_chart(f.surface, false),
            FaceSurface::Inherit,
            tol,
        )
        .is_ok()
    }
    fn try_split(b: &mut Body<Interval>, theta: f64, rise: f64, tol: Tol) -> bool {
        let before = vol(b, tol);
        for f in sphere_faces(b) {
            let mut t = b.clone();
            if split(&mut t, f, theta, tol) && (vol(&t, tol) - before - rise).abs() < 1e-6 {
                *b = t;
                return true;
            }
        }
        false
    }
    fn outcome(b: &Body<Interval>, tol: Tol) -> String {
        match topo::validate_geometric(b, tol) {
            Ok(_) => "PASS".into(),
            Err(e) => format!(
                "REFUSED {:?}",
                e.iter()
                    .map(|x| format!("{x:?}").chars().take(60).collect::<String>())
                    .collect::<Vec<_>>()
            ),
        }
    }

    #[test]
    fn p_interval_lunes() {
        let tol = Tol::witness();
        let ball = 4.0 * core::f64::consts::PI / 3.0;
        for c in [[0.0, 0.0, 0.0], [0.37, -1.3, 2.1]] {
            let base = sweep::test_support::ball_poled_y(
                iv(1.0),
                Vec3::new(iv(c[0]), iv(c[1]), iv(c[2])),
                tol,
            );
            let mut b = base.clone();
            for k in 0..6 {
                try_split(
                    &mut b,
                    0.1 + k as f64 * core::f64::consts::PI / 3.0,
                    0.0,
                    tol,
                );
            }
            eprintln!(
                "ROW iv c={c:?} lunes faces={} vol={:.6}: {}",
                sphere_faces(&b).len(),
                vol(&b, tol),
                outcome(&b, tol)
            );
            let mut two = b.clone();
            let ext = try_split(&mut two, 0.1 + core::f64::consts::PI / 6.0, ball, tol);
            eprintln!(
                "ROW iv c={c:?} +ext({ext}) faces={} vol={:.6}: {}",
                sphere_faces(&two).len(),
                vol(&two, tol),
                outcome(&two, tol)
            );
        }
        // both-poles: ball ∪ brick with a face plane x = 0 holding the y axis
        let ball_b =
            sweep::test_support::ball_poled_y(iv(1.0), Vec3::new(iv(0.0), iv(0.0), iv(0.0)), tol);
        let block: Body<Interval> =
            sweep::test_support::brick((-3.0, 0.0), (-2.0, 2.0), (-2.0, 2.0), tol);
        let ball_b = topo::test_support::finished("ball", ball_b, tol);
        let block = topo::test_support::finished("block", block, tol);
        match topo::boolean::union(&ball_b, &block, tol) {
            Ok(o) => {
                let mut body = Body::clone(&o.body().unwrap().body);
                eprintln!(
                    "ROW iv bp base faces={}: {}",
                    sphere_faces(&body).len(),
                    outcome(&body, tol)
                );
                for (i, th) in [0.0f64, 0.8, -0.7].iter().enumerate() {
                    try_split(&mut body, *th, 0.0, tol);
                    let faces = sphere_faces(&body);
                    let fl = faces.iter().fold(body.clone(), |b, &k| {
                        b.flipped_face_sense_for_tests(k).unwrap()
                    });
                    eprintln!(
                        "ROW iv bp split{i} faces={}: {} | flipped-all: {}",
                        faces.len(),
                        outcome(&body, tol),
                        outcome(&fl, tol)
                    );
                }
            }
            Err(e) => eprintln!("ROW iv bp union refuses {e:?}"),
        }
    }
}
