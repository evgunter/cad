//! **Where the patches join**: a side that more than one patch has as an
//! inside edge, and its split.
//!
//! Two faces that share two boundary chains can each make, as an inside
//! edge of its own triangulation, the same straight side between a point
//! of one chain and a point of the other; the joined mesh then has an edge
//! of four triangles. No lane can see that alone, and no lane reads
//! anything about the faces across its edges, so it is answered here, the
//! one place every patch is in hand.
//!
//! - **The patch keeps its chart corners.** A lane notes, for each of
//!   its triangles with two shared corners, the chart point of every
//!   corner and which are a singular point ([`Book::note`]). A patch is a
//!   function of its face and δ, these included.
//! - **The census is the join's** ([`census`]): on shared ids, before any
//!   patch is placed. A side used past twice per chord carrying it is
//!   split once in every patch that has it inside, and a face whose
//!   boundary chord carries it keeps it. Nothing is meshed again, no patch
//!   is chosen and nothing reads storage order.
//! - **The split is the face's** ([`split`]): at the point of its own
//!   surface over the side's midpoint in its chart, a pure function of the
//!   face, δ and the side. Every piece it makes is held as any triangle of
//!   its face: on the face's own certificate and, where its lane reads one,
//!   its fold, typed as the lane's own refusal; and to the winding read
//!   where the face joins the mesh (`[mesh.winding]`).

use std::collections::HashMap;

use geom_core::Point3;
use topo::{Body, FaceKey};

use crate::cert;
use crate::chart_table::ChartRow;
use crate::nurbs_cert::FaceBounds;
use crate::sizing::Eps;
use crate::tessellate::PatchVertex;
use crate::types::TessellateError;

/// A side between two shared mesh ids, the lesser first.
pub(crate) type Side = (u32, u32);

/// The side between `a` and `b`.
pub(crate) fn side(a: u32, b: u32) -> Side {
    (a.min(b), a.max(b))
}

/// **How a patch splits a side the join finds it has inside.**
#[derive(Clone, Debug)]
pub(crate) enum Splits {
    /// A planar patch: its chart is affine and its points are the shared
    /// ones themselves, so the point over a side's chart midpoint is its
    /// two ends' midpoint, and the plane has no certificate to read.
    Midpoint,
    /// A curved patch: the chart corners of its triangles with two shared
    /// corners, and whether its lane reads folds.
    Chart { noted: Vec<Noted>, folds: bool },
}

impl Splits {
    /// The heap the notes take.
    pub(crate) fn bytes(&self) -> usize {
        match self {
            Self::Midpoint => 0,
            Self::Chart { noted, .. } => noted.len() * core::mem::size_of::<Noted>(),
        }
    }
}

/// One noted triangle: its index in the patch, its corners' chart points
/// and which are a singular point, in the patch's winding.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Noted {
    tri: u32,
    uv: [[f64; 2]; 3],
    singular: [bool; 3],
}

/// What a lane notes while it emits.
#[derive(Default)]
pub(crate) struct Book {
    noted: Vec<Noted>,
}

impl Book {
    /// Notes the patch's triangle at index `tri`, as the patch holds it,
    /// if two of its corners are shared.
    pub(crate) fn note(
        &mut self,
        tri: usize,
        ids: [PatchVertex; 3],
        uv: [[f64; 2]; 3],
        singular: [bool; 3],
    ) {
        let shared = ids
            .iter()
            .filter(|v| matches!(v, PatchVertex::Shared(_)))
            .count();
        if shared >= 2 {
            #[allow(clippy::cast_possible_truncation)]
            self.noted.push(Noted {
                tri: tri as u32,
                uv,
                singular,
            });
        }
    }

    /// The patch's splits; `folds` where its lane reads folds. Only a
    /// triangle on a side between two shared ids that two of the patch's
    /// triangles have can be split (a side one triangle has is a chord of
    /// the face's own boundary), so only those are kept.
    pub(crate) fn splits(mut self, folds: bool, ids: &[[PatchVertex; 3]]) -> Splits {
        let sides = |tri: u32| {
            let t = ids[tri as usize];
            (0..3).filter_map(move |q| match (t[q], t[(q + 1) % 3]) {
                (PatchVertex::Shared(a), PatchVertex::Shared(b)) => Some(side(a, b)),
                _ => None,
            })
        };
        let mut uses: HashMap<Side, usize> = HashMap::new();
        for n in &self.noted {
            for s in sides(n.tri) {
                *uses.entry(s).or_default() += 1;
            }
        }
        self.noted.retain(|n| sides(n.tri).any(|s| uses[&s] >= 2));
        self.noted.shrink_to_fit();
        Splits::Chart {
            noted: self.noted,
            folds,
        }
    }
}

