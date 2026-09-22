# Sweep 15 — 2026-09-16: S-BOOL leaves the tracker

Sweep SHA: `32082e8a24fd826d75fc2529bc2a0468b6430f01` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
S-BOOL's directory is complete, `program.md` reads `status: closed`,
and every row in it is closed), so every path below is recoverable at
`git show 32082e8a24fd:work/bool/<FILE>`,
`git show 32082e8a24fd:docs/S-BOOL-EXIT-WALK.md` and
`git show 32082e8a24fd:docs/BOOL-<n>-SPEC.md`.

S-BOOL — boolean reach and containment — opened 2026-08-31 from the
ratified stream cut (`docs/WORK-STREAMS-2026-08.md` §S-BOOL) and closed
2026-09-16 on the walk Ev ratified on its PR (#2775, "lgtm!", merged
`3e4e0c8a3`). **Thirteen units**, every one merged on its own green
hosted head with a v6 dual: BOOL-1 (#1378), BOOL-2 (#1425), BOOL-3
(#1464), BOOL-8 (#1508), BOOL-11 (#1520), BOOL-13 (#1553), BOOL-12
(#1573), BOOL-9 (#2134), BOOL-10 (#2135), BOOL-5 (#2748), BOOL-6
(#2752), BOOL-7 (#2755), BOOL-4 (#2767) — ordinals 1100–1112, samples
#75, #84, #91, #93, #100, #104, #156, #164, #212, #214, #215, #216,
#217; window tally BOOL-9 +1 fable, BOOL-10 +1 opus, BOOL-7 +1 fable.
Per the sweep-5 rule the directory leaves whole — `program.md`,
`plan.md`, `log.md`, seven unit rows and the closed issue rows — with
every OPEN row re-homed first on the walk's own PR (below).

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `bool` | S-BOOL — boolean reach and containment | 2026-09-16 | this entry; the walk at the sweep SHA; the design at `docs/PATHS-DESIGN.md` §3/§4 (the Q1 chain's re-wordings, ratified in-chat 2026-09-01 and 2026-09-13), `crates/editor-core/ASSEMBLY.md` (interference decided by the material test) and `crates/topo/src/census.rs`'s arm-2 doc; the A/B record at ordinals 1100–1112 in `docs/MODEL-AB-LOG.md` |

### What survived, and where

The program's output is the tree, not its directory:

- **The doctrine.** The Q1 ruling chain in `docs/PATHS-DESIGN.md`
  §3/§4 — the straight continuation and the declared point-target
  continuation as structural joints, the declared arrival at the seam,
  the vertex table as a cache with authoring through the lattice only,
  `arc_continue` retired — under the sixth-round ruling (every
  zero-turn joint is a declared tangent joint; the lattice never asks
  whether carriers are the same). The declared-split arc form was
  built, reviewed and then declined on its cost; it is history at
  `f79fa7081`, not main.
- **The code.** `point_in_solid`'s cone and torus arms with the grazing
  posture escalating (BOOL-2/3); the coplanar-split citations restated
  (BOOL-1); the schema demolition (BOOL-13); the rim-free spherical
  wedge's props arm with `props_wedge_azimuth` and `props_band_opposite`
  (BOOL-5); the per-slab stacking fold in `loft.rs` (BOOL-6); the
  vdiff shadow-exec rung, per partner at the boolean's operand with its
  two halves recorded as limits (BOOL-7); the census's material
  containment test over a per-solid point-in-solid entry, with
  `InstanceInterference` as the decided refusal (BOOL-4).
- **The measured bounds, stated rather than overpromised.** Curls past
  a full turn build self-overlapping bodies with every tier silent
  (BLEND's row); the shadow-exec rung recovers the pruned-pair half of
  a `SideOf` vanish and neither the collapse half nor the OrderAlong
  half (WIRE's rows); the material test admits vertex-on-face and
  edge-in-face touches only as locally one-sided rests and blocks the
  other touch kinds, and the box gate still clears a partial overlap
  whose boundaries meet only in touches (CURVED's rows).
- **A successor program.** `work/paths/` — PATHS, the profile lattice —
  opened on the walk's PR per `work/README.md`'s rule, holding the
  eight lattice rows and the band 5000–5099.

### Residue re-homed before the deletion

Fifty-eight open rows moved on the walk's PR (#2775, an earlier commit
than this deletion), each carrying a "Re-homed at S-BOOL's exit"
note, ids unchanged, the Track Q rows' `parent: BOOL-Q` dropped:
twenty-three boolean, containment, join and declaration rows and six
Track Q rows (D280, D284, D95, G9, S173, S234) to `work/curved/` (its
charter inherits S-BOOL's ceded ground at this exit); eight lattice
rows to `work/paths/`; four to `work/topo/` (the provenance graft, the
two `topo::split` rows, the deferred void-birth marking); six to
`work/blend/` (the profile fillet door's three, the two loft findings,
the subdivided-side lowering — `crates/sweep/src/loft.rs` added to
BLEND's `paths`); one each to `work/guard/` (the raw-door gate),
`work/lib/` (the Python refusal-predicate pins) and `work/wire/` (the
collapse-half limit); D46, D57, D281 to `work/pred/`; D287, D66 to
`work/tint/`; H11 to `work/props/`; the two heat-sink demo rows to
`work/issues/` (demos/tour is in no program's `paths`). BOOL-Q closed
as dissolved; BOOL-4 and issue 750 closed at BOOL-4's merge. The
descendant-cycle repro patch travelled with the CURVED row that cites
it. Nothing else was open.

Filed by S-BOOL's units on other programs' slates and untouched by the
sweep: `work/props/certificate-types-have-public-fields-and-are-forgeable`,
`work/props/sphere-wedge-arm-does-not-fold-split-meridians-by-lineage`,
`work/props/props-curved-carries-two-readings-of-d9-unreachable-vs-poison`,
`work/props/sphere-flux-arm-refuses-partial-bands` (re-scoped),
`work/blend/skin-coincident-section-check-is-an-unbanded-f64-compare`
(re-scoped), `work/wire/order-along-qualifier-records-no-partner-so-its-pruned-pair-vanish-cannot-be-recovered`,
`work/lib/north-star-audit-verb-list-names-arc-continue`,
`work/issues/klein-scene-should-adopt-the-one-body-loop-sweep`,
`work/docm/pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1` (since
re-homed by DOCM's sweep), `work/curved/partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate`,
`work/curved/touch-kinds-without-a-local-side-analysis-block-the-material-test`,
`work/topo/an-inside-out-part-passes-tier-3-because-only-the-body-total-volume-is-pinned`.

What opens with this sweep: `crates/topo/src/boolean/*` and
`splitting/*` are CURVED's outright (the fence both `program.md`s
carried dissolves; CURVED's own keep-out prose still names it and is
CURVED's to re-word); `crates/editor-core/src/resolve/vdiff.rs` is
EDIT's (`resolve/` is EDIT's territory); `crates/profile/*` is PATHS';
`crates/sweep/src/loft.rs` is BLEND's.

### The docs that moved with the program

| doc | from | to |
| --- | --- | --- |
| `S-BOOL-EXIT-WALK.md` | `docs/` | deleted with this sweep; recoverable at the sweep SHA |
| `BOOL-4-SPEC.md`, `BOOL-5-SPEC.md`, `BOOL-6-SPEC.md`, `BOOL-7-SPEC.md`, `BOOL-9-SPEC.md`, `BOOL-10-SPEC.md`, `BOOL-12-SPEC.md` | `docs/` | deleted with this sweep (the seven binding specs whose units merged; the earlier six left `docs/` at their merges); recoverable at the sweep SHA |
