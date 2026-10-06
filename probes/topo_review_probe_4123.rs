//! Reviewer probe for PR #4123 (not for merge): sphere faces whose
//! chart rectangle, as `sphere_rect` reads it, might be TIGHTER than the
//! face.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Point3, Tol, Vec3};

use crate::body::Body;
use crate::boolean::boxes::{BoxFrame, face_box, sphere_reach};
use crate::entity::FaceKey;
use crate::geometry::SurfaceKey;
use crate::{FaceSurface, MefSite, MevSite};

fn radial(u: f64) -> Vec3<f64> {
    Vec3::new(u.cos(), u.sin(), 0.0)
}
fn at(u: f64, h: f64) -> Point3<f64> {
    let rho = (1.0 - h * h).sqrt();
    Point3::origin() + radial(u) * rho + Vec3::unit_z() * h
}
fn arc(
    body: &mut Body<f64>,
    sphere: SurfaceKey,
    carrier: Curve3<f64>,
    (t0, t1): (f64, f64),
) -> EdgeCurveSpec<f64> {
    let Curve3::Circle { center, axis, .. } = carrier else {
        unreachable!()
    };
    let plane = body.add_surface(Surface::Plane {
        origin: center,
        normal: axis,
        u_ref: if axis.z.abs() > 0.5 {
            Vec3::unit_x()
        } else {
            Vec3::unit_z()
        },
    });
    let witness = carrier.mid_point(t0, t1);
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: sphere,
            s2: plane,
            witness,
        },
        carrier,
        param_start: t0,
        param_end: t1,
    }
}
fn rim(h: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, h),
        axis: Vec3::unit_z(),
        radius: (1.0 - h * h).sqrt(),
        u_ref: Vec3::unit_x(),
    }
}
fn meridian(u: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::origin(),
        axis: radial(u).cross(Vec3::unit_z()),
        radius: 1.0,
        u_ref: radial(u),
    }
}

/// The unit ball, cut by the rim `z = h0` into a north cap and a south
/// remainder, the north cap carrying a STRUT meridian at azimuth 0 from
/// the rim up to `z = hc` (short of the pole). `(body, north, south)`.
fn strut_ball(h0: f64, hc: f64, north_sense: bool, south_sense: bool) -> (Body<f64>, FaceKey, FaceKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(at(0.0, h0), true).unwrap();
    let sphere = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Sphere {
                    center: Point3::origin(),
                    radius: 1.0,
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                },
                sense: true,
            },
        )
        .unwrap();
    let strut = arc(&mut body, sphere, meridian(0.0), (h0.asin(), hc.asin()));
    let e_s = body
        .mev(MevSite::Lone { r#loop: seed.r#loop }, at(0.0, hc), strut, tol)
        .unwrap();
    let r = arc(&mut body, sphere, rim(h0), (0.0, core::f64::consts::TAU));
    let made = body
        .mef(
            MefSite::Chords {
                he1: e_s.he_plus,
                he2: e_s.he_plus,
            },
            r,
            FaceSurface::Shared {
                key: sphere,
                sense: true,
            },
            tol,
        )
        .unwrap();
    let (north, south) = (seed.face, made.face);
    body.set_face_sense(north, north_sense).unwrap();
    body.set_face_sense(south, south_sense).unwrap();
    crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
    (body, north, south)
}

#[test]
#[ignore = "review probe"]
fn probe_strut_cap() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for (ns, ss) in [(true, true), (true, false), (false, true), (false, false)] {
        let (body, north, south) = strut_ball(-0.5, 0.3, ns, ss);
        let valid = crate::AtRestBody::validate(body.clone(), tol);
        eprintln!(
            "senses north={ns} south={ss}: validate = {}",
            match &valid {
                Ok(_) => "OK".to_string(),
                Err(e) => format!("{:?}", e.iter().take(2).collect::<Vec<_>>()),
            }
        );
        for (name, face) in [("north", north), ("south", south)] {
            let (lo, hi) = sphere_reach(&body, face, band, &BoxFrame::World);
            let b = face_box(&body, face, 0.0, band).unwrap();
            let census = crate::census::face_reach(&body, face, band);
            eprintln!(
                "  {name}: sphere_reach z [{:.6}, {:.6}] x [{:.6}, {:.6}]; face_box z [{:.6}, {:.6}]; census {:?}",
                lo.z, hi.z, lo.x, hi.x, b.min_z, b.max_z, census.map(|(l, h)| (l.z, h.z))
            );
        }
    }
}