/// A triangle's corners as the patch holds them: reversed where the
/// lane flips its chart's counter-clockwise order.
#[derive(Clone, Copy)]
pub(crate) struct Wound(pub(crate) bool);

impl Wound {
    pub(crate) fn of<T: Copy>(self, c: [T; 3]) -> [T; 3] {
        if self.0 { [c[0], c[2], c[1]] } else { c }
    }
}

/// Whether `tri`'s normal is not wound with `outward` at its centroid: a
/// zero dot, or a normal that is poison, is a fold, as the lanes read one.
fn wound_against(
    tri: [Point3<f64>; 3],
    outward: impl Fn(Point3<f64>) -> geom_core::Vec3<f64>,
) -> bool {
    let normal = (tri[1] - tri[0]).cross(tri[2] - tri[0]);
    let centroid = Point3::new(
        (tri[0].x + tri[1].x + tri[2].x) / 3.0,
        (tri[0].y + tri[1].y + tri[2].y) / 3.0,
        (tri[0].z + tri[1].z + tri[2].z) / 3.0,
    );
    normal.dot(outward(centroid)).partial_cmp(&0.0) != Some(core::cmp::Ordering::Greater)
}

/// **The census** on shared ids, before any patch is placed: each side
/// some patches use past twice per chord carrying it (twice where none
/// does), and the faces that have it inside, two of their triangles on it
/// and no chord of their own boundary along it. Answered per face, its
/// sides in order. A face holding a chord that carries the side keeps it.
///
/// `faces` are the patches' faces with each side between two shared ids
/// of their triangles, once per triangle; `chords` each chord segment's
/// side with the two faces of the edge carrying it.
pub(crate) fn census(
    faces: &[(FaceKey, Vec<Side>)],
    chords: &[(Side, [FaceKey; 2])],
) -> HashMap<FaceKey, Vec<Side>> {
    let mut carried: HashMap<Side, (usize, Vec<FaceKey>)> = HashMap::new();
    for &(s, holders) in chords {
        let c = carried.entry(s).or_default();
        c.0 += 1;
        c.1.extend(holders);
    }
    let mut uses: HashMap<Side, Vec<(FaceKey, usize)>> = HashMap::new();
    for (fk, sides) in faces {
        let mut own: HashMap<Side, usize> = HashMap::new();
        for &s in sides {
            *own.entry(s).or_default() += 1;
        }
        for (s, n) in own {
            uses.entry(s).or_default().push((*fk, n));
        }
    }
    let mut split: HashMap<FaceKey, Vec<Side>> = HashMap::new();
    for (s, users) in uses {
        let (n_chords, holders) = carried.get(&s).map_or((0, &[][..]), |c| (c.0, &c.1[..]));
        let total: usize = users.iter().map(|&(_, n)| n).sum();
        if total <= 2 * n_chords.max(1) {
            continue;
        }
        for (fk, n) in users {
            if n == 2 && !holders.contains(&fk) {
                split.entry(fk).or_default().push(s);
            }
        }
    }
    for sides in split.values_mut() {
        sides.sort_unstable();
    }
    split
}

/// A triangle's corners' chart points and which are a singular point.
type Corners = ([[f64; 2]; 3], [bool; 3]);

/// What [`split`] reads of the run: the body, the chord pass's spline
/// cell tables, δ and the run's ε (the certificate's verdict).
pub(crate) struct Run<'a> {
    pub(crate) body: &'a Body<f64>,
    pub(crate) bounds: &'a FaceBounds,
    pub(crate) delta: f64,
    pub(crate) eps: Eps,
}

