//! **The walk of a ruled face's two rows**: how the chart lane writes the
//! triangles of a cylinder or cone face whose straight side holds a
//! point (`walk::splits_a_straight_side`). No lattice and no
//! triangulation step: the face is straight along its rulings, so its
//! triangles join its two rows (its rims, or a rim and the apex) in order
//! of angle, and a point on a side divides the one triangle on that side.
//!
//! # The rows
//!
//! The face's loop is a cycle of rims (circle traversals) and straight
//! sides (line traversals), and its shape is one of two: two runs of rims
//! with a run of sides between them each way (a cylinder face, a frustum
//! face, a one-face full turn whose seam is both of its sides), or one
//! run of rims whose sides meet at the cone's apex, the second row then
//! the apex alone ([`rim_runs`], [`Rows::of`]).
//!
//! # Why every walked triangle is within δ
//!
//! Write `s` for a point's angle along row `a`'s direction. The front
//! edge `a_i–b_j` keeps `max(s(a_i), s(b_j)) ≤ min(s(a_{i+1}), s(b_{j+1}))`:
//! it holds at the start, where both rows begin on one side, and the walk
//! advances the row whose next point comes first, which keeps it. So
//! the triangle `(a_i, a_{i+1}, b_j)`, written when
//! `s(a_{i+1}) ≤ s(b_{j+1})`, has its corners' angles in `[s(a_i),
//! s(a_{i+1})]` when `s(b_j) ≥ s(a_i)`, and in `[s(b_j), s(b_{j+1})]`
//! otherwise: one chord of row `a` or one of row `b`, and the same for
//! the triangle advancing `b`. A triangle whose corners lie on the
//! surface between two rulings lies in the hull of that piece of
//! surface, and how far that hull stands from the surface is the column
//! step's bound (`sizing::sagitta_step`, `sizing::cone_grid_step`), at
//! which the chord pass chords each rim (`chords::tighten`). A point on
//! a side lies on that side's ruling, a straight segment in 3-D, so the
//! triangles it fans divide one walked triangle in its plane.
//!
//! Each triangle is still certified ([`crate::cert`]): the walk is how
//! the triangles are made, and the certificate is what holds them.
//!
//! # No edge of four triangles
//!
//! An edge two faces both make between two of their shared boundary
//! points bounds four triangles; the chart lane's CDT keeps none (its ear
//! and shared-side cures, `crate::trimmed`). A walked face needs no such
//! cure. Each of its edges spans at most one rim chord in angle: a rim
//! chord or a piece of a side (neighbours on the polygon), an edge
//! between the rows, or a fan edge from a side's point to the corner
//! across from that side. A neighbour holds points of this face's
//! boundary only along the edges it shares with it, so two points it holds
//! that are not neighbours on the polygon lie on this face's two sides,
//! or on a side and the rim corner at the other side's foot: the face's
//! whole span apart. The walk joins such points only on a face of one rim
//! chord, at most `sizing::MAX_ANGULAR_STEP` wide, and the neighbour then
//! cannot make the edge as well:
//!
//! - a plane holding both sides closes the face's rims with chords that
//!   are the rims' own, so its caps are two coincident chords, which do
//!   not mesh;
//! - another walked or rectangle-lane face on the same surface spans the
//!   rest of the turn, more than one chord, and its edges span one;
//! - a face of the CDT would make it as a side of a triangle spanning
//!   that rest of the turn: on a cone its own shared-side rule splits
//!   such a triangle, and on a cylinder its certificate refuses it unless
//!   every chord of the surface is within δ (a radius under δ/2), and even
//!   there the edge is the diagonal of its whole chart, across its
//!   lattice. No body here reaches that last corner.

use std::collections::HashSet;

/// How many runs of rims a face's loop has, given whether each of its
/// traversals is a line, in loop order. A face is walked with one run
/// (the second row is the apex) or two.
pub(crate) fn rim_runs(line: &[bool]) -> usize {
    let n = line.len();
    (0..n)
        .filter(|&i| !line[i] && line[(i + n - 1) % n])
        .count()
}

/// A walked face's two rows and its sides, by index into the trim
/// polygon's points: `a` and `b` each run one way in angle, from the
/// side `start` to the side `end`. `start` holds the side's points
/// strictly between `b[0]` and `a[0]` in polygon order, and `end` those
/// strictly between `a[last]` and `b[last]`. A row of the apex is
/// `b = [apex]`.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Rows {
    a: Vec<usize>,
    b: Vec<usize>,
    start: Vec<usize>,
    end: Vec<usize>,
}

