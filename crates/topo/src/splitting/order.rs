//! Joining order: the lexicographic null-edge sort (ch. 14 §14.7.1)
//! with a **TOTAL comparator** — the ε-banded `comp` sort of the book
//! (a non-transitive comparator used as a sort key, the notes' flagged
//! robustness landmine) is engineered out per the M3 plan/synthesis.
//!
//! # In-plane keys (the ch. 14 note's own refinement)
//!
//! Every null edge lies ON the split plane, so the plane-normal
//! coordinate carries no ordering information — at `f64` it is a
//! constant, and on the interval lane two independently-computed
//! crossings enclose the same constant in *different* enclosures whose
//! difference straddles zero: sorting on raw (x, y, z) would escalate
//! spuriously. The sort key is therefore the pair of **in-plane
//! coordinates** `(w·u, w·v)`, `w = p − origin`, against a
//! deterministic in-plane frame built from the plane alone: the first
//! member of the fixed [`containment schedule`](super::containment)
//! whose projection into the plane has a definitely-positive length
//! (**`split_join_frame_arm`**) gives `u`; `v = n × u`. For an
//! axis-aligned plane the frame is an exact coordinate pair.
//!
//! # The exact-order band
//!
//! Coordinate differences classify against profile's canonical-form
//! band — `Band::new(f64::from_bits(1), f64::from_bits(2))`: the open
//! interior (min-subnormal, 2·min-subnormal) contains no representable
//! `f64`, so at `f64` the comparison (**`split_join_order_u`** /
//! **`_v`**) is exact and total; a Zero means bit-level coincidence
//! (the coincident null-edge copies land here by construction). At the
//! interval scalar an enclosure straddling the hairline escalates
//! honestly — the replay contract: both lanes sort identically or the
//! interval lane refuses typed.
//!
//! Interval-lane coverage, honestly: any split whose crossings share
//! an in-plane u-coordinate arrived at through INEXACT arithmetic
//! refuses typed (the **`split_join_order_u`** hairline straddles) —
//! in practice the interval lane splits axis-aligned planes over
//! dyadic geometry, and tilted planes refuse. Documented contract,
//! not a bug.
//!
//! # Columns
//!
//! Exactness is right for telling points apart and wrong for one
//! question: whether crossings lie on one line parallel to `v`. The
//! join pairs a face's crossings along the face's section line, so
//! that line's crossings must come out in order along it; on a tilted
//! plane, crossings computed on one such line carry `u` keys that
//! differ in their last bits, and an exact `u` order shuffles them.
//! So `u` is read in **columns** — runs of the `u`-sorted points whose
//! successive gaps are Zero against the run's band
//! (**`split_join_order_column`**) — and `v` orders each column.
//! Chaining makes the column an equivalence by construction; a gap in
//! the band's ambiguity window refuses typed.
//!
//! Ties (both coordinates Zero — distinct null edges at one point,
//! e.g. the two tip-vertex runs of the Fig. 14.2 notch) keep
//! **insertion order** (the reduction's deterministic discovery
//! order): the sort is stable, and the topological neighbor criterion
//! disambiguates join partners at coincident positions.

use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::SplitPlane;
use crate::validate::decide;

/// The exact-order band (module docs; identical to profile's).
///
/// # Errors
///
/// [`BandError`] is structurally impossible for these constants; typed
/// through all the same (no panic paths in operator code).
pub(crate) fn exact_band() -> Result<Band, BandError> {
    Band::new(f64::from_bits(1), f64::from_bits(2))
}

