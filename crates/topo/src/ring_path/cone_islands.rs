//! Islands on a cone, each with its membership oracle: the fixtures
//! parity rows in `ring_path::tests` and the ring-lane rows in
//! `chord_join::cone_ring_rows` share.

use core::f64::consts::{FRAC_PI_6, PI};
use geom_core::{Band, Point3, Tol, Vec3};

pub(crate) fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Whether a fixture whose smallest feature is `feature` metres is read
/// at this run's band: the feature is ten thousand coincidence widths or
/// more. A smaller one is honestly in the band's reach, where a reading
/// escalates rather than answers.
pub(crate) fn read_at(feature: f64) -> bool {
    feature >= 1e4 * band().zero()
}

/// The scales, of 1 and `small`, whose fixtures of smallest feature
/// `smallest` (at scale 1) are read at this run's band ([`read_at`]); a
/// scale stood down is said so.
pub(crate) fn scales(small: f64, smallest: f64) -> Vec<f64> {
    let mut out = vec![1.0];
    if read_at(small * smallest) {
        out.push(small);
    } else {
        test_utils::vacuity::stood_down(
            "a cone island at scale 1e-3",
            "its smallest feature is within ten thousand coincidence widths at this ε",
        );
    }
    out
}

/// A rigid frame and a scale: the cone's local `x`, `y`, `z`, its apex,
/// and the length one local unit is.
#[derive(Clone, Copy)]
pub(crate) struct Frame {
    pub(crate) x: Vec3<f64>,
    pub(crate) y: Vec3<f64>,
    pub(crate) z: Vec3<f64>,
    pub(crate) o: Point3<f64>,
    pub(crate) s: f64,
}

impl Frame {
    /// The point at local coordinates `(x, y, z)`.
    pub(crate) fn at(&self, x: f64, y: f64, z: f64) -> Point3<f64> {
        self.o + self.dir(x, y, z) * self.s
    }

    /// This frame with one local unit `s` long.
    pub(crate) fn scaled(self, s: f64) -> Self {
        Self { s, ..self }
    }

    /// The direction of local `(x, y, z)`, unscaled.
    pub(crate) fn dir(&self, x: f64, y: f64, z: f64) -> Vec3<f64> {
        self.x * x + self.y * y + self.z * z
    }
}

/// The identity, a frame turned off every axis, and one with its
/// handedness kept but its axis reversed (`z ↦ −z`, `y ↦ −y`).
pub(crate) fn frames() -> [Frame; 3] {
    let id = Frame {
        x: Vec3::unit_x(),
        y: Vec3::unit_y(),
        z: Vec3::unit_z(),
        o: Point3::origin(),
        s: 1.0,
    };
    let z = Vec3::new(0.3, -0.5, 0.8).normalize();
    let x = Vec3::new(1.0, 0.2, 0.0).cross(z).normalize();
    let turned = Frame {
        x,
        y: z.cross(x),
        z,
        o: Point3::new(0.7, -1.1, 2.3),
        s: 1.0,
    };
    let flipped = Frame {
        x: Vec3::unit_x(),
        y: -Vec3::unit_y(),
        z: -Vec3::unit_z(),
        o: Point3::new(-0.4, 0.2, 0.1),
        s: 1.0,
    };
    [id, turned, flipped]
}

/// The cone of half-angle π/6 at the frame's apex, opening along its
/// `z` — or along `−z` (`mirror`), so that the points of
/// [`on_nappe`] lie on the carrier's negative nappe.
pub(crate) fn cone(f: Frame, mirror: bool) -> geom::Surface<f64> {
    geom::Surface::Cone {
        apex: f.o,
        axis: if mirror { -f.z } else { f.z },
        half_angle: FRAC_PI_6,
        u_ref: f.x,
    }
}

/// The point of the nappe along local `+z` at height `h`, azimuth `t`.
pub(crate) fn on_nappe(f: Frame, h: f64, t: f64) -> Point3<f64> {
    let r = h * FRAC_PI_6.tan();
    f.at(r * t.cos(), r * t.sin(), h)
}

/// The plane through `o` with normal `n`.
pub(crate) fn plane(o: Point3<f64>, n: Vec3<f64>) -> geom::Surface<f64> {
    geom::Surface::Plane {
        origin: o,
        normal: n,
        u_ref: n.orthonormal_basis().0,
    }
}

/// The section of `cone` by the plane through `o` with normal `n`, on a
/// cone a few `s` across.
fn section(
    cone: &geom::Surface<f64>,
    (o, n): (Point3<f64>, Vec3<f64>),
    s: f64,
) -> geom::Curve3<f64> {
    match geom_brep::plane_cone_section(&plane(o, n), cone, 4.0 * s, band()).unwrap() {
        geom_brep::PlaneConeSection::TiltedEllipse(c)
        | geom_brep::PlaneConeSection::AxisNormalCircle(c) => c,
        other => panic!("the fixture's planes cut ellipses: {other:?}"),
    }
}

/// The conic parameter of `p`, a point of `c`.
fn conic_param(c: &geom::Curve3<f64>, p: Point3<f64>) -> f64 {
    let (center, axis, u, a, b) = match *c {
        geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => (center, axis, u_ref, major, minor),
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => (center, axis, u_ref, radius, radius),
        _ => unreachable!(),
    };
    let w = p - center;
    (w.dot(axis.cross(u)) / b).atan2(w.dot(u) / a)
}

