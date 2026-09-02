# CERT-M1 R1 reviewer probes (frozen head 1318b6d10)

- `sole_bound_census.py` — independent re-take of H10's sole-`T: Bounds`
  census over `crates/*/src` (comments and string literals blanked,
  multi-line generic lists included). Result: 22 sites, matching the PR's
  roster exactly minus `dual.rs:817`, which is a WHERE-CLAUSE bound and so
  outside the stated methodology (the PR counts it; total 23 either way).

- Planted-defect probe (applied and reverted, not committed as a patch):
  a fixed 20-step outward widening of `DInterval::sin`'s `hi`. Result at
  effort 1: `max_endpoint_steps` fires ("24 steps ... allows 8") while the
  width ratio reaches only 45 against its ceiling of 64 — i.e. the new
  absolute bound catches exactly the class the ratio cannot see. The
  `[huge-window]` accumulator, which carries no `Ceiling`, did not score it.

- Gate probe (applied and reverted): a `T: Decide + Bounds` planted in
  `crates/bvh/src/aabb.rs` reds `scripts/gates/bounds-allowlist.sh`,
  confirming the second half of `bvh/src/lib.rs`'s new sentence.
