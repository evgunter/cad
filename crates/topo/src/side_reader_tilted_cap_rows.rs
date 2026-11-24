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
        let Surface::Plane { frame, .. } = *twin.proven_face(face).surface() else {
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
                println!(
                    "tilt {tilt:?} inside_out {inside_out} ±{half}: {reads}/6 aims read; reader: {}",
                    &rays[0][..rays[0].len().min(60)]
                );
            }
        }
    }
}

/// **A tilted moving cap's turn holds its members'**: about
/// a point of the cap `±2π`, about a point beside it `0`, the
/// enclosure printed for its width.
#[test]
fn a_tilted_moving_caps_turn_holds_its_members() {
    use crate::solid_contain::planar_turn;
    let tau = core::f64::consts::TAU;
    for tilt in TILTS {
        for (what, x, turns) in [
            ("inside", 0.37, 1.0),
            ("beside", 1.5, 0.0),
            ("near", 0.999, 1.0),
        ] {
            for half in [0.05, 0.25, 0.45] {
                let turn = over_tilted(half, tilt, false, |body, _| {
                    let (face, normal) = top_cap(body, geom_core::Bounds::lo);
                    // The cap's corner over the origin carries `top` itself.
                    let corner = body
                        .proven_face(face)
                        .edges()
                        .iter()
                        .map(|e| {
                            *body
                                .get_point(body.get_vertex(e.ends[0]).unwrap().point)
                                .unwrap()
                        })
                        .find(|c| {
                            geom_core::Bounds::hi(c.x) == 0.0 && geom_core::Bounds::hi(c.y) == 0.0
                        })
                        .expect("a corner over the origin");
                    let height = corner.z + Boxed::from_f64(tilt.0 * x + tilt.1 * 0.41);
                    let p = Point3::new(Boxed::from_f64(x), Boxed::from_f64(0.41), height);
                    planar_turn(body, face, normal, p).expect("four lines")
                });
                let (lo, hi) = (geom_core::Bounds::lo(turn), geom_core::Bounds::hi(turn));
                let want = turns * tau;
                assert!(
                    lo <= want && want <= hi,
                    "{tilt:?} {what} ±{half}: [{lo}, {hi}] misses {want}"
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

// ---- probe (M14): reach and tier 3′ over the fixture bricks ----

fn m14_label<T: Decide>(body: &Body<T>, face: FaceKey) -> String {
    match crate::face_normal::face_plane(body, face) {
        Some(p) => {
            let n = [p.normal.x, p.normal.y, p.normal.z].map(m14_mid);
            if n[2] > 0.5 {
                "top".to_owned()
            } else if n[2] < -0.5 {
                "bottom".to_owned()
            } else {
                format!("wall({:+.0},{:+.0})", n[0], n[1])
            }
        }
        None => "?".to_owned(),
    }
}

fn m14_rows<T: Decide + crate::props::AtRestPolicy + geom_core::Bounds>(
    what: &str,
    body: &Body<T>,
    band: Band,
    tol: Tol,
) {
    let faces: Vec<FaceKey> = body.faces().map(|(f, _)| f).collect();
    for (i, f) in faces.iter().enumerate() {
        println!("M14 {what} key {f:?} = {} #{i}", m14_label(body, *f));
    }
    for r in each_aim_report(body, band) {
        let label = faces
            .iter()
            .find(|f| r.starts_with(&format!("{f:?}:")))
            .map(|f| m14_label(body, *f))
            .unwrap_or_default();
        let cut: String = r.chars().take(700).collect();
        println!("M14 {what} aim {label} | {cut}");
    }
    for r in aimed_rays_report(body, band) {
        let cut: String = r.chars().take(300).collect();
        println!("M14 {what} reader | {cut}");
    }
    if std::env::var_os("BOXFINISH_M14_CENSUS").is_some() {
        let errors = crate::census::census_and_certify(
            body,
            &crate::boolean::ContactRecords::default(),
            band,
            tol,
            None,
        );
        println!("M14 {what} census | {} findings", errors.len());
        for e in errors {
            let cut: String = format!("{e:?}").chars().take(500).collect();
            println!("M14 {what} census-finding | {cut}");
        }
    }
}

fn m14_body<T: Decide + crate::props::AtRestPolicy>(kind: usize, top: T, tol: Tol) -> Body<T> {
    match kind {
        0 => brick_at(top, T::from_f64(1.0), false, tol),
        1 => lined_brick(top, tol),
        2 => tilted(top, TILTS[1], false, tol),
        _ => tilted(top, TILTS[2], false, tol),
    }
}

const M14_KINDS: [&str; 4] = ["brick", "lined", "tilt(0.3,0.2)", "tilt(-0.45,0.6)"];
const M14_WIDTHS: [f64; 4] = [0.05, 0.1, 0.25, 0.45];

/// Probe: prints only. `BOXFINISH_M14_FIX=1` fixes each aimed ray;
/// `BOXFINISH_M14_CENSUS=1` adds tier 3′'s census by itself.
#[test]
#[ignore = "probe: cargo nextest run -p topo m14_probe --run-ignored all --no-capture"]
fn m14_probe() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    for (kind, name) in M14_KINDS.iter().enumerate() {
        for w in M14_WIDTHS {
            for f in [-1.0, -0.5, 0.0, 0.5, 1.0] {
                let member = m14_body::<f64>(kind, 1.0 + f * w, tol);
                println!(
                    "M14 {name} w={w} member {} | {:?}",
                    1.0 + f * w,
                    aimed_rays_report(&member, band)
                );
            }
            let start = std::time::Instant::now();
            geom_core::sym::with_session(budget, || {
                let top = Sym::param_over(
                    ParamSymbol::new(test_utils::symbol_id("m14_top")),
                    Interval::from_bounds(1.0 - w, 1.0 + w),
                    1.0 - w,
                    1.0 + w,
                );
                m14_rows(
                    &format!("{name} w={w} sym"),
                    &m14_body::<Boxed>(kind, top, tol),
                    band,
                    tol,
                );
            });
            println!("M14 {name} w={w} sym took {:?}", start.elapsed());
            let start = std::time::Instant::now();
            let built = std::panic::catch_unwind(|| {
                m14_body::<Interval>(kind, Interval::from_bounds(1.0 - w, 1.0 + w), tol)
            });
            match built {
                Ok(body) => m14_rows(&format!("{name} w={w} iv"), &body, band, tol),
                Err(e) => println!(
                    "M14 {name} w={w} iv construction refuses | {}",
                    e.downcast_ref::<String>().map_or("?", |s| s.as_str())
                ),
            }
            println!("M14 {name} w={w} iv took {:?}", start.elapsed());
        }
    }
}
