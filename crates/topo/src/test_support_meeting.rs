//! **Holes meeting at one vertex of a plate's top**, and the check the
//! rows over them read: the fixture geometry the topo and editor-core
//! suites build the same bodies from, each through its own door (an
//! Euler-op prism here, a sketch and an extrude there).
//!
//! The plate is `[0, 3] × [0, 2] × [0, 1]`; each hole is a right prism
//! along an axis tilted out of its footprint, towards a bearing of its
//! own ([`Hole::lean`]). Leant along their bisectors ([`wedge`]) the
//! prisms move apart above the top and meet only at [`MEET`]; leant
//! across one another ([`leaned`], [`arch`]) they may cross above it.
//! The poses ([`Pose`], [`poses`]) move the whole scene rigidly.

use std::collections::BTreeMap;

use crate::AtRestBody;
use crate::body::Body;
use crate::entity::{Face, HalfEdgeKey, LoopBoundary};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::test_support::finished;
use crate::test_support_fixtures::{
    FaceGeometry, describe_as_intersections, line, plane, prism_ops,
};
use geom_core::{Point3, Tol};

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

/// [`wedge`] leaning towards the bearing `lean` (degrees) rather than
/// along its bisector: leant across another hole's sector, the prisms'
/// runs at [`MEET`] can nest about the top's normal.
pub fn leaned(a0: f64, a1: f64, k: usize, lean: f64) -> Hole {
    Hole {
        lean,
        ..wedge(a0, a1, k)
    }
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

/// Three wedges leant across one another ([`leaned`]): the first two
/// turned 60° and 240° off their bisectors. Their prisms' union meets
/// the top at [`MEET`] with three Out runs that nest about its normal.
pub fn arch() -> Vec<Hole> {
    vec![
        leaned(0.0, 30.0, 0, 75.0),
        leaned(120.0, 150.0, 1, 375.0),
        wedge(240.0, 270.0, 2),
    ]
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
            "a notch and two wedges apart",
            vec![
                notch(80.0, 100.0, 0, 2.0),
                wedge(200.0, 230.0, 1),
                wedge(250.0, 280.0, 2),
            ],
        ),
        (
            "a wide notch and two wedges",
            vec![
                notch(30.0, 150.0, 0, 2.0),
                wedge(200.0, 230.0, 1),
                wedge(300.0, 330.0, 2),
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
/// point, rounded to a micron ([`at`]), so the corners of several
/// vertices on one point (a pinch split per cone) are compared too.
///
/// It reads planar faces with straight edges of positive length, an
/// outer loop that is a cycle and corners whose two edges leave along
/// different directions, and refuses any other face rather than
/// measure it.
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
        if !matches!(
            body.get_loop(f.outer).map(|l| &l.boundary),
            Some(LoopBoundary::Cycle { .. })
        ) {
            return Err(format!("{fk:?}: its outer loop is not a cycle"));
        }
        let cycles = cycles_of(body, f);
        assert!(
            !cycles.is_empty(),
            "{fk:?}: a face with an outer cycle has no cycles"
        );
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
        let mut corners: BTreeMap<Point, Vec<(f64, f64)>> = BTreeMap::new();
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
                // Both edges along one direction: a corner of 0 or of 2π,
                // which the angles cannot tell apart.
                if !(1e-9..=TAU - 1e-9).contains(&sweep) {
                    return Err(format!(
                        "{fk:?}: a corner at {:?} whose edges leave along one direction",
                        cycle[i]
                    ));
                }
                corners.entry(at(here)).or_default().push((from, sweep));
            }
        }
        for (pk, cs) in corners {
            for (i, &(a, sa)) in cs.iter().enumerate() {
                for &(b, sb) in &cs[i + 1..] {
                    if (b - a).rem_euclid(TAU) < sa - 1e-9 || (a - b).rem_euclid(TAU) < sb - 1e-9 {
                        return Err(format!("{fk:?}: two corners at {pk:?} overlap"));
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

/// A point rounded to a micron.
pub type Point = (i64, i64, i64);

/// `p` rounded to a micron.
pub fn at(p: Point3<f64>) -> Point {
    let n = |x: f64| (x * 1e6).round() as i64;
    (n(p.x), n(p.y), n(p.z))
}

/// A body's geometry, key-free: each face by the points of its loops'
/// vertices, each edge by its ends' points, as sorted multisets.
///
/// # Panics
///
/// Where a half-edge of `body` has no start point: a body that is not
/// live.
#[must_use]
#[allow(clippy::expect_used)]
pub fn shape(body: &Body<f64>) -> (Vec<Vec<Point>>, Vec<[Point; 2]>) {
    let pt = |he| {
        at(body
            .half_edge_start_point(he)
            .expect("a half-edge of a live body has a start point"))
    };
    let mut faces: Vec<Vec<Point>> = body
        .faces()
        .map(|(_, f)| {
            let mut ps: Vec<Point> = cycles_of(body, f).into_iter().flatten().map(pt).collect();
            ps.sort_unstable();
            ps
        })
        .collect();
    faces.sort_unstable();
    let mut edges: Vec<[Point; 2]> = body
        .edges()
        .map(|(_, e)| {
            let mut ps = [e.he_plus, e.he_minus].map(pt);
            ps.sort_unstable();
            ps
        })
        .collect();
    edges.sort_unstable();
    (faces, edges)
}

/// Every order of `0..n`.
pub fn orders(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for order in orders(n - 1) {
        for i in 0..n {
            let mut o = order.clone();
            o.insert(i, n - 1);
            out.push(o);
        }
    }
    out
}

/// A rigid motion of the whole scene: `x ↦ r x + t`.
pub struct Pose {
    /// Its name in a row's labels.
    pub label: &'static str,
    r: [[f64; 3]; 3],
    t: [f64; 3],
}

impl Pose {
    /// The turn by `angle` about `axis`, then the shift `t`.
    pub fn turn(label: &'static str, axis: [f64; 3], angle: f64, t: [f64; 3]) -> Self {
        let l = axis.iter().map(|a| a * a).sum::<f64>().sqrt();
        let [x, y, z] = axis.map(|a| a / l);
        let (s, c) = angle.sin_cos();
        let d = 1.0 - c;
        let r = [
            [c + x * x * d, x * y * d - z * s, x * z * d + y * s],
            [y * x * d + z * s, c + y * y * d, y * z * d - x * s],
            [z * x * d - y * s, z * y * d + x * s, c + z * z * d],
        ];
        Self { label, r, t }
    }

    /// The pose then `self`.
    pub fn after(self, first: &Self) -> Self {
        let r = [0, 1, 2]
            .map(|i| [0, 1, 2].map(|j| (0..3).map(|k| self.r[i][k] * first.r[k][j]).sum::<f64>()));
        let t =
            [0, 1, 2].map(|i| (0..3).map(|k| self.r[i][k] * first.t[k]).sum::<f64>() + self.t[i]);
        Self {
            label: self.label,
            r,
            t,
        }
    }

    /// `p` moved.
    pub fn at(&self, p: [f64; 3]) -> Point3<f64> {
        let q = [0, 1, 2].map(|i| (0..3).map(|k| self.r[i][k] * p[k]).sum::<f64>() + self.t[i]);
        Point3::new(q[0], q[1], q[2])
    }
}

impl Pose {
    /// The identity.
    pub fn rest() -> Self {
        Self::turn("at rest", [0.0, 0.0, 1.0], 0.0, [0.0, 0.0, 0.0])
    }
}

/// The scene's poses: at rest, turned about the top's normal, turned in
/// general, and flipped (the top facing −z) at rest and turned.
pub fn poses() -> Vec<Pose> {
    let flip = || {
        Pose::turn(
            "flipped",
            [1.0, 0.0, 0.0],
            std::f64::consts::PI,
            [0.0, 0.0, 0.0],
        )
    };
    vec![
        Pose::rest(),
        Pose::turn("turned about z", [0.0, 0.0, 1.0], 0.65, [0.0, 0.0, 0.0]),
        Pose::turn("turned", [1.0, 2.0, 3.0], 0.7, [0.3, -0.2, 0.5]),
        flip(),
        Pose::turn(
            "flipped and turned",
            [-2.0, 1.0, 1.0],
            1.1,
            [-0.4, 0.1, 0.3],
        )
        .after(&flip()),
    ]
}

/// A box `[x, y, z]` placed by `pose`.
pub fn posed_box(what: &str, b: [(f64, f64); 3], pose: &Pose) -> AtRestBody<f64> {
    let [(x0, x1), (y0, y1), z] = b;
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &[(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
        z,
        |x, y, z| pose.at([x, y, z]),
        FaceGeometry::Certified,
        Tol::witness(),
    );
    describe_as_intersections(&mut body, Tol::witness());
    finished(what, body, Tol::witness())
}

/// A hole's prism placed by `pose`.
pub fn posed_prism(h: &Hole, pose: &Pose) -> AtRestBody<f64> {
    let [o, u, v, n] = h.frame();
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &h.profile(),
        (0.0, h.length),
        |x, y, z| pose.at([0, 1, 2].map(|i| o[i] + x * u[i] + y * v[i] + z * n[i])),
        FaceGeometry::Certified,
        Tol::witness(),
    );
    describe_as_intersections(&mut body, Tol::witness());
    finished("a tilted prism", body, Tol::witness())
}

/// A pyramid placed by `pose`: its apex, and its base's corners
/// counterclockwise seen from the apex's side.
///
/// # Panics
///
/// Where an Euler operator refuses, or the pyramid is not a finished
/// body (a base wound clockwise from the apex is inside out).
pub fn posed_pyramid(base: &[[f64; 3]], apex: [f64; 3], pose: &Pose) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let n = base.len();
    assert!(n >= 3, "a pyramid needs at least three base corners");
    let bot: Vec<_> = base.iter().map(|&q| pose.at(q)).collect();
    let top = pose.at(apex);
    let face = |corners: &[Point3<f64>]| FaceSurface::New {
        surface: plane(corners, tol),
        sense: true,
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(bot[0], true).unwrap();
    let mut chain = vec![
        body.mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            bot[1],
            line(bot[0], bot[1]),
            tol,
        )
        .unwrap(),
    ];
    for i in 2..n {
        let at = chain[i - 2].he_minus;
        let e = body.mev(
            MevSite::Fan { he1: at, he2: at },
            bot[i],
            line(bot[i - 1], bot[i]),
            tol,
        );
        chain.push(e.unwrap());
    }
    // The base, outward away from the apex: the corners reversed.
    let corners: Vec<_> = core::iter::once(seed.vertex)
        .chain(chain.iter().map(|m| m.vertex))
        .collect();
    let he_last = body
        .find_half_edge(seed.face, corners[n - 1], corners[n - 2])
        .unwrap();
    let rev: Vec<_> = core::iter::once(bot[0])
        .chain(bot[1..].iter().rev().copied())
        .collect();
    let bottom = body
        .mef(
            MefSite::Chords {
                he1: he_last,
                he2: chain[0].he_plus,
            },
            line(bot[n - 1], bot[0]),
            face(&rev),
            tol,
        )
        .unwrap();
    // The apex up from the first corner, then one side face per base
    // edge; the seed face is the last side.
    let at = chain[0].he_plus;
    let strut = body
        .mev(
            MevSite::Fan { he1: at, he2: at },
            top,
            line(bot[0], top),
            tol,
        )
        .unwrap();
    let mut he1 = strut.he_minus;
    for i in 1..n {
        let he2 = if i < n - 1 {
            chain[i].he_plus
        } else {
            bottom.he_plus
        };
        let side = body
            .mef(
                MefSite::Chords { he1, he2 },
                line(top, bot[i]),
                face(&[bot[i - 1], bot[i], top]),
                tol,
            )
            .unwrap();
        he1 = side.he_plus;
    }
    body.set_face_surface(seed.face, face(&[bot[n - 1], bot[0], top]))
        .unwrap();
    describe_as_intersections(&mut body, tol);
    finished("a pyramid", body, tol)
}

/// Boxes `[x, y, z]` placed by `pose`, as the solids of one body: where
/// two touch, the body holds its own contact there.
pub fn posed_boxes(what: &str, boxes: &[[(f64, f64); 3]], pose: &Pose) -> AtRestBody<f64> {
    let mut body = Body::<f64>::new();
    for &[(x0, x1), (y0, y1), z] in boxes {
        prism_ops(
            &mut body,
            &[(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
            z,
            |x, y, z| pose.at([x, y, z]),
            FaceGeometry::Certified,
            Tol::witness(),
        );
    }
    describe_as_intersections(&mut body, Tol::witness());
    finished(what, body, Tol::witness())
}
