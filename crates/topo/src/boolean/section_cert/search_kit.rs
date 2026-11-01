//! The section certificate's test kit shared by its rows and its
//! counterexample searches: the run's band, and the searches' draws.

#![allow(clippy::unwrap_used)]

use geom_core::{Band, Tol, Vec3};
use test_utils::fuzz::Rng;

/// The run's linear band.
pub(super) fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A uniformly random unit vector.
pub(super) fn unit_vec(rng: &mut Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.norm();
        if n > 0.2 && n < 1.0 {
            return v / n;
        }
    }
}

/// An offset from a margin's zero, relative, either sign: half the
/// draws log-uniform in `[0.03, 0.5]`, which a trace resolves, half in
/// `[1e-12, 0.5]`, into the band and through it.
pub(super) fn offset(rng: &mut Rng) -> f64 {
    let floor = if rng.unit() < 0.5 { 0.03f64 } else { 1e-12 };
    let m = (rng.range(floor.ln(), 0.5f64.ln())).exp();
    if rng.unit() < 0.5 { m } else { -m }
}

/// Whether every witness can take a component of its own from its
/// `candidates`, no two the same (augmenting paths).
pub(super) fn matched(candidates: &[Vec<usize>]) -> bool {
    fn augment(
        w: usize,
        candidates: &[Vec<usize>],
        owner: &mut std::collections::BTreeMap<usize, usize>,
        tried: &mut Vec<usize>,
    ) -> bool {
        for &c in &candidates[w] {
            if tried.contains(&c) {
                continue;
            }
            tried.push(c);
            let free = match owner.get(&c) {
                None => true,
                Some(&other) => augment(other, candidates, owner, tried),
            };
            if free {
                owner.insert(c, w);
                return true;
            }
        }
        false
    }
    let mut owner = std::collections::BTreeMap::new();
    (0..candidates.len()).all(|w| augment(w, candidates, &mut owner, &mut Vec::new()))
}

/// The root of `x`'s set in a union-find forest over a trace's cells,
/// halving the path on the way.
pub(super) fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

/// Joins the sets of cells `x` and `y`.
pub(super) fn join(parent: &mut [usize], x: usize, y: usize) {
    let (x, y) = (find(parent, x), find(parent, y));
    parent[x] = y;
}

/// Each active cell's component, numbered in cell order, and how many
/// there are.
pub(super) fn label(parent: &mut [usize], active: &[bool]) -> (Vec<Option<usize>>, usize) {
    let mut index = std::collections::BTreeMap::new();
    let comp = (0..active.len())
        .map(|k| {
            active[k].then(|| {
                let root = find(parent, k);
                let next = index.len();
                *index.entry(root).or_insert(next)
            })
        })
        .collect();
    (comp, index.len())
}

/// The components crossing the cells within `reach` of cell `(i, j)` on
/// a chart of `nu` columns, periodic, and `nv` rows (cell `j·nu + i`),
/// the rows periodic too where `wrap` is set.
pub(super) fn near_cells(
    comp: &[Option<usize>],
    (nu, nv): (usize, usize),
    (i, j): (isize, isize),
    reach: isize,
    wrap: bool,
) -> Vec<usize> {
    let mut out = Vec::new();
    for dj in -reach..=reach {
        let jj = j + dj;
        let jj = if wrap {
            jj.rem_euclid(nv as isize)
        } else if (0..nv as isize).contains(&jj) {
            jj
        } else {
            continue;
        };
        for di in -reach..=reach {
            let ii = (i + di).rem_euclid(nu as isize);
            if let Some(c) = comp[jj as usize * nu + ii as usize]
                && !out.contains(&c)
            {
                out.push(c);
            }
        }
    }
    out
}
