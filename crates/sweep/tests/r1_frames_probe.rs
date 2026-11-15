//! Reviewer probe (r1): every sweep door's stored surfaces and edge
//! carriers, dumped for a battery of builds, so the same file run on
//! the merge base and on the head gives a differential, and checked
//! against the body's own vertices and against the mesh's winding
//! (which reads the loops' traversal, never the stored sense).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, TAU};

use geom::Surface;
use geom_core::{Point2, Point3, RigidMotion, Tol, UnitVec3, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::Body;

fn tol() -> Tol {
    Tol::witness()
}

fn p(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn poly(pts: &[(f64, f64, f64)]) -> ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y, b)| (p(x, y), b)).collect())
}

fn circle(cx: f64, cy: f64, r: f64, sweep: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        p(cx + r, cy),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: p(cx, cy),
            radius: r,
            sweep,
        }),
    )])
}

fn tilted() -> SketchPlane<f64> {
    let rot = RigidMotion::rotation(UnitVec3::exact_z(), 0.4)
        .unwrap()
        .compose(RigidMotion::rotation(UnitVec3::exact_x(), 0.7).unwrap())
        .unwrap()
        .compose(RigidMotion::rotation(UnitVec3::exact_y(), -0.3).unwrap())
        .unwrap();
    SketchPlane::from_frame(rot.translated(Vec3::new(3.0, -2.0, 5.0)).unwrap())
}

fn fmt3(v: Vec3<f64>) -> String {
    format!("{:.17e} {:.17e} {:.17e}", v.x, v.y, v.z)
}

