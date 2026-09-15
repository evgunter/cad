# SUITE — suites, fixtures and the helpers they copy (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

Branch prefix: **`suite/`**. Away-channel tag `(SUITE orchestrator)`.
A/B ordinal band **SUITE = 4200–4299**.

## Charter

One shape, seven rows: **a test helper that was copied instead of
shared, or a suite that carries a stub of something that now exists.**
`S392`'s eighteen fixture sites, `S52` and
`topo-arena-census-duplicate-spellings` (which are the same open
residue seen from two directions — a generic `cube<T: Real>` and the
local body builders in three crates' suites), the part-resolver stubs
copied across `editor-core`'s mate and assembly suites, and `S391`'s
two orientation helpers.

Plus two rows that are test-side but not duplication: `D114`'s
differential probe, and `D403`'s tour walls that no probe drives.

## Territory — none, and the reason is the point

`crates/*/tests/*` and `crates/test-utils/*` are **S-TCOST's and
S-TINT's territory in every crate**, by both programs' `paths`. This
program claims nothing and does not try: it is a **slate of rows on
S-TINT's ground**, and every unit announces there.

The rule that governs the seam is the one these rows were written
under: they are about the *test-side mechanisms* — the guards, the
fixtures, the stand-downs — and where a
mechanism reaches into a crate's `src/`, the row is filed on that crate's
owner. A test needed by a `src` change belongs to the PR that makes the
change, not here.

**`S52` and `topo-arena-census-duplicate-spellings` are staffed as one
unit or one is closed against the other.** They describe the same two
remaining members; both entered this cut open, and the first lane to
read them settles that.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `S391` | **M** | Rename vs second centroid-chord helper, and both live readers need re-arguing | `crates/sweep/tests/common/orient.rs` (`first_wall_chord`, renamed from `stack_axis` by the unit), `crates/sweep/tests/m8_14_long_turn_sweep.rs`, `crates/sweep/tests/turning_orientation.rs` |
| `S52` | **M** (the two rows) / **H** (the class behind them) | Closed by a scalar-generic family in `crates/sweep/src/test_support.rs` reached through a featured dev edge | `crates/sweep/src/test_support.rs` (the home: `extruded` + `prism`/`brick`/`block`/`cube`/`corners`/`pocket_die`), `crates/{sweep,mesh,stl,step-export,step-import,editor-core}/tests/**`, three new `[dev-dependencies]` edges, `scripts/gates/test-features-dev-only.sh` |
| `topo-arena-census-duplicate-spellings` | **M** | Passenger (`rides_with: S52`); its §1–§3 verified closed, and the census name-drift taken rather than filed | `crates/{step-export,step-import}/tests/common/mod.rs` (`fev_census` / `arena_census`, both onto `topo::test_support::arena_counts`), `crates/sweep/tests/wild.rs` (`face_census`) |
| `editor-core-suites-carry-eleven-part-resolver-stubs` | **M** | Seventeen suites over two crates, not the row's eleven; the reconciliation was the unit and the migration was the easy half | `crates/editor-core/tests/fixture/resolver.rs` (the home and its guard), seventeen suites in `crates/editor-core/tests/` and `crates/viewer/tests/msolve3_placer_refused.rs` |
| `D114` | **M** | New differential instrument; bit-identity settled by `Probe`'s own newtype, corpus plumbing built | `crates/editor-core/tests/m4_pr8_k_probe.rs` (the differential), `crates/editor-core/tests/fixture/value_channel.rs` (the shared feed), `crates/editor-core/tests/{m5_pr5_corpus_probe,m10_di_dual_corpus}.rs`. The Track K fence held: nothing under `scripts/gates/` (GUARD's), `tools/` or `docs/K-REPORT.md` (INSTR's) was edited; four rows were filed on their owners instead |
| `D403` | **M** | Lift vs drive `stops()` settled as lift; the class has a second member | `demos/tour/src/teapot.rs` (`wall_probes`, walls 2–3), `demos/tour/src/torusvessel.rs` (`wall_probes`, wall 1), `demos/README.md` |
| `S392` | **H** on population, **M** on difficulty | 24 sites, not 18; 20 converted onto S52's home, 4 deliberately not | `crates/sweep/src/test_support.rs` (`loft_prism`, `loft_prism_at`, `loft_prism_sections`, `PRISM_*`, `stacked_at`), `crates/{sweep,mesh,editor-core,step-export}/tests/**`, `tools/tess-meter/tests/rows.rs`, three `[dev-dependencies]` edges. Not converted, each with a reason: `crates/step-import/tests/review_probes_m7_3.rs`, `demos/tour/src/skinned.rs`, `crates/pncad-py/tests/test_north_star.py`, `docs/GUIDE.md` |

## Exit criteria

This program opened without a criteria section, which is a defect in its
own charter: `docs/DOC-LEDGER.md` records that an exit walk quotes its
plan's criteria verbatim, and a plan with none leaves the walk nothing
to quote. Written here before the evidence, so the walk can check them
rather than paraphrase a charter.

- **X1.** Every row on the slate is `closed`, or `deferred` with its
  ratification cited in the body. No row is left `parked` on a trigger
  this program could have fired.
- **X2. A duplication row closes by a shared home, not by a deletion.**
  For each, the thing that was copied is spelled **once**, and the
  reviewer's standing question is answered at every call site the unit
  touched: does the shared helper still make that suite's intent
  readable, or has the suite become a call into a fixture nobody reads?
  A row that closes because its copies were deleted without a home is
  not closed against this criterion.
- **X3.** Each unit states what its sweep pattern could **not** match.
  A sweep whose blind spot is unstated is an unverified claim, not a
  negative result, and this program's own `S392` is the receipt for what
  that costs — its first count was a truncated grep read as a
  population.
- **X4. No unit minted a fresh instance of the defect it closed.** This
  is the trap `docs/prompts/reviewer-style-lane.md` records for exactly
  this shape of work, and naming it in a PR body has never prevented it;
  only a reader who did not write the fix has caught it. The walk names
  who checked, per unit.
- **X5.** Every residue a lane disclosed has its **own file**, on this
  slate or on the owning program's, minted at the moment of disclosure.
  A residue disclosed only in a `## Closed` section or a PR body is
  invisible to the re-homing sweep and dies with this directory.
- **X6. The program claimed no paths, start to finish.** `paths` stays
  empty in `program.md`, every unit announced to S-TINT (and `D403` to
  SHELL, whose scenes it touches by courtesy), and no `keep_out` clause
  anywhere in the tree had to be written to accommodate a claim this
  program made.

## Order

`S391` opens — two helpers, one rename, and it is the smallest thing
here. Then the `S52` / `topo-arena-census` pair as one unit, then the
`editor-core` stub migration, where the drift between copies has to be
reconciled rather than merged away and the suite population has to be
re-counted before anything moves.

`D114` and `D403` are instrument rows and independent of the rest.
`S392` goes last: eighteen sites over six crates plus a tool, a demo and
a Python suite, four of which its own body says are not convertible.

## Review posture

Test-only, S-TINT's posture: one style review per unit, no dual and no
A/B row. The reviewer's standing question on every unit here is the one
`S52`'s history already answered wrongly once — **does the shared helper
still make each suite's intent readable at its call site**, or has the
suite become a call into a fixture nobody reads?
## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