// ---------------------------------------------------------------------
// The sweep: rectangle (and L-shaped) sphere sheets in general frames.
// ---------------------------------------------------------------------

use geom_core::UnitVec3;

/// A sphere frame: centre, radius, and the orthonormal `(ê, â×ê, â)`.
#[derive(Clone, Copy)]
struct Frame {
    c: Point3<f64>,
    r: f64,
    e: Vec3<f64>,
    f: Vec3<f64>,
    a: Vec3<f64>,
}
impl Frame {
    fn new(c: Point3<f64>, r: f64, a: Vec3<f64>, hint: Vec3<f64>) -> Self {
        let a = a * (1.0 / a.norm());
        let e = hint - a * hint.dot(a);
        let e = e * (1.0 / e.norm());
        Self { c, r, e, f: a.cross(e), a }
    }
    fn dir(&self, v: Vec3<f64>) -> Vec3<f64> {
        self.e * v.x + self.f * v.y + self.a * v.z
    }
    fn pt(&self, p: Point3<f64>) -> Point3<f64> {
        self.c + self.dir(p - Point3::origin()) * self.r
    }
    fn circle(&self, c: Curve3<f64>) -> Curve3<f64> {
        let Curve3::Circle { center, axis, radius, u_ref } = c else { unreachable!() };
        Curve3::Circle {
            center: self.pt(center),
            axis: self.dir(axis),
            radius: radius * self.r,
            u_ref: self.dir(u_ref),
        }
    }
    /// The world point at azimuth `u`, latitude `v`.
    fn at_uv(&self, u: f64, v: f64) -> Point3<f64> {
        self.c + (self.e * (v.cos() * u.cos()) + self.f * (v.cos() * u.sin()) + self.a * v.sin()) * self.r
    }
}

/// A corner in the chart: `(u, latitude)`; a pole corner has `|v| = π/2`
/// and its `u` is ignored.
type Corner = (f64, f64);

fn is_pole(v: f64) -> bool {
    (v.abs() - core::f64::consts::FRAC_PI_2).abs() < 1e-15
}
fn local(u: f64, v: f64) -> Point3<f64> {
    if is_pole(v) {
        return Point3::new(0.0, 0.0, v.signum());
    }
    Point3::new(v.cos() * u.cos(), v.cos() * u.sin(), v.sin())
}

/// The local carrier and params of the chart edge `p → q`.
fn edge_local(p: Corner, q: Corner) -> (Curve3<f64>, (f64, f64)) {
    if !is_pole(p.1) && !is_pole(q.1) && p.1 == q.1 {
        // A rim.
        let h = p.1.sin();
        let rho = p.1.cos();
        let centre = Point3::new(0.0, 0.0, h);
        if q.0 > p.0 {
            (
                Curve3::Circle { center: centre, axis: Vec3::unit_z(), radius: rho, u_ref: Vec3::unit_x() },
                (p.0, q.0),
            )
        } else {
            (
                Curve3::Circle { center: centre, axis: -Vec3::unit_z(), radius: rho, u_ref: radial(p.0) },
                (0.0, p.0 - q.0),
            )
        }
    } else {
        // A meridian at the non-pole end's azimuth.
        let u = if is_pole(p.1) { q.0 } else { p.0 };
        let up = radial(u).cross(Vec3::unit_z());
        if q.1 > p.1 {
            (
                Curve3::Circle { center: Point3::origin(), axis: up, radius: 1.0, u_ref: radial(u) },
                (p.1, q.1),
            )
        } else {
            (
                Curve3::Circle { center: Point3::origin(), axis: -up, radius: 1.0, u_ref: radial(u) },
                (-p.1, -q.1),
            )
        }
    }
}