/// **Splits `sides` in a placed patch**, in order: each at the point of
/// the face's surface over its midpoint in the chart (an end at a singular
/// point gives the midpoint the other end's `u`: it has no azimuth,
/// `crate::chart_table`), appended to `positions`, and each of the two
/// triangles on it made two, wound as it was. A triangle with two sides
/// split is split twice. Then every piece is read as its lane reads a
/// triangle.
///
/// # Errors
///
/// The face's refusal where a piece fails: [`TessellateError::FoldedTriangle`],
/// then the certificate's ([`cert::FaceVerdict::refusal`]).
pub(crate) fn split(
    face: FaceKey,
    sides: &[Side],
    splits: &Splits,
    run: &Run<'_>,
    positions: &mut Vec<Point3<f64>>,
    triangles: &mut Vec<[u32; 3]>,
) -> Result<(), TessellateError> {
    let (noted, folds) = match splits {
        Splits::Midpoint => {
            for &(a, b) in sides {
                let (pa, pb) = (positions[a as usize], positions[b as usize]);
                let mid = Point3::new(
                    0.5 * (pa.x + pb.x),
                    0.5 * (pa.y + pb.y),
                    0.5 * (pa.z + pb.z),
                );
                cut(face, (a, b), positions, mid, triangles, |_, _| {});
            }
            return Ok(());
        }
        Splits::Chart { noted, folds } => (noted, *folds),
    };
    let (Ok(surface), Some(sense)) = (
        crate::tessellate::face_surface(run.body, face),
        run.body.get_face(face).map(|f| f.sense),
    ) else {
        unreachable!("face {face:?}: its lane meshed it, so the body holds it and its surface");
    };
    let Ok(row) = ChartRow::of(run.body, face, surface, run.bounds) else {
        unreachable!("face {face:?}: its lane read this chart row and meshed the face");
    };
    // Each triangle's chart corners where noted, and whether it is a piece.
    let mut charts: Vec<Option<Corners>> = vec![None; triangles.len()];
    for n in noted {
        charts[n.tri as usize] = Some((n.uv, n.singular));
    }
    let mut pieces = vec![false; triangles.len()];
    for &(a, b) in sides {
        let on: Vec<usize> = (0..triangles.len())
            .filter(|&i| (0..3).any(|q| side(triangles[i][q], triangles[i][(q + 1) % 3]) == (a, b)))
            .collect();
        // The side's ends' chart points, as each triangle on it holds them.
        let ends = |i: usize| {
            let Some((uv, singular)) = charts[i] else {
                unreachable!(
                    "face {face:?}: a triangle on split side {:?} has two shared corners, and its lane notes every such triangle",
                    (a, b)
                );
            };
            let at = |id: u32| {
                let Some(q) = (0..3).find(|&q| triangles[i][q] == id) else {
                    unreachable!(
                        "face {face:?}: a triangle on side {:?} has both its ends",
                        (a, b)
                    );
                };
                (uv[q], singular[q])
            };
            (at(a), at(b))
        };
        let bits = |((ua, _), (ub, _)): (([f64; 2], bool), ([f64; 2], bool))| {
            (ua.map(f64::to_bits), ub.map(f64::to_bits))
        };
        let [one, two] = on[..] else {
            unreachable!(
                "face {face:?}: the census splits side {:?}, which two of its triangles have, and {} have it",
                (a, b),
                on.len()
            );
        };
        if bits(ends(one)) != bits(ends(two)) {
            unreachable!(
                "face {face:?}: its two triangles on side {:?} hold its ends at different chart \
                 points, a seam's boundary chord, which a face holding that chord keeps",
                (a, b)
            );
        }
        let ((ua, sa), (ub, sb)) = ends(one);
        let m = midpoint(surface, (ua, sa), (ub, sb));
        let point = surface.eval(m[0], m[1]);
        let mut made: Vec<(usize, [[f64; 2]; 3], [bool; 3])> = Vec::new();
        cut(face, (a, b), positions, point, triangles, |i, q| {
            let (uv, s) = charts[i].unwrap_or(([[0.0; 2]; 3], [false; 3]));
            let (x, y, z) = (q, (q + 1) % 3, (q + 2) % 3);
            made.push((i, [uv[x], m, uv[z]], [s[x], false, s[z]]));
            made.push((usize::MAX, [m, uv[y], uv[z]], [false, s[y], s[z]]));
        });
        // `cut` keeps the first half at the triangle's index and appends the
        // second, in the order the triangles were met.
        for (i, uv, s) in made {
            if i == usize::MAX {
                charts.push(Some((uv, s)));
                pieces.push(true);
            } else {
                charts[i] = Some((uv, s));
                pieces[i] = true;
            }
        }
    }
    let mut verdict = cert::FaceVerdict::new();
    let mut folded = false;
    for (i, t) in triangles.iter().enumerate() {
        if !pieces[i] {
            continue;
        }
        let Some((uv, singular)) = charts[i] else {
            unreachable!("face {face:?}: every piece has its chart corners");
        };
        let tri = t.map(|v| positions[v as usize]);
        let (bound, scale) = row.cert(tri, uv, singular);
        verdict.read(bound, run.delta, scale, run.eps);
        if let (true, ChartRow::Analytic(analytic)) = (folds, &row) {
            folded |= wound_against(tri, |p| analytic.outward(sense, p).vec());
        }
    }
    if folded {
        return Err(TessellateError::FoldedTriangle { face });
    }
    verdict.refusal(face, run.delta)
}

