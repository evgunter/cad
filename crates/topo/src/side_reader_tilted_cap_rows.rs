//! **A moving cap whose normal has no zero component**: a brick whose
//! top cap is `z = h + s·x + r·y`, `h` the box, read over the box and at
//! members across it.
//!
//! The cap moves along `z` and not along its normal, so the height its
//! vertices share has a part IN the cap's plane, which no reading across
//! the normal drops. The turn's enclosure here is as wide as the box
//! makes it under every spelling measured (the three-component dot, the
//! dot across the normal, a stored frame), so these rows hold what is
//! true of it, not a width: every read is a member's, and the turn's
//! enclosure holds the members' turn
//! (`work/boxfinish/a-moving-face-whose-normal-has-no-zero-component-reads-its-side-only-over-a-narrow-box.md`).

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use super::*;

/// The brick over `[0, 1]²` whose top cap is `z = top + s·x + r·y`,
/// every edge's line stated from its start along its exact unit
/// direction (as `lined_brick`), every stated normal negated where
/// `inside_out`. Generic over the scalar so an `f64` member is the
/// same construction.
fn tilted<T: Decide + crate::props::AtRestPolicy>(
    top: T,
    (s, r): (f64, f64),
    inside_out: bool,
    tol: Tol,
) -> Body<T> {
    let iv = T::from_f64;
    let twin = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let mut body = Body::<T>::new();
    prism_ops(
        &mut body,
        &SQUARE,
        (0.0, 1.0),
        |x, y, z| {
            Point3::new(
                iv(x),
                iv(y),
                if z == 1.0 {
                    top + iv(s * x + r * y)
                } else {
                    iv(z)
                },
            )
        },
        FaceGeometry::Declined,
        tol,
    );
    let sign = if inside_out { -1.0 } else { 1.0 };
    for (face, _) in twin.faces() {
        let Ok(&Surface::Plane { frame, .. }) = twin.proven_face(face).surface() else {
            panic!("a brick's face is planar");
        };
        let (mut normal, mut u_ref) = (frame.axis(), frame.u_ref());
        if normal.z > 0.5 {
            normal = Vec3::new(-s, -r, 1.0).normalize();
            u_ref = Vec3::new(1.0, 0.0, s).normalize();
        }
        let corner = body.proven_face(face).edges()[0].ends[0];
        let origin = *body
            .get_vertex(corner)
            .and_then(|v| body.get_point(v.point))
            .expect("the twin's keys");
        body.set_face_surface(
            face,
            FaceSurface::New {
                surface: Surface::Plane {
                    frame: geom::Frame::new(origin, normal.map(|x| iv(sign * x)), u_ref.map(iv)),
                },
                sense: true,
            },
        )
        .expect("the face takes its plane");
    }
    let mut stated = std::collections::BTreeSet::new();
    for (face, _) in twin.faces() {
        for e in twin.proven_face(face).edges() {
            let plus = twin
                .edges
                .get(e.edge)
                .and_then(|edge| edge.claim(e.half))
                .expect("the twin's half claims its edge")
                .plus;
            if !plus || !stated.insert(e.edge) {
                continue;
            }
            let at = |b: &Body<f64>, v| {
                *b.get_point(b.get_vertex(v).expect("a vertex").point)
                    .expect("a point")
            };
            let (p0, p1) = (at(&twin, e.ends[0]), at(&twin, e.ends[1]));
            let ends = e.ends.map(|v| {
                *body
                    .get_point(body.get_vertex(v).expect("the twin's keys").point)
                    .expect("a point")
            });
            let mut spec = geom_brep::EdgeCurveSpec::line_between(ends[0], ends[1]);
            if p0.z == p1.z {
                // Horizontal: its exact direction at unit height.
                let lift = |p: Point3<f64>| {
                    Point3::new(p.x, p.y, if p.z == 1.0 { s * p.x + r * p.y } else { 0.0 })
                };
                let d = lift(p1) - lift(p0);
                let len = d.norm();
                spec.carrier = geom::Curve3::line(
                    ends[0],
                    // A chord over its length, each an `f64` constant of the row.
                    geom_core::UnitVec3::checked((d / len).map(iv)),
                );
                spec.param_end = iv(len);
            } else {
                // Upright: up `z`, as long as its top end stands.
                let (base, sgn) = if p0.z == 0.0 { (p0, 1.0) } else { (p1, -1.0) };
                spec.carrier = geom::Curve3::line(
                    ends[0],
                    geom_core::UnitVec3::checked(Vec3::new(0.0, 0.0, sgn).map(iv)),
                );
                spec.param_end = top + iv(s * base.x + r * base.y);
            }
            // Where the stated line does not certify over the box, the
            // minted one stays.
            let _ = body.set_edge_curve(e.edge, spec, tol);
        }
    }
    body
}

