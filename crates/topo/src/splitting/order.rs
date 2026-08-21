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
        let r = Vec3::new(T::from_f64(r[0]), T::from_f64(r[1]), T::from_f64(r[2]));
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
        margin: geom_core::MarginDiag::Invalid,
        band,
        predicate: Some("split_join_frame_arm"),
    }))
}

/// Total lexicographic comparison of two on-plane points by their
/// in-plane coordinates through the exact-order band — the
/// **`split_join_order_u`**/**`_v`** predicates.
///
/// # Errors
///
/// [`Indeterminate`] only on the interval lane (an enclosure pair
/// whose difference straddles the hairline without being an exact
/// tie).
pub(super) fn lex_cmp<T: Decide>(
    p: &Point3<T>,
    q: &Point3<T>,
    origin: &Point3<T>,
    frame: (Vec3<T>, Vec3<T>),
    exact: Band,
) -> Result<core::cmp::Ordering, Indeterminate> {
    use core::cmp::Ordering;
    let (wp, wq) = (*p - *origin, *q - *origin);
    for (name, a, b) in [
        ("split_join_order_u", wp.dot(frame.0), wq.dot(frame.0)),
        ("split_join_order_v", wp.dot(frame.1), wq.dot(frame.1)),
    ] {
        match decide(name, Margin::of(a - b), exact)? {
            Sign::Negative => return Ok(Ordering::Less),
            Sign::Positive => return Ok(Ordering::Greater),
            Sign::Zero => {}
        }
    }
    Ok(core::cmp::Ordering::Equal)
}

/// Stable insertion sort of point indices by the in-plane comparator
/// (null-edge counts are small; the quadratic sweep is the documented
/// posture, like mesh's CDT note). Returns the sorted permutation.
///
/// # Errors
///
/// The first comparator escalation, verbatim.
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
    let mut order: Vec<usize> = (0..points.len()).collect();
    for i in 1..order.len() {
        let mut j = i;
        while j > 0 {
            let (a, b) = (order[j - 1], order[j]);
            if lex_cmp(&points[b], &points[a], &plane.origin, frame, exact)?
                == core::cmp::Ordering::Less
            {
                order.swap(j - 1, j);
                j -= 1;
            } else {
                break;
            }
        }
    }
    Ok(order)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use super::*;

    fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
        Point3::new(x, y, z)
    }

    fn plane_y1() -> SplitPlane<f64> {
        SplitPlane {
            origin: p3(0.0, 1.0, 0.0),
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
            p3(2.0, 1.0, 0.0),
            p3(1.0, 1.0, 5.0),
            p3(1.0, 1.0, 5.0), // bit-identical tie with index 1
            p3(1.0, 1.0, -1.0),
            p3(-7.0, 1.0, 9.0),
        ];
        let order = sort_indices_by_point(&pts, &plane_y1(), band, exact).unwrap();
        // u = x ascending; v = (n × u)·w = −z, so larger z sorts first.
        assert_eq!(order, vec![4, 1, 2, 3, 0]);
    }

    /// The comparator is exact: values one ULP apart order strictly
    /// (an ε-banded comparator would call them equal — the
    /// engineered-out fragility).
    #[test]
    fn one_ulp_apart_orders_strictly() {
        let band = Band::linear(Tol::witness()).unwrap();
        let exact = exact_band().unwrap();
        let plane = plane_y1();
        let frame = in_plane_frame(&plane, 1.0, band).unwrap();
        let a = p3(1.0, 1.0, 0.0);
        let b = p3(f64::from_bits(1.0f64.to_bits() + 1), 1.0, 0.0);
        let cmp = |p, q| lex_cmp(&p, &q, &plane.origin, frame, exact).unwrap();
        assert_eq!(cmp(a, b), core::cmp::Ordering::Less);
        assert_eq!(cmp(b, a), core::cmp::Ordering::Greater);
        assert_eq!(cmp(a, a), core::cmp::Ordering::Equal);
    }
}