/// Dumps and checks one body.
fn dump(name: &str, body: &Body<f64>) {
    println!("CASE {name}");
    // Mesh winding: an independent outward.
    let mesh = mesh::tessellate(body, 5e-3, tol());
    let mesh = match mesh {
        Ok(m) => Some(m),
        Err(e) => {
            println!("  MESHERR {e:?}");
            None
        }
    };
    if let Some(m) = &mesh {
        println!("  VOLUME {:.12e}", mesh::validate::signed_volume(m));
    }
    let pts: Vec<_> = body.vertex_points().collect();
    for (fi, (fk, _)) in body.faces().enumerate() {
        let pose = topo::readback::face_pose(body, fk);
        let surface = body.face_surface(fk).expect("surface");
        // vertices on this face
        let on: Vec<Point3<f64>> = pts
            .iter()
            .filter(|(vk, _)| body.faces_of_vertex(*vk).unwrap().contains(&fk))
            .map(|(_, q)| *q)
            .collect();
        let sense = pose.as_ref().map(|p| p.sense).unwrap_or(true);
        let s = if sense { 1.0 } else { -1.0 };
        // mesh agreement: fraction of triangles whose winding normal
        // agrees with the stored outward normal at the centroid.
        let mut agree = (0usize, 0usize);
        let mut centroid = Vec3::new(0.0, 0.0, 0.0);
        let mut ntri = 0.0;
        if let Some(m) = &mesh {
            for patch in m.patches.iter().filter(|pt| pt.face == fk) {
                for t in &patch.triangles {
                    let [a, b, c] = t.map(|i| m.positions[i as usize]);
                    let tn = (b - a).cross(c - a);
                    let cen = Point3::new(
                        (a.x + b.x + c.x) / 3.0,
                        (a.y + b.y + c.y) / 3.0,
                        (a.z + b.z + c.z) / 3.0,
                    );
                    centroid = centroid + (cen - Point3::origin());
                    ntri += 1.0;
                    let outward = match surface {
                        Surface::Plane { normal, .. } => Some(*normal * s),
                        Surface::Cylinder { origin, axis, .. } => {
                            let d = cen - *origin;
                            Some((d - *axis * d.dot(*axis)) * s)
                        }
                        _ => None,
                    };
                    if let Some(o) = outward {
                        agree.1 += 1;
                        if tn.dot(o) > 0.0 {
                            agree.0 += 1;
                        }
                    }
                }
            }
        }
        match surface {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => {
                let res = on
                    .iter()
                    .map(|q| (*q - *origin).dot(*normal).abs())
                    .fold(0.0, f64::max);
                let far = on
                    .iter()
                    .map(|q| (*q - *origin).norm())
                    .fold(f64::INFINITY, f64::min);
                println!(
                    "  FACE {fi} PLANE sense {sense} n {} off {:.17e} u {} | unit {:.3e} uunit {:.3e} perp {:.3e} vres {:.3e} origin_to_nearest_vertex {:.6e} mesh_agree {}/{}",
                    fmt3(*normal),
                    (*origin - Point3::origin()).dot(*normal),
                    fmt3(*u_ref),
                    (normal.norm() - 1.0).abs(),
                    (u_ref.norm() - 1.0).abs(),
                    normal.dot(*u_ref).abs(),
                    res,
                    far,
                    agree.0,
                    agree.1
                );
                println!("  FACE {fi} PLANEORIGIN {}", fmt3(*origin - Point3::origin()));
            }
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => {
                let res = on
                    .iter()
                    .map(|q| {
                        let d = *q - *origin;
                        ((d - *axis * d.dot(*axis)).norm() - *radius).abs()
                    })
                    .fold(0.0, f64::max);
                println!(
                    "  FACE {fi} CYL sense {sense} o {} a {} r {:.17e} u {} | unit {:.3e} uunit {:.3e} perp {:.3e} vres {:.3e} mesh_agree {}/{}",
                    fmt3(*origin - Point3::origin()),
                    fmt3(*axis),
                    radius,
                    fmt3(*u_ref),
                    (axis.norm() - 1.0).abs(),
                    (u_ref.norm() - 1.0).abs(),
                    axis.dot(*u_ref).abs(),
                    res,
                    agree.0,
                    agree.1
                );
            }
            other => {
                let txt = format!("{other:?}");
                println!("  FACE {fi} OTHER sense {sense} {}", &txt[..txt.len().min(600)]);
            }
        }
        let _ = (centroid, ntri);
    }
    for (ei, (ek, _)) in body.edges().enumerate() {
        let curve = body.stored_curve(ek).expect("stored curve");
        let (t0, t1) = curve.params();
        let carrier = curve.carrier();
        let (va, vb) = body.edge_vertices(ek).unwrap();
        let qa = topo::readback::vertex_point(body, va).unwrap();
        let qb = topo::readback::vertex_point(body, vb).unwrap();
        let (e0, e1) = (carrier.eval(t0), carrier.eval(t1));
        // either orientation
        let fwd = (e0 - qa).norm().max((e1 - qb).norm());
        let rev = (e0 - qb).norm().max((e1 - qa).norm());
        let txt = format!("{carrier:?}");
        println!(
            "  EDGE {ei} span {:.17e} {:.17e} endres {:.3e} {}",
            t0,
            t1,
            fwd.min(rev),
            &txt[..txt.len().min(700)]
        );
    }
}

fn ext(name: &str, plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>, e: Extrusion<f64>) {
    let profile = match Profile::new(plane, loops).validate(tol()) {
        Ok(p) => p,
        Err(err) => {
            println!("CASE {name}\n  PROFILE-REFUSED {err:?}");
            return;
        }
    };
    match extrude(&profile, e, tol()) {
        Ok(b) => {
            dump(name, &b.body);
            println!("  REVERSED {}", b.reversed);
        }
        Err(err) => {
            let t = format!("{err:?}");
            println!("CASE {name}\n  REFUSED {}", &t[..t.len().min(400)]);
        }
    }
}

fn rev(
    name: &str,
    plane: SketchPlane<f64>,
    loops: Vec<ProfileLoop<f64>>,
    axis: RevolveAxis<f64>,
    r: Revolution<f64>,
) {
    let profile = match Profile::new(plane, loops).validate(tol()) {
        Ok(p) => p,
        Err(err) => {
            println!("CASE {name}\n  PROFILE-REFUSED {err:?}");
            return;
        }
    };
    match revolve(&profile, axis, r, tol()) {
        Ok(b) => {
            dump(name, &b.body);
            match sweep::revolved_caps(&b) {
                Ok(c) => println!(
                    "  CAPS start o {} n {} end o {} n {}",
                    fmt3(c.start.origin - Point3::origin()),
                    fmt3(c.start.axis),
                    fmt3(c.end.origin - Point3::origin()),
                    fmt3(c.end.axis)
                ),
                Err(e) => println!("  CAPS {e:?}"),
            }
        }
        Err(err) => {
            let t = format!("{err:?}");
            println!("CASE {name}\n  REFUSED {}", &t[..t.len().min(400)]);
        }
    }
}