/// `read` on [`tilted`] at the symbolic box scalar, its height over
/// `1 ± half`.
fn over_tilted<R>(
    half: f64,
    tilt: (f64, f64),
    inside_out: bool,
    read: impl FnOnce(&Body<Boxed>, Band) -> R,
) -> R {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    geom_core::sym::with_session(budget, || {
        let top = Sym::param_over(
            ParamSymbol::new(test_utils::symbol_id("tilted_cap_top")),
            Interval::from_bounds(1.0 - half, 1.0 + half),
            1.0 - half,
            1.0 + half,
        );
        read(&tilted(top, tilt, inside_out, tol), band)
    })
    .0
}

const TILTS: [(f64, f64); 3] = [(0.0, 0.0), (0.3, 0.2), (-0.45, 0.6)];

/// **Every member reads what the box reads**: for each tilt
/// and orientation, the box's own aims read no wrong side, and the
/// reader's answer, where it answers, is every sampled member's
/// (members across the box, at its ends, and the `f64` body read at
/// each). Prints what reads, per width.
#[test]
fn a_tilted_boxed_cap_reads_only_its_members_side() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    for tilt in TILTS {
        // Upright only: an inside-out brick's own aims step out of their
        // faces and read nothing.
        for inside_out in [false] {
            let (want, wrong) = if inside_out {
                ("Ok(Side(In))", "Side(Out)")
            } else {
                ("Ok(Side(Out))", "Side(In)")
            };
            for half in [0.05, 0.1, 0.15, 0.25, 0.35, 0.45] {
                for f in [-1.0, -0.5, 0.0, 0.5, 1.0] {
                    let member = tilted::<f64>(1.0 + f * half, tilt, inside_out, tol);
                    let got = aimed_rays_report(&member, band);
                    if got != [want] {
                        println!(
                            "member {tilt:?} io={inside_out} at {}: {got:?}",
                            1.0 + f * half
                        );
                    }
                    for g in &got {
                        assert!(!g.contains(wrong), "member: {g}");
                    }
                }
                let (each, rays) = over_tilted(half, tilt, inside_out, |b, band| {
                    (each_aim_report(b, band), aimed_rays_report(b, band))
                });
                let reads = each
                    .iter()
                    .filter(|r| r.ends_with(&format!(": {want}")))
                    .count();
                for r in &each {
                    assert!(!r.contains(wrong), "{tilt:?} io={inside_out} ±{half}: {r}");
                }
                assert!(
                    rays[0] == want || rays[0].starts_with("Ok(Unread"),
                    "{tilt:?} io={inside_out} ±{half}: {rays:?}"
                );
                // The top cap's own ray and the reader read by rays to
                // `± 0.15` at every tilt: wider, a tilted body's stops
                // are its edges' stored lines, as wide as their chords.
                if half <= 0.15 {
                    assert_eq!(rays[0], want, "{tilt:?} ±{half}: the reader");
                    assert!(
                        each[0].ends_with(&format!(": {want}")),
                        "{tilt:?} ±{half}: the top cap's ray: {}",
                        each[0]
                    );
                }
                println!(
                    "tilt {tilt:?} inside_out {inside_out} ±{half}: {reads}/6 aims read; reader: {}",
                    &rays[0][..rays[0].len().min(60)]
                );
            }
        }
    }
}