/// One edge of an [`Island`], travelled from its corner to the next:
/// `carrier` from `params.0` to `params.1` (either way), and the
/// plane it is the section of, `None` for a ruling.
#[derive(Clone)]
pub(crate) struct IslandEdge {
    pub(crate) carrier: geom::Curve3<f64>,
    pub(crate) params: (f64, f64),
    pub(crate) plane: Option<(Point3<f64>, Vec3<f64>)>,
}

/// The arc of the section by `(o, n)` from `x0` to `x1` whose
/// midpoint `keep` holds.
fn arc_between(
    (cone, s): (&geom::Surface<f64>, f64),
    (o, n): (Point3<f64>, Vec3<f64>),
    (x0, x1): (Point3<f64>, Point3<f64>),
    keep: &dyn Fn(Point3<f64>) -> bool,
) -> IslandEdge {
    let c = section(cone, (o, n), s);
    let (t0, mut t1) = (conic_param(&c, x0), conic_param(&c, x1));
    if t1 < t0 {
        t1 += 2.0 * PI;
    }
    if !keep(c.mid_point(t0, t1)) {
        t1 -= 2.0 * PI;
    }
    assert!(keep(c.mid_point(t0, t1)), "one of the two arcs is kept");
    IslandEdge {
        carrier: c,
        params: (t0, t1),
        plane: Some((o, n)),
    }
}

/// The ruling segment from `x0` to `x1`, both on one ruling of the
/// cone at `apex`.
fn ruling(apex: Point3<f64>, (x0, x1): (Point3<f64>, Point3<f64>)) -> IslandEdge {
    let dir = (x1 - x0).normalize();
    IslandEdge {
        carrier: geom::Curve3::Line { origin: apex, dir },
        params: ((x0 - apex).dot(dir), (x1 - apex).dot(dir)),
        plane: None,
    }
}

/// A closed loop on a cone: its corners, the edge leaving each, and
/// its membership oracle.
pub(crate) struct Island {
    pub(crate) corners: Vec<Point3<f64>>,
    pub(crate) edges: Vec<IslandEdge>,
    pub(crate) inside: Box<dyn Fn(Point3<f64>) -> bool>,
}

/// **A lune between two ellipses**, scaled by `s` about the apex:
/// the box quadrant `n̂₁·x > c₁, n̂₂·x > c₂` (perpendicular normals
/// 40° and 50° off the axis), whose edge pierces the nappe at
/// `s·(0.5, ±0.707, 1.5)` — the far side of one section and the
/// apex side of the other. At `s = 1` it spans heights 1.3 to 2.9.
pub(crate) fn lune(f: Frame, cone: &geom::Surface<f64>, s: f64) -> Island {
    let b = 40f64.to_radians();
    let n1 = f.dir(b.sin(), 0.0, b.cos());
    let n2 = f.dir(b.cos(), 0.0, -b.sin());
    let e = f.at(0.5 * s, 0.0, 1.5 * s);
    let (c1, c2) = (n1.dot(e - f.o), n2.dot(e - f.o));
    let y = (0.75 - 0.25f64).sqrt() * s;
    let (xa, xb) = (f.at(0.5 * s, y, 1.5 * s), f.at(0.5 * s, -y, 1.5 * s));
    let side = move |n: Vec3<f64>, c: f64| move |p: Point3<f64>| n.dot(p - f.o) > c;
    Island {
        corners: vec![xa, xb],
        edges: vec![
            arc_between((cone, f.s), (e, n1), (xa, xb), &side(n2, c2)),
            arc_between((cone, f.s), (e, n2), (xb, xa), &side(n1, c1)),
        ],
        inside: Box::new(move |p| side(n1, c1)(p) && side(n2, c2)(p)),
    }
}

/// **A sector of the band between a parallel and an ellipse**: below
/// the circle at height 2.2, above the section by the plane 30° off
/// the axis at offset 0.6, and between the rulings at azimuths −0.6
/// and 0.9 — two arcs and two rulings, from the corner on the circle
/// at −0.6.
pub(crate) fn sector(f: Frame, cone: &geom::Surface<f64>) -> Island {
    let (ta, tb, top) = (-0.6f64, 0.9f64, 2.2);
    let n = f.dir(0.5, 0.0, 0.75f64.sqrt());
    let lowest = |t: f64| 0.6 * f.s / n.dot(on_nappe(f, 1.0, t) - f.o);
    let azimuth = move |p: Point3<f64>| {
        let w = p - f.o;
        w.dot(f.y).atan2(w.dot(f.x))
    };
    let between = move |p: Point3<f64>| (ta..tb).contains(&azimuth(p));
    let corners = vec![
        on_nappe(f, top, ta),
        on_nappe(f, top, tb),
        on_nappe(f, lowest(tb), tb),
        on_nappe(f, lowest(ta), ta),
    ];
    let edges = vec![
        arc_between(
            (cone, f.s),
            (f.at(0.0, 0.0, top), f.z),
            (corners[0], corners[1]),
            &between,
        ),
        ruling(f.o, (corners[1], corners[2])),
        arc_between(
            (cone, f.s),
            (f.o + n * (0.6 * f.s), n),
            (corners[2], corners[3]),
            &between,
        ),
        ruling(f.o, (corners[3], corners[0])),
    ];
    Island {
        corners,
        edges,
        inside: Box::new(move |p| {
            between(p) && n.dot(p - f.o) > 0.6 * f.s && (p - f.o).dot(f.z) < top * f.s
        }),
    }
}
