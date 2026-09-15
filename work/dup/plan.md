# S-DUP — one thing spelled n times (plan)

**STATUS: OPEN (2026-09-15).** Opened as SUITE's successor, on Ev's
approval on the SUITE exit walk's `[ev]` PR (#2685).

Branch prefix: **`dup/`**. Away-channel tag `(S-DUP orchestrator)`.
A/B ordinal band **S-DUP = 4900–4999**, allocated and unused until the
A/B protocol is next run.

## Charter

**One thing spelled *n* times, where the copies are test-side
vocabulary.** Not "a test that cannot go red" — that is S-TINT's
charter and this program defers to it. The two questions look alike and
are not: a suite with no assertions is a *coverage* defect, and a
fixture built from scratch in six crates is a *vocabulary* defect. A row
that is genuinely both goes to S-TINT first.

## Why a successor rather than folding into S-TINT

SUITE's eight rows closed and its slate is empty. What it established is
that **its rows were samples**: the class behind each was larger than
the row by roughly an order of magnitude, and the discrepancy was
discovered by the unit, not by the row. Twelve rows now sit on S-TINT's
slate naming those classes. `work/README.md` says a dozen items on one
territory is a successor's opening slate; this is that.

## Territory — none, inherited from SUITE with its reason

The homes live in other programs' `src/` (`sweep::test_support` is
BLEND's, `topo::test_support_impl` is TOPO's) and the copies live in
`crates/*/tests/*`, which is S-TCOST's and S-TINT's in every crate. So
this program claims nothing and announces by seam, exactly as SUITE did.
X6 held for SUITE start to finish; it is the inherited default here.

## The opening slate — twelve rows on S-TINT

They are S-TINT's files and stay there unless and until this program
claims one by `git mv`, per `work/README.md`'s one-file-one-item rule.

| row | the class, as measured |
| --- | --- |
| `topo-tests-brick-copies` | 24 `fn brick` in 22 `crates/topo/tests/` suites; blocked on whether `topo` may dev-depend on `sweep`, which is a crate-graph decision with three options worked out in the row |
| `tests-common-body-fixtures-triplicated` | `ball`, `donut`, `l_prism` byte-identical across three `tests/common` trees; `cone`, `washer`, `holed_prism` **already drifted**, so it reconciles rather than merges |
| `mass-properties-bit-comparison-has-thirty-spellings` | ~110 occurrences over 31 files, and the spellings **disagree about whether the pads are in the comparison** — which is what gives the class teeth |
| `run-with-a-prior-evaluation-has-seven-private-copies` | seven files, of which three are byte-identical pairs: one function and three option presets, written out seven times |
| `sweep-boolean-suite-brick-and-prism-copies` | the `prism(pts, h)` half, re-scoped against the real home |
| `sweep-test-support-two-wrapper-conventions` | five `x`/`x_at<T>` pairs, two justified and three avoidable; and `_at` carries two unrelated senses across eleven doors |
| `three-part-resolver-stub-residues-resist-the-shared-fixture` | `asm2a`'s superset store, `asm_r2a`'s signature, `asm4`'s `usize` constant |
| `mate6r1-shared-has-eleven-tests-and-no-assertions` | S-TINT's by charter; listed because the unit that found it was a duplication unit |
| `orient-module-prose-accumulation` | one argument spelled six times across three files; prose duplication, and nothing routes prose |
| `editor-core-raw-twin-planes-unreconciled` | two spellings of one placement, kept in step by hand |
| `corpus-result-node-loops-skip-silently` | the shape, not a copy count |
| `value-channel-digest-tag-24-collides` | two tags claiming 24 |

## Method — what SUITE learned, as the way this program works

1. **A row's count is a candidate list until re-taken.** Four of
   SUITE's eight rows were undercounts; `S392`'s went 11 → 18 → 21 → 23
   → 24, defeated in turn by the row, the orchestrator, the lane and the
   reviewer. **Re-take every census at the merge base** and put the
   receipt in the PR.
2. **Grep the construction, not the name.** Name-shaped censuses miss
   the renamed copy every time — `boxy` for `brick`, a `Probe` cube,
   `impl editor_core::PartResolver for` spelled path-qualified.
3. **Re-derive the SCOPE, not just the pattern.** Two of SUITE's misses
   were where the lane looked, not what it matched: one crate instead of
   the workspace, and `crates/ demos/ tools/ scripts/` with `docs/`
   never in the set. `git grep` over every tracked file with **no path
   argument** is the only form that makes no path claim. A wrong scope
   is worse than a wrong pattern because it returns tidy results and a
   confident count.
4. **A mutation is a sweep instrument.** Planting a wrong constant and
   counting what reddens found a consumer no grep had named, and
   measured the guard that fixed it (73 → 224 rows red).
5. **X4 is not optional and not self-checkable.** Every SUITE unit that
   closed a duplication minted or nearly minted one, twice inside the
   paragraph naming the trap. Only a reader who did not write the fix
   ever caught it. **Every unit here gets that reader.**
6. **A shared home, not a deletion**, and the standing question at every
   call site: does the helper still make that suite's intent readable?
   Answering *no* at a site is a result — SUITE did it at three.

## Review posture

Test-side, S-TINT's posture: one style review per unit, and a full
review where a unit changes manifests, feature gates, or what a suite
measures. No A/B unless Ev reinstates it.

## Exit criteria

Inherited from SUITE's X1–X6 (`docs/SUITE-EXIT-WALK.md` at the SHA
`docs/DOC-LEDGER.md` names), with one change earned this session:

- **X1–X2, X4–X6** unchanged.
- **X3 is widened**: each unit states what its sweep could not match
  **and the scope it ran over**, with the scope re-derived rather than
  inherited. SUITE met X3 in the letter and was defeated twice by the
  half it did not ask for.
