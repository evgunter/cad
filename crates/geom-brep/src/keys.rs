//! Typed keys into a body's geometry arenas.
//!
//! These are slotmap key *types*, not arenas: the arenas themselves live
//! in `topo`'s `Body<T>` (points, curves, surfaces), which re-exports
//! these types unchanged. They are defined **here**, one layer below
//! `topo`, because D2's intensional edge descriptions reference surfaces
//! *by arena key* ([`crate::EdgeDescription::Intersection`] holds two
//! `SurfaceKey`s) — the description layer has to be able to name arena
//! slots without depending on the arena store above it.
//!
//! # Lineage scoping (Q1)
//!
//! A key is meaningful only within the lineage of the body that minted
//! it. The full stale-vs-foreign story is documented in the `topo::body`
//! module docs (their `Key validity` section);
//! the one-line consequence for this crate: certification never resolves
//! keys itself — resolution is injected by the caller (a lookup closure
//! over the owning body's arenas), so a key never silently crosses
//! lineages inside this layer.

use slotmap::new_key_type;

new_key_type! {
    /// Typed key into a body's point arena (vertex geometry).
    pub struct PointKey;

    /// Typed key into a body's curve arena (edge geometry).
    pub struct CurveKey;

    /// Typed key into a body's surface arena (face geometry).
    pub struct SurfaceKey;
}

/// The two surfaces an intrinsic edge description names (D2): the
/// locus is a component of S₁∩S₂, and intersection is symmetric, so
/// the pair is a SET. The one constructor stores the keys in key
/// order, so two builders of one locus mint equal pairs whatever
/// order they named the surfaces in.
///
/// Equal keys are representable: a description naming one surface
/// twice is refused at the certification door
/// (`IntersectionSameSurface`), where the other geometric refusals
/// are, not here.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfacePair {
    lo: SurfaceKey,
    hi: SurfaceKey,
}

impl SurfacePair {
    /// The pair `{a, b}`.
    pub fn new(a: SurfaceKey, b: SurfaceKey) -> Self {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        Self { lo, hi }
    }

    /// The pair `{ka, kb}` with `a` and `b`, the values for `ka` and
    /// `kb`, in the pair's key order: the order every reading that is
    /// not symmetric in its two surfaces takes them in, so a
    /// constructor and the certificate read one order.
    pub fn sorted<V>(ka: SurfaceKey, a: V, kb: SurfaceKey, b: V) -> (Self, [V; 2]) {
        let pair = Self::new(ka, kb);
        (pair, if ka <= kb { [a, b] } else { [b, a] })
    }

    /// Both keys, in key order.
    pub fn keys(self) -> [SurfaceKey; 2] {
        [self.lo, self.hi]
    }

    /// Whether `k` is one of the two.
    pub fn contains(self, k: SurfaceKey) -> bool {
        self.lo == k || self.hi == k
    }

    /// The member that is not `k`, or `None` when `k` is not a member.
    /// For a pair naming one surface twice, `other(k)` is `k`.
    pub fn other(self, k: SurfaceKey) -> Option<SurfaceKey> {
        if self.lo == k {
            Some(self.hi)
        } else if self.hi == k {
            Some(self.lo)
        } else {
            None
        }
    }

    /// The pair with each key passed through `f`, rebuilt through
    /// [`SurfacePair::new`] so the result is in key order of the NEW
    /// keys. A failed lookup aborts the remap.
    pub fn try_map<E>(
        self,
        mut f: impl FnMut(SurfaceKey) -> Result<SurfaceKey, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(f(self.lo)?, f(self.hi)?))
    }

    /// [`SurfacePair::try_map`] for an infallible `f`.
    pub fn map(self, mut f: impl FnMut(SurfaceKey) -> SurfaceKey) -> Self {
        Self::new(f(self.lo), f(self.hi))
    }
}

impl core::fmt::Debug for SurfacePair {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { lo, hi } = self;
        f.debug_set().entry(lo).entry(hi).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::SlotMap;

    fn three() -> [SurfaceKey; 3] {
        let mut arena: SlotMap<SurfaceKey, ()> = SlotMap::with_key();
        [arena.insert(()), arena.insert(()), arena.insert(())]
    }

    #[test]
    fn surface_pair_is_unordered() {
        let [a, b, c] = three();
        assert_eq!(
            SurfacePair::new(a, b),
            SurfacePair::new(b, a),
            "order of naming is not stored"
        );
        assert_eq!(
            format!("{:?}", SurfacePair::new(a, b)),
            format!("{:?}", SurfacePair::new(b, a)),
            "Debug is order-free"
        );
        assert_ne!(
            SurfacePair::new(a, b),
            SurfacePair::new(a, c),
            "different sets differ"
        );
        assert_eq!(
            SurfacePair::new(b, a).keys(),
            [a, b],
            "keys come back in key order"
        );
        assert_eq!(
            SurfacePair::sorted(b, "b", a, "a"),
            (SurfacePair::new(a, b), ["a", "b"]),
            "values follow their keys into key order"
        );
    }

    #[test]
    fn surface_pair_membership_and_other() {
        let [a, b, c] = three();
        let p = SurfacePair::new(b, a);
        assert!(
            p.contains(a) && p.contains(b) && !p.contains(c),
            "contains is membership"
        );
        assert_eq!(
            (p.other(a), p.other(b), p.other(c)),
            (Some(b), Some(a), None),
            "other is the complement"
        );
        assert_eq!(
            SurfacePair::new(a, a).other(a),
            Some(a),
            "a doubled key is its own other"
        );
    }

    #[test]
    fn surface_pair_remap_goes_back_through_the_constructor() {
        let [a, b, c] = three();
        // a ↦ c reverses the key order; the result is still a canonical pair.
        let swapped = SurfacePair::new(a, b).map(|k| if k == a { c } else { k });
        assert_eq!(swapped.keys(), [b, c], "remap re-sorts");
        assert_eq!(swapped, SurfacePair::new(c, b), "remap equals a fresh mint");
        let r: Result<SurfacePair, ()> =
            SurfacePair::new(a, b).try_map(|k| if k == b { Err(()) } else { Ok(k) });
        assert_eq!(r, Err(()), "a failed lookup aborts");
    }
}
