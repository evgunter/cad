//! **Scratch measurement (not for merge): the obstacle mechanism for
//! N2's `Borders` rule.**
//!
//! For every parent a finished boolean or union body holds as two or
//! more pieces, compute the obstacle partition three ways and compare:
//! - `truth`: the planar definition, by brute force — the parent's
//!   member faces and its pieces are tessellated, mapped to the
//!   parent's own chart (a plane, or an unrolled cylinder) and
//!   rasterized; an obstacle is a 4-connected component of parent cells
//!   no piece holds;
//! - `fp`: the face-path variant — two divider walls are one obstacle
//!   when a path of the finished body's faces joins them without
//!   passing through a face of the parent's own member(s);
//! - `walk`: the finished-body walk with no geometry — obstacle
//!   boundary edges joined through shared vertices and through the
//!   parent's unheld rim stretches, so an obstacle with two boundary
//!   loops that carry pieces (an island) splits, and two obstacles
//!   meeting at a vertex (a pinch) join.
//!
//! Run: `cargo test -p editor-core --test all borders_probe -- --ignored --nocapture`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::emit_shared_rim_several::permutations;
use crate::fixture::{insert, len, on_frame, table};

use editor_core::{
    BooleanOp, Entry, EntityKey, EntityKind, LoopProgram, Node, ProfileDoc, ProfileProgram,
    RecipeNodeId, RoleSeg, StableName,
};
use geom_core::{Point3, Tol};
use topo::{Body, EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, VertexKey};

// ---------------------------------------------------------------- names

/// A face name's parent: the name without its trailing `Fragment`s.
fn parent_of(n: &StableName) -> StableName {
    let mut p = n.clone();
    while matches!(p.path.last(), Some(RoleSeg::Fragment(_))) {
        p.path.pop();
    }
    p
}

/// The member faces a parent name lists: `(operand node, name there)`.
fn member_faces(
    n: &StableName,
    ab: Option<(RecipeNodeId, RecipeNodeId)>,
    out: &mut BTreeSet<(RecipeNodeId, StableName)>,
) {
    match n.path.first() {
        Some(RoleSeg::FromMember { member, of }) => {
            out.insert((*member, of.name().clone()));
        }
        Some(RoleSeg::FromA(of)) => {
            out.insert((ab.expect("a pair name").0, of.name().clone()));
        }
        Some(RoleSeg::FromB(of)) => {
            out.insert((ab.expect("a pair name").1, of.name().clone()));
        }
        Some(RoleSeg::Merged(set)) => {
            for c in set {
                member_faces(c, ab, out);
            }
        }
        _ => {}
    }
}

fn short(n: &StableName) -> String {
    n.path.iter().map(seg).collect::<Vec<_>>().join("/")
}

fn seg(s: &RoleSeg) -> String {
    match s {
        RoleSeg::FromMember { member, of } => format!("m{}.{}", member.0, short(of.name())),
        RoleSeg::FromA(of) => format!("A.{}", short(of.name())),
        RoleSeg::FromB(of) => format!("B.{}", short(of.name())),
        RoleSeg::Merged(v) => format!("Merged[{}]", v.iter().map(short).collect::<Vec<_>>().join(",")),
        RoleSeg::Fragment(_) => "#frag".into(),
        other => {
            let d = format!("{other:?}");
            let d = d
                .replace("Lateral(Piece { step: StepId(", "Lat(")
                .replace("), role: Leg })", ")")
                .replace(" ", "");
            d
        }
    }
}

// ---------------------------------------------------------------- topology

fn face_hes(body: &Body<f64>, f: FaceKey) -> Vec<Vec<HalfEdgeKey>> {
    let face = body.get_face(f).expect("a live face");
    let mut out = Vec::new();
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        if let LoopBoundary::Cycle { first } = body.get_loop(lk).expect("a live loop").boundary {
            out.push(body.loop_cycle(first).expect("a closed cycle"));
        }
    }
    out
}

fn he_face(body: &Body<f64>, he: HalfEdgeKey) -> FaceKey {
    let h = body.get_half_edge(he).unwrap();
    body.get_loop(h.parent_loop).unwrap().face
}

fn mate(body: &Body<f64>, he: HalfEdgeKey) -> HalfEdgeKey {
    let e = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
    if e.he_plus == he { e.he_minus } else { e.he_plus }
}

fn he_ends(body: &Body<f64>, he: HalfEdgeKey) -> [VertexKey; 2] {
    [
        body.get_half_edge(he).unwrap().start,
        body.half_edge_end(he).unwrap(),
    ]
}

