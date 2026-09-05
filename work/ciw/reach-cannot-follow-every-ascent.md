---
id: reach-cannot-follow-every-ascent
kind: issue
title: The read reach's chain resolver does not follow every ascent, and the fail-closed sweep is not total
status: open
opened: 2026-09-05
refs: [closure-reaches-tree-wide-guards, 1889]
---

Disclosed by `closure-reaches-tree-wide-guards`, which is the unit that built
the reach. It is a residue of that unit, not a defect it introduced: every case
below is one the DEPENDENT closure alone also misses, and misses more widely.

`scripts/ci-filter.py`'s `_resolve_chain` follows `join`/`push` with a string
literal, `parent`/`pop`, `ancestors`, the `concat!` and array spellings, and one
level of `let`/`fn` binding. `_file_destinations` then sweeps every
`..`-bearing literal no chain accounted for and measures it **from the crate
directory**, so an unfollowable spelling still lands somewhere real.

**What is left.** An ascent whose base is an expression the resolver cannot
follow AND whose literal climbs no further than a sibling is measured at the
sibling rather than at the root. Concretely: a guard that reaches the
repository root through two `..`-bearing literals joined onto two different
unfollowable bases would be read as reaching two siblings.

**What is under it.** `classify` bails to TIER=all when the reach finds no
tree-wide guard at all, so a scanner that stops reading wholesale is loud. That
is a floor against the derivation breaking, not against one guard being read
too shallowly.

**What would close it.** Following the chain through more than one binding
level, or through a helper in another crate. Neither is measured as needed:
the tree today spells every ascent five ways and the resolver reads all five
(`crates/test-utils/tests/reader_census.rs:284`,
`crates/geom-core/tests/bounds_census.rs:381`,
`crates/geom-core/tests/flagged_census.rs:234`,
`crates/bvh/tests/aggregator_headers.rs:63`,
`crates/pncad-py/src/prose_census.rs:170`,
`crates/mesh/tests/profile_overrides.rs:104`), and no sixth spelling exists to
measure against.