impl Rows {
    /// The rows of a one-loop polygon `points` whose point `i` starts a
    /// traversal that is a line when `line[i]`, with `singular` the mesh
    /// ids of the chart's singular point; `None` where its shape is
    /// neither of [`rim_runs`]' two, or one run of rims has no apex
    /// between its sides.
    pub(crate) fn of(
        points: &[(f64, f64, u32)],
        line: &[bool],
        singular: &HashSet<u32>,
    ) -> Option<Self> {
        let n = points.len();
        let starts: Vec<usize> = (0..n)
            .filter(|&i| !line[i] && line[(i + n - 1) % n])
            .collect();
        // A run of rims: its points to the corner its last chord reaches.
        let run = |i: usize| {
            let mut v = vec![i];
            let mut k = i;
            while !line[k] {
                k = (k + 1) % n;
                v.push(k);
            }
            v
        };
        let between = |from: usize, to: usize| {
            let mut v = Vec::new();
            let mut k = (from + 1) % n;
            while k != to {
                v.push(k);
                k = (k + 1) % n;
            }
            v
        };
        match *starts.as_slice() {
            [first, second] => {
                let (a, mut b) = (run(first), run(second));
                let end = between(*a.last()?, b[0]);
                let start = between(*b.last()?, a[0]);
                b.reverse();
                Some(Self { a, b, start, end })
            }
            [only] => {
                let a = run(only);
                let sides = between(*a.last()?, a[0]);
                let apex = |k: &usize| singular.contains(&points[*k].2);
                let p = sides.iter().position(apex)?;
                let q = sides.iter().rposition(apex)?;
                Some(Self {
                    b: vec![sides[p]],
                    end: sides[..p].to_vec(),
                    start: sides[q + 1..].to_vec(),
                    a,
                })
            }
            _ => None,
        }
    }

    /// **The walk** (module docs): the face's triangles as polygon index
    /// triples, wound as the polygon is.
    ///
    /// Each triangle's first side runs the polygon's way: `a_i → a_{i+1}`
    /// on row `a`, `b_{j+1} → b_j` on row `b`, which the polygon walks
    /// backwards. A split side's points are fanned from the corner
    /// opposite the side in the one triangle that has the side's two
    /// ends as an edge.
    pub(crate) fn walk(&self, points: &[(f64, f64, u32)]) -> Vec<[usize; 3]> {
        let (a, b) = (&self.a, &self.b);
        let (n, m) = (a.len() - 1, b.len() - 1);
        let dir = if points[a[n]].0 >= points[a[0]].0 {
            1.0
        } else {
            -1.0
        };
        let s = |k: usize| dir * points[k].0;
        let mut out: Vec<[usize; 3]> = Vec::with_capacity(n + m + 2);
        let (mut i, mut j) = (0, 0);
        while i < n || j < m {
            if i < n && (j == m || s(a[i + 1]) <= s(b[j + 1])) {
                out.push([a[i], a[i + 1], b[j]]);
                i += 1;
            } else {
                out.push([b[j + 1], b[j], a[i]]);
                j += 1;
            }
        }
        for (from, side, to) in [(b[0], &self.start, a[0]), (a[n], &self.end, b[m])] {
            if side.is_empty() {
                continue;
            }
            // The side `from → to` is an edge of exactly one triangle, the
            // first or the last walked.
            let (t, q) = out
                .iter()
                .enumerate()
                .find_map(|(t, tri)| {
                    (0..3)
                        .find(|&q| tri[q] == from && tri[(q + 1) % 3] == to)
                        .map(|q| (t, q))
                })
                .unwrap_or_else(|| {
                    unreachable!("the walk's first and last triangles hold its two sides")
                });
            let corner = out.swap_remove(t)[(q + 2) % 3];
            let chain: Vec<usize> = std::iter::once(from)
                .chain(side.iter().copied())
                .chain(std::iter::once(to))
                .collect();
            out.extend(chain.windows(2).map(|w| [corner, w[0], w[1]]));
        }
        out
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Rows, rim_runs};
    use std::collections::HashSet;

    /// The signed area of a chart triangle.
    fn area(p: &[(f64, f64, u32)], t: [usize; 3]) -> f64 {
        let [a, b, c] = t.map(|k| (p[k].0, p[k].1));
        (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1)
    }

