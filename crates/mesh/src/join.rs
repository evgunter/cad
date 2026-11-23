//! **Where the patches join**: a side that more than one patch has as an
//! inside edge, and its split.
//!
//! What the split is for is the mesh's shape where two patches meet, not
//! its volume. Two faces that share two boundary chains can each make, as
//! an inside edge of its own triangulation, the same straight side between
//! a point of one chain and a point of the other; the joined mesh then has
//! an edge of four triangles. And two faces closer than δ can each lay
//! nothing but the points they share over one region (a cap thinner than
//! δ, a sliver), so that the two patches cover it with the same points and
//! the mesh encloses nothing there. No lane can see either alone, and no
//! lane reads anything about the faces across its edges, so both are
//! answered here, the one place every patch is in hand. A split puts a
//! point of each face's own surface into the patch; it promises no volume
//! (the class it reaches and what it measured are in the unit's own item,
//! `work/tesslane/the-mesh-cures-a-side-two-faces-can-both-make-three-ways.md`).
//!
//! - **The patch keeps its chart corners.** A lane notes, for each of
//!   its triangles with two shared corners, the chart point of every
//!   corner and which are a singular point ([`Book::push`]). A patch is a
//!   function of its face and δ, these included.
//! - **The census is the join's** ([`census`]): on shared ids, before any
//!   patch is placed. A face splits a side it has inside where the side is
//!   used past twice per chord carrying it, and where its face and the
//!   face across both lay a triangle all of whose corners are points they
//!   share. A face whose boundary chord carries the side keeps it. Nothing
//!   is meshed again, no patch is chosen and nothing reads storage order.
//! - **The split is the face's** ([`split`]): at the foot on its own
//!   surface of the side's straight midpoint, a pure function of the face,
//!   δ and the side. Every piece it makes is held as a triangle of its face
//!   is: on the face's own certificate and, on an analytic face, its fold,
//!   typed as the lane's own refusal; and to the winding read where the
//!   face joins the mesh (`[mesh.winding]`).

use std::collections::HashMap;

use geom_core::Point3;
use topo::{Body, FaceKey};

use crate::cert;
use crate::chart_table::ChartRow;
use crate::nurbs_cert::FaceBounds;
use crate::sizing::Eps;
use crate::tessellate::PatchVertex;
use crate::types::TessellateError;
use crate::walk::edge_key;

/// A side between two shared mesh ids, the lesser first
/// ([`crate::walk::edge_key`]).
pub(crate) type Side = (u32, u32);

/// **A triangle's sides between two shared corners**, each once: the one
/// spelling of that walk, for the census and for the notes. A corner is
/// its shared id, or `None` where it is the patch's own point.
pub(crate) fn shared_sides(t: [Option<u32>; 3]) -> impl Iterator<Item = Side> {
    (0..3).filter_map(move |q| Some(edge_key(t[q]?, t[(q + 1) % 3]?)))
}

/// **Whether a side is used past what its chords allow**: two triangle
/// uses per chord carrying it, two where none does. The one spelling of
/// that rule, for the census and for [`crate::tessellate`]'s check after
/// the split.
pub(crate) const fn overused(uses: usize, chords: usize) -> bool {
    uses > 2 * if chords == 0 { 1 } else { chords }
}

/// **A patch's chart corners**, as its splits read them: those of its
/// triangles with two shared corners on a side two of its triangles have.
/// A planar patch holds none (its split reads no chart).
#[derive(Clone, Debug, Default)]
pub(crate) struct Splits {
    noted: Vec<Noted>,
}

impl Splits {
    /// The heap the notes take.
    pub(crate) fn bytes(&self) -> usize {
        self.noted.len() * core::mem::size_of::<Noted>()
    }
}