/// A one-face sphere sheet whose outer loop runs through `corners` in
/// order, closed back to the first. `(body, face)`; `None` when an
/// Euler op refuses.
fn sheet(fr: &Frame, corners: &[Corner], tol: Tol) -> Option<(Body<f64>, FaceKey, Vec<crate::entity::VertexKey>)> {
    let mut body = Body::<f64>::new();
    let p0 = fr.pt(local(corners[0].0, corners[0].1));
    let seed = body.mvfs(p0, true).map_err(|e| eprintln!("  build err: {e:?}")).ok()?;
    let sphere = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Sphere { center: fr.c, radius: fr.r, axis: fr.a, u_ref: fr.e },
                sense: true,
            },
        )
        .map_err(|e| eprintln!("  build err: {e:?}")).ok()?;
    let spec = |body: &mut Body<f64>, p: Corner, q: Corner| {
        let (c, t) = edge_local(p, q);
        arc(body, sphere, fr.circle(c), t)
    };
    let n = corners.len();
    let s = spec(&mut body, corners[0], corners[1]);
    let first = body
        .mev(MevSite::Lone { r#loop: seed.r#loop }, fr.pt(local(corners[1].0, corners[1].1)), s, tol)
        .map_err(|e| eprintln!("  build err: {e:?}")).ok()?;
    let mut prev = first;
    let mut verts = vec![seed.vertex, first.vertex];
    for i in 1..n - 1 {
        let s = spec(&mut body, corners[i], corners[i + 1]);
        let e = body
            .mev(
                MevSite::Fan { he1: prev.he_minus, he2: prev.he_minus },
                fr.pt(local(corners[i + 1].0, corners[i + 1].1)),
                s,
                tol,
            )
            .map_err(|e| eprintln!("  build err: {e:?}")).ok()?;
        verts.push(e.vertex);
        prev = e;
    }
    let he = body.find_half_edge(seed.face, verts[n - 1], verts[n - 2])?;
    let s = spec(&mut body, corners[n - 1], corners[0]);
    let face = body
        .mef(
            MefSite::Chords { he1: he, he2: first.he_plus },
            s,
            FaceSurface::Shared { key: sphere, sense: true },
            tol,
        )
        .map_err(|e| eprintln!("  build err: {e:?}")).ok()?
        .face;
    crate::pcurves::mint_pcurves(&mut body, tol).map_err(|e| eprintln!("  build err: {e:?}")).ok()?;
    Some((body, face, verts))
}

/// Does `face`'s outer loop visit `verts` in construction order (the
/// corner lists are all counter-clockwise in the chart)?
fn loop_follows(body: &Body<f64>, face: FaceKey, verts: &[crate::entity::VertexKey]) -> bool {
    let f = body.get_face(face).unwrap();
    let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!()
    };
    let seq: Vec<_> = body.loop_cycle(first).unwrap().into_iter().map(|he| body.get_half_edge(he).unwrap().start).collect();
    let n = seq.len();
    assert_eq!(n, verts.len());
    let i = seq.iter().position(|v| *v == verts[0]).unwrap();
    if seq[(i + 1) % n] == verts[1] {
        true
    } else {
        assert_eq!(seq[(i + 1) % n], verts[n - 1]);
        false
    }
}