    /// A chart rectangle `[0, 1] × [0, 1]` walked counter-clockwise: row
    /// `v = 0` at `us_a`, its side `u = 1` holding `right` points, row
    /// `v = 1` at `us_b` walked back, its side `u = 0` holding `left`.
    fn rectangle(us_a: &[f64], right: &[f64], us_b: &[f64], left: &[f64]) -> Vec<(f64, f64, u32)> {
        let mut p: Vec<(f64, f64)> = us_a.iter().map(|&u| (u, 0.0)).collect();
        p.extend(right.iter().map(|&v| (1.0, v)));
        p.extend(us_b.iter().rev().map(|&u| (u, 1.0)));
        p.extend(left.iter().rev().map(|&v| (0.0, v)));
        #[allow(clippy::cast_possible_truncation)]
        p.into_iter()
            .enumerate()
            .map(|(k, (u, v))| (u, v, k as u32))
            .collect()
    }

    /// Each point's traversal is a line exactly on the two sides.
    fn lines(p: &[(f64, f64, u32)]) -> Vec<bool> {
        let n = p.len();
        (0..n)
            .map(|i| {
                let (q, r) = (p[i], p[(i + 1) % n]);
                (q.0 == r.0) && (q.0 == 0.0 || q.0 == 1.0)
            })
            .collect()
    }

    /// **The walk covers the face once, each triangle within one chord of
    /// one row, a side point adding one triangle.** On chart rectangles
    /// whose rows are chorded at different counts and steps, with zero,
    /// one and three points on either side: the triangles are wound as
    /// the polygon is (positive area), their areas sum to the
    /// rectangle's, there are `boundary points − 2` of them, and each
    /// one's corners lie within one chord of row `a` or of row `b` in
    /// `u`. A walk advancing the wrong row, a fan from the wrong corner, or
    /// a side left unfanned breaks one of these.
    #[test]
    fn the_walk_covers_the_face_once_within_one_chord() {
        let rows: [&[f64]; 4] = [
            &[0.0, 0.5, 1.0],
            &[0.0, 0.2, 0.4, 0.6, 0.8, 1.0],
            &[0.0, 0.13, 0.5, 0.51, 0.97, 1.0],
            &[0.0, 1.0],
        ];
        let sides: [&[f64]; 3] = [&[], &[0.5], &[0.2, 0.5, 0.9]];
        let mut cases = 0;
        for &ra in &rows {
            for &rb in &rows {
                for &right in &sides {
                    for &left in &sides {
                        let p = rectangle(ra, right, rb, left);
                        let rows = Rows::of(&p, &lines(&p), &HashSet::new()).unwrap();
                        let tris = rows.walk(&p);
                        let what = format!("rows {ra:?} / {rb:?}, sides {right:?} / {left:?}");
                        assert_eq!(tris.len(), p.len() - 2, "{what}: count");
                        let total: f64 = tris.iter().map(|&t| area(&p, t)).sum();
                        assert!((total - 2.0).abs() < 1e-12, "{what}: area {total}");
                        let chord = |row: &[f64], lo: f64, hi: f64| {
                            row.windows(2).any(|w| w[0] <= lo && hi <= w[1])
                        };
                        for &t in &tris {
                            assert!(area(&p, t) > 0.0, "{what}: {t:?} wound against the polygon");
                            let us = t.map(|k| p[k].0);
                            let (lo, hi) = (us[0].min(us[1]).min(us[2]), us[0].max(us[1]).max(us[2]));
                            assert!(
                                chord(ra, lo, hi) || chord(rb, lo, hi),
                                "{what}: {t:?} spans [{lo}, {hi}], more than one chord"
                            );
                        }
                        cases += 1;
                    }
                }
            }
        }
        assert_eq!(cases, 144);
    }

    /// **A face through the apex walks to it.** A chart triangle, row `a`
    /// at `v = 1` and the apex the point `(0, 0)` with id 99 on side `u =
    /// 0`, a point on each side: two walked triangles to the apex, and
    /// each side's point fanned from the rim point next to the side.
    #[test]
    fn a_face_through_the_apex_walks_to_it() {
        // a: (0,1) (0.5,1) (1,1); end side (1, 0.5); apex; start side (0, 0.5).
        let p = vec![
            (1.0, 1.0, 0),
            (0.5, 1.0, 1),
            (0.0, 1.0, 2),
            (0.0, 0.5, 3),
            (0.0, 0.0, 99),
            (1.0, 0.5, 5),
        ];
        let line = [false, false, true, true, true, true];
        assert_eq!(rim_runs(&line), 1);
        let rows = Rows::of(&p, &line, &HashSet::from([99])).unwrap();
        assert_eq!(
            rows,
            Rows {
                a: vec![0, 1, 2],
                b: vec![4],
                start: vec![5],
                end: vec![3],
            }
        );
        let mut tris = rows.walk(&p);
        tris.sort_unstable();
        assert_eq!(tris, vec![[1, 2, 3], [1, 3, 4], [1, 4, 5], [1, 5, 0]]);
    }
}
