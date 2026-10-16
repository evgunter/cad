//! Islands on a torus, each a rectangle of its chart bounded by two
//! meridian arcs and two parallel arcs, with its membership oracle: the
//! fixtures the parity rows in `ring_path::tests` and the ring-lane rows
//! in `chord_join::torus_ring_rows` share. The torus is `R = 2`,
//! `r = 0.8` in the frame's units, about the frame's `z`.

use super::cone_islands::Frame;
use core::f64::consts::PI;
use geom_core::{Point3, Vec3};

/// The torus's radii, in frame units.
pub(crate) const RADII: (f64, f64) = (2.0, 0.8);

/// The torus about the frame's `z` (or `−z`, `mirror`: the same point
/// set with its chart turned the other way), its seam at the frame's `x`.
pub(crate) fn torus(f: Frame, mirror: bool) -> geom::Surface<f64> {
    geom::Surface::Torus {
        center: f.o,
        axis: if mirror { -f.z } else { f.z },
        major_radius: RADII.0 * f.s,
        minor_radius: RADII.1 * f.s,
        u_ref: f.x,
    }
}

/// The point of the torus at azimuth `u` about the frame's `z` and tube
/// angle `v` (0 on the outer equator, π/2 on top).
pub(crate) fn on_torus(f: Frame, u: f64, v: f64) -> Point3<f64> {
    let (big, small) = RADII;
    let rho = big + small * v.cos();
    f.at(rho * u.cos(), rho * u.sin(), small * v.sin())
}

/// `p`'s azimuth and tube angle, each in `(−π, π]`.
pub(crate) fn chart(f: Frame, p: Point3<f64>) -> (f64, f64) {
    let w = (p - f.o) / f.s;
    let (x, y, z) = (w.dot(f.x), w.dot(f.y), w.dot(f.z));
    (y.atan2(x), z.atan2(x.hypot(y) - RADII.0))
}

/// One edge of an [`Island`], from its corner to the next: `carrier`
/// from `params.0` to `params.1`, and the plane it is the section of.
#[derive(Clone)]
pub(crate) struct IslandEdge {
    pub(crate) carrier: geom::Curve3<f64>,
    pub(crate) params: (f64, f64),
    pub(crate) plane: (Point3<f64>, Vec3<f64>),
}

/// The meridian at azimuth `u` from tube angle `v0` to `v1`: its
/// parameter is the tube angle.
pub(crate) fn meridian(f: Frame, u: f64, (v0, v1): (f64, f64)) -> IslandEdge {
    let out = f.dir(u.cos(), u.sin(), 0.0);
    let centre = f.o + out * (RADII.0 * f.s);
    let axis = out.cross(f.z);
    IslandEdge {
        carrier: geom::Curve3::Circle {
            center: centre,
            axis,
            radius: RADII.1 * f.s,
            u_ref: out,
        },
        params: (v0, v1),
        plane: (centre, axis),
    }
}

/// The parallel at tube angle `v` from azimuth `u0` to `u1`: its
/// parameter is the azimuth.
pub(crate) fn parallel(f: Frame, v: f64, (u0, u1): (f64, f64)) -> IslandEdge {
    let (big, small) = RADII;
    let centre = f.at(0.0, 0.0, small * v.sin());
    IslandEdge {
        carrier: geom::Curve3::Circle {
            center: centre,
            axis: f.z,
            radius: (big + small * v.cos()) * f.s,
            u_ref: f.x,
        },
        params: (u0, u1),
        plane: (centre, f.z),
    }
}

/// A closed loop on the torus: its corners, the edge leaving each, and
/// its membership oracle.
pub(crate) struct Island {
    pub(crate) corners: Vec<Point3<f64>>,
    pub(crate) edges: Vec<IslandEdge>,
    pub(crate) inside: Box<dyn Fn(Point3<f64>) -> bool>,
}

/// **The chart rectangle `(u0, u1) × (v0, v1)`**, both within
/// `(−π, π)`: from its corner `(u0, v0)` along the parallel `v0`, up
/// the meridian `u1`, back along the parallel `v1` and down the meridian
/// `u0`.
pub(crate) fn rectangle(f: Frame, (u0, u1): (f64, f64), (v0, v1): (f64, f64)) -> Island {
    Island {
        corners: vec![
            on_torus(f, u0, v0),
            on_torus(f, u1, v0),
            on_torus(f, u1, v1),
            on_torus(f, u0, v1),
        ],
        edges: vec![
            parallel(f, v0, (u0, u1)),
            meridian(f, u1, (v0, v1)),
            parallel(f, v1, (u1, u0)),
            meridian(f, u0, (v1, v0)),
        ],
        inside: Box::new(move |p| {
            let (u, v) = chart(f, p);
            u0 < u && u < u1 && v0 < v && v < v1
        }),
    }
}

/// The islands every row reads, each named: on the outer side, on the
/// inner side, over the tube's top (an outer parallel and an inner
/// one), between the two parallels of one plane (`v` and `π − v`), and
/// one wider than half a turn round the axis on the inner side, whose
/// meridians' planes hold their partners across the axis inside the
/// azimuths the island spans.
pub(crate) fn islands(f: Frame) -> Vec<(&'static str, Island)> {
    island_spans()
        .into_iter()
        .map(|(name, us, vs)| (name, rectangle(f, us, vs)))
        .collect()
}