fn pt(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

// ---------------------------------------------------------------- chart

#[derive(Clone, Copy)]
enum Chart {
    Plane {
        o: Point3<f64>,
        u: [f64; 3],
        v: [f64; 3],
    },
    Cyl {
        o: Point3<f64>,
        axis: [f64; 3],
        r: f64,
        uref: [f64; 3],
        w: [f64; 3],
    },
}

fn sub(a: Point3<f64>, b: Point3<f64>) -> [f64; 3] {
    [a.x - b.x, a.y - b.y, a.z - b.z]
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn v3(p: &geom_core::Vec3<f64>) -> [f64; 3] {
    [p.x, p.y, p.z]
}

impl Chart {
    fn of(body: &Body<f64>, f: FaceKey) -> Option<Chart> {
        let s = body.get_surface(body.get_face(f)?.surface)?;
        match s {
            topo::Surface::Plane {
                origin,
                normal,
                u_ref,
            } => {
                let n = v3(normal);
                let u = v3(u_ref);
                Some(Chart::Plane {
                    o: *origin,
                    u,
                    v: cross(n, u),
                })
            }
            topo::Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => {
                let a = v3(axis);
                let u = v3(u_ref);
                Some(Chart::Cyl {
                    o: *origin,
                    axis: a,
                    r: *radius,
                    uref: u,
                    w: cross(a, u),
                })
            }
            _ => None,
        }
    }
    fn period(&self) -> Option<f64> {
        match self {
            Chart::Plane { .. } => None,
            Chart::Cyl { r, .. } => Some(2.0 * core::f64::consts::PI * r),
        }
    }
    fn map(&self, p: Point3<f64>) -> (f64, f64) {
        match *self {
            Chart::Plane { o, u, v } => {
                let d = sub(p, o);
                (dot(d, u), dot(d, v))
            }
            Chart::Cyl {
                o, axis, r, uref, w, ..
            } => {
                let d = sub(p, o);
                let h = dot(d, axis);
                let mut t = dot(d, w).atan2(dot(d, uref));
                if t < 0.0 {
                    t += 2.0 * core::f64::consts::PI;
                }
                (t * r, h)
            }
        }
    }
}

// ---------------------------------------------------------------- raster

const OUT: i32 = -1;
const FREE: i32 = -2;

struct Grid {
    x0: f64,
    y0: f64,
    h: f64,
    nx: usize,
    ny: usize,
    periodic: bool,
    lab: Vec<i32>,
}

impl Grid {
    fn idx(&self, i: i64, j: i64) -> Option<usize> {
        let i = if self.periodic {
            i.rem_euclid(self.nx as i64)
        } else if i < 0 || i >= self.nx as i64 {
            return None;
        } else {
            i
        };
        if j < 0 || j >= self.ny as i64 {
            return None;
        }
        Some(j as usize * self.nx + i as usize)
    }
    fn cell(&self, x: f64, y: f64) -> Option<usize> {
        let i = ((x - self.x0) / self.h).floor() as i64;
        let j = ((y - self.y0) / self.h).floor() as i64;
        self.idx(i, j)
    }
    fn at(&self, x: f64, y: f64) -> i32 {
        self.cell(x, y).map_or(OUT, |k| self.lab[k])
    }
    /// Paints `val` into every cell whose centre lies in the triangle,
    /// where the cell currently reads `when` (or anything if `None`).
    fn paint(&mut self, t: [(f64, f64); 3], val: i32, when: Option<i32>) {
        let shifts: Vec<f64> = if self.periodic {
            let p = self.nx as f64 * self.h;
            vec![-p, 0.0, p]
        } else {
            vec![0.0]
        };
        for s in shifts {
            let t = t.map(|(x, y)| (x + s, y));
            let (xmin, xmax) = (
                t.iter().map(|p| p.0).fold(f64::MAX, f64::min),
                t.iter().map(|p| p.0).fold(f64::MIN, f64::max),
            );
            let (ymin, ymax) = (
                t.iter().map(|p| p.1).fold(f64::MAX, f64::min),
                t.iter().map(|p| p.1).fold(f64::MIN, f64::max),
            );
            let i0 = ((xmin - self.x0) / self.h - 0.5).floor().max(0.0) as i64;
            let i1 = (((xmax - self.x0) / self.h - 0.5).ceil() as i64).min(self.nx as i64 - 1);
            let j0 = ((ymin - self.y0) / self.h - 0.5).floor().max(0.0) as i64;
            let j1 = (((ymax - self.y0) / self.h - 0.5).ceil() as i64).min(self.ny as i64 - 1);
            let area = (t[1].0 - t[0].0) * (t[2].1 - t[0].1) - (t[2].0 - t[0].0) * (t[1].1 - t[0].1);
            if area.abs() < 1e-18 {
                continue;
            }
            for j in j0..=j1 {
                for i in i0..=i1 {
                    let x = self.x0 + (i as f64 + 0.5) * self.h;
                    let y = self.y0 + (j as f64 + 0.5) * self.h;
                    let e = |a: (f64, f64), b: (f64, f64)| {
                        ((b.0 - a.0) * (y - a.1) - (b.1 - a.1) * (x - a.0)) * area.signum()
                    };
                    if e(t[0], t[1]) >= 0.0 && e(t[1], t[2]) >= 0.0 && e(t[2], t[0]) >= 0.0 {
                        let k = j as usize * self.nx + i as usize;
                        if when.is_none_or(|w| self.lab[k] == w) {
                            self.lab[k] = val;
                        }
                    }
                }
            }
        }
    }
    fn neighbours4(&self, k: usize) -> Vec<usize> {
        let (i, j) = ((k % self.nx) as i64, (k / self.nx) as i64);
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .filter_map(|&(di, dj)| self.idx(i + di, j + dj))
            .collect()
    }
    fn neighbours8(&self, k: usize) -> Vec<usize> {
        let (i, j) = ((k % self.nx) as i64, (k / self.nx) as i64);
        let mut out = Vec::new();
        for di in -1..=1 {
            for dj in -1..=1 {
                if (di, dj) != (0, 0) {
                    if let Some(n) = self.idx(i + di, j + dj) {
                        out.push(n);
                    }
                }
            }
        }
        out
    }
    fn on_border(&self, k: usize) -> bool {
        let (i, j) = (k % self.nx, k / self.nx);
        j == 0 || j + 1 == self.ny || (!self.periodic && (i == 0 || i + 1 == self.nx))
    }
}

/// Unwraps a triangle's u coordinates on a periodic chart so no side
/// spans more than half the period.
fn unwrap_tri(t: [(f64, f64); 3], period: Option<f64>) -> [(f64, f64); 3] {
    let Some(p) = period else { return t };
    let mut t = t;
    for k in 1..3 {
        while t[k].0 - t[0].0 > p / 2.0 {
            t[k].0 -= p;
        }
        while t[0].0 - t[k].0 > p / 2.0 {
            t[k].0 += p;
        }
    }
    t
}

fn unwrap_near(x: f64, near: f64, period: Option<f64>) -> f64 {
    let Some(p) = period else { return x };
    let mut x = x;
    while x - near > p / 2.0 {
        x -= p;
    }
    while near - x > p / 2.0 {
        x += p;
    }
    x
}

// ---------------------------------------------------------------- the probe

/// What lies across one piece edge, in the parent's own surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Far {
    /// An obstacle (truth component).
    Obst(usize),
    /// Outside the parent's region (a rim).
    Out,
    /// Another piece of the same parent.
    Piece(usize),
    /// The raster could not tell.
    Unknown,
}

/// One piece edge.
struct PEdge {
    piece: usize,
    edge: EdgeKey,
    ends: [VertexKey; 2],
    /// The face across it.
    wall: FaceKey,
    far: Far,
}

/// One group's measurement.
#[derive(Default, Debug)]
pub(crate) struct GroupReport {
    pub parent: String,
    pub pieces: usize,
    pub islands: usize,
    pub pinch_pieces: usize,
    pub pinch_obstacles: usize,
    pub walk_split: usize,
    pub obstacles: usize,
    pub dividers: usize,
    /// truth Borders per piece, sorted multiset of sorted wall lists.
    pub borders_truth: Vec<Vec<String>>,
    pub borders_fp: Vec<Vec<String>>,
    pub borders_fpp: Vec<Vec<String>>,
    pub borders_walk: Vec<Vec<String>>,
    pub part_fp_differs: bool,
    pub part_fpp_differs: bool,
    pub part_walk_differs: bool,
    /// Edges the face-path rule would count as obstacle borders that
    /// the raster puts on the rim (a flush neighbour), and the reverse.
    pub fp_rim_confusions: usize,
    pub fp_excluded_walls: usize,
    pub unknown_edges: usize,
    pub note: String,
    /// Kernel record: whether a log exists for the node.
    pub rec_logged: bool,
    pub rec_paths: String,
    pub borders_rec: Vec<Vec<String>>,
    pub part_rec_differs: bool,
    /// Truth obstacle edges the record classes as nothing.
    pub rec_missing: usize,
    /// Edges the record classes as an obstacle that truth does not.
    pub rec_spurious: usize,
    pub rec_unresolved: usize,
}

/// Tessellation cache, by node.
pub(crate) struct Meshes {
    by_node: HashMap<RecipeNodeId, mesh::types::Mesh>,
}

fn tessellate(body: &Body<f64>) -> mesh::types::Mesh {
    mesh::tessellate(body, 2e-4, Tol::witness()).expect("tessellates")
}

fn face_tris(m: &mesh::types::Mesh, f: FaceKey) -> Vec<[Point3<f64>; 3]> {
    m.patches
        .iter()
        .filter(|p| p.face == f)
        .flat_map(|p| {
            p.triangles
                .iter()
                .map(|t| t.map(|i| m.positions[i as usize]))
        })
        .collect()
}

fn edge_poly(m: &mesh::types::Mesh, e: EdgeKey) -> Vec<Point3<f64>> {
    m.boundaries
        .iter()
        .find(|b| b.edge == e)
        .map(|b| b.points.iter().map(|&i| m.positions[i as usize]).collect())
        .unwrap_or_default()
}

/// The groups of one finished body.
///
/// `ab`: the operand nodes of a pair boolean (their names use
/// `FromA`/`FromB`), `None` for a union.
pub(crate) fn probe_body(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
    ab: Option<(RecipeNodeId, RecipeNodeId)>,
) -> Vec<GroupReport> {
    let body = body_of(ev, node);
    let t = table(ev, node);
    let m = tessellate(body);
    let mut member_mesh: HashMap<RecipeNodeId, mesh::types::Mesh> = HashMap::new();

    // Face → (published name, parent), by entity.
    let mut name_of: BTreeMap<FaceKey, StableName> = BTreeMap::new();
    let mut tied: BTreeSet<FaceKey> = BTreeSet::new();
    for (n, e) in t.iter() {
        if n.kind != EntityKind::Face {
            continue;
        }
        match e {
            Entry::Unique(x) => {
                if let EntityKey::Face(f) = x.key {
                    name_of.insert(f, n.clone());
                }
            }
            Entry::Tied(xs) => {
                for x in xs {
                    if let EntityKey::Face(f) = x.key {
                        name_of.insert(f, n.clone());
                        tied.insert(f);
                    }
                }
            }
        }
    }
    let parent: BTreeMap<FaceKey, StableName> =
        name_of.iter().map(|(f, n)| (*f, parent_of(n))).collect();
    let members_of = |p: &StableName| -> BTreeSet<RecipeNodeId> {
        let mut s = BTreeSet::new();
        member_faces(p, ab, &mut s);
        s.into_iter().map(|(m, _)| m).collect()
    };
    let mut groups: BTreeMap<StableName, Vec<FaceKey>> = BTreeMap::new();
    for (f, p) in &parent {
        if tied.contains(f) {
            continue;
        }
        groups.entry(p.clone()).or_default().push(*f);
    }
    // Face adjacency of the finished body.
    let mut adj: BTreeMap<FaceKey, BTreeSet<FaceKey>> = BTreeMap::new();
    for (_, e) in body.edges() {
        let (f, g) = (he_face(body, e.he_plus), he_face(body, e.he_minus));
        adj.entry(f).or_default().insert(g);
        adj.entry(g).or_default().insert(f);
    }

    let mut out = Vec::new();
    for (pname, pieces) in groups {
        if pieces.len() < 2 {
            continue;
        }
        let mut rep = GroupReport {
            parent: short(&pname),
            pieces: pieces.len(),
            ..Default::default()
        };
        let Some(chart) = Chart::of(body, pieces[0]) else {
            rep.note = "unsupported carrier".into();
            out.push(rep);
            continue;
        };
        let period = chart.period();
        // Parent region: the member faces' triangles.
        let mut mf = BTreeSet::new();
        member_faces(&pname, ab, &mut mf);
        let mut region: Vec<[(f64, f64); 3]> = Vec::new();
        for (mnode, mname) in &mf {
            let mbody = body_of(ev, *mnode);
            let mm = member_mesh
                .entry(*mnode)
                .or_insert_with(|| tessellate(mbody));
            let keys: Vec<FaceKey> = match table(ev, *mnode).lookup(mname) {
                Some(Entry::Unique(x)) => match x.key {
                    EntityKey::Face(f) => vec![f],
                    _ => vec![],
                },
                Some(Entry::Tied(xs)) => xs
                    .iter()
                    .filter_map(|x| match x.key {
                        EntityKey::Face(f) => Some(f),
                        _ => None,
                    })
                    .collect(),
                None => vec![],
            };
            for f in keys {
                for tri in face_tris(mm, f) {
                    region.push(unwrap_tri(tri.map(|p| chart.map(p)), period));
                }
            }
        }
        if region.is_empty() {
            rep.note = "no member region".into();
            out.push(rep);
            continue;
        }
        let piece_tris: Vec<Vec<[(f64, f64); 3]>> = pieces
            .iter()
            .map(|&f| {
                face_tris(&m, f)
                    .into_iter()
                    .map(|tri| unwrap_tri(tri.map(|p| chart.map(p)), period))
                    .collect()
            })
            .collect();
        // Grid.
        let h = 0.004;
        let (mut xmin, mut xmax, mut ymin, mut ymax) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for tri in &region {
            for p in tri {
                xmin = xmin.min(p.0);
                xmax = xmax.max(p.0);
                ymin = ymin.min(p.1);
                ymax = ymax.max(p.1);
            }
        }
        let (x0, nx, periodic) = match period {
            Some(p) => (0.0, (p / h).ceil() as usize, true),
            None => {
                let x0 = xmin - 4.0 * h - 0.000_731;
                (x0, ((xmax - x0) / h).ceil() as usize + 5, false)
            }
        };
        let y0 = ymin - 4.0 * h - 0.000_593;
        let ny = ((ymax - y0) / h).ceil() as usize + 5;
        let h = match period {
            Some(p) => p / nx as f64,
            None => h,
        };
        let mut g = Grid {
            x0,
            y0,
            h,
            nx,
            ny,
            periodic,
            lab: vec![OUT; nx * ny],
        };
        for tri in &region {
            g.paint(*tri, FREE, None);
        }
        for (i, tris) in piece_tris.iter().enumerate() {
            for tri in tris {
                g.paint(*tri, i as i32, None);
            }
        }
        // Truth components.
        let mut comp = vec![usize::MAX; g.lab.len()];
        let mut ncomp = 0;
        for k in 0..g.lab.len() {
            if g.lab[k] != FREE || comp[k] != usize::MAX {
                continue;
            }
            let mut q = VecDeque::from([k]);
            comp[k] = ncomp;
            while let Some(c) = q.pop_front() {
                for n in g.neighbours4(c) {
                    if g.lab[n] == FREE && comp[n] == usize::MAX {
                        comp[n] = ncomp;
                        q.push_back(n);
                    }
                }
            }
            ncomp += 1;
        }
        // Piece edges and what lies across each.
        let piece_ix: BTreeMap<FaceKey, usize> =
            pieces.iter().enumerate().map(|(i, f)| (*f, i)).collect();
        let mut pedges: Vec<PEdge> = Vec::new();
        for (i, &f) in pieces.iter().enumerate() {
            for lp in face_hes(body, f) {
                for he in lp {
                    let e = body.get_half_edge(he).unwrap().edge;
                    let wall = he_face(body, mate(body, he));
                    let poly = edge_poly(&m, e);
                    let mut tally: BTreeMap<Far, usize> = BTreeMap::new();
                    for w in poly.windows(2) {
                        let a = chart.map(w[0]);
                        let b0 = chart.map(w[1]);
                        let b = (unwrap_near(b0.0, a.0, period), b0.1);
                        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                        let l = (dx * dx + dy * dy).sqrt();
                        if l < 1e-12 {
                            continue;
                        }
                        let n = (-dy / l, dx / l);
                        for s in [0.25, 0.5, 0.75] {
                            let mx = a.0 + dx * s;
                            let my = a.1 + dy * s;
                            let d = 3.0 * g.h;
                            let l1 = g.at(mx + n.0 * d, my + n.1 * d);
                            let l2 = g.at(mx - n.0 * d, my - n.1 * d);
                            let across = if l1 == i as i32 && l2 != i as i32 {
                                Some((l2, (mx - n.0 * d, my - n.1 * d)))
                            } else if l2 == i as i32 && l1 != i as i32 {
                                Some((l1, (mx + n.0 * d, my + n.1 * d)))
                            } else {
                                None
                            };
                            let far = match across {
                                None => Far::Unknown,
                                Some((OUT, _)) => Far::Out,
                                Some((FREE, (x, y))) => {
                                    Far::Obst(comp[g.cell(x, y).unwrap()])
                                }
                                Some((j, _)) => Far::Piece(j as usize),
                            };
                            *tally.entry(far).or_default() += 1;
                        }
                    }
                    let far = tally
                        .iter()
                        .filter(|(k, _)| **k != Far::Unknown)
                        .max_by_key(|(_, c)| **c)
                        .map(|(k, _)| *k)
                        .unwrap_or(Far::Unknown);
                    if far == Far::Unknown {
                        rep.unknown_edges += 1;
                    }
                    pedges.push(PEdge {
                        piece: i,
                        edge: e,
                        ends: he_ends(body, he),
                        wall,
                        far,
                    });
                }
            }
        }
        let wall_name = |f: FaceKey| -> String {
            parent.get(&f).map_or_else(|| "?".into(), short)
        };
        // Borders under a partition: `class(edge) -> Option<id>`.
        let borders = |class: &dyn Fn(&PEdge) -> Option<usize>| -> (Vec<Vec<String>>, BTreeMap<usize, BTreeSet<usize>>) {
            let mut touch: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
            for pe in &pedges {
                if let Some(c) = class(pe) {
                    touch.entry(c).or_default().insert(pe.piece);
                }
            }
            let mut per: Vec<BTreeSet<String>> = vec![BTreeSet::new(); pieces.len()];
            for pe in &pedges {
                if let Some(c) = class(pe) {
                    if touch[&c].len() >= 2 {
                        per[pe.piece].insert(wall_name(pe.wall));
                    }
                }
            }
            let mut v: Vec<Vec<String>> = per.into_iter().map(|s| s.into_iter().collect()).collect();
            v.sort();
            (v, touch)
        };
        let truth_class = |pe: &PEdge| match pe.far {
            Far::Obst(c) => Some(c),
            _ => None,
        };
        let (bt, touch_t) = borders(&truth_class);
        rep.borders_truth = bt;
        rep.obstacles = touch_t.len();
        rep.dividers = touch_t.values().filter(|s| s.len() >= 2).count();

        // Islands: a piece every edge of which borders one obstacle.
        for i in 0..pieces.len() {
            let fars: BTreeSet<Far> = pedges.iter().filter(|p| p.piece == i).map(|p| p.far).collect();
            if fars.len() == 1 && matches!(fars.iter().next(), Some(Far::Obst(_))) {
                rep.islands += 1;
            }
        }
        // Pinches: pieces sharing a vertex and no edge.
        let mut pv: Vec<BTreeSet<VertexKey>> = vec![BTreeSet::new(); pieces.len()];
        let mut pe_set: Vec<BTreeSet<EdgeKey>> = vec![BTreeSet::new(); pieces.len()];
        for pe in &pedges {
            pv[pe.piece].extend(pe.ends);
            pe_set[pe.piece].insert(pe.edge);
        }
        for i in 0..pieces.len() {
            for j in i + 1..pieces.len() {
                if !pv[i].is_disjoint(&pv[j]) && pe_set[i].is_disjoint(&pe_set[j]) {
                    rep.pinch_pieces += 1;
                }
            }
        }
        // Pinch vertices: one vertex on edges bordering two obstacles.
        let mut v_obst: BTreeMap<VertexKey, BTreeSet<usize>> = BTreeMap::new();
        for pe in &pedges {
            if let Far::Obst(c) = pe.far {
                for v in pe.ends {
                    v_obst.entry(v).or_default().insert(c);
                }
            }
        }
        rep.pinch_obstacles = v_obst.values().filter(|s| s.len() >= 2).count();

        // Face-path: components of the finished body's faces minus the
        // parent's member(s)' faces (`fp`) or minus the parent's own
        // pieces (`fpp`).
        let pmembers = members_of(&pname);
        let fp_comp = |excluded: &dyn Fn(FaceKey) -> bool| -> BTreeMap<FaceKey, usize> {
            let mut c: BTreeMap<FaceKey, usize> = BTreeMap::new();
            let mut n = 0;
            for (f, _) in body.faces() {
                if excluded(f) || c.contains_key(&f) {
                    continue;
                }
                let mut q = VecDeque::from([f]);
                c.insert(f, n);
                while let Some(x) = q.pop_front() {
                    for &y in adj.get(&x).into_iter().flatten() {
                        if !excluded(y) && !c.contains_key(&y) {
                            c.insert(y, n);
                            q.push_back(y);
                        }
                    }
                }
                n += 1;
            }
            c
        };
        let excl_members = |f: FaceKey| {
            parent
                .get(&f)
                .is_some_and(|p| !members_of(p).is_disjoint(&pmembers))
        };
        let excl_pieces = |f: FaceKey| piece_ix.contains_key(&f);
        let eix: BTreeMap<EdgeKey, usize> =
            pedges.iter().enumerate().map(|(i, p)| (p.edge, i)).collect();
        let cm = fp_comp(&excl_members);
        let cp = fp_comp(&excl_pieces);
        // Which edges the face-path rule sees as obstacle borders: the
        // truth's (member-edge cells tell rims apart), so the rule is
        // judged only on how it groups them.
        let fp_class = |pe: &PEdge| match pe.far {
            Far::Obst(_) => cm.get(&pe.wall).copied().or(Some(usize::MAX - eix[&pe.edge])),
            _ => None,
        };
        let fpp_class = |pe: &PEdge| match pe.far {
            Far::Obst(_) => cp.get(&pe.wall).copied(),
            _ => None,
        };
        for pe in &pedges {
            let naive_border = !excl_members(pe.wall) && !piece_ix.contains_key(&pe.wall);
            let truth_border = matches!(pe.far, Far::Obst(_));
            if naive_border != truth_border && pe.far != Far::Unknown {
                rep.fp_rim_confusions += 1;
            }
            if truth_border && excl_members(pe.wall) {
                rep.fp_excluded_walls += 1;
            }
        }
        let (bf, _) = borders(&fp_class);
        let (bfp, _) = borders(&fpp_class);
        rep.borders_fp = bf;
        rep.borders_fpp = bfp;
        rep.part_fp_differs = !same_partition(&pedges, &truth_class, &fp_class);
        rep.part_fpp_differs = !same_partition(&pedges, &truth_class, &fpp_class);

        // Walk without geometry: an edge's class is (obstacle, the
        // complement component of that obstacle its piece lies in),
        // classes joined where they share a vertex.
        let mut piece_cc: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        for c in 0..ncomp {
            let cc = complement_components(&g, &comp, c);
            let mut loops_with_pieces = BTreeSet::new();
            for i in 0..pieces.len() {
                if pedges.iter().any(|p| p.piece == i && p.far == Far::Obst(c)) {
                    // A cell of piece i.
                    let k = g.lab.iter().position(|&l| l == i as i32);
                    if let Some(k) = k {
                        piece_cc.insert((c, i), cc[k]);
                        loops_with_pieces.insert(cc[k]);
                    }
                }
            }
            if loops_with_pieces.len() >= 2 {
                rep.walk_split += 1;
            }
        }
        let mut classes: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        for pe in &pedges {
            if let Far::Obst(c) = pe.far {
                let k = (c, *piece_cc.get(&(c, pe.piece)).unwrap_or(&usize::MAX));
                let n = classes.len();
                classes.entry(k).or_insert(n);
            }
        }
        let mut uf: Vec<usize> = (0..classes.len()).collect();
        fn find(uf: &mut [usize], x: usize) -> usize {
            let mut x = x;
            while uf[x] != x {
                uf[x] = uf[uf[x]];
                x = uf[x];
            }
            x
        }
        let mut at_v: BTreeMap<VertexKey, usize> = BTreeMap::new();
        for pe in &pedges {
            if let Far::Obst(c) = pe.far {
                let k = classes[&(c, *piece_cc.get(&(c, pe.piece)).unwrap_or(&usize::MAX))];
                for v in pe.ends {
                    if let Some(&o) = at_v.get(&v) {
                        let (a, b) = (find(&mut uf, o), find(&mut uf, k));
                        uf[a.max(b)] = a.min(b);
                    } else {
                        at_v.insert(v, k);
                    }
                }
            }
        }
        let walk_of: BTreeMap<(usize, usize), usize> = classes
            .iter()
            .map(|(k, &i)| (*k, find(&mut uf, i)))
            .collect();
        let walk_class = |pe: &PEdge| match pe.far {
            Far::Obst(c) => walk_of
                .get(&(c, *piece_cc.get(&(c, pe.piece)).unwrap_or(&usize::MAX)))
                .copied(),
            _ => None,
        };
        let (bw, _) = borders(&walk_class);
        rep.borders_walk = bw;
        rep.part_walk_differs = !same_partition(&pedges, &truth_class, &walk_class);

        // Kernel record, chained.
        let log = editor_core::names::borders_probe_log::LOG
            .lock()
            .unwrap()
            .get(&node)
            .cloned();
        if let Some(log) = log {
            rep.rec_logged = true;
            rep.rec_paths = log.paths.join(",");
            for n in &log.notes {
                *NOTES.lock().unwrap().entry(n.clone()).or_default() += 1;
            }
            let ds: Vec<usize> = (0..log.discards.len())
                .filter(|&i| !log.discards[i].member_faces.is_disjoint(&mf))
                .collect();
            let mut uf: BTreeMap<usize, usize> = ds.iter().map(|&i| (i, i)).collect();
            fn root(uf: &mut BTreeMap<usize, usize>, mut x: usize) -> usize {
                while uf[&x] != x {
                    x = uf[&x];
                }
                x
            }
            let mut seam_of: BTreeMap<EdgeKey, Vec<usize>> = BTreeMap::new();
            for &i in &ds {
                rep.rec_unresolved += log.discards[i].unresolved;
                for &k in &log.discards[i].seams {
                    seam_of.entry(k).or_default().push(i);
                }
            }
            let join = |uf: &mut BTreeMap<usize, usize>, a: usize, b: usize| {
                let (ra, rb) = (root(uf, a), root(uf, b));
                uf.insert(ra.max(rb), ra.min(rb));
            };
            for &i in &ds {
                for k in &log.discards[i].touches {
                    for &j in seam_of.get(k).into_iter().flatten() {
                        join(&mut uf, i, j);
                    }
                }
            }
            for v in seam_of.values() {
                for w in v.windows(2) {
                    join(&mut uf, w[0], w[1]);
                }
            }
            // Dead ancestors' lineage, from every discard's chains.
            let mut dead_parent: BTreeMap<EdgeKey, EdgeKey> = BTreeMap::new();
            for d in &log.discards {
                for ch in &d.chains {
                    for w in ch.windows(2) {
                        dead_parent.insert(w[0], w[1]);
                    }
                }
            }
            let mut rec_of: BTreeMap<EdgeKey, Option<usize>> = BTreeMap::new();
            for pe in &pedges {
                let mut e = pe.edge;
                let mut found = None;
                for _ in 0..100_000 {
                    if let Some(v) = seam_of.get(&e) {
                        found = Some(root(&mut uf, v[0]));
                        break;
                    }
                    match body.edge_provenance_of(e) {
                        Some(topo::Provenance::SplitEdge { edge: p }) => e = *p,
                        Some(_) => break,
                        None => match dead_parent.get(&e) {
                            Some(&p) => e = p,
                            None => break,
                        },
                    }
                }
                if found.is_none() && matches!(pe.far, Far::Obst(_)) && std::env::var("BORDERS_DEBUG").is_ok() {
                    let mut chain = vec![pe.edge];
                    let mut e = pe.edge;
                    while let Some(topo::Provenance::SplitEdge { edge: p }) = body.edge_provenance_of(e) {
                        e = *p;
                        chain.push(e);
                    }
                    let prov = body.edge_provenance_of(e).map(|p| format!("{p:?}"));
                    let none = body.edges().filter(|(k, _)| body.edge_provenance_of(*k).is_none()).count();
                    println!("      edges without provenance: {none} of {}", body.edges().count());
                    println!(
                        "      MISSING piece {} wall {} chain {:?} rootprov {:?}",
                        pe.piece, wall_name(pe.wall), chain, prov.map(|s| s.chars().take(120).collect::<String>())
                    );
                    for &i in &ds {
                        let d = &log.discards[i];
                        println!("        d{i} step {} b={} seams {:?} touches {:?} mf {:?}", d.step, d.operand_b, d.seams, d.touches, d.member_faces.iter().map(|(m, n)| format!("{}:{}", m.0, short(n))).collect::<Vec<_>>());
                    }
                }
                rec_of.insert(pe.edge, found);
                match (found.is_some(), matches!(pe.far, Far::Obst(_))) {
                    (false, true) => rep.rec_missing += 1,
                    (true, false) => rep.rec_spurious += 1,
                    _ => {}
                }
            }
            let rec_class = |pe: &PEdge| rec_of.get(&pe.edge).copied().flatten();
            let (br, _) = borders(&rec_class);
            rep.borders_rec = br;
            rep.part_rec_differs = !same_partition(&pedges, &truth_class, &rec_class);
        }
        out.push(rep);
    }
    out
}

fn same_partition(
    pedges: &[PEdge],
    a: &dyn Fn(&PEdge) -> Option<usize>,
    b: &dyn Fn(&PEdge) -> Option<usize>,
) -> bool {
    let mut ab: BTreeMap<usize, usize> = BTreeMap::new();
    let mut ba: BTreeMap<usize, usize> = BTreeMap::new();
    for pe in pedges {
        match (a(pe), b(pe)) {
            (Some(x), Some(y)) => {
                if *ab.entry(x).or_insert(y) != y || *ba.entry(y).or_insert(x) != x {
                    return false;
                }
            }
            (None, None) => {}
            _ => return false,
        }
    }
    true
}

/// The 8-connected components of every cell not in obstacle `c`; cells
/// beyond the grid are one component with the border cells.
fn complement_components(g: &Grid, comp: &[usize], c: usize) -> Vec<usize> {
    let mut cc = vec![usize::MAX; g.lab.len()];
    let mut n = 0;
    let outside = 0;
    // Seed the outside from border cells.
    let mut q = VecDeque::new();
    for k in 0..g.lab.len() {
        if g.on_border(k) && comp[k] != c {
            cc[k] = outside;
            q.push_back(k);
        }
    }
    n += 1;
    let flood = |q: &mut VecDeque<usize>, cc: &mut Vec<usize>, id: usize| {
        while let Some(x) = q.pop_front() {
            for y in g.neighbours8(x) {
                if comp[y] != c && cc[y] == usize::MAX {
                    cc[y] = id;
                    q.push_back(y);
                }
            }
        }
    };
    flood(&mut q, &mut cc, outside);
    for k in 0..g.lab.len() {
        if comp[k] != c && cc[k] == usize::MAX {
            cc[k] = n;
            let mut q = VecDeque::from([k]);
            flood(&mut q, &mut cc, n);
            n += 1;
        }
    }
    cc
}

// ---------------------------------------------------------------- fixtures

pub(crate) type Bx = ((f64, f64), (f64, f64), (f64, f64));

/// One member of a constructed fixture.
#[derive(Clone)]
pub(crate) enum Mem {
    /// x, y, (z0, dz).
    Block(Bx),
    /// A prism: plan polygon loops at z0, extruded dz.
    Prism(Vec<Vec<(f64, f64)>>, f64, f64),
    /// A tube: centre, outer, inner radius, z0, dz.
    Tube((f64, f64), f64, f64, f64, f64),
    /// A profile in the xz plane at y = y1, extruded toward −y by dy.
    Side(Vec<(f64, f64)>, f64, f64),
}

fn add(doc: ProfileDoc, m: &Mem) -> (ProfileDoc, RecipeNodeId) {
    match m {
        Mem::Block((x, y, z)) => block(doc, *x, *y, z.0, z.1),
        Mem::Prism(loops, z0, dz) => {
            let (doc, p) = on_frame(doc, [0.0, 0.0, *z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], loops.clone());
            insert(doc, Node::Extrude { profile: p, distance: len(*dz) })
        }
        Mem::Tube((cx, cy), ro, ri, z0, dz) if *ri == 0.0 => {
            let (doc, plane) = insert(doc, crate::fixture::frame([0.0, 0.0, *z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
            let (doc, p) = insert(
                doc,
                Node::Profile(ProfileProgram {
                    plane,
                    loops: vec![LoopProgram::circle_split(*cx, *cy, *ro, 3, 0.0).unwrap()],
                    ids: Vec::new(),
                }),
            );
            insert(doc, Node::Extrude { profile: p, distance: len(*dz) })
        }
        Mem::Tube((cx, cy), ro, ri, z0, dz) => {
            let (doc, plane) = insert(doc, crate::fixture::frame([0.0, 0.0, *z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
            let (doc, p) = insert(
                doc,
                Node::Profile(ProfileProgram {
                    plane,
                    loops: vec![
                        LoopProgram::circle_split(*cx, *cy, *ro, 2, 0.0).unwrap(),
                        LoopProgram::circle_split(*cx, *cy, *ri, 3, 0.3).unwrap(),
                    ],
                    ids: Vec::new(),
                }),
            );
            insert(doc, Node::Extrude { profile: p, distance: len(*dz) })
        }
        Mem::Side(pts, y1, dy) => {
            let (doc, p) = on_frame(doc, [0.0, *y1, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], vec![pts.clone()]);
            insert(doc, Node::Extrude { profile: p, distance: len(*dy) })
        }
    }
}

pub(crate) struct Fixture {
    pub label: &'static str,
    pub members: Vec<Mem>,
    /// `Some(op)`: a pair boolean of members 0 and 1 in that order.
    pub pair: Option<BooleanOp>,
}

const PL: Bx = ((0.0, 3.0), (0.0, 2.0), (0.0, 1.0));
const PL3: Bx = ((0.0, 3.0), (0.0, 3.0), (0.0, 1.0));

pub(crate) fn fixtures() -> Vec<Fixture> {
    let u = |label, members| Fixture { label, members, pair: None };
    // Every feature's z0, top and y ends differ from every other
    // member's, so no two members touch flush (an undeclared flush
    // contact refuses).
    let slab = Mem::Block(((1.4, 1.6), (-1.0, 3.0), (0.5, 1.5)));
    let tube = Mem::Tube((1.5, 1.0), 0.6, 0.4, 0.5, 1.5);
    let ngon = |cx: f64, cy: f64, r: f64, n: usize, rev: bool| -> Vec<(f64, f64)> {
        let mut v: Vec<(f64, f64)> = (0..n)
            .map(|k| {
                let t = 2.0 * core::f64::consts::PI * (k as f64 + 0.5) / n as f64;
                (cx + r * t.cos(), cy + r * t.sin())
            })
            .collect();
        if rev {
            v.reverse();
        }
        v
    };
    let ptube = Mem::Prism(
        vec![ngon(1.5, 1.0, 0.6, 16, false), ngon(1.5, 1.0, 0.4, 16, true)],
        0.5,
        1.5,
    );
    let stair = vec![
        (1.0, -1.0),
        (1.2, -1.0),
        (1.2, 0.9),
        (1.7, 0.9),
        (1.7, 3.0),
        (1.5, 3.0),
        (1.5, 1.1),
        (1.0, 1.1),
    ];
    let ring = vec![
        vec![(1.0, 0.5), (2.0, 0.5), (2.0, 1.5), (1.0, 1.5)],
        vec![(1.2, 0.7), (1.2, 1.3), (1.8, 1.3), (1.8, 0.7)],
    ];
    let s1 = Mem::Block(((1.4, 1.6), (-1.0, 4.0), (0.5, 1.5)));
    let s2 = Mem::Block(((1.5, 3.5), (1.45, 1.55), (0.47, 1.23)));
    let r = Mem::Block(((0.4, 0.8), (1.45, 1.9), (0.44, 0.86)));
    let tall = Mem::Block(((1.4, 1.6), (-1.0, 3.0), (0.5, 2.5)));
    vec![
        u("tube_on_plate", vec![Mem::Block(PL), tube.clone()]),
        u("ptube_on_plate", vec![Mem::Block(PL), ptube.clone()]),
        u(
            "ptube_slab",
            vec![Mem::Block(PL), ptube.clone(), Mem::Block(((1.45, 1.55), (-1.0, 3.0), (0.47, 1.03)))],
        ),
        u(
            "ptube_tall_slab",
            vec![Mem::Block(PL), ptube.clone(), Mem::Block(((1.45, 1.55), (-1.0, 3.0), (0.47, 2.03)))],
        ),
        u(
            "tube_slab",
            vec![Mem::Block(PL), tube.clone(), Mem::Block(((1.45, 1.55), (-1.0, 3.0), (0.47, 1.03)))],
        ),
        u("ring_rib", vec![Mem::Block(PL), Mem::Prism(ring, 0.5, 1.5)]),
        u(
            "slab_boss",
            vec![Mem::Block(PL), slab.clone(), Mem::Block(((0.4, 0.8), (0.8, 1.2), (0.47, 1.03)))],
        ),
        u(
            "three_strips",
            vec![
                Mem::Block(PL),
                Mem::Block(((1.0, 1.3), (-1.0, 3.0), (0.5, 1.3))),
                Mem::Block(((1.2, 1.6), (-1.1, 3.1), (0.47, 1.53))),
                Mem::Block(((1.5, 1.8), (-1.2, 3.2), (0.44, 1.16))),
            ],
        ),
        u("aligned_boss", vec![Mem::Block(PL3), s1.clone(), s2.clone(), r]),
        u("t_junction", vec![Mem::Block(PL3), s1, s2]),
        u("staircase", vec![Mem::Block(PL), Mem::Prism(vec![stair], 0.5, 1.5)]),
        u(
            "staircase3",
            vec![
                Mem::Block(PL),
                Mem::Block(((1.0, 1.2), (-1.0, 1.05), (0.5, 1.5))),
                Mem::Block(((1.05, 1.65), (0.9, 1.1), (0.47, 1.33))),
                Mem::Block(((1.5, 1.7), (0.95, 3.0), (0.44, 1.16))),
            ],
        ),
        u(
            "overhang",
            vec![
                Mem::Block(PL),
                tall.clone(),
                Mem::Side(
                    vec![(0.4, 0.47), (0.8, 0.47), (0.8, 2.2), (1.5, 2.2), (1.5, 2.45), (0.4, 2.45)],
                    1.2,
                    0.4,
                ),
            ],
        ),
        u(
            "overhang4",
            vec![
                Mem::Block(PL),
                tall.clone(),
                Mem::Block(((0.4, 0.8), (0.8, 1.2), (0.47, 2.03))),
                Mem::Block(((0.5, 1.5), (0.85, 1.15), (2.2, 0.25))),
            ],
        ),
        u(
            "arch_over_slab",
            vec![
                Mem::Block(PL),
                slab.clone(),
                Mem::Side(
                    vec![
                        (0.8, 0.47),
                        (1.0, 0.47),
                        (1.0, 2.1),
                        (2.0, 2.1),
                        (2.0, 0.47),
                        (2.2, 0.47),
                        (2.2, 2.4),
                        (0.8, 2.4),
                    ],
                    1.2,
                    0.4,
                ),
            ],
        ),
        u(
            "cup_on_plate",
            vec![Mem::Block(PL), ptube.clone(), Mem::Block(((0.8, 2.2), (0.3, 1.7), (1.9, 0.3)))],
        ),
        u(
            "pinch",
            vec![
                Mem::Block(PL),
                Mem::Block(((1.0, 1.5), (-1.0, 1.0), (0.5, 1.5))),
                Mem::Block(((1.5, 2.0), (1.0, 3.0), (0.47, 1.23))),
            ],
        ),
        Fixture {
            label: "sub_through_slot",
            members: vec![Mem::Block(PL), Mem::Block(((1.4, 1.6), (-1.0, 3.0), (-1.0, 3.0)))],
            pair: Some(BooleanOp::Subtract),
        },
        Fixture {
            label: "sub_blind_slot",
            members: vec![Mem::Block(PL), Mem::Block(((1.4, 1.6), (-1.0, 3.0), (0.5, 3.0)))],
            pair: Some(BooleanOp::Subtract),
        },
        Fixture {
            label: "sub_two_through_slots",
            members: vec![
                Mem::Block(PL),
                Mem::Prism(
                    vec![vec![(1.0, -1.0), (1.2, -1.0), (1.2, 3.0), (1.0, 3.0)]],
                    -1.0,
                    3.0,
                ),
            ],
            pair: Some(BooleanOp::Subtract),
        },
        Fixture {
            label: "sub_through_pring_groove",
            members: vec![
                Mem::Block(PL),
                Mem::Prism(vec![ngon(1.5, 1.0, 0.6, 16, false), ngon(1.5, 1.0, 0.4, 16, true)], -1.0, 3.0),
            ],
            pair: Some(BooleanOp::Subtract),
        },
        Fixture {
            label: "sub_blind_pring_groove",
            members: vec![
                Mem::Block(PL),
                Mem::Prism(vec![ngon(1.5, 1.0, 0.6, 16, false), ngon(1.5, 1.0, 0.4, 16, true)], 0.5, 3.0),
            ],
            pair: Some(BooleanOp::Subtract),
        },
        Fixture {
            label: "pair_union_round_tube",
            members: vec![Mem::Block(PL), tube.clone()],
            pair: Some(BooleanOp::Union),
        },
        u("round_boss_slab", vec![Mem::Block(PL), Mem::Tube((1.5, 1.0), 0.6, 0.0, 0.44, 1.8), slab.clone()]),
        Fixture {
            label: "pair_union_slab",
            members: vec![Mem::Block(PL), slab],
            pair: Some(BooleanOp::Union),
        },
        Fixture {
            label: "pair_union_ptube",
            members: vec![Mem::Block(PL), ptube],
            pair: Some(BooleanOp::Union),
        },
    ]
}

/// Every run of a fixture: `(order label, evaluation, node to read, ab)`.
pub(crate) fn fixture_runs(
    fx: &Fixture,
    mut each: impl FnMut(&str, &editor_core::Evaluation<f64>, RecipeNodeId, Option<(RecipeNodeId, RecipeNodeId)>),
) {
    let mut doc = ProfileDoc::empty_derived("borders_probe", Tol::witness());
    let mut ids = Vec::new();
    for m in &fx.members {
        let (d, id) = add(doc, m);
        doc = d;
        ids.push(id);
    }
    if let Some(op) = fx.pair {
        let (d, n) = insert(
            doc,
            Node::Boolean {
                op,
                a: ids[0],
                b: ids[1],
                declare: None,
            },
        );
        let ev = run(&d);
        each("[0,1]", &ev, n, Some((ids[0], ids[1])));
        return;
    }
    for order in permutations(&(0..ids.len()).collect::<Vec<_>>()) {
        let members = order.iter().map(|&i| ids[i]).collect();
        let (d, n) = insert(
            doc.clone(),
            Node::Union {
                members,
                declare: None,
            },
        );
        let ev = run(&d);
        each(&format!("{order:?}"), &ev, n, None);
    }
}

// ---------------------------------------------------------------- the row

#[derive(Default)]
struct Tally {
    runs: usize,
    refused: usize,
    groups: usize,
    island_groups: usize,
    pinch_groups: usize,
    pinchv_groups: usize,
    walk_split_groups: usize,
    walk_diff_groups: usize,
    walk_borders_diff: usize,
    fp_part_diff: usize,
    fp_borders_diff: usize,
    fpp_part_diff: usize,
    fpp_borders_diff: usize,
    rim_conf: usize,
    excl_walls: usize,
    unknown: usize,
    unsupported: usize,
    order_dependent_truth: usize,
    fp_nodiv: usize,
    rec_groups: usize,
    rec_part_diff: usize,
    rec_borders_diff: usize,
    rec_missing: usize,
    rec_spurious: usize,
    rec_unresolved: usize,
    order_dependent_rec: usize,
}

fn report_case(
    label: &str,
    runs: &mut Vec<(String, Result<Vec<GroupReport>, String>)>,
    total: &mut Tally,
    detail: bool,
) {
    let mut t = Tally::default();
    let mut sigs: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut rsigs: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (order, r) in runs.iter() {
        t.runs += 1;
        let Ok(groups) = r else {
            t.refused += 1;
            continue;
        };
        if let Some(g) = groups.first() {
            for p in g.rec_paths.split(',') {
                *PATHS.lock().unwrap().entry(if p.is_empty() { "none".to_string() } else { p.to_string() }).or_default() += 1;
            }
        }
        let mut sig = Vec::new();
        let mut rsig = Vec::new();
        for g in groups {
            t.groups += 1;
            if !g.note.is_empty() {
                t.unsupported += 1;
                if detail {
                    println!("  {label} {order} {}: {}", g.parent, g.note);
                }
                continue;
            }
            sig.push(format!("{}={:?}", g.parent, g.borders_truth));
            rsig.push(format!("{}={:?}", g.parent, g.borders_rec));
            if g.rec_logged {
                t.rec_groups += 1;
                t.rec_part_diff += usize::from(g.part_rec_differs);
                t.rec_borders_diff += usize::from(g.borders_rec != g.borders_truth);
                t.rec_missing += usize::from(g.rec_missing > 0);
                t.rec_spurious += usize::from(g.rec_spurious > 0);
                t.rec_unresolved += usize::from(g.rec_unresolved > 0);
            }
            t.island_groups += usize::from(g.islands > 0);
            t.pinch_groups += usize::from(g.pinch_pieces > 0);
            t.pinchv_groups += usize::from(g.pinch_obstacles > 0);
            t.walk_split_groups += usize::from(g.walk_split > 0);
            t.walk_diff_groups += usize::from(g.part_walk_differs);
            t.walk_borders_diff += usize::from(g.borders_walk != g.borders_truth);
            t.fp_part_diff += usize::from(g.part_fp_differs);
            t.fp_borders_diff += usize::from(g.borders_fp != g.borders_truth);
            t.fp_nodiv += usize::from(
                g.borders_fp.iter().all(|b| b.is_empty()) && g.borders_truth.iter().any(|b| !b.is_empty()),
            );
            t.fpp_part_diff += usize::from(g.part_fpp_differs);
            t.fpp_borders_diff += usize::from(g.borders_fpp != g.borders_truth);
            t.rim_conf += usize::from(g.fp_rim_confusions > 0);
            t.excl_walls += usize::from(g.fp_excluded_walls > 0);
            t.unknown += usize::from(g.unknown_edges > 0);
            let odd = g.part_rec_differs
                || g.borders_rec != g.borders_truth
                || g.islands > 0
                || g.pinch_pieces > 0
                || g.pinch_obstacles > 0
                || g.part_walk_differs
                || g.part_fp_differs
                || g.borders_fp != g.borders_truth
                || g.fp_rim_confusions > 0
                || g.fp_excluded_walls > 0
                || g.unknown_edges > 0;
            if detail && (odd || std::env::var("BORDERS_ALL").is_ok()) {
                println!(
                    "  {label} {order} {} pieces={} obst={} div={} isl={} pinchP={} pinchV={} walkSplit={} walkDiff={} fpDiff={} fppDiff={} rimConf={} exclW={} unk={}",
                    g.parent, g.pieces, g.obstacles, g.dividers, g.islands, g.pinch_pieces,
                    g.pinch_obstacles, g.walk_split, g.part_walk_differs, g.part_fp_differs,
                    g.part_fpp_differs, g.fp_rim_confusions, g.fp_excluded_walls, g.unknown_edges
                );
                println!("      truth {:?}", g.borders_truth);
                if g.borders_fp != g.borders_truth {
                    println!("      fp    {:?}", g.borders_fp);
                }
                if g.borders_fpp != g.borders_truth {
                    println!("      fpp   {:?}", g.borders_fpp);
                }
                if g.borders_walk != g.borders_truth {
                    println!("      walk  {:?}", g.borders_walk);
                }
                if g.rec_logged && (g.borders_rec != g.borders_truth || g.part_rec_differs) {
                    println!(
                        "      rec   {:?} partDiff={} missing={} spurious={} unresolved={} paths={}",
                        g.borders_rec, g.part_rec_differs, g.rec_missing, g.rec_spurious,
                        g.rec_unresolved, g.rec_paths
                    );
                }
            }
        }
        sig.sort();
        sigs.insert(order.clone(), sig);
        rsig.sort();
        rsigs.insert(order.clone(), rsig);
    }
    let mut by_tag: BTreeMap<String, BTreeSet<&Vec<String>>> = BTreeMap::new();
    for (o, s) in &sigs {
        let tag = o.rsplit(' ').next().unwrap_or("").to_string();
        by_tag.entry(tag).or_default().insert(s);
    }
    let distinct: BTreeSet<&Vec<String>> = by_tag
        .values()
        .filter(|v| v.len() > 1)
        .flat_map(|v| v.iter().copied())
        .collect();
    t.order_dependent_truth = usize::from(distinct.len() > 1);
    let mut rby_tag: BTreeMap<String, BTreeSet<&Vec<String>>> = BTreeMap::new();
    for (o, s) in &rsigs {
        let tag = o.rsplit(' ').next().unwrap_or("").to_string();
        rby_tag.entry(tag).or_default().insert(s);
    }
    t.order_dependent_rec = usize::from(rby_tag.values().any(|v| v.len() > 1));
    if detail && t.order_dependent_rec > 0 {
        println!("  {label}: RECORD Borders differ across orders");
        for (o, s) in &rsigs {
            println!("    {o}: {s:?}");
        }
    }
    if detail && distinct.len() > 1 {
        println!("  {label}: truth Borders differ across orders ({} variants)", distinct.len());
        for (o, s) in &sigs {
            println!("    {o}: {s:?}");
        }
    }
    println!(
        "ROW {label:24} runs={:3} refused={:3} groups={:4} island={:3} pinchP={:3} pinchV={:3} walkSplit={:3} walkPartDiff={:3} walkBordDiff={:3} fpPartDiff={:3} fpBordDiff={:3} fppPartDiff={:3} fppBordDiff={:3} rimConf={:3} exclW={:3} unk={:3} unsup={:3} orderDep={} fpNoDiv={:3} | rec groups={:3} partDiff={:3} bordDiff={:3} missing={:3} spurious={:3} unres={:3} orderDep={}",
        t.runs, t.refused, t.groups, t.island_groups, t.pinch_groups, t.pinchv_groups,
        t.walk_split_groups, t.walk_diff_groups, t.walk_borders_diff, t.fp_part_diff,
        t.fp_borders_diff, t.fpp_part_diff, t.fpp_borders_diff, t.rim_conf, t.excl_walls,
        t.unknown, t.unsupported, t.order_dependent_truth, t.fp_nodiv, t.rec_groups, t.rec_part_diff,
        t.rec_borders_diff, t.rec_missing, t.rec_spurious, t.rec_unresolved, t.order_dependent_rec
    );
    total.runs += t.runs;
    total.fp_nodiv += t.fp_nodiv;
    total.rec_groups += t.rec_groups;
    total.rec_part_diff += t.rec_part_diff;
    total.rec_borders_diff += t.rec_borders_diff;
    total.rec_missing += t.rec_missing;
    total.rec_spurious += t.rec_spurious;
    total.rec_unresolved += t.rec_unresolved;
    total.order_dependent_rec += t.order_dependent_rec;
    total.refused += t.refused;
    total.groups += t.groups;
    total.island_groups += t.island_groups;
    total.pinch_groups += t.pinch_groups;
    total.pinchv_groups += t.pinchv_groups;
    total.walk_split_groups += t.walk_split_groups;
    total.walk_diff_groups += t.walk_diff_groups;
    total.walk_borders_diff += t.walk_borders_diff;
    total.fp_part_diff += t.fp_part_diff;
    total.fp_borders_diff += t.fp_borders_diff;
    total.fpp_part_diff += t.fpp_part_diff;
    total.fpp_borders_diff += t.fpp_borders_diff;
    total.rim_conf += t.rim_conf;
    total.excl_walls += t.excl_walls;
    total.unknown += t.unknown;
    total.unsupported += t.unsupported;
    total.order_dependent_truth += t.order_dependent_truth;
}

fn detail() -> bool {
    std::env::var("BORDERS_DETAIL").is_ok()
}

fn only() -> Option<String> {
    std::env::var("BORDERS_ONLY").ok()
}

#[test]
#[ignore = "measurement probe"]
fn borders_probe_constructed() {
    let mut total = Tally::default();
    for fx in fixtures() {
        if only().is_some_and(|o| !fx.label.contains(&o)) {
            continue;
        }
        let mut runs = Vec::new();
        fixture_runs(&fx, |order, ev, node, ab| {
            let r = match failure(ev, node) {
                Some(e) => Err(format!("{e:?}")),
                None => std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| probe_body(ev, node, ab)))
                    .map_err(|_| "PROBE PANIC (tessellation or raster)".to_string()),
            };
            if let Err(e) = &r {
                if detail() {
                    println!("  {} {order} refused: {}", fx.label, &e[..e.len().min(160)]);
                }
            }
            runs.push((order.to_string(), r));
        });
        report_case(fx.label, &mut runs, &mut total, detail());
    }
    print_total("constructed", &total);
}

#[test]
#[ignore = "measurement probe"]
fn borders_probe_corpus() {
    use crate::emit_union_rim_piece_ranks::{cases, runs};
    let mut total = Tally::default();
    for case in cases() {
        if only().is_some_and(|o| !case.label.contains(&o)) {
            continue;
        }
        let mut rs = Vec::new();
        runs(&case, |at, ev, _, unions| {
            for &(tag, union) in unions {
                let r = match failure(ev, union) {
                    Some(e) => Err(format!("{e:?}")),
                    None => Ok(probe_body(ev, union, None)),
                };
                if let Err(e) = &r {
                    if detail() {
                        println!("  {} {at} {tag} refused: {}", case.label, &e[..e.len().min(200)]);
                    }
                }
                rs.push((format!("{at} {tag}"), r));
            }
        });
        report_case(&case.label, &mut rs, &mut total, detail());
    }
    print_total("corpus", &total);
}

static NOTES: std::sync::Mutex<BTreeMap<String, usize>> = std::sync::Mutex::new(BTreeMap::new());
static PATHS: std::sync::Mutex<BTreeMap<String, usize>> = std::sync::Mutex::new(BTreeMap::new());

fn print_total(what: &str, t: &Tally) {
    println!("STEP PATHS (runs with a split group) {what}: {:?}", PATHS.lock().unwrap());
    println!("RECORD NOTES (per group read) {what}: {:?}", NOTES.lock().unwrap());
    println!(
        "TOTAL {what}: runs={} refused={} groups={} island={} pinchP={} pinchV={} walkSplit={} walkPartDiff={} walkBordDiff={} fpPartDiff={} fpBordDiff={} fppPartDiff={} fppBordDiff={} rimConf={} exclW={} unk={} unsup={} orderDepCases={} fpNoDiv={} | rec groups={} partDiff={} bordDiff={} missing={} spurious={} unres={} orderDepCases={}",
        t.runs, t.refused, t.groups, t.island_groups, t.pinch_groups, t.pinchv_groups,
        t.walk_split_groups, t.walk_diff_groups, t.walk_borders_diff, t.fp_part_diff,
        t.fp_borders_diff, t.fpp_part_diff, t.fpp_borders_diff, t.rim_conf, t.excl_walls,
        t.unknown, t.unsupported, t.order_dependent_truth, t.fp_nodiv, t.rec_groups, t.rec_part_diff,
        t.rec_borders_diff, t.rec_missing, t.rec_spurious, t.rec_unresolved, t.order_dependent_rec
    );
}
