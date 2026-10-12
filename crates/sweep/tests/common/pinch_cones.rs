//! **The cones of a boolean's boundary at a point**, read without the
//! kernel from the operands' convex pieces, and the vertices a built
//! body holds there: a pinch is one vertex per cone (Ev, PR 4057), so a
//! pinch row compares the two.
//!
//! The count is exact for planar pieces: each piece plane through the
//! point is a great circle of the point's link (its sphere of
//! directions), the circles cut the sphere into cells, and a cell is in
//! the result where the op of the two operands' links holds there. The
//! result's in-cells and out-cells fall into connected components, and
//! the cones are `in + out − 1`, the result's boundary cycles round the
//! point. A cone is *free* where the boundary passes the point without
//! a corner, and needs no vertex there: bounded by one great circle
//! alone (a plane through the point), or a lune, bounded by two
//! half-circles from one direction to its opposite (a straight edge
//! through the point, the shape the output stage's join leaves no
//! vertex on: `boolean::edge_join`).
//!
//! **Deliberately not absorbed**, and the whole of it:
//! [`super::differential`]'s polygon oracles and `outcome` line, which
//! meter volumes and say nothing at a point; and the pose geometry
//! (cubes, frames, clipped volumes) of `join_pierce_runs_sweep.rs` and
//! `join_pierce_strut_facing.rs`, each one suite's.

use topo::{Body, LoopBoundary, VertexKey};

/// A point direction or a plane normal.
pub type V3 = [f64; 3];

/// A half-space `n · x ≤ d`.
pub type Plane = (V3, f64);

/// A solid as convex pieces, each an intersection of half-spaces, the
/// pieces meeting only on their boundaries.
pub type Pieces = Vec<Vec<Plane>>;

/// The boolean whose cones are counted.
#[derive(Clone, Copy, Debug)]
pub enum Op {
    /// `x ∪ y`.
    Union,
    /// `x ∩ y`.
    Intersect,
    /// `x ∖ y`.
    Subtract,
}

fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn scale(a: V3, s: f64) -> V3 {
    a.map(|c| c * s)
}

fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn unit(a: V3) -> V3 {
    scale(a, 1.0 / dot(a, a).sqrt())
}

/// Two unit vectors completing `m` to a right-handed frame.
fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}

/// A solid's link at `p`: for each piece holding `p`, the unit normals
/// of its planes through `p`. A direction is in the piece's link where
/// it runs strictly inside every one of them.
fn link(p: V3, pieces: &[Vec<Plane>]) -> Vec<Vec<V3>> {
    pieces
        .iter()
        .filter_map(|q| {
            let mut through = Vec::new();
            for &(n, d) in q {
                let s = (dot(n, p) - d) / dot(n, n).sqrt();
                if s > 1e-9 {
                    return None;
                }
                if s.abs() <= 1e-9 {
                    through.push(unit(n));
                }
            }
            Some(through)
        })
        .collect()
}

fn in_link(link: &[Vec<V3>], s: V3) -> bool {
    link.iter().any(|q| q.iter().all(|&n| dot(n, s) < 0.0))
}