/// The deterministic in-plane frame `(u, v)` (module docs). Returns
/// `Err` with the last arm diagnostics only if every schedule member
/// projects degenerately — unreachable for a unit normal (the three
/// axes are members). `arm` is the caller's lever arm in meters (the
/// spread of the points to be ordered): the SCHEDULE triples are bare
/// numbers, so the projected norm alone would be a dimensionless
/// comparand against the length band (rim-dimensional audit, class
/// (c)); the honest margin is `sin(member, plane NORMAL) × arm` (the
/// member's in-plane fraction `|d|/|r|`) — the in-plane displacement
/// the frame direction commands at the data's own scale.
pub(super) fn in_plane_frame<T: Decide>(
    plane: &SplitPlane<T>,
    arm: T,
    band: Band,
) -> Result<(Vec3<T>, Vec3<T>), Indeterminate> {
    let mut last = None;
    for r in &super::containment::SCHEDULE {
        let r = r.map(T::from_f64);
        let d = r - plane.normal * plane.normal.dot(r);
        match decide(
            "split_join_frame_arm",
            Margin::levered(d.norm() / r.norm(), arm),
            band,
        ) {
            Ok(Sign::Positive) => {
                let u = d.normalize();
                return Ok((u, plane.normal.cross(u)));
            }
            Ok(_) => {}
            Err(diag) => last = Some(diag),
        }
    }
    Err(last.unwrap_or(Indeterminate {
        margin: geom_core::MarginDiag::INVALID,
        band,
        predicate: Some("split_join_frame_arm"),
        terminal_sliver: false,
    }))
}

/// The join order of on-plane points: the columns of `u` in ascending
/// order, each ordered by ascending `v`, ties in insertion order
/// (module docs). Returns the sorted permutation. Null-edge counts are
/// small; the sorts are insertion sorts, like mesh's CDT note.
///
/// # Errors
///
/// The first escalation, verbatim: an exact comparison on the interval
/// lane, or a column gap in the band's ambiguity window.
pub(super) fn sort_indices_by_point<T: Decide>(
    points: &[Point3<T>],
    plane: &SplitPlane<T>,
    band: Band,
    exact: Band,
) -> Result<Vec<usize>, Indeterminate> {
    // Fewer than two points sort trivially — no frame (and no arm)
    // is needed.
    if points.len() < 2 {
        return Ok((0..points.len()).collect());
    }
    // The lever arm for the frame gate: the points' own spread from
    // the plane origin (evaluation-lane fold, meters).
    let mut arm = T::zero();
    for p in points {
        arm = arm.max((*p - plane.origin).norm());
    }
    let frame = in_plane_frame(plane, arm, band)?;
    let key = |i: usize, axis: Vec3<T>| (points[i] - plane.origin).dot(axis);
    let by_u = insertion_sort((0..points.len()).collect(), |a, b| {
        exact_cmp(
            "split_join_order_u",
            key(a, frame.0),
            key(b, frame.0),
            exact,
        )
    })?;
    let mut order = Vec::with_capacity(points.len());
    let mut column: Vec<usize> = Vec::new();
    for &i in &by_u {
        if let Some(&last) = column.last() {
            let gap = key(i, frame.0) - key(last, frame.0);
            if decide("split_join_order_column", Margin::of(gap), band)? == Sign::Positive {
                order.extend(sort_column(&mut column, |a, b| {
                    exact_cmp(
                        "split_join_order_v",
                        key(a, frame.1),
                        key(b, frame.1),
                        exact,
                    )
                })?);
            }
        }
        column.push(i);
    }
    order.extend(sort_column(&mut column, |a, b| {
        exact_cmp(
            "split_join_order_v",
            key(a, frame.1),
            key(b, frame.1),
            exact,
        )
    })?);
    Ok(order)
}

/// One column, back in insertion order and then sorted by `cmp`;
/// leaves `column` empty.
fn sort_column(
    column: &mut Vec<usize>,
    cmp: impl Fn(usize, usize) -> Result<core::cmp::Ordering, Indeterminate>,
) -> Result<Vec<usize>, Indeterminate> {
    let mut members = core::mem::take(column);
    members.sort_unstable();
    insertion_sort(members, cmp)
}

