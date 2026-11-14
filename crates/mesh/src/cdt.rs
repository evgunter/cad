//! **A mesh point as spade sees it**, and the equality spade reads it
//! by: the one home of both rules, called by every lane that hands a
//! point to a CDT (`planar`'s insert feed, `curved`'s `insert`,
//! `trimmed`'s `cdt_point`) and by [`ids_at_two_uvs`], which every
//! lane's "one mesh vertex at two parameter locations" reasoning reads.
//! The rules are spade's, not any lane's.
//!
//! # The coordinate floor
//!
//! Spade's `insert` and `bulk_load_cdt` refuse a coordinate in the open
//! band `(0, MIN_ALLOWED_VALUE)`, `MIN_ALLOWED_VALUE` = 2⁻¹⁴²: the
//! bottom of the exponent range over which its exact predicates neither
//! underflow nor overflow (Shewchuk). No chart coordinate a lane means
//! is that small. One that lands there is the float residue of a
//! quantity that is zero in real arithmetic: a determinant with a
//! repeated row, or a dot product against an axis whose exact-zero
//! components came out as ulps — `planar`'s chart frame on noisy
//! positions, a turned ball's seam column (`atan2` of the meridian's
//! mid point against a `v_ref` an ulp off its axis).
//!
//! [`spade_point`] passes every point through spade's own
//! [`mitigate_underflow`], which writes `0.0` for a coordinate in that
//! band and moves nothing else: the floor itself, every value above it,
//! `-0.0`, NaN and ±∞ pass bit-identical. A subnormal is inside the band
//! and becomes `0.0`.
//!
//! **That is not value snapping.** Snapping moves a coordinate the
//! construction produced onto a nearby one it did not, manufacturing
//! an agreement. Nothing in the band is a value the CDT may keep; the
//! choice the floor offers is never "corrupt a real value or not", only
//! "mesh, or refuse" over values spade accepts in no case. It declines
//! to invent a nonzero the construction did not have.
//!
//! **What it knows is nothing**, and a site that knows WHY a coordinate
//! is zero still writes the zero itself (`planar`'s far-point write);
//! the floor is the backstop under every such site, not a substitute
//! for one.
//!
//! **Genuine degeneracy still refuses.** NaN and infinity pass through,
//! so a degenerate frame or chart reaches `insert` and refuses typed
//! (`TessellateError::Triangulation`).
//!
//! The band collapses distinct values: `-4.8e-49` and `0.0` are one
//! spade point. So "distinct" below means distinct AFTER the floor.

use std::collections::{HashMap, HashSet};

use spade::{Point2 as SpadePoint, mitigate_underflow};

/// The point spade sees for chart coordinates `(u, v)` (module docs,
/// "The coordinate floor").
pub(crate) fn spade_point(u: f64, v: f64) -> SpadePoint<f64> {
    mitigate_underflow(SpadePoint::new(u, v))
}

/// The ids that appear at two or more DISTINCT UV locations among
/// `entries` — the "one mesh vertex, two parameter locations" state
/// that both tessellation lanes have to reason about.
///
/// **"Distinct" means what SPADE means by it**: two entries are at one
/// location when their [`spade_point`]s are `==`, which is spade's
/// vertex lookup (`PartialEq` on `Point2<f64>`). So `-0.0` and `0.0`
/// are one location, and so are a sub-floor coordinate and `0.0`; a
/// `to_bits` or a raw `f64` compare would read "apart" exactly where
/// spade dedupes. Two entries spade merges are one CDT vertex and
/// cannot be fanned apart; two it keeps can.
///
/// A lane whose CDT applies a further map (the trimmed lane's lattice
/// metric) keeps this agreement only if that map is injective on
/// [`spade_point`]'s values; `trimmed`'s `cdt_point` says why it is.
pub(crate) fn ids_at_two_uvs(entries: impl IntoIterator<Item = (f64, f64, u32)>) -> HashSet<u32> {
    let mut seen: HashMap<u32, SpadePoint<f64>> = HashMap::new();
    let mut repeated: HashSet<u32> = HashSet::new();
    for (u, v, id) in entries {
        let at = spade_point(u, v);
        if seen.insert(id, at).is_some_and(|p| p != at) {
            repeated.insert(id);
        }
    }
    repeated
}

#[cfg(test)]
mod tests {
    use super::{ids_at_two_uvs, spade_point};
    use spade::MIN_ALLOWED_VALUE;

    /// Both halves of the floor, at the values that tell an exact floor
    /// from an over- or under-reaching one: every sub-floor nonzero of
    /// either sign (a subnormal included) enters as `0.0`, and every
    /// value spade accepts enters bit-identical — the floor itself, the
    /// next value up, a small legal value, `-0.0`, NaN and ±∞.
    #[test]
    fn the_floor_moves_the_band_and_nothing_else() {
        let below = [
            -4.838_220_930_617_058e-49,
            4.838_220_930_617_058e-49,
            MIN_ALLOWED_VALUE.next_down(),
            (-MIN_ALLOWED_VALUE).next_up(),
            f64::from_bits(1),
            -f64::MIN_POSITIVE,
        ];
        for c in below {
            let p = spade_point(c, c);
            assert_eq!(
                (p.x.to_bits(), p.y.to_bits()),
                (0.0f64.to_bits(), 0.0f64.to_bits()),
                "{c:e} is inside spade's refused band and must enter as +0.0"
            );
        }
        let kept = [
            MIN_ALLOWED_VALUE,
            -MIN_ALLOWED_VALUE,
            MIN_ALLOWED_VALUE.next_up(),
            1e-30,
            -0.0,
            0.0,
            core::f64::consts::PI,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ];
        for c in kept {
            let p = spade_point(c, 1.0);
            assert_eq!(
                p.x.to_bits(),
                c.to_bits(),
                "{c:e} is a value spade accepts or refuses typed; it must pass bit-identical"
            );
            assert!(
                spade::validate_vertex(&p).is_ok() == (c.is_finite()),
                "{c:e}: legal exactly when finite once floored"
            );
        }
    }

    /// "Distinct" is distinct after the floor: a sub-floor coordinate
    /// and `0.0` are one location, `-0.0` and `0.0` are one, and an ulp
    /// apart at seam magnitude is two.
    #[test]
    fn distinct_is_distinct_to_spade() {
        let dust = [(-4.8e-49, 1.0, 7u32), (1.0, 0.5, 8), (0.0, 1.0, 7)];
        assert!(
            ids_at_two_uvs(dust).is_empty(),
            "a sub-floor u and 0.0 are one spade vertex"
        );
        let signed_zero = [(0.0, 0.0, 7u32), (-0.0, 0.0, 7)];
        assert!(ids_at_two_uvs(signed_zero).is_empty(), "-0.0 == 0.0");
        let seam = core::f64::consts::TAU;
        let ulp = [(seam, 0.0, 7u32), (seam.next_up(), 0.0, 7)];
        assert_eq!(
            ids_at_two_uvs(ulp).into_iter().collect::<Vec<_>>(),
            vec![7],
            "an ulp apart at 2π is two spade vertices"
        );
        let floor = [(MIN_ALLOWED_VALUE, 0.0, 7u32), (0.0, 0.0, 7)];
        assert_eq!(
            ids_at_two_uvs(floor).into_iter().collect::<Vec<_>>(),
            vec![7],
            "the floor itself is a value spade keeps apart from 0.0"
        );
    }
}