/// The cones of `op(x, y)`'s boundary at `p`, and how many of them are
/// free (module docs). `Err` where the count is undefined at this
/// resolution: two piece planes through `p` within 1e-7 of each other
/// but not the same, a sample on a circle, a cell too small to sample,
/// or an arrangement vertex the result's boundary passes more than once.
pub fn cones_at(
    p: V3,
    x: &[Vec<Plane>],
    y: &[Vec<Plane>],
    op: Op,
) -> Result<(usize, usize), &'static str> {
    let (lx, ly) = (link(p, x), link(p, y));
    let inside = |s: V3| {
        let (a, b) = (in_link(&lx, s), in_link(&ly, s));
        match op {
            Op::Union => a || b,
            Op::Intersect => a && b,
            Op::Subtract => a && !b,
        }
    };
    let mut circles: Vec<V3> = Vec::new();
    for &n in lx.iter().chain(&ly).flatten() {
        let mut same = false;
        for &c in &circles {
            let off = dot(cross(c, n), cross(c, n)).sqrt();
            if off < 1e-12 {
                same = true;
            } else if off < 1e-7 {
                return Err("two circles nearly coincide");
            }
        }
        if !same {
            circles.push(n);
        }
    }
    // A cell is named by which side of every circle it lies on.
    let side = |s: V3| -> Option<Vec<bool>> {
        circles
            .iter()
            .map(|&c| {
                let v = dot(c, s);
                (v.abs() >= 1e-14).then_some(v > 0.0)
            })
            .collect()
    };
    let mut cells: Vec<(Vec<bool>, bool)> = Vec::new();
    let meet = |s: V3, cells: &mut Vec<(Vec<bool>, bool)>| -> Result<bool, &'static str> {
        let sv = side(s).ok_or("a sample on a circle")?;
        let held = inside(s);
        if !cells.iter().any(|c| c.0 == sv) {
            cells.push((sv, held));
        }
        Ok(held)
    };
    match circles.len() {
        0 => return Ok((0, 0)),
        1 => {
            meet(circles[0], &mut cells)?;
            meet(scale(circles[0], -1.0), &mut cells)?;
        }
        _ => {
            // Every cell has an arrangement vertex on its boundary:
            // sample round each vertex, just off it.
            for i in 0..circles.len() {
                for j in i + 1..circles.len() {
                    for sense in [1.0, -1.0] {
                        let v = scale(unit(cross(circles[i], circles[j])), sense);
                        let (e1, e2) = basis(v);
                        let mut angles: Vec<f64> = Vec::new();
                        for &c in &circles {
                            if dot(c, v).abs() < 1e-12 {
                                let t = unit(cross(c, v));
                                for tt in [t, scale(t, -1.0)] {
                                    angles.push(dot(tt, e2).atan2(dot(tt, e1)));
                                }
                            }
                        }
                        angles.sort_by(f64::total_cmp);
                        let dirs: Vec<V3> = (0..angles.len())
                            .map(|k| {
                                let a1 = angles
                                    .get(k + 1)
                                    .copied()
                                    .unwrap_or(angles[0] + std::f64::consts::TAU);
                                let mid = 0.5 * (angles[k] + a1);
                                add(scale(e1, mid.cos()), scale(e2, mid.sin()))
                            })
                            .collect();
                        let samples = round_vertex(v, &dirs, &circles)?;
                        let mut held = Vec::new();
                        for s in samples {
                            held.push(meet(s, &mut cells)?);
                        }
                        let turns = (0..held.len())
                            .filter(|&k| held[k] != held[(k + 1) % held.len()])
                            .count();
                        if turns > 2 {
                            return Err("the boundary passes an arrangement vertex twice");
                        }
                    }
                }
            }
        }
    }
    // Cells across one circle with the same verdict are one region.
    let n = cells.len();
    let mut root: Vec<usize> = (0..n).collect();
    fn find(root: &mut [usize], mut i: usize) -> usize {
        while root[i] != i {
            root[i] = root[root[i]];
            i = root[i];
        }
        i
    }
    for i in 0..n {
        for j in i + 1..n {
            let differ = cells[i]
                .0
                .iter()
                .zip(&cells[j].0)
                .filter(|(a, b)| a != b)
                .count();
            if cells[i].1 == cells[j].1 && differ == 1 {
                let (a, b) = (find(&mut root, i), find(&mut root, j));
                root[a] = b;
            }
        }
    }
    let mut regions = [
        std::collections::BTreeSet::new(),
        std::collections::BTreeSet::new(),
    ];
    for i in 0..n {
        let r = find(&mut root, i);
        regions[usize::from(cells[i].1)].insert(r);
    }
    let (held, open) = (regions[1].len(), regions[0].len());
    if held == 0 || open == 0 {
        return Ok((0, 0));
    }
    // A free cone: one region is one circle's hemisphere (a plane through
    // `p`), or two circles' common quarter of the sphere (a lune, whose
    // boundary is a straight edge through `p`).
    let mut members: std::collections::BTreeMap<usize, std::collections::BTreeSet<usize>> =
        Default::default();
    for i in 0..n {
        let r = find(&mut root, i);
        members.entry(r).or_default().insert(i);
    }
    let is_region = |cut: &dyn Fn(&[bool]) -> bool| {
        let part: std::collections::BTreeSet<usize> =
            (0..n).filter(|&i| cut(&cells[i].0)).collect();
        members.values().any(|m| *m == part)
    };
    let m = circles.len();
    let halves = (0..m)
        .filter(|&c| [true, false].into_iter().any(|s| is_region(&|v| v[c] == s)))
        .count();
    let lunes = (0..m)
        .flat_map(|c| (c + 1..m).map(move |d| (c, d)))
        .flat_map(|(c, d)| {
            [(true, true), (true, false), (false, true), (false, false)].map(|s| (c, d, s))
        })
        .filter(|&(c, d, (s, t))| is_region(&|v| v[c] == s && v[d] == t))
        .count();
    Ok((held + open - 1, halves + lunes))
}