/// Stable insertion sort under a fallible comparator.
fn insertion_sort(
    mut order: Vec<usize>,
    cmp: impl Fn(usize, usize) -> Result<core::cmp::Ordering, Indeterminate>,
) -> Result<Vec<usize>, Indeterminate> {
    for i in 1..order.len() {
        let mut j = i;
        while j > 0 && cmp(order[j], order[j - 1])? == core::cmp::Ordering::Less {
            order.swap(j - 1, j);
            j -= 1;
        }
    }
    Ok(order)
}

/// One in-plane coordinate pair compared through the exact-order band.
fn exact_cmp<T: Decide>(
    name: &'static str,
    a: T,
    b: T,
    exact: Band,
) -> Result<core::cmp::Ordering, Indeterminate> {
    Ok(match decide(name, Margin::of(a - b), exact)? {
        Sign::Negative => core::cmp::Ordering::Less,
        Sign::Positive => core::cmp::Ordering::Greater,
        Sign::Zero => core::cmp::Ordering::Equal,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    fn plane_y1() -> SplitPlane<f64> {
        SplitPlane {
            origin: Point3::new(0.0, 1.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    /// Totality and stability on the y = 1 plane: keys are (x, z)
    /// exactly (u = e_x, v = −e_z for n = e_y); bit-identical ties
    /// keep insertion order.
    #[test]
    fn lex_sort_total_and_stable() {
        let band = Band::linear(Tol::witness()).unwrap();
        let exact = exact_band().unwrap();
        let pts = [
            Point3::new(2.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 5.0),
            Point3::new(1.0, 1.0, 5.0), // bit-identical tie with index 1
            Point3::new(1.0, 1.0, -1.0),
            Point3::new(-7.0, 1.0, 9.0),
        ];
        let order = sort_indices_by_point(&pts, &plane_y1(), band, exact).unwrap();
        // u = x ascending; v = (n × u)·w = −z, so larger z sorts first.
        assert_eq!(order, vec![4, 1, 2, 3, 0]);
    }

    /// A `u` one ULP apart is one column, ordered by `v`; a `v` one
    /// ULP apart orders strictly. The four crossings of a ringed cap on
    /// a tilted plane carry `u` keys that differ in the last bits
    /// (`a_split_through_a_ringed_cap_chords_it_beside_its_hole`, in
    /// sweep's `split_section_rings`), and ordering on those bits is
    /// what chorded the cap across its hole.
    #[test]
    fn a_column_one_ulp_wide_orders_by_v_and_v_orders_exactly() {
        let band = Band::linear(Tol::witness()).unwrap();
        let exact = exact_band().unwrap();
        let up = |x: f64| f64::from_bits(x.to_bits() + 1);
        // On y = 1, u = x and v = −z: the column x ≈ 1 runs along z.
        let pts = [
            Point3::new(up(1.0), 1.0, 2.0),
            Point3::new(1.0, 1.0, -2.0),
            Point3::new(up(up(1.0)), 1.0, 1.0),
            Point3::new(1.0, 1.0, up(1.0)),
            Point3::new(0.5, 1.0, -9.0),
        ];
        let order = sort_indices_by_point(&pts, &plane_y1(), band, exact).unwrap();
        assert_eq!(
            order,
            vec![4, 0, 3, 2, 1],
            "x = 0.5 first, then the column x ≈ 1 by descending z, \
             z one ULP above 1 strictly first"
        );
    }

    /// A `u` gap inside the band's ambiguity window refuses rather than
    /// guess which column the point is in.
    #[test]
    fn a_column_gap_in_the_window_refuses() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let exact = exact_band().unwrap();
        let mid = 0.5 * (1.0 + tol.k()) * tol.eps();
        let pts = [Point3::new(1.0, 1.0, 0.0), Point3::new(1.0 + mid, 1.0, 1.0)];
        let err = sort_indices_by_point(&pts, &plane_y1(), band, exact).unwrap_err();
        assert_eq!(err.predicate, Some("split_join_order_column"));
    }
}