/// **A tilted moving cap's turn seen along a fixed line holds its
/// members' and decides**: about a point of a line through the cap
/// `±2π`, beside it `0`, each narrower than `π` at `± 0.25`, where the
/// turn about a point riding the cap is `5` to `9` wide under every
/// spelling. Each vertex's enclosure enters once, against the line's
/// own point, so the turn is as wide as the cap moves across the line.
/// The enclosures are printed for their widths.
#[test]
fn a_tilted_moving_caps_turn_holds_its_members() {
    use crate::solid_contain::planar_turn;
    let tau = core::f64::consts::TAU;
    let (along, across) = AIM_INCIDENCE;
    for tilt in TILTS {
        let n = Vec3::new(-tilt.0, -tilt.1, 1.0).normalize();
        let (b1, _) = n.orthonormal_basis();
        let d = (n * -along + b1 * across).map(Boxed::from_f64);
        for (what, x, turns) in [("through", 0.37, 1.0), ("beside", 1.5, 0.0)] {
            let q = Point3::new(x, 0.41, 1.0 + tilt.0 * x + tilt.1 * 0.41).map(Boxed::from_f64);
            for half in [0.05, 0.25, 0.45] {
                let turn = over_tilted(half, tilt, false, |body, _| {
                    let (face, _) = top_cap(body, geom_core::Bounds::lo);
                    planar_turn(body, face, d, q).expect("four lines")
                });
                let (lo, hi) = (geom_core::Bounds::lo(turn), geom_core::Bounds::hi(turn));
                let want = turns * tau;
                assert!(
                    lo <= want.abs() && want.abs() <= hi || lo <= -want && -want <= hi,
                    "{tilt:?} {what} ±{half}: [{lo}, {hi}] misses ±{want}"
                );
                assert!(
                    half > 0.25 || hi - lo < core::f64::consts::PI,
                    "{tilt:?} {what} ±{half}: [{lo}, {hi}] decides nothing"
                );
                println!(
                    "turn {tilt:?} {what} ±{half}: [{lo}, {hi}] width {:e}",
                    hi - lo
                );
            }
        }
    }
}