/// One sample in each cell round the arrangement vertex `v`, along
/// `dirs`: each sample is on `v`'s side of every circle missing `v`. A
/// great-circle arc shorter than a half-turn crosses another great circle
/// at most once and a circle through its start not at all, so such a
/// sample lies in the cell its direction opens into at `v`. The step
/// shrinks until that holds, so a cell narrower than any fixed step is
/// still sampled.
fn round_vertex(v: V3, dirs: &[V3], circles: &[V3]) -> Result<Vec<V3>, &'static str> {
    let off: Vec<(V3, bool)> = circles
        .iter()
        .filter(|&&c| dot(c, v).abs() >= 1e-12)
        .map(|&c| (c, dot(c, v) > 0.0))
        .collect();
    let mut step = 1e-5;
    while step > 1e-13 {
        let samples: Vec<V3> = dirs.iter().map(|&d| unit(add(v, scale(d, step)))).collect();
        if samples
            .iter()
            .all(|&s| off.iter().all(|&(c, side)| (dot(c, s) > 0.0) == side))
        {
            return Ok(samples);
        }
        step /= 8.0;
    }
    Err("a cell round an arrangement vertex is too small to sample")
}

/// The vertices of `body` at exactly `at`.
pub fn vertices_at(body: &Body<f64>, at: V3) -> Vec<VertexKey> {
    body.vertex_points()
        .filter(|(_, p)| [p.x, p.y, p.z] == at)
        .map(|(k, _)| k)
        .collect()
}

/// How many faces of `body` run through two distinct vertices at `at`.
pub fn faces_through_two_vertices_at(body: &Body<f64>, at: V3) -> usize {
    let at_v = vertices_at(body, at);
    body.faces()
        .filter(|(_, f)| {
            let mut met = Vec::new();
            for &l in std::iter::once(&f.outer).chain(&f.rings) {
                if let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                    for he in body.loop_cycle(first).unwrap() {
                        let v = body.get_half_edge(he).unwrap().start;
                        if at_v.contains(&v) && !met.contains(&v) {
                            met.push(v);
                        }
                    }
                }
            }
            met.len() > 1
        })
        .count()
}

/// Where `body` breaks one vertex per cone at `at`, the finding, else
/// `None`: it holds as many vertices there as `op(x, y)` has cones, a
/// free cone (module docs) free to hold none.
pub fn cone_finding(
    body: &Body<f64>,
    at: V3,
    (x, y, op): (&[Vec<Plane>], &[Vec<Plane>], Op),
) -> Option<String> {
    let held = vertices_at(body, at).len();
    match cones_at(at, x, y, op) {
        Ok((c, free)) if (c - free..=c).contains(&held) => None,
        Ok((c, free)) => Some(format!(
            "{held} vertices at {at:?} for {c} cones ({free} free)"
        )),
        Err(why) => Some(format!("the cones at {at:?} are not counted: {why}")),
    }
}

/// Where `body` does not hold a pinch at `at` whose vertices share one
/// point key, the finding, else `None`: fewer than two vertices there is
/// a finding too, so a pinch row cannot pass on a point it lost.
/// [`shared_point_finding`] is the reading for a point that may hold one.
pub fn point_key_finding(body: &Body<f64>, at: V3) -> Option<String> {
    let at_v = vertices_at(body, at);
    if at_v.len() < 2 {
        return Some(format!(
            "{} vertices at {at:?}: no pinch to share a key",
            at_v.len()
        ));
    }
    shared_point_finding(body, at)
}

/// Where `body`'s vertices at `at`, if several, do not all share one
/// point key, the finding, else `None`. A point holding one vertex or
/// none passes: [`cone_finding`] counts them.
pub fn shared_point_finding(body: &Body<f64>, at: V3) -> Option<String> {
    let at_v = vertices_at(body, at);
    let point = |k| body.get_vertex(k).unwrap().point;
    at_v.iter()
        .any(|&k| point(k) != point(at_v[0]))
        .then(|| format!("the vertices at {at:?} do not share one point: {at_v:?}"))
}

/// Where a class `zip::share_points` rebound on this thread since the
/// last drain held keys at different points, the finding, else `None`;
/// with the number of classes drained. The rebind reads no position, so
/// this pins its premise: the seams tie only keys holding one point,
/// bit for bit.
pub fn shared_point_spread_finding() -> (usize, Option<String>) {
    let classes = topo::take_shared_points();
    let bits = |p: &[String; 3]| p.clone().map(|c| c.parse::<f64>().unwrap().to_bits());
    let finding = classes
        .iter()
        .find(|c| c.iter().any(|p| bits(p) != bits(&c[0])))
        .map(|c| format!("a rebound class holds keys at different points: {c:?}"));
    (classes.len(), finding)
}
