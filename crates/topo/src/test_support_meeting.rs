//! **Holes meeting at one vertex of a plate's top**, and the check the
//! rows over them read: the fixture geometry the topo and editor-core
//! suites build the same bodies from, each through its own door (an
//! Euler-op prism here, a sketch and an extrude there).
//!
//! The plate is `[0, 3] × [0, 2] × [0, 1]`; each hole is a right prism
//! along an axis tilted out of its footprint, so above the top the
//! prisms lean apart and meet only at [`MEET`].

use std::collections::BTreeMap;

use crate::body::Body;
use crate::entity::{Face, HalfEdgeKey, LoopBoundary, VertexKey};

/// The plate, `[x, y, z]` bounds.
pub const PLATE: [(f64, f64); 3] = [(0.0, 3.0), (0.0, 2.0), (0.0, 1.0)];

/// Where the holes meet: a vertex of the plate's top.
pub const MEET: [f64; 3] = [1.5, 1.0, 1.0];

/// A hole in the plate's top: its footprint (counterclockwise from +z,
/// one corner at [`MEET`]), the bearing its axis leans towards, and the
/// prism's reach below [`MEET`] and its length, along the axis.
pub struct Hole {
    pub footprint: Vec<(f64, f64)>,
    pub lean: f64,
    pub below: f64,
    pub length: f64,
}

/// The axis's tilt: horizontal travel per unit rise.
const TILT: f64 = 0.3;

impl Hole {
    /// The tilted frame the prism stands on, `[origin, u, v, n]`: origin
    /// `below` under [`MEET`] along the axis `n`, `u` horizontal,
    /// `u × v = n`.
    pub fn frame(&self) -> [[f64; 3]; 4] {
        let (s, c) = self.lean.to_radians().sin_cos();
        let k = TILT.mul_add(TILT, 1.0).sqrt();
        let n = [TILT * c / k, TILT * s / k, 1.0 / k];
        let u = [-s, c, 0.0];
        let v = [
            n[1] * u[2] - n[2] * u[1],
            n[2] * u[0] - n[0] * u[2],
            n[0] * u[1] - n[1] * u[0],
        ];
        let o = [0, 1, 2].map(|i| MEET[i] - self.below * n[i]);
        [o, u, v, n]
    }

    /// The footprint projected along the axis into the frame.
    pub fn profile(&self) -> Vec<(f64, f64)> {
        let [o, u, v, _] = self.frame();
        let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        self.footprint
            .iter()
            .map(|&(x, y)| {
                let d = [x - o[0], y - o[1], 1.0 - o[2]];
                (dot(d, u), dot(d, v))
            })
            .collect()
    }

    /// The prism's volume above the plate's top: the profile's area
    /// times the axial length above the top at its centroid, the height
    /// above the top being affine over the profile.
    pub fn above(&self) -> f64 {
        let [o, u, v, n] = self.frame();
        let p = self.profile();
        let (mut a, mut cx, mut cy) = (0.0, 0.0, 0.0);
        for i in 0..p.len() {
            let ((x0, y0), (x1, y1)) = (p[i], p[(i + 1) % p.len()]);
            let w = x0 * y1 - x1 * y0;
            a += w / 2.0;
            cx += (x0 + x1) * w / 6.0;
            cy += (y0 + y1) * w / 6.0;
        }
        let z = o[2] + (cx / a) * u[2] + (cy / a) * v[2];
        a * (self.length - (1.0 - z) / n[2])
    }
}

/// A wedge over the sector `a0..a1` (degrees) of radius 0.4 about
/// [`MEET`], leaning along its bisector; `k` staggers its reach and
/// length.
pub fn wedge(a0: f64, a1: f64, k: usize) -> Hole {
    notch(a0, a1, k, 0.4)
}