/// The signed chart area of `face`'s outer loop, read from its vertex
/// order alone (positive: counter-clockwise in `(u, v)`, i.e. about the
/// outward normal). Pole vertices take the azimuth of the next vertex.
fn loop_chart_orientation(body: &Body<f64>, face: FaceKey, fr: &Frame) -> f64 {
    let f = body.get_face(face).unwrap();
    let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!()
    };
    let mut pts = Vec::new();
    for he in body.loop_cycle(first).unwrap() {
        let v = body.get_half_edge(he).unwrap().start;
        let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let w = (p - fr.c) * (1.0 / fr.r);
        let (x, y, z) = (w.dot(fr.e), w.dot(fr.f), w.dot(fr.a));
        pts.push((y.atan2(x), z.clamp(-1.0, 1.0).asin(), x.hypot(y) < 1e-12));
    }
    // Unwrap azimuth continuously; poles copy the neighbour's.
    let n = pts.len();
    for i in 0..n {
        if pts[i].2 {
            pts[i].0 = pts[(i + 1) % n].0;
        }
    }
    let mut u: Vec<f64> = vec![pts[0].0];
    for i in 1..n {
        let mut d = pts[i].0 - pts[i - 1].0;
        while d > core::f64::consts::PI {
            d -= core::f64::consts::TAU;
        }
        while d < -core::f64::consts::PI {
            d += core::f64::consts::TAU;
        }
        u.push(u[i - 1] + d);
    }
    let mut area = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        area += u[i] * pts[j].1 - u[j] * pts[i].1;
    }
    area
}

/// Is chart point `(u, v)` inside the polygon `corners` (axis-parallel
/// edges in the chart; the azimuth taken modulo a turn into the
/// polygon's own range)?
fn in_poly(corners: &[Corner], u: f64, v: f64) -> bool {
    // Replace pole corners by their meridian's azimuth for the polygon.
    let n = corners.len();
    let mut poly: Vec<(f64, f64)> = Vec::new();
    for i in 0..n {
        let (cu, cv) = corners[i];
        if is_pole(cv) {
            let prev = corners[(i + n - 1) % n].0;
            let next = corners[(i + 1) % n].0;
            poly.push((prev, cv));
            poly.push((next, cv));
        } else {
            poly.push((cu, cv));
        }
    }
    let umin = poly.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let mut uu = u;
    while uu < umin {
        uu += core::f64::consts::TAU;
    }
    while uu >= umin + core::f64::consts::TAU {
        uu -= core::f64::consts::TAU;
    }
    let mut inside = false;
    let m = poly.len();
    for i in 0..m {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % m];
        if (y1 > v) != (y2 > v) {
            let x = x1 + (v - y1) * (x2 - x1) / (y2 - y1);
            if uu < x {
                inside = !inside;
            }
        }
    }
    inside
}