/// One noted triangle: its index in the patch, its corners' chart points
/// and which are a singular point, in the patch's winding.
#[derive(Clone, Copy, Debug)]
struct Noted {
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
    /// **Pushes a triangle onto the patch's `triangles`**, as the patch
    /// holds it, and notes its chart corners if two of its corners are
    /// shared: the note's index is the push's, so the two cannot drift.
    pub(crate) fn push(
        &mut self,
        triangles: &mut Vec<[PatchVertex; 3]>,
        ids: [PatchVertex; 3],
        uv: [[f64; 2]; 3],
        singular: [bool; 3],
    ) {
        if ids.iter().filter(|v| v.shared().is_some()).count() >= 2 {
            #[allow(clippy::cast_possible_truncation)]
            self.noted.push(Noted {
                tri: triangles.len() as u32,
                uv,
                singular,
            });
        }
        triangles.push(ids);
    }

    /// The patch's splits. Only a triangle on a side between two shared
    /// ids that two of the patch's triangles have can be split (a side one
    /// triangle has is a chord of the face's own boundary), so only those
    /// are kept.
    pub(crate) fn splits(mut self, ids: &[[PatchVertex; 3]]) -> Splits {
        let sides = |tri: u32| shared_sides(ids[tri as usize].map(PatchVertex::shared));
        let mut uses: HashMap<Side, usize> = HashMap::new();
        for n in &self.noted {
            for s in sides(n.tri) {
                *uses.entry(s).or_default() += 1;
            }
        }
        self.noted.retain(|n| sides(n.tri).any(|s| uses[&s] >= 2));
        self.noted.shrink_to_fit();
        Splits { noted: self.noted }
    }
}

