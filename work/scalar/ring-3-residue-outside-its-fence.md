---
id: ring-3-residue-outside-its-fence
kind: issue
title: RING-3 residue outside its fence: prose and one gate entry still name the retired ring
status: open
priority: P4
cost: E
opened: 2026-09-24
refs: [ring-3-ring-dissolves-into-interval, H5]
---

## What

RING-3 dissolves `RingInterval` into `Interval` inside its fence
(`docs/RING-3-SPEC.md` §5). Its sweep for the retired name and the
prose that describes it without naming it found five sites on ground
the fence does not reach; each still states something about the
retired type, and none is forced by a gate today.

- `crates/geom-brep/src/intersect.rs` (GERM/REACH/TANG) — three
  refusal messages (the `route`-side cone/torus arm near `:318`, and
  the two near `:2075` and `:2137`) say a certified root is one "the
  exact-arithmetic ring lacks". The certification scalar is `Interval`,
  which has an outward-rounded `sqrt`; what is true is C9's design
  statement that certification arithmetic takes no root. RING-3
  re-worded the same sentence in `ssi/certify.rs` (`composite_form`)
  to "a certified root, which certification arithmetic does not take";
  these three want the same words. Check first that no test matches
  the old text (`m5_pr7_ssi.rs` matches "ring-computable implicit
  enclosure", which none of the three changes).
- `docs/CURVED-SPIRIC-DESIGN.md` (CURVED) `:83`, `:140-143`, `:305` —
  "an outward-rounded `sqrt` in `RingInterval`, which retires the
  torus", `RingInterval::poison()`, the `ring_interval.rs` module doc,
  and "`Bounds`/`Enclosure` say what bracket…". The first is a design
  point, not naming: the scalar certification now runs on HAS an
  outward-rounded `sqrt`, so whether the torus arm's blocker is the
  arithmetic or C9's transcendental-free rule is the question to put
  to that doc's owner.
- `scripts/gates/interval-square-allowlist.sh` (GUARD) —
  `ALLOWLISTED_HOMES` (`:255`) and the rationale (`:174`) name
  `crates/geom-core/src/ring_interval.rs`, which no longer exists. The
  gate goes quiet on a missing file (its own header), so nothing reds;
  the entry is stale, and `interval.rs` inherited the ring's bodies
  without inheriting an entry (its production code has no `x * x`, so
  none is owed today).
- `local-scripts/bt-scalars.sh` (CIW/MIRROR) `:20` counts `RingInterval`
  in a monomorphisation dump; the count is now always 0.
- `crates/geom-core/Cargo.toml` `:99` (a manifest; RING-4 rewrites this
  paragraph with the feature) names `tests/ring_interval_differential.rs`,
  renamed to `tests/interval_backend_differential.rs` by RING-3.

## Fix

Re-word each to what is true, in whichever PR next touches the file;
the manifest comment rides RING-4.