/// A named chart rectangle: its azimuths and its tube angles.
pub(crate) type Span = (&'static str, (f64, f64), (f64, f64));

/// [`islands`]' chart rectangles.
pub(crate) fn island_spans() -> [Span; 5] {
    [
        ("outer", (-0.5, 0.6), (-0.4, 0.7)),
        ("inner", (-0.3, 0.8), (1.9, 2.5)),
        ("over the top", (-1.0, 0.2), (0.9, 2.2)),
        ("one plane's parallels", (0.3, 1.4), (0.6, PI - 0.6)),
        ("wide, inside", (-1.6, 1.7), (2.3, 2.9)),
    ]
}

/// The islands bounded by the tube's top or bottom circle (the parallels
/// `v = ±π/2`, of radius `R`, whose planes touch the torus along them),
/// each named: up to the top from the outer side, down to the bottom,
/// from the top over to the inner side, and the whole outer side between
/// the two, as a donut's outer face is.
pub(crate) fn crest_islands(f: Frame) -> Vec<(&'static str, Island)> {
    use core::f64::consts::FRAC_PI_2;
    vec![
        ("up to the top", rectangle(f, (-0.5, 0.6), (0.2, FRAC_PI_2))),
        (
            "down to the bottom",
            rectangle(f, (-0.5, 0.6), (-FRAC_PI_2, -0.2)),
        ),
        (
            "from the top inward",
            rectangle(f, (-0.3, 0.8), (FRAC_PI_2, 2.5)),
        ),
        (
            "the outer side",
            rectangle(f, (-1.0, 0.7), (-FRAC_PI_2, FRAC_PI_2)),
        ),
    ]
}

/// The arc of the spiric oval the plane `x = d` (frame units, normal the
/// frame's `x`) cuts on the `+y` side of the axis (`side` true) or the
/// `−y` side, from tube angle `v0` to `v1`: its parameter is the tube
/// angle, its plane parallel to the axis. `|d| < R − r`.
pub(crate) fn oval(f: Frame, d: f64, side: bool, (v0, v1): (f64, f64)) -> IslandEdge {
    let (big, small) = RADII;
    let (u_ref, offset) = if side { (f.x, d) } else { (-f.x, -d) };
    IslandEdge {
        carrier: geom::Curve3::Spiric {
            center: f.o,
            axis: f.z,
            u_ref,
            major_radius: big * f.s,
            minor_radius: small * f.s,
            offset: offset * f.s,
        },
        params: (v0, v1),
        plane: (f.at(d, 0.0, 0.0), f.x),
    }
}

/// The azimuth at tube angle `v` where the plane `x = d` meets the
/// torus, on the `+y` side (`side`) or the `−y` side.
fn azimuth_at(d: f64, side: bool, v: f64) -> f64 {
    let (big, small) = RADII;
    let u = (d / (big + small * v.cos())).acos();
    if side { u } else { -u }
}

/// **The strip `d0 < x < d1` of the torus between the tube angles `v0`
/// and `v1`, on one side of the axis's trace**: bounded by the parallel
/// `v0`, the oval of `x = d0` (on the `+y` side, where `x` falls as the
/// azimuth grows), the parallel `v1` and the oval of `x = d1`. On the
/// `−y` side the same chart region is mirrored, and traversed the same
/// way round in the chart.
pub(crate) fn oval_strip(
    f: Frame,
    (d0, d1): (f64, f64),
    side: bool,
    (v0, v1): (f64, f64),
) -> Island {
    let at = |d: f64, v: f64| on_torus(f, azimuth_at(d, side, v), v);
    let (near, far) = if side { (d1, d0) } else { (d0, d1) };
    Island {
        corners: vec![at(near, v0), at(far, v0), at(far, v1), at(near, v1)],
        edges: vec![
            parallel(
                f,
                v0,
                (azimuth_at(near, side, v0), azimuth_at(far, side, v0)),
            ),
            oval(f, far, side, (v0, v1)),
            parallel(
                f,
                v1,
                (azimuth_at(far, side, v1), azimuth_at(near, side, v1)),
            ),
            oval(f, near, side, (v1, v0)),
        ],
        inside: Box::new(move |p| {
            let (u, v) = chart(f, p);
            let x = (p - f.o).dot(f.x) / f.s;
            v0 < v && v < v1 && d0 < x && x < d1 && (u > 0.0) == side
        }),
    }
}

/// The islands bounded by spiric ovals, each named: a strip on the outer
/// side of the tube on each side of the axis's trace, one on the inner
/// side, one whose planes stand either side of the axis, and one bounded
/// by an oval and a meridian.
pub(crate) fn spiric_islands(f: Frame) -> Vec<(&'static str, Island)> {
    let (v0, v1, d) = (-0.6, 0.9, 0.3);
    let u_at = |v: f64| azimuth_at(d, true, v);
    vec![
        (
            "ovals, outer, +y",
            oval_strip(f, (0.2, 1.0), true, (-0.5, 0.8)),
        ),
        (
            "ovals, outer, −y",
            oval_strip(f, (0.2, 1.0), false, (-0.5, 0.8)),
        ),
        ("ovals, inner", oval_strip(f, (0.1, 0.9), true, (1.8, 2.75))),
        (
            "ovals across the axis",
            oval_strip(f, (-0.7, 0.4), true, (-0.7, 0.4)),
        ),
        (
            "an oval and a meridian",
            Island {
                corners: vec![
                    on_torus(f, u_at(v0), v0),
                    on_torus(f, 1.95, v0),
                    on_torus(f, 1.95, v1),
                    on_torus(f, u_at(v1), v1),
                ],
                edges: vec![
                    parallel(f, v0, (u_at(v0), 1.95)),
                    meridian(f, 1.95, (v0, v1)),
                    parallel(f, v1, (1.95, u_at(v1))),
                    oval(f, d, true, (v1, v0)),
                ],
                inside: Box::new(move |p| {
                    let (u, v) = chart(f, p);
                    let x = (p - f.o).dot(f.x) / f.s;
                    v0 < v && v < v1 && x < d && u > 0.0 && u < 1.95
                }),
            },
        ),
    ]
}