/// **The census** on shared ids, before any patch is placed: per face, the
/// sides it has inside (two of its triangles on the side, and no chord of
/// its own boundary along it) that it splits, in order. A face holding a
/// chord that carries a side keeps it. A face splits a side it has inside
/// where either:
///
/// - **the side is overused** ([`overused`]): some patches use it past
///   twice per chord carrying it, which joined is an edge of four
///   triangles; or
/// - **the pair trigger**: the side is a side of one of the face's
///   triangles all of whose corners are chord points the face F shares
///   with one face G (an end of a chord segment of an edge whose two faces
///   are F and G), and G has such a triangle toward F. Then F and G both
///   lay, over some region, only points they share, and their patches
///   can cover it with the same points (a cap thinner than δ against its
///   plane). It reads nothing of G but shared ids and the corners of its
///   patch's triangles, and no ε.
///
/// `faces` are the patches' faces with each triangle's corners as shared
/// ids (`None` for a point of the patch's own); `chords` each chord
/// segment's side with the two faces of the edge carrying it.
///
/// A face uses a side between two shared ids at most twice: inside, one
/// triangle on each side of it; on its boundary, once per traversal of the
/// edge whose chord carries it, and a face traverses an edge at most twice
/// (a seam). A third use is a lane's own triangulation folded over the side,
/// a bug, which this census does not split and [`crate::tessellate`]'s check
/// after the split reports (`[failure.taxonomy]`, an internal bug).
pub(crate) fn census(
    faces: &[(FaceKey, Vec<[Option<u32>; 3]>)],
    chords: &[(Side, [FaceKey; 2])],
) -> HashMap<FaceKey, Vec<Side>> {
    let mut carried: HashMap<Side, (usize, Vec<FaceKey>)> = HashMap::new();
    // Each shared id's faces across: the pairs (F, G), F ≠ G, of every
    // chord segment it is an end of.
    let mut across: HashMap<u32, Vec<(FaceKey, FaceKey)>> = HashMap::new();
    for &(s, [f, g]) in chords {
        let c = carried.entry(s).or_default();
        c.0 += 1;
        c.1.extend([f, g]);
        if f != g {
            for id in [s.0, s.1] {
                let pairs = across.entry(id).or_default();
                for pair in [(f, g), (g, f)] {
                    if !pairs.contains(&pair) {
                        pairs.push(pair);
                    }
                }
            }
        }
    }
    let held = |s: &Side, fk: FaceKey| carried.get(s).is_some_and(|c| c.1.contains(&fk));
    // Each face's uses of each side, and the pair trigger's triangles: per
    // (F, G), F's triangles whose corners are all F–G chord points.
    let mut own: HashMap<FaceKey, HashMap<Side, usize>> = HashMap::new();
    let mut paired: HashMap<(FaceKey, FaceKey), Vec<[u32; 3]>> = HashMap::new();
    for (fk, triangles) in faces {
        let uses = own.entry(*fk).or_default();
        for &t in triangles {
            for s in shared_sides(t) {
                *uses.entry(s).or_default() += 1;
            }
            let [Some(a), Some(b), Some(c)] = t else {
                continue;
            };
            for &(f, g) in across.get(&a).map_or(&[][..], Vec::as_slice) {
                let toward = |id: u32| across.get(&id).is_some_and(|p| p.contains(&(f, g)));
                if f == *fk && toward(b) && toward(c) {
                    paired.entry((f, g)).or_default().push([a, b, c]);
                }
            }
        }
    }
    let inside = |fk: FaceKey, s: &Side| {
        own.get(&fk).and_then(|u| u.get(s)) == Some(&2) && !held(s, fk)
    };
    let mut split: HashMap<FaceKey, Vec<Side>> = HashMap::new();
    let mut users: HashMap<Side, (usize, Vec<FaceKey>)> = HashMap::new();
    for (fk, uses) in &own {
        for (s, &n) in uses {
            let u = users.entry(*s).or_default();
            u.0 += n;
            u.1.push(*fk);
        }
    }
    for (s, (total, faces_on)) in &users {
        if overused(*total, carried.get(s).map_or(0, |c| c.0)) {
            for &fk in faces_on {
                if inside(fk, s) {
                    split.entry(fk).or_default().push(*s);
                }
            }
        }
    }
    for ((f, g), triangles) in &paired {
        if !paired.contains_key(&(*g, *f)) {
            continue;
        }
        for &t in triangles {
            for s in shared_sides(t.map(Some)) {
                if inside(*f, &s) {
                    split.entry(*f).or_default().push(s);
                }
            }
        }
    }
    // In mesh-id order, which the ids' minting order makes a function of
    // the body and δ (D9), not of storage: a triangle with two sides split
    // takes its diagonal from this order.
    for sides in split.values_mut() {
        sides.sort_unstable();
        sides.dedup();
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
/// the face's surface [`point_at`] names, appended to `positions`, and each
/// of the two triangles on it made two, wound as it was. A triangle with
/// two sides split is split twice. Then every piece is held as a triangle
/// of its face is ([`held`]).
///
/// On a plane the foot of a side's straight midpoint is that midpoint, and
/// a piece lies in its triangle's plane, inside it: it can neither fold
/// nor leave a surface it has no certificate against, so nothing is read.
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
    let (Ok(surface), Some(sense)) = (
        crate::tessellate::face_surface(run.body, face),
        run.body.get_face(face).map(|f| f.sense),
    ) else {
        unreachable!("face {face:?}: its lane meshed it, so the body holds it and its surface");
    };
    if matches!(surface, geom::Surface::Plane { .. }) {
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
    let Ok(row) = ChartRow::of(run.body, face, surface, run.bounds) else {
        unreachable!("face {face:?}: its lane read this chart row and meshed the face");
    };
    // Each triangle's chart corners where noted, and whether it is a piece.
    let mut charts: Vec<Option<Corners>> = vec![None; triangles.len()];
    for n in &splits.noted {
        charts[n.tri as usize] = Some((n.uv, n.singular));
    }
    let mut pieces = vec![false; triangles.len()];
    for &(a, b) in sides {
        // The side's two triangles, as the census counted them: one scan
        // of the patch per side, which a patch's few splits keep small.
        let on: Vec<usize> = (0..triangles.len())
            .filter(|&i| crate::walk::sides(triangles[i]).contains(&(a, b)))
            .collect();
        let [one, two] = on[..] else {
            unreachable!(
                "face {face:?}: the census splits side {:?}, which two of its triangles have, and {} have it",
                (a, b),
                on.len()
            );
        };
        // The side's ends' chart points, as a triangle on it holds them.
        let ends = |i: usize| {
            let Some((uv, singular)) = charts[i] else {
                unreachable!(
                    "face {face:?}: a triangle on split side {:?} has two shared corners on a \
                     side two of its triangles have, and its lane notes every such triangle",
                    (a, b)
                );
            };
            [a, b].map(|id| {
                let Some(q) = (0..3).find(|&q| triangles[i][q] == id) else {
                    unreachable!("face {face:?}: a triangle on side {:?} has both its ends", (a, b));
                };
                (uv[q], singular[q])
            })
        };
        // An end at a singular point is one end whatever `u` its copies
        // carry (the rectangle lane keeps the apex as one chart vertex per
        // column); any other end is one chart point in both triangles. Two
        // triangles holding a regular end at two chart points would be the
        // two sides of a seam, whose chord is the face's own boundary: the
        // census never splits a side its face's chord carries.
        let same = |(p, sp): ([f64; 2], bool), (q, sq): ([f64; 2], bool)| {
            (sp && sq) || (!sp && !sq && p.map(f64::to_bits) == q.map(f64::to_bits))
        };
        let ([ea, eb], [fa, fb]) = (ends(one), ends(two));
        if !(same(ea, fa) && same(eb, fb)) {
            unreachable!(
                "face {face:?}: its two triangles on side {:?} hold an end at two chart points, \
                 a seam's boundary chord, which a face holding that chord keeps",
                (a, b)
            );
        }
        let m = point_at(
            surface,
            (positions[a as usize], positions[b as usize]),
            ea,
            eb,
        );
        let point = surface.eval(m[0], m[1]);
        cut(face, (a, b), positions, point, triangles, |i, q| {
            let Some((uv, s)) = charts[i] else {
                unreachable!("face {face:?}: a triangle on side {:?} is noted, read above", (a, b));
            };
            let (x, y, z) = (q, (q + 1) % 3, (q + 2) % 3);
            // `cut` keeps the half on the side's first end at the
            // triangle's index and appends the other, in the order met.
            charts[i] = Some(([uv[x], m, uv[z]], [s[x], false, s[z]]));
            pieces[i] = true;
            charts.push(Some(([m, uv[y], uv[z]], [false, s[y], s[z]])));
            pieces.push(true);
        });
    }
    let made = triangles
        .iter()
        .enumerate()
        .filter(|&(i, _)| pieces[i])
        .map(|(i, t)| {
            let Some((uv, singular)) = charts[i] else {
                unreachable!("face {face:?}: every piece has its chart corners");
            };
            (t.map(|v| positions[v as usize]), uv, singular)
        });
    // A piece is not a lattice triangle, so the rectangle lane's reason to
    // read no fold on its own does not reach it: every analytic face's
    // pieces are fold-read. A spline face's are held to its certificate
    // alone, as the spline arm holds its own triangles (it has no outward
    // normal in closed form to read).
    let outward = |p| match &row {
        ChartRow::Analytic(analytic) => Some(analytic.outward(sense, p).vec()),
        ChartRow::Spline { .. } => None,
    };
    held(face, made, &row, &outward, run)
}

/// **The pieces of a split, read as their face's lane reads a triangle**:
/// each on the face's certificate (`row`), and, where `outward` answers,
/// wound with the face.
///
/// # Errors
///
/// The face's refusal: [`TessellateError::FoldedTriangle`], then the
/// certificate's ([`cert::FaceVerdict::refusal`]).
fn held(
    face: FaceKey,
    pieces: impl Iterator<Item = ([Point3<f64>; 3], [[f64; 2]; 3], [bool; 3])>,
    row: &ChartRow<'_>,
    outward: &dyn Fn(Point3<f64>) -> Option<geom_core::Vec3<f64>>,
    run: &Run<'_>,
) -> Result<(), TessellateError> {
    let mut verdict = cert::FaceVerdict::new();
    let mut folded = false;
    for (tri, uv, singular) in pieces {
        let (bound, scale) = row.cert(tri, uv, singular);
        verdict.read(bound, run.delta, scale, run.eps);
        let normal = (tri[1] - tri[0]).cross(tri[2] - tri[0]);
        folded |= outward(crate::validate::centroid(tri))
            .is_some_and(|n| crate::validate::wound_against(normal, n));
    }
    if folded {
        return Err(TessellateError::FoldedTriangle { face });
    }
    verdict.refusal(face, run.delta)
}

/// **The chart point a side is split at**: the foot on the face's surface
/// of the side's straight midpoint (`geom::Surface::foot`), read back to
/// the chart on the copy nearest the side's chart midpoint. One rule on
/// every analytic surface the foot has a closed form on: cylinder, cone,
/// sphere, ring torus.
///
/// - **At a singular end** (a cone's apex, a sphere's pole) the side runs
///   along a straight generator or a meridian great circle, and its foot
///   is the point on it at half the other end's slant or latitude: the
///   chart midpoint with the other end's `u` (a singular point has no
///   azimuth, `crate::chart_table`).
/// - **The copy.** The foot's angles are read modulo a turn, and the copy
///   nearest the chart midpoint is the foot's own: the foot of a chord's
///   midpoint lies, in each angle, between the chord's ends, and a side of
///   a chart triangle spans less than a turn in each, so the foot is
///   within half a turn of the ends' mean. The ends of a cone face's side
///   lie on one nappe (a face bounds one nappe, `geom::Surface::Cone`),
///   and so does the foot of their midpoint.
/// - **By surface kind, the chart midpoint**: a spindle sheet and a spline
///   or fitted face have no foot in closed form, and keep it. So does a
///   side whose midpoint has no single foot, on a cylinder's, cone's or
///   torus's axis, at a sphere's centre or on a torus's tube circle:
///   there a whole circle of the surface is the foot alike, and the chart
///   midpoint's point is on it for every kind but the cone.
///
/// Measured over every side the `mesh` suite's patches could split, at
/// the default ε (the unit's item and the filed row on pieces that would
/// fold): the foot folds no piece on a cone or sphere, where the chart
/// midpoint folded 697 and 83; 16 against 46 on a torus; the same 6 on a
/// cylinder, where the two are one point. Measured, not guarded here.
fn point_at(
    surface: &geom::Surface<f64>,
    (pa, pb): (Point3<f64>, Point3<f64>),
    (a, sa): ([f64; 2], bool),
    (b, sb): ([f64; 2], bool),
) -> [f64; 2] {
    use geom::Surface;
    use geom_core::Real;
    let chart = [
        match (sa, sb) {
            (true, false) => b[0],
            (false, true) => a[0],
            _ => 0.5 * (a[0] + b[0]),
        },
        0.5 * (a[1] + b[1]),
    ];
    let mid = Point3::new(
        0.5 * (pa.x + pb.x),
        0.5 * (pa.y + pb.y),
        0.5 * (pa.z + pb.z),
    );
    let foot = match surface {
        Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. } => {
            surface.foot(mid)
        }
        _ => None,
    };
    let (Some(foot), Some(frame)) = (foot, surface.frame()) else {
        return chart;
    };
    let turn = geom_core::period::Period::<f64>::angle().length();
    let near = |x: f64, to: f64| x + turn * ((to - x) / turn).round();
    let d = foot - frame.origin();
    let (x, y, h) = (d.dot(frame.u_ref()), d.dot(frame.v_ref()), d.dot(frame.axis()));
    let rho = Real::sqrt(x.powi(2) + y.powi(2));
    match *surface {
        Surface::Cylinder { .. } => [near(Real::atan2(y, x), chart[0]), h],
        Surface::Cone { .. } => {
            // The mirror nappe (`v < 0`) runs along −radial(u).
            let nappe = if h < 0.0 { -1.0 } else { 1.0 };
            [
                near(Real::atan2(nappe * y, nappe * x), chart[0]),
                nappe * Real::sqrt(rho.powi(2) + h.powi(2)),
            ]
        }
        Surface::Sphere { .. } => [near(Real::atan2(y, x), chart[0]), Real::atan2(h, rho)],
        Surface::Torus { major_radius, .. } => [
            near(Real::atan2(y, x), chart[0]),
            near(Real::atan2(h, rho - major_radius), chart[1]),
        ],
        _ => chart,
    }
}

/// Appends `point` and makes each of the two triangles on side `(a, b)`
/// two: the half on `a` at the triangle's index, the half on `b` appended,
/// both wound as it was. `each` is told each triangle cut and the index of
/// the side's first corner in it, before the cut.
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
        if let Some(q) = (0..3).find(|&q| edge_key(t[q], t[(q + 1) % 3]) == (a, b)) {
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom::Surface;
    use geom_core::{Point3, Tol, Vec3};

    use super::{Run, census, held, point_at};
    use crate::chart_table::ChartRow;
    use crate::nurbs_cert::FaceBounds;
    use crate::sizing::Eps;
    use crate::types::TessellateError;
    use crate::walk::edge_key as side;

    fn frame() -> geom::Frame<f64> {
        geom::Frame::new(
            Point3::new(0.2, -0.1, 0.3),
            Vec3::new(0.0, 0.6, 0.8),
            Vec3::new(1.0, 0.0, 0.0),
        )
    }

    /// **A side splits at the foot of its straight midpoint, read back to
    /// the chart on the copy nearest its chart midpoint.** On a cylinder,
    /// cones of half-angle 0.1 to 1.2 on either nappe, a sphere and a ring
    /// torus, sides up to a third of a turn wide and across the seam
    /// (`u` past a turn): the split point is on the surface, nearer the
    /// straight midpoint than the chart midpoint's point is (or as near),
    /// its `u` between the ends'. From a cone's apex or a sphere's pole:
    /// the other end's `u` at half its slant or latitude, which is that
    /// side's foot. A spindle sheet keeps the chart midpoint.
    #[test]
    fn a_side_splits_at_the_foot_of_its_straight_midpoint() {
        let mut surfaces = vec![
            Surface::Cylinder { frame: frame(), radius: 2.0 },
            Surface::Sphere { frame: frame(), radius: 1.5 },
            Surface::Torus { frame: frame(), major_radius: 2.0, minor_radius: 0.5 },
        ];
        for half_angle in [0.1, 0.5, 1.2] {
            surfaces.push(Surface::Cone { frame: frame(), half_angle });
        }
        for surface in &surfaces {
            let nappes: &[f64] = if matches!(surface, Surface::Cone { .. }) { &[1.0, -1.0] } else { &[1.0] };
            for &nappe in nappes {
                for (a, b) in [
                    ([0.1, 0.4], [0.9, 1.0]),
                    ([5.9, 0.4], [6.6, 0.5]),
                    ([1.0, 1.1], [3.0, 0.3]),
                ] {
                    let (a, b) = ([a[0], nappe * a[1]], [b[0], nappe * b[1]]);
                    let (pa, pb) = (surface.eval(a[0], a[1]), surface.eval(b[0], b[1]));
                    let m = point_at(surface, (pa, pb), (a, false), (b, false));
                    let mid = Point3::new(0.5 * (pa.x + pb.x), 0.5 * (pa.y + pb.y), 0.5 * (pa.z + pb.z));
                    let at = surface.eval(m[0], m[1]);
                    let chart = surface.eval(0.5 * (a[0] + b[0]), 0.5 * (a[1] + b[1]));
                    let foot = surface.foot(mid).unwrap();
                    assert!(
                        (at - foot).norm() < 1e-12,
                        "{surface:?}, {a:?} to {b:?}: {m:?} is {:e} off the foot",
                        (at - foot).norm()
                    );
                    assert!(
                        (at - mid).norm() <= (chart - mid).norm() + 1e-12,
                        "{surface:?}: the foot is no farther than the chart midpoint's point"
                    );
                    assert!(
                        (a[0].min(b[0])..=a[0].max(b[0])).contains(&m[0]),
                        "{surface:?}: u {} between the ends'",
                        m[0]
                    );
                }
            }
            let (apex, end) = match surface {
                Surface::Cone { .. } => ([7.0, 0.0], [0.5, 2.0]),
                Surface::Sphere { .. } => ([7.0, core::f64::consts::FRAC_PI_2], [0.5, 0.3]),
                _ => continue,
            };
            let (pa, pb) = (surface.eval(apex[0], apex[1]), surface.eval(end[0], end[1]));
            let m = point_at(surface, (pa, pb), (apex, true), (end, false));
            assert!(
                (m[0] - 0.5).abs() < 1e-12 && (m[1] - 0.5 * (apex[1] + end[1])).abs() < 1e-12,
                "{surface:?}: from the singular point, the other end's u at half its slant or latitude: {m:?}"
            );
        }
        let spindle = Surface::Spindle { frame: frame(), major_radius: 0.4, minor_radius: 1.0 };
        let (a, b) = ([0.1, 0.4], [0.9, 1.0]);
        let ends = (spindle.eval(a[0], a[1]), spindle.eval(b[0], b[1]));
        assert_eq!(
            point_at(&spindle, ends, (a, false), (b, false)),
            [0.5, 0.7],
            "a spindle sheet keeps its chart midpoint"
        );
    }

    /// **`held` alone, the unit row it is**: a piece handed straight to it
    /// (the rows through `split` are `tests/split_rulings.rs`'s). The rod
    /// sliver's cylinder face (`trimmed::tests::a_rod_sliver`), its chart
    /// row, and one piece spanning a third of a radian of the rod: within δ
    /// 0.1 it passes; at δ 10⁻⁶ it refuses `CertificateExceeded`; wound
    /// against the face it refuses `FoldedTriangle`, and where no outward
    /// normal answers (a spline face's), not.
    #[test]
    fn held_reads_a_piece_on_its_certificate_and_its_fold() {
        let body = crate::trimmed::tests::a_rod_sliver();
        let (face, surface) = body
            .faces()
            .find_map(|(fk, _)| {
                let s = body.face_surface(fk)?;
                matches!(s, Surface::Cylinder { .. }).then_some((fk, s))
            })
            .expect("the sliver has a cylinder face");
        let bounds = FaceBounds::new();
        let row = ChartRow::of(&body, face, surface, &bounds).unwrap();
        let sense = body.get_face(face).unwrap().sense;
        let ChartRow::Analytic(analytic) = &row else {
            panic!("a cylinder's row is analytic");
        };
        let outward = |p| Some(analytic.outward(sense, p).vec());
        let none = |_| None;
        let uv = [[0.0, 0.5], [0.3, 0.5], [0.0, 1.5]];
        let tri = uv.map(|[u, v]| surface.eval(u, v));
        let reversed = [tri[0], tri[2], tri[1]];
        let ruv = [uv[0], uv[2], uv[1]];
        let run = |delta| Run {
            body: &body,
            bounds: &bounds,
            delta,
            eps: Eps::at(Tol::witness()),
        };
        // Which winding is outward is the face's sense: read it.
        let wound = |t: [Point3<f64>; 3]| {
            let n = (t[1] - t[0]).cross(t[2] - t[0]);
            n.dot(analytic.outward(sense, crate::validate::centroid(t)).vec()) > 0.0
        };
        let ((good, guv), (bad, buv)) = if wound(tri) {
            ((tri, uv), (reversed, ruv))
        } else {
            ((reversed, ruv), (tri, uv))
        };
        let piece = |t, uv| std::iter::once((t, uv, [false; 3]));
        assert!(
            held(face, piece(good, guv), &row, &outward, &run(0.1)).is_ok(),
            "within δ: held"
        );
        assert!(
            matches!(
                held(face, piece(good, guv), &row, &outward, &run(1e-6)),
                Err(TessellateError::CertificateExceeded { .. })
            ),
            "past δ: the certificate refuses"
        );
        assert!(
            matches!(
                held(face, piece(bad, buv), &row, &outward, &run(0.1)),
                Err(TessellateError::FoldedTriangle { .. })
            ),
            "wound against the face: a fold"
        );
        assert!(
            held(face, piece(bad, buv), &row, &none, &run(0.1)).is_ok(),
            "no outward normal to read"
        );
    }

    /// **The census splits a side in every patch that has it inside, and
    /// a face whose boundary chord carries it keeps it.** Three faces of a
    /// cube for keys: `f` holds both halves of the edge whose chord carries
    /// side `(1, 2)` (a seam, two of its triangles on it, both boundary),
    /// `g` has the side inside, `h` uses it nowhere. Four uses against one
    /// chord's two: `g` splits and `f` does not. A side no chord carries
    /// that `g` and `h` both have inside splits in both; one only `g` has
    /// is no one's to split.
    #[test]
    fn the_census_splits_inside_and_a_boundary_keeps_its_chord() {
        let body = sweep::test_support::cube::<f64>(1.0, Tol::witness());
        let keys: Vec<_> = body.faces().map(|(k, _)| k).collect();
        let (f, g, h) = (keys[0], keys[1], keys[2]);
        // Each triangle as its corners; `None` a point of the patch's own.
        let on = |a: u32, b: u32| [Some(a), Some(b), None];
        let faces = vec![
            (f, vec![on(2, 1), on(1, 2)]),
            (g, vec![on(1, 2), on(2, 1), on(5, 7), on(7, 5), on(8, 9), on(9, 8)]),
            (h, vec![on(5, 7), on(7, 5)]),
        ];
        let split = census(&faces, &[(side(2, 1), [f, f])]);
        assert_eq!(split.get(&g), Some(&vec![side(1, 2), side(5, 7)]), "g splits both");
        assert_eq!(split.get(&h), Some(&vec![side(5, 7)]), "h splits the free side");
        assert_eq!(split.get(&f), None, "f keeps its boundary chord");
    }

    /// **The pair trigger.** Faces `f` and `g` meet along one edge whose
    /// chord runs 0–1–2–3–0 round a loop, the cap's circle, and each lays
    /// two triangles on those four points alone, on different diagonals:
    /// no side is used past its chords, and each splits its own diagonal.
    /// Where `g` lays a point of its own in every triangle, it has no
    /// triangle toward `f`, and neither splits; nor does a face whose
    /// triangle's corners are chord points it shares with two faces.
    #[test]
    fn the_pair_trigger_splits_where_both_faces_lay_only_shared_points() {
        let body = sweep::test_support::cube::<f64>(1.0, Tol::witness());
        let keys: Vec<_> = body.faces().map(|(k, _)| k).collect();
        let (f, g, k) = (keys[0], keys[1], keys[2]);
        let ring = [side(0, 1), side(1, 2), side(2, 3), side(3, 0)];
        let chords: Vec<_> = ring.iter().map(|&s| (s, [f, g])).collect();
        let t = |a, b, c| [Some(a), Some(b), Some(c)];
        let faces = vec![
            (f, vec![t(0, 1, 2), t(0, 2, 3)]),
            (g, vec![t(1, 0, 3), t(1, 3, 2)]),
        ];
        let split = census(&faces, &chords);
        assert_eq!(split.get(&f), Some(&vec![side(0, 2)]), "f splits its diagonal");
        assert_eq!(split.get(&g), Some(&vec![side(1, 3)]), "g splits its diagonal");
        let own = |a, b| [Some(a), Some(b), None];
        let faces = vec![
            (f, vec![t(0, 1, 2), t(0, 2, 3)]),
            (g, vec![own(1, 0), own(2, 1), own(3, 2), own(0, 3)]),
        ];
        assert!(census(&faces, &chords).is_empty(), "g lays a point of its own: no trigger");
        // Point 3 shared with `k` only: `f`'s triangles reach two faces.
        let chords = vec![
            (side(0, 1), [f, g]),
            (side(1, 2), [f, g]),
            (side(2, 3), [f, k]),
            (side(3, 0), [f, k]),
        ];
        let faces = vec![
            (f, vec![t(0, 1, 3), t(1, 2, 3)]),
            (g, vec![t(1, 0, 2)]),
        ];
        assert!(census(&faces, &chords).is_empty(), "corners shared with two faces: no trigger");
    }
}
