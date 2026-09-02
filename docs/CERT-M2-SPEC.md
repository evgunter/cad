# CERT-M2 — the supertrait bound made visible, and the lane census (S213, D222, H-f's opening)

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md` §CERT-M;
difficulty logged at spec: **M/L**). Read
`docs/prompts/implementer-discipline.md` in full before starting. The
Track M table in `docs/SMELL-SCAN-2026-08.md` (rows `S213`, `D222`,
`H5`, `S90-impl`), finding `S213`, `docs/SMELL-H-LOG.md` **H-R3** and
**H-R16** (read both in full — they are the ruling this unit executes
the first step of), and the `Bounds` scope rule's M7-8 entry in
`crates/geom-core/src/real.rs` (now `real::bounds_allowlist`) are the
primary specification. Branch `cert/m2-supertrait-bound`.

## What this unit is, and is not

`H-R16` rules that the four lane traits go to zero by a
**three-function split** — structural half, certified half, and a
composed entry carrying the union of bounds, so the call a dual could
not survive is unwritable rather than refused — and that `H-f`'s
opening task is a **census, not a change**: `PropsQuadLane` splits
nearly free because its certified half is already a closure parameter
(`topo/src/props.rs:186`'s `mass_properties_impl`), and nothing yet
establishes that the other three do. This unit is that opening: the
free split, executed, plus the census the ruling asks for. It takes:

1. **S213** — `topo::validate_geometric<T: PropsQuadLane>` carries the
   `Bounds` obligation by SUPERTRAIT, where `bounds-allowlist.sh`'s
   KNOWN GAP 2 cannot see it and where the scope rule's own M7-8 entry
   says doors do it differently. Close it the way `H-R16` spells:
   `validate_geometric_structural<T: Decide>` (checks 1–6),
   `validate_geometric_certified<T: Decide + Bounds + CertifiedEnclosure>`
   (check 7), and `validate_geometric` composed with `?` so the
   `errors.is_empty()` sequencing fact becomes the composition (H-R16's
   "must not miss" 1). The composed entry's bound is then a sole
   `CertifiedBounds`-class bound the gate CAN see — say which
   instrument sees it after the change (`bounds_census.rs`, landed by
   CERT-M1, pins sole bounds; run it and update its roster). The ONE
   in-repo site that moves to the structural half is
   `sweep/tests/extrude_acceptance.rs:565` (H-R16); `topo/tests/
   geometric_cube.rs:236` asserts `validate_geometric` SUCCEEDS at
   `Dual64` — that row moves to the structural half too, and its
   bitwise value-channel comparison keeps running there. `PropsQuadLane`
   itself: delete it if the split leaves it with no job, or keep it as
   a method-less name (the `EvalScalar` shape, H-R16's "non-issue") and
   say which. The dual's refusal at check 7 is no longer a run-time arm
   anywhere — verify by grep that no `LaneUnsupported`/refusing impl for
   this trait survives.
2. **D222** — `topo/src/props.rs:1166-1176`'s private `fn br<T:
   CertifiedEnclosure>` is `RingInterval::from_certified` under another
   name with its own restatement: inline it, delete the restatement,
   keep `bracket_seam_tests` pinning the crossing (the fence for this
   file is drawn by this unit — see below).
3. **H-f's census** of the other three lane traits, as a checked
   artefact in the PR body (not code): for each of `PcurveFittedLane`
   (`geom-brep/src/pcurve_cache.rs:907`), `EdgeNurbsLane`
   (`edge_nurbs.rs:214`) and `ChartRegionLane`
   (`topo/src/chart_region.rs:252`) — where the certified sub-operation
   sits inside its mixed pass, what the pass hands back without it
   (H-R16's contract asymmetry: a weaker object vs less information),
   whether the certified half is already a parameter, what the absence
   channel carries (`Ok(None)` vs `Err(LaneUnsupported)` vs
   `Option<Result>` — S3's three spellings; `ChartRegionLane` routes two
   absences to one `CensusUnsupported`), and which in-repo callers
   instantiate the pass at a dual and assert what. Each ends in a
   verdict: splits free / splits with a new contract (name it) / does
   not split (why). That census is CERT-M3's specification.
4. **S90-impl / #883** — NOT unparked. Re-read the parked branch as a
   measurement against today's tree (does `fillet_edges` still reach
   `editor_core::eval::evaluate`; is the mixed pass still mixed) and
   record the reading in the row; the fillet seam's split is CERT-M3's
   question or nobody's, per the census.

It does NOT take the other three splits (CERT-M3), `S1`/`S2`'s
scalar-ladder residue (`RingInterval` vs an always-on `Interval`;
`Dual` in `Real`) or any rewriting of `Dual` arithmetic — `dual.rs` and
Dual-at-certified-gates semantics are **M10's** (`docs/S-CERT-PLAN.md`
cross-program interfaces) — nor `S55` (`Enclosure`'s consumer, deferred
pending the Bounds split).

**Fence (drawn by the orchestrator for this unit; rule 1 otherwise):**
Track M's territory (`crates/geom-core/src/{real,ring_interval,dual,
interval,k_stats}.rs`, `interval-transcendentals/`, `crates/bvh/`, their
tests) PLUS `crates/topo/src/props.rs` and `crates/topo/src/validate.rs`
(the split's two files; `props.rs` is no track's — this unit draws that
fence, and the draw is recorded in the PR body), PLUS the two test rows
named above (`topo/tests/geometric_cube.rs`,
`sweep/tests/extrude_acceptance.rs`) for the ONE edit each that the
split forces, PLUS `crates/geom-core/tests/bounds_census.rs`'s roster.
`scripts/gates/bounds-allowlist.sh` (Track K) is NOT in fence: if the
split changes what the gate must allowlist, FILE the row (D209's
sibling) and land green without it — if it cannot land green without
the gate edit, stop and report; the orchestrator widens or sequences.
`topo/src/chart_region.rs`, `geom-brep/src/{pcurve_cache,edge_nurbs}.rs`,
`sweep/src/blend/` are read for the census and NOT edited. These files
are live ground for other programs (VERBS, MATE, SEAT, BLEND, PCURVE):
re-merge main before opening the PR and read the interplay.

## Posture

- ε: none of these rows is ε-keyed (a bound is a compile-time fact;
  the validator's arithmetic is unchanged) — say so; `CI-Config:
  lane=both` (the split moves a certification bound and the interval
  lane is where it certifies) with `eps=1e-12` and the argument stated;
  three-ε local sweep on every changed row; `cargo test -p topo -p
  sweep` both lanes.
- The vocabulary is H-R16's: a tightened bound does not *refuse* —
  the call is unwritable. Do not write "typed refusal" for it.
- Review: the program's standard v6 dual, ADVERSARIAL-weighted on the
  H-g criterion — this unit changes what `Body<Dual64>` can call at a
  public door of a library crate; the failure mode is a tightened PASS
  (H-R3's doors/passes distinction), and the edit that respects it and
  the one that breaks it differ by one bound. Red-first: a row that
  shows the dual still validates structurally (checks 1–6) and CANNOT
  call the composed entry (a `compile_fail` doctest, honest per S216 —
  it must fail for the stated reason).
- Landing conventions (rule 3): delete `S213` and `D222` from the Track M
  table and `S213`'s finding text; `H5` STAYS with a note that its
  census is in PR <n> and CERT-M3 executes it; `S90-impl` stays parked
  with the re-read recorded. Expect the SMELL-SCAN conflict.
- No `Co-Authored-By`; rows spelled out; push early; the discipline
  doc's lane rules in full.

## Acceptance

- `validate_geometric` composed of a structural and a certified half
  with the union bound on the entry; the two moved test rows green;
  the `errors.is_empty()` fact is the composition; no refusing impl
  for `PropsQuadLane` survives; `bounds_census.rs` roster updated and
  green; D222's alias gone.
- The three-trait census with a verdict per trait; #883's re-read.
- Sweep obligation: other doors that carry a certification bound by
  supertrait only (grep `: .*Lane` bounds and supertrait chains across
  `crates/*/src`) — hit list with dispositions; state what the pattern
  cannot match. D2-addendum classification for the composed entry
  (nothing minted; a run-time refusal retired into unwritability —
  say which row of the addendum that is).
