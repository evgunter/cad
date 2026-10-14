//! A union–find whose class root is the class's LEAST key, so the root
//! of a class depends only on the class, never on the order its unions
//! were made in — the one shape every emitter that groups by
//! connectivity needs, since a name may not depend on visiting order.

use std::collections::BTreeMap;

/// Disjoint classes over ordered keys, each rooted at its least key.
pub(crate) struct LeastRoot<K: Ord + Copy> {
    link: BTreeMap<K, K>,
}

impl<K: Ord + Copy> LeastRoot<K> {
    pub(crate) fn new() -> Self {
        Self {
            link: BTreeMap::new(),
        }
    }

    /// Adds `k` as a class of its own, if it is not in one yet.
    pub(crate) fn insert(&mut self, k: K) {
        self.link.entry(k).or_insert(k);
    }

    /// The root of `k`'s class (`k` itself for a key never inserted).
    pub(crate) fn root(&self, mut k: K) -> K {
        while let Some(&up) = self.link.get(&k).filter(|up| **up != k) {
            k = up;
        }
        k
    }

    /// Joins the classes of `a` and `b`, inserting either as needed.
    pub(crate) fn join(&mut self, a: K, b: K) {
        self.insert(a);
        self.insert(b);
        let (ra, rb) = (self.root(a), self.root(b));
        self.link.insert(ra.max(rb), ra.min(rb));
    }

    /// Every inserted key.
    pub(crate) fn keys(&self) -> impl Iterator<Item = K> + '_ {
        self.link.keys().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::LeastRoot;

    #[test]
    fn a_root_is_the_least_key_whatever_the_union_order() {
        for order in [[(3, 1), (1, 2)], [(1, 2), (3, 1)], [(2, 1), (1, 3)]] {
            let mut uf = LeastRoot::new();
            uf.insert(4);
            for (a, b) in order {
                uf.join(a, b);
            }
            for k in [1, 2, 3] {
                assert_eq!(uf.root(k), 1, "{order:?}: {k}");
            }
            assert_eq!(uf.root(4), 4, "{order:?}: an unjoined key is its own root");
        }
    }
}