fn dist(depth: f64, side: ExtrudeSide) -> Extrusion<f64> {
    Extrusion::Distance { depth, side }
}

fn y_axis() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

#[test]
fn r1_probe_dump() {
    let quad = || poly(&[(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0), (0.0, 1.0, 0.0)]);
    let cw = || poly(&[(0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 1.0, 0.0), (1.0, 0.0, 0.0)]);
    let bulged = || poly(&[(0.0, 0.0, 0.0), (2.0, 0.0, 0.3), (2.0, 1.5, 0.0), (0.0, 1.5, -0.2)]);
    let plate = || poly(&[(-3.0, -2.0, 0.0), (3.0, -2.0, 0.0), (3.0, 2.0, 0.0), (-3.0, 2.0, 0.0)]);
    let sq_hole = || poly(&[(-2.0, -1.0, 0.0), (-2.0, 1.0, 0.0), (-1.0, 1.0, 0.0), (-1.0, -1.0, 0.0)]);
    let sq_hole_ccw =
        || poly(&[(-2.0, -1.0, 0.0), (-1.0, -1.0, 0.0), (-1.0, 1.0, 0.0), (-2.0, 1.0, 0.0)]);
    let lens = || poly(&[(0.0, 0.0, 0.5), (2.0, 0.0, 0.5)]);
    // A D: a short chord closed by a major arc (bulge > 1).
    let dee = || poly(&[(0.0, 0.0, 0.0), (1.0, 0.0, 2.5)]);
    let far = || {
        poly(&[
            (1.0e6, 1.0e6, 0.0),
            (1.0e6 + 1.0, 1.0e6, 0.3),
            (1.0e6 + 1.0, 1.0e6 + 1.0, 0.0),
            (1.0e6, 1.0e6 + 1.0, 0.0),
        ])
    };
    let along = ExtrudeSide::Along;
    let against = ExtrudeSide::Against;
    let xy = SketchPlane::<f64>::xy;

    ext("e1 quad along", xy(), vec![quad()], dist(1.0, along));
    ext("e2 quad against", xy(), vec![quad()], dist(1.0, against));
    ext("e3 cw quad along", xy(), vec![cw()], dist(1.0, along));
    ext("e3b cw quad against", xy(), vec![cw()], dist(1.0, against));
    ext("e4 negative depth", xy(), vec![quad()], dist(-1.0, along));
    ext("e5 bulged tilted along", tilted(), vec![bulged()], dist(0.8, along));
    ext("e6 bulged tilted against", tilted(), vec![bulged()], dist(0.8, against));
    ext(
        "e7 plate holes tilted along",
        tilted(),
        vec![plate(), sq_hole(), circle(1.0, 0.0, 0.5, TAU)],
        dist(0.6, along),
    );
    ext(
        "e8 plate holes tilted against",
        tilted(),
        vec![plate(), sq_hole(), circle(1.0, 0.0, 0.5, TAU)],
        dist(0.6, against),
    );
    ext(
        "e8b plate ccw hole, cw circle",
        xy(),
        vec![plate(), sq_hole_ccw(), circle(1.0, 0.0, 0.5, -TAU)],
        dist(0.6, along),
    );
    let n = tilted().normal().get();
    ext("e9 vector +n tilted", tilted(), vec![bulged()], Extrusion::Vector(n * 1.5));
    ext("e10 vector -n tilted", tilted(), vec![bulged()], Extrusion::Vector(n * -0.7));
    ext(
        "e10b vector -n holes",
        tilted(),
        vec![plate(), sq_hole(), circle(1.0, 0.0, 0.5, TAU)],
        Extrusion::Vector(n * -0.7),
    );
    let u = tilted().frame().u().get();
    ext(
        "e11 vector nearly in plane",
        tilted(),
        vec![bulged()],
        Extrusion::Vector(u + n * 1e-3),
    );
    ext(
        "e11b vector slightly oblique",
        tilted(),
        vec![bulged()],
        Extrusion::Vector(n + u * 1e-10),
    );
    ext("e12 circle along", tilted(), vec![circle(0.5, 0.25, 1.0, TAU)], dist(1.0, along));
    ext("e13 circle against", tilted(), vec![circle(0.5, 0.25, 1.0, TAU)], dist(1.0, against));
    ext("e13b cw circle along", tilted(), vec![circle(0.5, 0.25, 1.0, -TAU)], dist(1.0, along));
    ext("e14 lens along", tilted(), vec![lens()], dist(1.0, along));
    ext("e15 lens against", tilted(), vec![lens()], dist(1.0, against));
    ext("e16 dee along", tilted(), vec![dee()], dist(1.0, along));
    ext("e17 dee against", tilted(), vec![dee()], dist(1.0, against));
    ext("e18 far along", xy(), vec![far()], dist(1.0, along));
    ext("e19 far tilted against", tilted(), vec![far()], dist(1.0, against));

    // A sketch whose origin is far from its profile, the body near the
    // world origin: the frame's origin is 1.4e6 (or 1.4e4) from the face.
    for (tag, big) in [("1e6", 1.0e6), ("1e4", 1.0e4), ("3e3", 3.0e3), ("1e3", 1.0e3), ("3e2", 3.0e2), ("1e2", 1.0e2)] {
        let rot = tilted().frame().turn_alone();
        let back = rot.transform_vec(Vec3::new(-big, -big, 0.0));
        let plane = || SketchPlane::from_frame(rot.translated(back).unwrap());
        let prof = || {
            poly(&[
                (big, big, 0.0),
                (big + 1.0, big, 0.3),
                (big + 1.0, big + 1.0, 0.0),
                (big, big + 1.0, 0.0),
            ])
        };
        ext(&format!("e20 far-origin {tag} along"), plane(), vec![prof()], dist(1.0, along));
        ext(&format!("e21 far-origin {tag} against"), plane(), vec![prof()], dist(1.0, against));
        rev(
            &format!("r20 far-origin {tag} partial"),
            plane(),
            vec![prof()],
            RevolveAxis { origin: p(big - 1.0, big), dir: Vec2::new(0.0, 1.0) },
            Revolution::Partial(-1.0),
        );
    }

    // Lofts.
    let sq = |a: f64| poly(&[(-a, -a, 0.0), (a, -a, 0.0), (a, a, 0.0), (-a, a, 0.0)]);
    let lofts: Vec<(&str, SketchPlane<f64>, SketchPlane<f64>)> = vec![
        ("l1 prism", xy(), SketchPlane::xy_at(Point3::new(0.0, 0.0, 1.0))),
        ("l2 shifted top", xy(), SketchPlane::xy_at(Point3::new(0.4, -0.3, 1.0))),
        (
            "l3 tilted top",
            xy(),
            SketchPlane::from_frame(
                RigidMotion::rotation(UnitVec3::exact_x(), 0.3)
                    .unwrap()
                    .translated(Vec3::new(0.2, 0.1, 1.5))
                    .unwrap(),
            ),
        ),
        (
            "l4 both tilted",
            tilted(),
            SketchPlane::from_frame(tilted().frame().translated(n * 2.0).unwrap()),
        ),
        (
            "l5 top below (opposite order)",
            xy(),
            SketchPlane::xy_at(Point3::new(0.0, 0.0, -1.0)),
        ),
        (
            "l6 top facing opposite",
            xy(),
            SketchPlane::from_frame(
                RigidMotion::rotation(UnitVec3::exact_x(), core::f64::consts::PI)
                    .unwrap()
                    .translated(Vec3::new(0.0, 0.0, 1.0))
                    .unwrap(),
            ),
        ),
    ];
    for (name, a, b) in lofts {
        match loft_body(&[vec![sq(1.0)], vec![sq(0.6)]], &[a, b], 1, tol()) {
            Ok(l) => dump(name, &l.body),
            Err(e) => {
                let t = format!("{e:?}");
                println!("CASE {name}\n  REFUSED {}", &t[..t.len().min(400)]);
            }
        }
    }
    match loft_body(
        &[vec![circle(0.0, 0.0, 1.0, TAU)], vec![circle(0.0, 0.0, 0.6, TAU)]],
        &[xy(), SketchPlane::xy_at(Point3::new(0.3, 0.0, 1.0))],
        1,
        tol(),
    ) {
        Ok(l) => dump("l7 circles", &l.body),
        Err(e) => println!("CASE l7 circles\n  REFUSED {e:?}"),
    }

    // Revolves.
    let band = || poly(&[(1.0, 0.0, 0.0), (1.5, 0.0, 0.0), (1.5, 1.0, 0.0), (1.0, 1.0, 0.0)]);
    let bandb = || poly(&[(1.0, 0.0, 0.0), (1.5, 0.0, 0.25), (1.5, 1.0, 0.0), (1.0, 1.0, 0.0)]);
    let touch = || poly(&[(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0), (0.0, 1.0, 0.0)]);
    let tri_touch = || poly(&[(0.0, 0.0, 0.0), (1.0, 0.5, 0.0), (0.0, 1.0, 0.0)]);
    let neg_axis = RevolveAxis {
        origin: p(0.0, 0.0),
        dir: Vec2::new(0.0, -1.0),
    };
    let skew_axis = RevolveAxis {
        origin: p(-0.5, 0.2),
        dir: Vec2::new(0.3, 1.0),
    };
    rev("r1 band +q", xy(), vec![band()], y_axis(), Revolution::Partial(FRAC_PI_2));
    rev("r2 band -q", xy(), vec![band()], y_axis(), Revolution::Partial(-FRAC_PI_2));
    rev("r3 band near full", xy(), vec![band()], y_axis(), Revolution::Partial(6.2));
    rev("r3b band near full neg", xy(), vec![band()], y_axis(), Revolution::Partial(-6.2));
    rev("r4 touch +", xy(), vec![touch()], y_axis(), Revolution::Partial(1.0));
    rev("r5 touch -", xy(), vec![touch()], y_axis(), Revolution::Partial(-1.0));
    rev("r5b tri touch -", tilted(), vec![tri_touch()], y_axis(), Revolution::Partial(-2.0));
    rev("r6 band full", xy(), vec![band()], y_axis(), Revolution::Full);
    rev("r7 touch full", xy(), vec![touch()], y_axis(), Revolution::Full);
    rev("r8 circle +", xy(), vec![circle(2.0, 0.0, 0.5, TAU)], y_axis(), Revolution::Partial(FRAC_PI_2));
    rev("r9 circle - tilted", tilted(), vec![circle(2.0, 0.0, 0.5, TAU)], y_axis(), Revolution::Partial(-1.3));
    rev("r10 bulged tilted -", tilted(), vec![bandb()], y_axis(), Revolution::Partial(-2.5));
    rev("r11 bulged tilted + negaxis", tilted(), vec![bandb()], neg_axis, Revolution::Partial(2.5));
    rev("r12 bulged tilted skew axis", tilted(), vec![bandb()], skew_axis, Revolution::Partial(-0.9));
    rev("r13 bulged tilted full", tilted(), vec![bandb()], y_axis(), Revolution::Full);
    rev("r14 circle full tilted", tilted(), vec![circle(2.0, 0.0, 0.5, TAU)], skew_axis, Revolution::Full);
    rev(
        "r15 band with hole -",
        tilted(),
        vec![
            poly(&[(1.0, 0.0, 0.0), (3.0, 0.0, 0.0), (3.0, 2.0, 0.0), (1.0, 2.0, 0.0)]),
            circle(2.0, 1.0, 0.4, TAU),
        ],
        y_axis(),
        Revolution::Partial(-1.1),
    );
    // A semicircle on the axis: a sphere, with a pinned arc start.
    rev(
        "r16 half disc on axis partial",
        tilted(),
        vec![poly(&[(0.0, -1.0, 1.0), (0.0, 1.0, 0.0)])],
        y_axis(),
        Revolution::Partial(1.2),
    );
    rev(
        "r17 half disc on axis full",
        tilted(),
        vec![poly(&[(0.0, -1.0, 1.0), (0.0, 1.0, 0.0)])],
        y_axis(),
        Revolution::Full,
    );
}
