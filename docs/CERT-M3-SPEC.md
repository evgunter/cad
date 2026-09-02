# CERT-M3 — zero lane traits: the remaining three splits (H5's lane-trait collapse, H-R16 executed)

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md` §CERT-M;
difficulty logged at spec: **L**; **ADV** — the unit evicts `Dual` at
public doors of two library crates on a ruling whose own scope note is
"at least for now"). Read `docs/prompts/implementer-discipline.md` in
full before starting. The primary specification is **CERT-M2's census**
(its PR body, the verdict per trait) executed under `docs/SMELL-H-LOG.md`
**H-R16** (the three-function split; unwritability, not refusal) and
**H-R3** (the doors tighten; the passes keep their lanes — the split is
what preserves the capability H-R3 protects); findings `S3` (the four
instances and the crate-DAG obstacle), `S44` (the provenance caveat
this unit states and does not resolve), and the Track M `H5` row.
Branch `cert/m3-zero-lane-traits`. Sequenced AFTER CERT-M2 merges.

## What this unit is, and is not

Delete `PcurveFittedLane` (`geom-brep/src/pcurve_cache.rs:907`),
`EdgeNurbsLane` (`geom-brep/src/edge_nurbs.rs:214`) and
`ChartRegionLane` (`topo/src/chart_region.rs:252`) — sixteen impls
down to zero — by the same split CERT-M2 executed for `PropsQuadLane`:
each mixed pass becomes a structural half bounded `T: Decide`, a
certified half bounded `T: Decide + Bounds + CertifiedEnclosure`, and a
composed entry carrying the union, so the program that would have
needed refusing is unwritable. CERT-M2's census says per trait whether
that is free, wants a new contract, or does not split — **this unit
takes the free and the contract-priced ones and lands the contract it
prices**; a trait the census says does not split is argued in the PR
body with the census's own reason and left, and `H5` records it. The
crate-DAG obstacle S3's steelman measured (no single crate can host
all four methods; the blanket-impl collapse fails coherence) is not an
obstacle to the SPLIT — the split hosts nothing: it deletes.

It does NOT take: `S90-impl`/#883 (the fillet seam — parked on a
ruling; if CERT-M2's re-read says the seam still reaches a mixed pass,
this unit records the reading and does not unpark it); any rewriting of
`Dual` arithmetic or `dual.rs` (M10's — the plan's cross-program
interfaces); `S1`/`S2`'s scalar-ladder residue and `S55` (deferred
pending the Bounds split — this unit RE-HOMES those residues off the
`H5` row onto a surviving row or onto `DESIGN.md`'s M10 roadmap entry
by pointer, so `H5` can be deleted without losing them); the
`bounds-allowlist.sh` gate's own logic (Track K).

**Fence (drawn by the orchestrator; rule 1 otherwise):** Track M's
territory PLUS the three trait homes and their passes —
`crates/geom-brep/src/{pcurve_cache.rs,edge_nurbs.rs}`,
`crates/topo/src/chart_region.rs` (both Track Q's; live ground for
PCURVE/MATE/VERBS — re-merge main before every push and read the
interplay), `crates/topo/src/{props.rs,validate.rs}` (CERT-M2's fence,
inherited), `crates/editor-core/src/eval/mod.rs` for the `EvalScalar`
alias and the `:865` inert-bound site only (Track V's; one edit), PLUS
every in-repo caller the split forces to name a half (list them in the
body — `sweep/tests/m5_pr11_quad_props.rs`'s
`dual_lane_keeps_the_closed_form_refusal` is the shape H-R3 names), PLUS
`scripts/gates/bounds-allowlist.sh`'s FILE LIST only (the roster of
files licensed to carry compound bounds changes when four narrow trait
names become direct bounds — the orchestrator widens the fence to that
list and to `crates/geom-core/tests/bounds_census.rs`'s roster; the
gate's logic and header prose stay Track K's, filed).

## Posture

- ε: bounds are compile-time facts; the passes' arithmetic is
  unchanged — say so; `CI-Config: lane=both eps=1e-12` with the
  argument; three-ε local sweep on every moved row; `cargo test -p topo
  -p geom-brep -p sweep -p editor-core` in both lanes.
- Red-first, per trait: a `compile_fail` doctest (honest per S216 — it
  must fail for the stated reason) showing the composed entry is
  unwritable at `Dual64`, and a green row showing the structural half
  still runs at `Dual64` with the value channel bitwise-equal to the
  `f64` build where the old test asserted that.
- The absence channel: S3 measured three spellings (`Ok(None)`,
  `Err(LaneUnsupported)`, `Option<Result>` routing two absences to one
  error). With the traits gone there is no absence to signal at the
  type level — say what each consumer's former absence arm becomes
  (deleted, or a structural-half result) and prove no consumer still
  matches on a variant nothing can produce (D2 addendum row 0 — say so
  per variant retired).
- Review: standard v6 dual, ADVERSARIAL: the reviewers' first target is
  a tightened PASS (H-R3) — construct a `Body<Dual64>` and drive it
  through every door that used to admit it, and show what it can still
  do; the second is the crate-boundary coherence (a split that
  re-introduces a trait to satisfy the DAG has re-minted the pattern —
  rule 5).
- S44's exposure is STATED in the PR body and not resolved: the dual's
  refusal rests on a ruling, not on the type; the M10 roadmap entry
  holds the open question.
- Landing (rule 3): delete `H5` from the Track M table with its
  residues re-homed (S1/S2/S44/S55 pointers); delete `S3`'s finding text
  only if the four instances are gone (else member by member); `S90`'s
  and `S90-impl`'s text updated in place per their "one home" sentence.
  Expect the SMELL-SCAN conflict.
- No `Co-Authored-By`; rows spelled out; push early to
  `cert/m3-zero-lane-traits`; the lane rules in full.

## Acceptance

- Zero lane traits with a refusing impl in `crates/*/src` (grep proves
  it); every former mixed pass split with the union bound on its
  composed entry; every forced caller named and moved; the gate's file
  list and the census roster updated and green; the compile_fail rows
  honest.
- Sweep obligation: other `Decide`-subtrait-with-refusing-impl shapes
  anywhere in `crates/*/src` (the pattern S3 named, by its shape, not
  its four names); hit list; what the pattern cannot match.
- Deviations stated; D2-addendum classification per retired variant and
  per composed entry.