/// **The chart point a side between chart points `a` and `b` is split at**:
/// its midpoint in the chart, where an end at a singular point gives it
/// the other end's `u` (a singular point has no azimuth,
/// `crate::chart_table`); on a cone, the midpoint of the side in its
/// development (`trimmed::developed`, unrounded), read back to the chart
/// on the branch of `u` nearest the chart midpoint. On a cone the two
/// differ, both on the surface, and the development's is the one whose
/// pieces fold less often (measured over every side the suite's cone
/// faces could split: the walk's 685 against 1,675, the rectangle lane's
/// 10 against 57, the trimmed lane's 0 against 6). At the apex the two
/// agree: the development puts it at the origin.
fn midpoint(
    surface: &geom::Surface<f64>,
    (a, sa): ([f64; 2], bool),
    (b, sb): ([f64; 2], bool),
) -> [f64; 2] {
    use geom_core::Real;
    let u = match (sa, sb) {
        (true, false) => b[0],
        (false, true) => a[0],
        _ => 0.5 * (a[0] + b[0]),
    };
    let chart = [u, 0.5 * (a[1] + b[1])];
    let geom::Surface::Cone { half_angle, .. } = *surface else {
        return chart;
    };
    if sa || sb {
        return chart;
    }
    // The nappe both ends lie on: the sign of their slant `v`.
    let nappe = if a[1] + b[1] < 0.0 { -1.0 } else { 1.0 };
    let k = nappe * Real::sin(half_angle);
    let develop = |p: [f64; 2]| {
        let (sin, cos) = Real::sin_cos(k * p[0]);
        let r = nappe * p[1];
        (r * sin, r * cos)
    };
    let ((xa, ya), (xb, yb)) = (develop(a), develop(b));
    let (x, y) = (0.5 * (xa + xb), 0.5 * (ya + yb));
    let theta = Real::atan2(x, y) / k;
    let period = core::f64::consts::TAU / k.abs();
    let turns = ((chart[0] - theta) / period).round();
    [theta + turns * period, nappe * Real::sqrt(x * x + y * y)]
}

/// Appends `point` and makes each of the two triangles on side `(a, b)`
/// two: the half on `a` at the triangle's index, the half on `b` appended,
/// both wound as it was. `each` is told each triangle cut and the index of
/// the side's first corner in it.
fn cut(
    face: FaceKey,
    (a, b): Side,
    positions: &mut Vec<Point3<f64>>,
    point: Point3<f64>,
    triangles: &mut Vec<[u32; 3]>,
    mut each: impl FnMut(usize, usize),
) {
    #[allow(clippy::cast_possible_truncation)]
    let m = positions.len() as u32;
    positions.push(point);
    let mut on = 0;
    for i in 0..triangles.len() {
        let t = triangles[i];
        if let Some(q) = (0..3).find(|&q| side(t[q], t[(q + 1) % 3]) == (a, b)) {
            on += 1;
            let (x, y, z) = (t[q], t[(q + 1) % 3], t[(q + 2) % 3]);
            each(i, q);
            triangles[i] = [x, m, z];
            triangles.push([m, y, z]);
        }
    }
    if on != 2 {
        unreachable!(
            "face {face:?}: the census splits side {:?}, which two of its triangles have, and {on} have it",
            (a, b)
        );
    }
}