#[test]
#[ignore = "review probe"]
fn probe_sphere_rect_sweep() {
    use core::f64::consts::{FRAC_PI_2, PI};
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let rects: Vec<(&str, Vec<Corner>)> = {
        let rect = |(u0, u1): (f64, f64), (v0, v1): (f64, f64)| -> Vec<Corner> {
            if is_pole(v1) {
                vec![(u0, v0), (u1, v0), (0.0, v1)]
            } else if is_pole(v0) {
                vec![(u1, v1), (u0, v1), (0.0, v0)]
            } else {
                vec![(u0, v0), (u1, v0), (u1, v1), (u0, v1)]
            }
        };
        let mut out = Vec::new();
        for (un, u) in [
            ("u[0.2,1.4]", (0.2, 1.4)),
            ("u[-0.5,0.5]", (-0.5, 0.5)),
            ("u[5.9,6.6]", (5.9, 6.6)),
            ("u[0.1,6.2]", (0.1, 6.2)),
            ("u[-3,3.1]", (-3.0, 3.1)),
            ("u[3,3.3]", (3.0, 3.3)),
            ("u[1,1.001]", (1.0, 1.001)),
        ] {
            for (vn, v) in [
                ("v[-0.3,0.4]", (-0.3f64, 0.4f64)),
                ("v[1.2,pole]", (1.2, FRAC_PI_2)),
                ("v[pole,-0.2]", (-FRAC_PI_2, -0.2)),
                ("v[-1.5,1.5]", (-1.5, 1.5)),
                ("v[0.3,pole-1e-6]", (0.3, FRAC_PI_2 - 1e-6)),
                ("v[-pole+1e-9,0.1]", (-FRAC_PI_2 + 1e-9, 0.1)),
                ("v[0.5,0.50001]", (0.5, 0.50001)),
                ("v[0.3,pole-1e-3]", (0.3, FRAC_PI_2 - 1e-3)),
                ("v[-pole+1e-4,0.1]", (-FRAC_PI_2 + 1e-4, 0.1)),
                ("v[pole-1e-3 .. pole]", (FRAC_PI_2 - 1e-3, FRAC_PI_2)),
            ] {
                out.push((format!("{un}x{vn}"), rect(u, v)));
            }
        }
        // L-shapes (reflex corner), both orientations of the notch.
        out.push(("L1".into(), vec![(0.2, -0.3), (2.5, -0.3), (2.5, 0.1), (1.0, 0.1), (1.0, 0.6), (0.2, 0.6)]));
        out.push(("L2".into(), vec![(-1.0, -0.6), (4.0, -0.6), (4.0, 0.7), (3.0, 0.7), (3.0, 0.0), (-1.0, 0.0)]));
        out.push(("L3-pole".into(), vec![(0.2, -0.3), (2.5, -0.3), (2.5, 0.1), (1.0, 0.1), (0.0, FRAC_PI_2)]));
        out.push(("U-wide".into(), vec![(0.0, -0.5), (6.0, -0.5), (6.0, 0.5), (5.0, 0.5), (5.0, 0.0), (1.0, 0.0), (1.0, 0.5), (0.0, 0.5)]));
        out.into_iter().map(|(n, c)| (Box::leak(n.into_boxed_str()) as &str, c)).collect()
    };
    let frames = |s: f64| {
        vec![
            ("id", Frame::new(Point3::origin(), s, Vec3::unit_z(), Vec3::unit_x())),
            (
                "tilt",
                Frame::new(Point3::new(2.0 * s, -1.0 * s, 3.0 * s), 1.25 * s, Vec3::new(0.3, 0.4, 0.866), Vec3::new(1.0, -0.2, 0.1)),
            ),
            (
                "flip",
                Frame::new(Point3::new(-0.5 * s, 0.7 * s, 0.0), 0.8 * s, Vec3::new(-0.2, -0.9, 0.1), Vec3::new(0.0, 0.3, 1.0)),
            ),
        ]
    };
    let aims = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.3, 0.9, -0.2),
        Vec3::new(-0.8, 0.1, 0.5),
        Vec3::new(0.2, -0.7, 0.6),
        Vec3::new(-0.5, -0.5, -0.7),
    ];
    let (mut built, mut skipped, mut bad, mut rect_read, mut ball_read) = (0, 0, 0, 0, 0);
    for scale in [1e-3, 1.0, 1e3] {
        for (fname, fr) in frames(scale) {
            for (rname, corners) in &rects {
                let Some((body0, face, verts)) = sheet(&fr, corners, tol) else {
                    skipped += 1;
                    eprintln!("SKIP build {rname} {fname} s={scale}");
                    continue;
                };
                for sense in [true, false] {
                    let mut body = body0.clone();
                    body.set_face_sense(face, sense).unwrap();
                    built += 1;
                    let ccw = loop_follows(&body, face, &verts);
                    // The face is the polygon iff the loop is CCW about
                    // the face's own normal.
                    let is_rect = ccw == sense;
                    // Dense sample of the true region.
                    let n = 400;
                    let mut samples: Vec<Point3<f64>> = Vec::new();
                    for i in 0..=n {
                        let v = -FRAC_PI_2 + PI * f64::from(i) / f64::from(n);
                        for j in 0..=n {
                            let u = 2.0 * PI * f64::from(j) / f64::from(n);
                            if in_poly(corners, u, v) == is_rect {
                                samples.push(fr.at_uv(u, v));
                            }
                        }
                    }
                    // Plus a fine grid over the polygon's own bounding
                    // window (or its corners for the complement).
                    let umin = corners.iter().map(|c| c.0).fold(f64::INFINITY, f64::min);
                    let umax = corners.iter().map(|c| c.0).fold(f64::NEG_INFINITY, f64::max);
                    let vmin = corners.iter().map(|c| c.1).fold(f64::INFINITY, f64::min);
                    let vmax = corners.iter().map(|c| c.1).fold(f64::NEG_INFINITY, f64::max);
                    if is_rect {
                        let m = 400;
                        for i in 0..=m {
                            let v = vmin + (vmax - vmin) * f64::from(i) / f64::from(m);
                            for j in 0..=m {
                                let u = umin + (umax - umin) * f64::from(j) / f64::from(m);
                                if in_poly(corners, u, v) || i == 0 || i == m || j == 0 || j == m {
                                    if in_poly(corners, u, v) {
                                        samples.push(fr.at_uv(u, v));
                                    }
                                }
                            }
                        }
                        for c in corners {
                            samples.push(fr.at_uv(c.0, c.1));
                        }
                    }
                    let b = face_box(&body, face, 0.0, band).unwrap();
                    let lanes: Vec<(String, Vec3<f64>, (f64, f64))> = {
                        let mut l = vec![
                            ("face_box x".to_string(), Vec3::unit_x(), (b.min_x, b.max_x)),
                            ("face_box y".to_string(), Vec3::unit_y(), (b.min_y, b.max_y)),
                            ("face_box z".to_string(), Vec3::unit_z(), (b.min_z, b.max_z)),
                        ];
                        for aim in aims {
                            let aim = aim * (1.0 / aim.norm());
                            let frame = BoxFrame::aimed(UnitVec3::new(aim, "probe", band).unwrap());
                            if let Some((lo, hi)) = crate::census::face_reach_in(&body, face, band, &frame) {
                                l.push((format!("census aim {aim:?}"), aim, (lo.x, hi.x)));
                            }
                            let (lo, hi) = sphere_reach(&body, face, band, &frame);
                            l.push((format!("sphere_reach aim {aim:?}"), aim, (lo.x, hi.x)));
                        }
                        l
                    };
                    // Did the rule read a rectangle (narrower than the ball
                    // along z of the frame's axis)?
                    let (lo, hi) = sphere_reach(
                        &body,
                        face,
                        band,
                        &BoxFrame::aimed(UnitVec3::new(fr.a, "probe", band).unwrap()),
                    );
                    if hi.x - lo.x < 2.0 * fr.r * (1.0 - 1e-9) || true {
                        let ball = (fr.c - Point3::origin()).dot(fr.a);
                        let whole = (lo.x - (ball - fr.r)).abs() < 1e-12 * scale && (hi.x - (ball + fr.r)).abs() < 1e-12 * scale;
                        if whole {
                            ball_read += 1;
                        } else {
                            rect_read += 1;
                        }
                        if fname == "tilt" && scale == 1.0 {
                            eprintln!("READ {rname} sense={sense} rect={is_rect} axial-whole={whole} axial=[{:.4}, {:.4}]", lo.x - ball, hi.x - ball);
                        }
                    }
                    for (lane, e, (lo, hi)) in lanes {
                        let (mut slo, mut shi) = (f64::INFINITY, f64::NEG_INFINITY);
                        for p in &samples {
                            let x = e.dot(*p - Point3::origin());
                            slo = slo.min(x);
                            shi = shi.max(x);
                        }
                        let slack = 1e-12 * scale.max(1.0) * 10.0;
                        if lo > slo + slack || hi < shi - slack {
                            bad += 1;
                            eprintln!(
                                "UNDER {rname} {fname} s={scale} sense={sense} rect={is_rect} {lane}: box [{lo:.9e}, {hi:.9e}] samples [{slo:.9e}, {shi:.9e}] (lo over by {:.3e}, hi under by {:.3e})",
                                lo - slo,
                                shi - hi
                            );
                        }
                    }
                }
            }
        }
    }
    eprintln!("built {built} (skipped builds {skipped}); rectangle read {rect_read}, ball/zone read {ball_read}; under-coverage {bad}");
    assert_eq!(bad, 0);
}