/// [`wedge`] of radius `r`: past the plate's edge where `r` reaches it,
/// so its hole notches the top's boundary rather than lying inside it.
pub fn notch(a0: f64, a1: f64, k: usize, r: f64) -> Hole {
    let at = |a: f64| {
        let (s, c) = a.to_radians().sin_cos();
        (r.mul_add(c, MEET[0]), r.mul_add(s, MEET[1]))
    };
    let k = k as f64;
    Hole {
        footprint: vec![(MEET[0], MEET[1]), at(a0), at(a1)],
        lean: (a0 + a1) / 2.0,
        below: 0.03f64.mul_add(-k, 0.5),
        length: 0.11f64.mul_add(-k, 1.5),
    }
}

/// An L-shaped hole whose reflex corner is [`MEET`], leaving the
/// quadrant x < 1.5, y < 1 free and leaning away from it.
pub fn ell() -> Hole {
    Hole {
        footprint: vec![
            (1.0, 1.0),
            (1.5, 1.0),
            (1.5, 0.5),
            (2.0, 0.5),
            (2.0, 1.5),
            (1.0, 1.5),
        ],
        lean: 45.0,
        below: 0.43,
        length: 1.37,
    }
}

pub fn two_wedges() -> Vec<Hole> {
    vec![wedge(0.0, 50.0, 0), wedge(120.0, 170.0, 1)]
}

pub fn three_wedges() -> Vec<Hole> {
    vec![
        wedge(0.0, 50.0, 0),
        wedge(120.0, 170.0, 1),
        wedge(240.0, 290.0, 2),
    ]
}

pub fn four_wedges() -> Vec<Hole> {
    vec![
        wedge(0.0, 50.0, 0),
        wedge(90.0, 140.0, 1),
        wedge(180.0, 230.0, 2),
        wedge(270.0, 320.0, 3),
    ]
}

/// Three wedges on one side, leaving the top a reflex sector at [`MEET`].
pub fn wedges_on_one_side() -> Vec<Hole> {
    vec![
        wedge(0.0, 40.0, 0),
        wedge(60.0, 100.0, 1),
        wedge(120.0, 160.0, 2),
    ]
}

/// An L-shaped hole and two wedges in the quadrant it leaves.
pub fn ell_and_wedges() -> Vec<Hole> {
    vec![ell(), wedge(190.0, 220.0, 1), wedge(235.0, 260.0, 2)]
}

/// The holes inside the top, labelled: [`two_wedges`] and the four
/// fixtures after it.
pub fn inner_rows() -> Vec<(&'static str, Vec<Hole>)> {
    vec![
        ("two wedges", two_wedges()),
        ("three wedges", three_wedges()),
        ("four wedges", four_wedges()),
        ("three wedges on one side", wedges_on_one_side()),
        ("an L and two wedges", ell_and_wedges()),
    ]
}

/// Holes of which some notch the top's boundary ([`notch`]), labelled:
/// the top less them is several faces of one plane meeting at
/// [`MEET`].
pub fn notch_rows() -> Vec<(&'static str, Vec<Hole>)> {
    vec![
        (
            "three notches",
            vec![
                notch(80.0, 100.0, 0, 2.0),
                notch(200.0, 220.0, 1, 2.5),
                notch(320.0, 340.0, 2, 2.5),
            ],
        ),
        (
            "two notches and a wedge",
            vec![
                notch(80.0, 100.0, 0, 2.0),
                notch(260.0, 280.0, 1, 2.0),
                wedge(150.0, 200.0, 2),
            ],
        ),
        (
            "two notches and two wedges",
            vec![
                notch(80.0, 100.0, 0, 2.0),
                notch(260.0, 280.0, 1, 2.0),
                wedge(150.0, 200.0, 2),
                wedge(330.0, 20.0 + 360.0, 3),
            ],
        ),
        (
            "a notch and two wedges",
            vec![
                notch(80.0, 100.0, 0, 2.0),
                wedge(150.0, 200.0, 1),
                wedge(300.0, 350.0, 2),
            ],
        ),
    ]
}

