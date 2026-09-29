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
(`docs/RING-3-SPEC.md` §5, extended for comments and docs at the fix
pass to `geom-brep/src/{pcurve_cache,certify,edge_nurbs}.rs`,
`geom-core/src/dual.rs` and `interval-transcendentals/tests/review_fuzz_exact.rs`,
all re-worded there, as were the TCOST/TINT test files' comments). What its sweeps found on ground the fence still
does not reach is below; each states something about the retired type
as current, and none is forced by a gate today.

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
- `local-scripts/bt-scalars.sh` (CIW/MIRROR) `:20` counts `RingInterval`
  in a monomorphisation dump; the count is now always 0.
- `crates/geom-core/Cargo.toml` `:99` (a manifest; RING-4 rewrites this
  paragraph with the feature) names `tests/ring_interval_differential.rs`,
  renamed to `tests/interval_backend_differential.rs` by RING-3.
- `crates/geom-brep/src/pcurve_cache.rs` (PCERT) — the
  `FittedLaneUnsupported` `Display` message (`PcurveCertifyError`'s
  `fmt`, near `:921`) says the bound is "an exact-arithmetic-ring hull".
  It is runtime text, so the fix pass's comments-only extension did not
  reach it; "a certification hull bound (C9)" is what is true. Check
  first that no row matches the old substring (none did at the fix
  pass: `grep -rn 'arithmetic-ring' crates`).
- `crates/topo/src/validate.rs` (ATREST) near `:3961` — "the C9 ring
  the plane × NURBS certificate lives in": certification arithmetic
  (C9), as `edge_nurbs.rs` now says.
- `crates/topo/src/{census.rs:2783, validate.rs:1346, instance.rs:62}`
  (CONTACT, ATREST, unowned) name "the C9-ring conformal-rest / partial-embedding class". Whether
  "C9-ring" there means the certification arithmetic or the exclusion
  ring the same sentence names is the owner's to say; if the former,
  it wants the same re-word.

Past-tense mentions are true as written and need nothing:
`topo/tests/{m6_3_chart_completion.rs:275, review_ssiflat_r1_probes.rs:40}`
("re-measured when the C9 ring became a newtype"),
`mesh/tests/budget_meter.rs:95` ("the C9 ring padded"),
`geom-core/tests/ring2_r2_probes.rs:117` (a port pinned at a sha), and
`docs/GENERICS-BUILD-COST.md`'s dated measurements.

## Fix

Re-word each to what is true, in whichever PR next touches the file;
the manifest comment rides RING-4.
