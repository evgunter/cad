# Sweep 16 — 2026-09-16: S-MESH leaves the tracker

Sweep SHA: `9f043ec2712b880f26879183b6f9dc828abe0254` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
S-MESH's directory is complete, `program.md` reads `status: closed`,
and every row in it is closed), so every path below is recoverable at
`git show 9f043ec2712b:work/mesh/<FILE>`,
`git show 9f043ec2712b:docs/S-MESH-EXIT-WALK.md` and
`git show 9f043ec2712b:docs/MESH-12-SPEC.md`.

S-MESH — mesh honesty and budget — opened 2026-08-31 from the ratified
stream cut (`docs/WORK-STREAMS-2026-08.md` §S-MESH) and closed
2026-09-16 on the walk Ev ratified on its PR (#2776, "lgtm", merged
`6b1efa457`). **Eleven units**, every one merged on its own green
hosted head with a v6 dual: MESH-1 (#1389), MESH-2 (#1421), MESH-3
(#1460), MESH-5 (#1507), MESH-4 (#1517), MESH-6 (#1545), MESH-7
(#1565), MESH-8 (#1585), MESH-10 (#1595), MESH-11 (#1599), MESH-12
(#1617) — ordinals 1200–1210 in that dispatch order (MESH-5 at 1203
before MESH-4 at 1204), samples #76, #82, #88, #92, #96, #101, #106,
#110, #112, #113, #157; no tally candidate in eleven duals. MESH-9
never ran: parked on issue 950 behind its typed trigger, it moves to
TESS parked. Three rulings ratified in-program (Ev, in chat,
2026-09-01): Q1, S65 stays compiled out; Q2, option (d) — the
input-quality detectors relocate body-side; Q3, explicit doors and no
transitive floor. Per the sweep-5 rule the directory leaves whole —
`program.md`, `plan.md`, `log.md`, the MESH-12 and MESH-R rows and the
three closed issue rows — with every OPEN row re-homed first on the
walk's own PR (below).

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `mesh` | S-MESH — mesh honesty and budget | 2026-09-16 | this entry; the walk at the sweep SHA; the code it left — `crates/mesh`'s `Eps` operations, `props::require_iso_rectangle` and `require_one_chart_branch`, `topo::coherence::examine_chart_coherence`, `props_meridian_span_winding`; the A/B record at ordinals 1200–1210 in `docs/MODEL-AB-LOG.md` |

### What survived, and where

The program's output is the tree, not its directory:

- **The doctrine.** ε as named operations on a type (MESH-4's `Eps`
  newtype — separates / coincident / dominates / pad); the never-infer
  guard shape — structural rungs only, the identified side never
  asserted from values (MESH-3); shape predicates as explicit doors
  each consumer cites, under Q3 (MESH-7, MESH-11); body-data coherence
  examined body-side and non-gating, under Q2 (MESH-8); the MESH-4
  two-build digest as the D9 instrument every later unit in both
  programs ran.
- **The code.** The walk's loop-area fold anchored at the loop's own
  bbox centre (MESH-1); the chart frame's structurally-zero far-point
  v and spade's underflow floor (MESH-2); the undeclared-pole guard in
  `walk::loop_polygon` (MESH-3); the `nu == 1` one-strip schedule,
  decided by measurement (MESH-5); the two mechanical
  `cfg(debug_assertions)` censuses for S65's uncovered cases (MESH-6);
  `props::require_iso_rectangle` and `require_one_chart_branch`
  (MESH-7, MESH-11); `topo::coherence::examine_chart_coherence`
  (MESH-8); the split-meridian lineage fold in `torus_parse`
  (MESH-10); `props_meridian_span_winding`, the typed refusal for a
  sphere meridian span past the winding bound (MESH-12).
- **The measured bounds, stated rather than overpromised.** The
  one-element grid's axes still drop the schedule (issue 1513, TESS's
  row); a rim-only sphere cap still panics at the census (issue 1615,
  un-parked at MESH-12, TESS's row); the stored spans are read raw
  past the winding bound (issue 1618, PROPS' row); the two-argument ε
  form was disclosed at MESH-4's fix pass as unadoptable without
  moving bytes.
- **A successor program.** `work/tess/` — TESS, the tessellation
  kernel — opened on the walk's PR per `work/README.md`'s rule,
  holding the fourteen mesh findings, the parked MESH-9, the seven
  Track R rows and the band 5100–5199.

### Residue re-homed before the deletion

Twenty-seven open rows moved on the walk's PR (#2776, earlier commits
than this deletion), each carrying a "Re-homed at S-MESH's exit" note,
ids unchanged, the Track R rows' `parent: MESH-R` dropped: fourteen
mesh findings (among them `rim-chords-exceed-snapped-column-count`,
MESH-9's trigger, and VIEW's rider
`degenerate-normal-rows-model-resolution-cites-a-deleted-helper`,
filed into `work/mesh/` after the walk was cut and moved beside its
parent), MESH-9 itself and the seven Track R rows S28, S236, S237,
D300, D303, D304, C23 to `work/tess/`;
`stored-spans-read-raw-past-winding-bound` (issue 1618) and the two
`props/quad.rs` rows C3 and D30 to `work/props/`;
`cert10-strict-gap-floor-gates-on-a-varying-seed` and
`sentinel-markers-with-no-reader-are-grep-only` to `work/tint/`.
MESH-R closed as dissolved. Nothing else was open.

This sweep retires, on three rows, the `refs:` entries that named rows
leaving with the directory (MESH-12, MESH-R,
`saturated-sphere-span-folds-short`,
`rim-continuation-witness-fixture-needed`,
`mesh-cert10-fold-fuzz-row-flakes-on-a-fresh-seed`):
`work/props/stored-spans-read-raw-past-winding-bound`,
`work/tess/rim-only-sphere-cap-panics-at-census` and
`work/tint/fuzz-rows-discard-trials-against-a-floor-that-counts-them`
(whose only ref it was; a sentence naming the sweep SHA replaces it).
The rows are otherwise untouched.

Filed by S-MESH's units on other programs' slates and untouched by the
sweep: `work/fix/coherence-findings-have-no-consumer` (1587, closed
since), `work/topo/graft-copies-provenance-keys-verbatim` (1597),
`work/props/two-face-sphere-split-measures-zero-volume` (1598, closed
since), `work/props/props-refusal-cannot-carry-measured-overshoot`
(1602), `work/tint/pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1`
(the GUI-1 seeded fuzz guard that reddened MESH-12's landing, proven
not the PR's; filed on DOCM, re-homed by DOCM's sweep).

What opens with this sweep: `crates/mesh/*` and
`crates/topo/src/coherence.rs` are TESS's outright (its `program.md`
`paths` names them); the keep-out prose in FIX, INSTR, PIPE, PRED,
PROPS, TOPO and TRIM's `program.md`s that still names S-MESH as the
owner of that ground now means TESS and is each program's to re-word;
the tess-budget re-baseline stays PROPS' under the keep-out it
inherited from S-MESH's.

### The docs that moved with the program

| doc | from | to |
| --- | --- | --- |
| `S-MESH-EXIT-WALK.md` | `docs/` | deleted with this sweep; recoverable at the sweep SHA |
| `MESH-12-SPEC.md` | `docs/` | deleted with this sweep (the one binding spec still in `docs/` — its unit merged 2026-09-08 and the spec outlived the merge; the earlier ten left `docs/` under the standing per-unit rule, listed above); recoverable at the sweep SHA |