/// **Every face's corners at one vertex are angularly disjoint** about
/// the face's Newell normal: a loop through a vertex twice or more,
/// or two loops through one vertex, pass their corners there without
/// crossing. A corner sweeps counterclockwise from its leaving edge to
/// its arriving one, the face on the left. Corners are grouped by
/// vertex key, so coincident vertices are compared only as the
/// topology joins them.
///
/// It reads planar faces with straight edges of positive length, and
/// refuses any other face rather than measure it.
pub fn corners_disjoint(body: &Body<f64>) -> Result<(), String> {
    use std::f64::consts::TAU;
    for (fk, f) in body.faces() {
        if !matches!(
            body.get_surface(f.surface),
            Some(geom::Surface::Plane { .. })
        ) {
            return Err(format!(
                "{fk:?} is not planar: the check reads planar faces"
            ));
        }
        let cycles = cycles_of(body, f);
        for &he in cycles.iter().flatten() {
            let straight = body
                .get_half_edge(he)
                .and_then(|h| body.get_edge(h.edge))
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(|c| c.certified())
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }));
            if !straight {
                return Err(format!(
                    "{fk:?}: {he:?} is not a straight edge: the check reads straight edges"
                ));
            }
        }
        let pt = |he| {
            body.half_edge_start_point(he)
                .ok_or_else(|| format!("{he:?} has no start point"))
        };
        let start = |he: HalfEdgeKey| {
            body.get_half_edge(he)
                .map(|h| h.start)
                .ok_or_else(|| format!("{he:?} does not resolve"))
        };
        let outer = cycles[0]
            .iter()
            .map(|&he| pt(he))
            .collect::<Result<Vec<_>, _>>()?;
        let (mut nx, mut ny, mut nz) = (0.0, 0.0, 0.0);
        for i in 0..outer.len() {
            let (a, b) = (outer[i], outer[(i + 1) % outer.len()]);
            nx += (a.y - b.y) * (a.z + b.z);
            ny += (a.z - b.z) * (a.x + b.x);
            nz += (a.x - b.x) * (a.y + b.y);
        }
        let n = geom_core::Vec3::new(nx, ny, nz).normalize();
        let seed = if n.x.abs() < 0.9 {
            geom_core::Vec3::new(1.0, 0.0, 0.0)
        } else {
            geom_core::Vec3::new(0.0, 1.0, 0.0)
        };
        let u = n.cross(seed).normalize();
        let v = n.cross(u);
        let angle = |d: geom_core::Vec3<f64>| d.dot(v).atan2(d.dot(u)).rem_euclid(TAU);
        let mut corners: BTreeMap<VertexKey, Vec<(f64, f64)>> = BTreeMap::new();
        for cycle in &cycles {
            let m = cycle.len();
            for i in 0..m {
                let (prev, here, next) = (
                    pt(cycle[(i + m - 1) % m])?,
                    pt(cycle[i])?,
                    pt(cycle[(i + 1) % m])?,
                );
                let (out, back) = (next - here, prev - here);
                if out.norm() < 1e-12 || back.norm() < 1e-12 {
                    return Err(format!("{fk:?}: a zero-length edge at {:?}", cycle[i]));
                }
                let from = angle(out);
                let sweep = (angle(back) - from).rem_euclid(TAU);
                corners
                    .entry(start(cycle[i])?)
                    .or_default()
                    .push((from, sweep));
            }
        }
        for (vk, cs) in corners {
            for (i, &(a, sa)) in cs.iter().enumerate() {
                for &(b, sb) in &cs[i + 1..] {
                    if (b - a).rem_euclid(TAU) < sa - 1e-9 || (a - b).rem_euclid(TAU) < sb - 1e-9 {
                        return Err(format!("{fk:?}: two corners at {vk:?} overlap"));
                    }
                }
            }
        }
    }
    Ok(())
}

/// The half-edge cycles of a face's loops, outer first.
pub fn cycles_of(body: &Body<f64>, f: &Face) -> Vec<Vec<HalfEdgeKey>> {
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .filter_map(|l| match body.get_loop(l)?.boundary {
            LoopBoundary::Cycle { first } => body.loop_cycle(first),
            LoopBoundary::Empty { .. } => None,
        })
        .collect()
}
