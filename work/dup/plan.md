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
| `topo-tests-brick-copies` | **claimed 2026-09-16.** 23 `fn brick` across 23 `crates/topo/tests/` files (the row's 24 counted `brick_with_torus_face_at`, a different fixture), every one a line over `common::prism_z`, plus 11 renamed or inline box spellings the name-shaped census could not see. Its stated blocker — a `topo`→`sweep` dev-dependency — was not one: the copies already build the body the row said they could not, and `topo`'s manifest already carries three edges of that kind. Split; the residue is the row below |
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
| `brick-has-two-constructions-and-two-homes` | **opened 2026-09-16**, not inherited: the Euler-built and extrude-built boxes are two spellings in two homes, and the shared home is `topo`'s, downhill, not `sweep`'s uphill. Owes a measurement before a fix |

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
measures. **No A/B** — Ev, 2026-09-16, and it is not to be reinstated
without him.

Ev set the same rule from the other side in the same sitting:
**style-only by default, a full review reserved for units whose logic
is tricky to get right.** The two readings agree, because a unit that
changes what a suite MEASURES is exactly a unit whose logic is tricky —
its diff can be green and wrong, and green is then evidence about the
new assertion rather than about the kernel. So the tier is decided by
one question: **can this unit change a verdict?** A diff that only
moves a fixture's declaration site cannot; a diff that reconciles two
drifted fixtures, or unifies comparisons that disagree about what they
cover, can.

### The slate, tiered

| row | tier | why |
| --- | --- | --- |
| `topo-tests-brick-copies` | style | 24 wrappers over one builder every suite already imports; no call site's body changes |
| `sweep-boolean-suite-brick-and-prism-copies` | style | byte-identical copies, home already exists |
| `run-with-a-prior-evaluation-has-seven-private-copies` | style | one `run_with_prior`, seven call sites, no assertion moves |
| `sweep-test-support-two-wrapper-conventions` | style | a signature convention and two senses of `_at`; no verdict rides on it |
| `three-part-resolver-stub-residues-resist-the-shared-fixture` | style | three residues against a fixture that already exists |
| `orient-module-prose-accumulation` | style | prose only |
| `value-channel-digest-tag-24-collides` | **full** | the discriminator is not injective and a comment says it is; the fix has to make the tag allocation stop being hand-written, or it mints the next collision |
| `tests-common-body-fixtures-triplicated` | **full** | three of six fixtures have already DRIFTED, so reconciling them changes what `mesh`, `stl` and `step-export` suites measure — a merge would silently pick one behaviour |
| `mass-properties-bit-comparison-has-thirty-spellings` | **full** | ~110 sites that disagree about whether the pads are in the comparison; unifying them decides 31 suites' coverage. Likely more than one unit |
| `editor-core-raw-twin-planes-unreconciled` | **full** | the rows assert two spellings reach the same body; the fix decides whether that equality is enforced or merely restated |
| `brick-has-two-constructions-and-two-homes` | **full** | owes a measurement first, and if the bodies differ the remedy inverts from "share it" to "name them apart" |

Two rows on the opening slate are **not this program's** and stay with
S-TINT: `mate6r1-shared-has-eleven-tests-and-no-assertions` (a coverage
defect, S-TINT's charter) and `corpus-result-node-loops-skip-silently`
(a shape, not a copy count). They were listed because the units that
found them were duplication units, which is provenance, not ownership.

## Exit criteria

Inherited from SUITE's X1–X6 (`docs/SUITE-EXIT-WALK.md` at the SHA
`docs/DOC-LEDGER.md` names), with one change earned this session:

- **X1–X2, X4–X6** unchanged.
- **X3 is widened**: each unit states what its sweep could not match
  **and the scope it ran over**, with the scope re-derived rather than
  inherited. SUITE met X3 in the letter and was defeated twice by the
  half it did not ask for.
