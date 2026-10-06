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
    let at = |a: f64| {
        let (s, c) = a.to_radians().sin_cos();
        (0.4f64.mul_add(c, MEET[0]), 0.4f64.mul_add(s, MEET[1]))
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

/// **Every face's corners at one vertex are angularly disjoint** about
/// the face's Newell normal: a loop through a vertex twice or more,
/// or two loops through one vertex, pass their corners there without
/// crossing. A corner sweeps counterclockwise from its leaving edge to
/// its arriving one, the face on the left. Corners are grouped by
/// vertex key, so coincident vertices are compared only as the
/// topology joins them. Planar faces with straight edges only.
pub fn corners_disjoint(body: &Body<f64>) -> Result<(), String> {
    use std::f64::consts::TAU;
    for (fk, f) in body.faces() {
        let cycles = cycles_of(body, f);
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
                let from = angle(next - here);
                let sweep = (angle(prev - here) - from).rem_euclid(TAU);
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
fn cycles_of(body: &Body<f64>, f: &Face) -> Vec<Vec<HalfEdgeKey>> {
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .filter_map(|l| match body.get_loop(l)?.boundary {
            LoopBoundary::Cycle { first } => body.loop_cycle(first),
            LoopBoundary::Empty { .. } => None,
        })
        .collect()
}
