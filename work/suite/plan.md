# SUITE — suites, fixtures and the helpers they copy (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

Branch prefix: **`suite/`**. Away-channel tag `(SUITE orchestrator)`.
A/B ordinal band **SUITE = 4200–4299**.

## Charter

One shape, eight rows: **a test helper that was copied instead of
shared, or a suite that carries a stub of something that now exists.**
`S392`'s eighteen fixture sites, `S52` and
`topo-arena-census-duplicate-spellings` (which are the same open
residue seen from two directions — a generic `cube<T: Real>` and the
local body builders in three crates' suites), `genus` spelled eleven
times and growing one copy per suite exactly as its row predicted, the
eleven part-resolver stubs in `editor-core`'s mate and assembly suites,
and `S391`'s two orientation helpers.

Plus two rows that are test-side but not duplication: `D114`'s
differential probe, and `D403`'s tour walls that no probe drives.

## Territory — none, and the reason is the point

`crates/*/tests/*` and `crates/test-utils/*` are **S-TCOST's and
S-TINT's territory in every crate**, by both programs' `paths`. This
program claims nothing and does not try: it is a **slate of rows on
S-TINT's ground**, and every unit announces there.

The rule that governs the seam is `docs/CODE-QUALITY-CONVENTIONS.md`'s and is
not restated differently here: W's rows are about the *test-side
mechanisms* — the guards, the fixtures, the stand-downs — and where a
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
| `S391` | **M** | Rename vs second centroid-chord helper, and both live readers need re-arguing | `crates/sweep/tests/common/orient.rs` (`stack_axis`), `crates/sweep/tests/m8_14_long_turn_sweep.rs` |
| `genus-rings-helper-spelled-nine-times` | **M** | Mechanical deletions, but the shared home must cross a separate workspace; copies now 11, not 9. | new home in `crates/topo/src/test_support*`, re-export via `crates/pncad/src/`, deletions in `crates/topo/src/review_m1_pr{3,4}.rs`, `crates/sweep/tests/{verbs_shell,verbs_shell_r2_probes,verbs_shell_r2b,shellfix1_r1_probes}.rs`, `demos/tour/src/*.rs`, `demos/tour/tests/*.rs` |
| `S52` | **M** | Open remainder is a scalar generalization plus a 3-crate fixture consolidation | roll-up territory: `crates/topo/src/test_support_impl.rs`, `crates/sweep/src/test_support.rs`, `crates/{topo,sweep,mesh,step-export,editor-core,stl}/tests/**`, `crates/*/Cargo.toml`, `scripts/gates/test-features-dev-only.sh`. **2 sub-rows still open** (generic `cube<T: Real>` in `crates/sweep/tests/m6_surgery_interval.rs`; local body builders in `crates/{mesh,step-export,editor-core}/tests/`); the rest landed in #668/#679 |
| `topo-arena-census-duplicate-spellings` | **M** | Same open remainder as S52: scalar generalization plus 3-crate fixture sharing | `crates/sweep/tests/m6_surgery_interval.rs` + `crates/sweep/src/test_support.rs` (generic `cube`), `crates/{mesh,step-export,editor-core}/tests/` body builders (items 1–3 landed in #679) |
| `editor-core-suites-carry-eleven-part-resolver-stubs` | **M** | Migrate ten-plus suites, reconcile drift between copied stubs and helpers | `crates/editor-core/tests/fixture/`, `crates/editor-core/tests/{mate1_member_vocab,mate1_r1_probes,mate1r2_probes,mate6_gather_mints,mate6r1_shared,mate6r2_probes,asm_r2b_assembly,fix_pattern_mate_crossing,rev_fix_xsplit_unreachable,msolve1_transform_aware,msolve2_member_chain}.rs` |
| `D114` | **M** | New differential instrument; what "bit-identity vs f64" asserts needs deciding and corpus plumbing | `crates/editor-core/tests/m5_pr5_corpus_probe.rs`, `crates/editor-core/tests/m4_pr8_k_probe.rs`, new differential test in `crates/editor-core/tests/`; Track K fence `scripts/gates/*`, `tools/*`, `docs/K-REPORT.md` |
| `D403` | **M** | Row's own first question is lift-a-helper vs drive `stops()`; not settled | `demos/tour/src/teapot.rs` (`stops`, walls 2–3) |
| `S392` | **H** | 18 sites, six crates plus tour, tool and Python; cross-track population, four unconvertible | `crates/sweep/tests/*` (8 sites), `crates/mesh/tests/*` (4), `crates/editor-core/tests/corpus/loft_prism.rs`, `crates/step-export/tests/common/mod.rs` + `tests/fixtures/*.expect`, `crates/step-import/tests/review_probes_m7_3.rs`, `crates/pncad-py/tests/test_north_star.py`, `demos/tour/src/skinned.rs`, `tools/tess-meter/tests/rows.rs`, `crates/sweep/src/skin.rs` |

## Order

`S391` opens — two helpers, one rename, and it is the smallest thing
here. Then `genus-rings-helper-…`, whose only real question is that the
shared home has to cross into a separate cargo workspace to reach the
`demos/tour` copies.

Then the `S52` / `topo-arena-census` pair as one unit, then the
`editor-core` stub migration (which is eleven suites and is where the
drift between copies has to be reconciled rather than merged away).

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