/// **The `f64` turn's decision against the three-component form**: cap vertices jittered off the plane by up to the band, points
/// from deep inside to a nanometre off an edge or a vertex; the two
/// forms' `|turn| > π` must agree everywhere, and how far apart the two
/// turns lie is printed. A counterexample search: it goes red only on a
/// point where the two forms decide differently.
#[test]
fn the_f64_turn_decides_as_the_three_component_form_did() {
    use crate::solid_contain::planar_turn;
    let tol = Tol::witness();
    let mut rng = test_utils::fuzz::start("planar_turn_forms");
    let replay = test_utils::fuzz::replay();
    let mut next = move || rng.unit();
    let points = test_utils::fuzz::scaled(4_000);
    let (mut worst, mut n) = (0.0_f64, 0);
    for tilt in TILTS {
        let body = tilted::<f64>(1.0, tilt, false, tol);
        // Jitter the cap's four vertices off its plane by up to 1e-9.
        let (face, normal) = top_cap(&body, |z| z);
        let mut jittered = body.clone();
        for e in body.proven_face(face).edges() {
            let v = jittered.get_vertex(e.ends[0]).expect("a vertex").point;
            let p = *jittered.get_point(v).expect("a point");
            *jittered.points.get_mut(v).expect("a point") = p + normal * ((next() - 0.5) * 2e-9);
        }
        let corners: Vec<Point3<f64>> = jittered
            .proven_face(face)
            .edges()
            .iter()
            .map(|e| {
                *jittered
                    .get_point(jittered.get_vertex(e.ends[0]).unwrap().point)
                    .unwrap()
            })
            .collect();
        let old = |p: Point3<f64>| {
            let mut t = 0.0;
            for i in 0..corners.len() {
                let (u, w) = (corners[i] - p, corners[(i + 1) % corners.len()] - p);
                t += normal.dot(u.cross(w)).atan2(u.dot(w));
            }
            t
        };
        for _ in 0..points {
            let scale = [0.5, 1e-3, 1e-6, 1e-9][(next() * 4.0) as usize];
            let (x, y) = match (next() * 3.0) as usize {
                0 => (next(), next()),
                1 => (
                    (next() - 0.5) * scale + f64::from(u8::from(next() < 0.5)),
                    next(),
                ),
                _ => (
                    (next() - 0.5) * scale + f64::from(u8::from(next() < 0.5)),
                    (next() - 0.5) * scale + f64::from(u8::from(next() < 0.5)),
                ),
            };
            let p = Point3::new(x, y, 1.0 + tilt.0 * x + tilt.1 * y);
            let new = planar_turn(&jittered, face, normal, p).expect("four lines");
            let was = old(p);
            n += 1;
            assert_eq!(
                new.abs() > core::f64::consts::PI,
                was.abs() > core::f64::consts::PI,
                "{tilt:?} at ({x}, {y}): new {new}, old {was} ({replay})"
            );
            worst = worst.max((new - was).abs());
        }
    }
    println!("f64 turn: {n} points, worst |new − old| = {worst:e}");
}

/// **A family that turns inside out within its box is read as neither
/// side.** The upright brick with every plane's stated normal scaled by
/// a parameter over `[−0.5, 1]`: the member at `1` is the brick, whose
/// side is `Out`, and every member below `0` is the brick inside out,
/// whose side is `In`. Every aim is set aside and the reader's rays are
/// unread: a ray's count turns with the sign of `d·n̂`, which the box
/// does not decide.
#[test]
fn a_family_that_turns_inside_out_within_its_box_is_read_as_neither_side() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    for sign in [1.0, -1.0] {
        let member = tilted::<f64>(1.0, (0.0, 0.0), sign < 0.0, tol);
        let want = if sign > 0.0 { "Out" } else { "In" };
        assert_eq!(
            aimed_rays_report(&member, band),
            [format!("Ok(Side({want}))")],
            "the member whose normals are scaled by {sign}"
        );
    }
    let (each, reader) = geom_core::sym::with_session(budget, || {
        let flip = Sym::param_over(
            ParamSymbol::new(test_utils::symbol_id("tilted_cap_flip")),
            Interval::from_bounds(-0.5, 1.0),
            -0.5,
            1.0,
        );
        let mut body = tilted(Boxed::from_f64(1.0), (0.0, 0.0), false, tol);
        let faces: Vec<_> = body.faces().map(|(face, _)| face).collect();
        for face in faces {
            let Surface::Plane { frame } = *body.proven_face(face).surface().expect("a plane")
            else {
                panic!("a brick's face is planar");
            };
            body.set_face_surface(
                face,
                FaceSurface::New {
                    surface: Surface::Plane {
                        frame: geom::Frame::new(
                            frame.origin(),
                            frame.axis().map(|x| x * flip),
                            frame.u_ref(),
                        ),
                    },
                    sense: true,
                },
            )
            .expect("the face takes its plane");
        }
        (each_aim_report(&body, band), aimed_rays_report(&body, band))
    })
    .0;
    assert_eq!(each.len(), 6, "a brick offers six aims");
    for read in &each {
        assert!(
            read.contains("Ok(SetAside("),
            "an aim of a family that turns inside out reads a side: {read}"
        );
    }
    assert!(
        reader[0].starts_with("Ok(Unread"),
        "the reader's rays: {reader:?}"
    );
}
