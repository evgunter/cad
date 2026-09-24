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
| `topo-tests-brick-copies` | **CLOSED 2026-09-16, PR #2720.** The row said 24; the class was **66** — 23 named `fn brick`, 17 renamed or inline, 26 let-bound `Prism`s read only for `.body`. Three instruments, no one of which found half of it. Its stated blocker — a `topo`→`sweep` dev-dependency — was not one: the copies already build the body the row said they could not, and `topo`'s manifest already carries three edges of that kind. Split; the residue is the row below |
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

## Method — what THIS program has added, 2026-09-16 to 09-19

Seven units, seven merged PRs. Every one had its stated count move
while it was being worked: 5 → 8, 2 → 16, 11 → 14 → 17, 17 → 19,
56 → 145. Item 1 is not a caution here, it is the observed rule.

7. **Denominator-first classification, when the count will not settle.**
   Five instruments ran on the `solid_of_face` class and the count moved
   every time. The sixth inverted the question: instead of searching for
   the *walk*, enumerate the **terminal read** every member must contain
   — every textual `.solid` in every tracked file, all roots, all cfgs,
   149 hits — and classify each one **backwards** by where its receiver
   came from. **A search over a shape has a blind spot for every way the
   shape can be written; an enumeration over a required atom has one
   only where the atom itself can hide.** Close the atom's own blind
   spots by measurement (no `.solid()` accessor exists, no
   `Shell { solid, .. }` destructuring exists) and state the one that
   cannot be closed (a macro-assembled member).

8. **A disclosed blind spot is an instruction to run a third
   instrument, not a licence to publish the count.** The cube row wrote
   *"it undercounts every builder that loops"* and then published. The
   loop-written copy it named was sitting in `src/`, declaring itself a
   copy in its own doc comment.

9. **Publish the hit list, not the pattern** (`implementer-discipline`
   §5, which earned its keep twice here). A reconstructed instrument
   fired on the one site its own unit had missed — so the member was
   inside reach and simply was not dispositioned. **A bucket
   disposition is where a census loses things**, not the threshold,
   which is where everyone looks. Name every hit and what became of it.

10. **The instruments this program has, and what each cannot see.**
    Arity-shaped (misses a loop-written builder); name-shaped (misses a
    new name — structurally, always); geometry- or type-shaped (misses a
    member that declines the axis); **prose** (finds a self-declared
    copy that no code census reaches — `git grep 'in-crate copy'`);
    **structural needle**, the distinctive step rather than the arity;
    **type-directed**, `#[deprecated]` on the fields and pair the
    warning spans — the compiler is a census instrument and reads what
    no regex can, but `cargo check --workspace` does not compile every
    cargo root and feature-gated code never type-checks, so it never
    warns; **sibling-door re-census** — take the door a change cites as
    its *precedent* and census that door's own walk; and
    denominator-first, above.

11. **A mutation is the only proof a fold is safe.** Twice this program
    folded a refusal that named *which* key went stale onto a door that
    could name nothing, and **every test stayed green** — 727 once,
    4405 the next. Plant the flattening. If nothing reds, the guard is
    the deliverable, not the fold.

12. **Stale modality, not just stale numbers.** Text whose scope was
    true when written, read later as though it had none: a
    justification (*"no manifest edge is added"*) read as a
    prohibition; a rule retracted in `memories/` and still stated in
    nineteen files; a constant recording a measurement nobody re-took;
    and **a door minted by a fix pass, whose doc sentence inherits that
    pass's fence silently.** Before waiting on Ev for any of it, run
    `git log -S'<sentence>' --all` and read the author — **`--all` is
    not optional in this shallow clone**, or the check lands on a
    parentless bot render commit and misattributes ratified text.

13. **A door's rustdoc carries an invariant for a user, not a
    measurement for a future lane.** Two doors in a row shipped a census
    sentence with nothing holding it true. The fix is not to guard the
    sentence; it is to make no census claim at the door and let the row
    hold the measurement, dated, with its instruments named.

14. **File the residue as items, one per seam owner** (`work/README.md`,
    Ev 2026-09-06). *"That sweep sees items, not sentences."* A lane
    argued that separate rows would mint the duplicate this program
    exists to prevent; the argument against duplicate rows is an
    argument for one row per owner, not for zero. Where a row sits off
    its territory owner's slate, **say why in the row's own body**.

15. **The orchestrator is inside method item 1.** Six times this
    sitting a number or a posture arrived in a lane's report, went into
    the next brief as fact, and was wrong — a baseline off by 14, a
    hazard population of 2 that was 16, a posture claim about `expect`
    that the code did not carry. **Do not put a count or a posture in a
    brief; tell the lane to measure it.** A delta can be right while the
    baseline under it is wrong, and checking the delta feels like
    checking the number.

16. **A plant that relaxes a one-sided assertion is not a probe.** The
    newest and most dangerous way a mutation lies. A fixture was planted
    AWAY from a rim to test whether anything watched it; the consuming
    row asserts `examined == 0` for a plate clear of the rim, so the
    plant could only ever make the assertion easier, and the suite went
    green. Read naively that is "this fixture is asserted by nothing" —
    a coverage finding, filed, wrong. Re-planting TOWARD the rim
    reddened it at once. So before reading a plant's result, **say which
    direction makes the predicate harder**, and plant that way; a
    `count == 0`, `is_empty` or "loses none" row is satisfied by every
    move in one direction. Verified independently by the reviewer
    (2026-09-20, `dup/one-line-fixture-wrappers`), with the caveat that
    matters: the trap is a property of the fixture's **row set**, not of
    the fixture. A sibling fixture in the same unit reds in both
    directions because it also feeds an accepting corpus row; the clean
    isolation is a fixture whose only consumer asserts one-sidedly.
    Its relatives: a symmetric change cannot reach a row that asserts
    additivity of a pair built from two copies of one fixture (the same
    unit found a site live on a fixture's height and dead on both
    in-plane extents, whose FIRST plant left the suite green), and a
    plant in a door the folded site does not route through cannot reach
    it at all. **One plant is not a probe; a plant whose direction you
    have not argued is not a probe either.**

17. **A plant's restore must restore exactly what the plant changed,
    and nothing else.** A lane's replant loop began each iteration with
    `git checkout <file>`, which silently reverted three uncommitted
    fixes before the plants ran — so the figures it was about to
    publish described a tree it did not mean. `git checkout` on a path
    is a whole-file revert with **no memory of what it is reverting**,
    and cannot tell the plant from any other edit in the file.
    "Commit before planting" is one sufficient way to make a blunt
    revert safe and is good hygiene, but it is not the rule: it fails
    the moment someone plants in a tree carrying an unrelated edit.
    The rule is a copy/restore of the file's pre-plant bytes, or a
    patch/reverse-patch pair. **The reusable half is the detection**:
    the lane found it by diffing against `HEAD`, and that check belongs
    in any plant harness. (2026-09-20, `dup/one-line-fixture-wrappers`;
    the general form is the reviewer's, not the lane's.)

18. **A measurement taken to prove an instrument unreliable needs the
    same re-take as any other measurement.** A lane documenting that
    `git log -S` cannot answer a ratification question in this
    shallow clone reported, as its evidence, that both queries returned
    *"the same five commits, none of which touches either file"*. Each
    returns **108**; the sets differ; **107 are parentless**, so
    `--name-only` shows them touching the file. Every particular was
    wrong and the conclusion was right — and better supported by the
    true figures, because 108 non-answers at graft boundaries is a
    stronger demonstration than five irrelevant commits. Item 15's
    shape inside the row that exists to warn about it: **the number
    offered as proof that a number cannot be trusted is still a
    number.**

19. **One plant cannot both prove a site reached and prove its answer
    unasserted.** The operative test, in the lane's own words: **to
    prove a site is REACHED, plant something no answer can satisfy; to
    prove its answer is UNASSERTED, plant a different answer.** And the
    shape that makes a wrong control look rigorous: **a plant that
    replaces a function's body with its own null return value is not a
    control — it is the same experiment with a wider swing.**
    How it arose: an order plant reddened nothing, and the lane — right
    to refuse to publish *"the order is unasserted"* without a control —
    replaced the whole body with `None` and got nothing again. That
    cannot tell *called and unasserted* from *never called*. The
    discriminating plant, `panic!`, gave **732 / 1**: reached by exactly
    one row, answer wholly unasserted. The conclusion survived by luck
    of the tree, not by the control. A row claiming a site is dark owes
    the divergent line in its table.
    Verified in the field the same day: a shared door's value, planted
    at 50x coarser, 10x finer and **5000x coarser** with nothing red,
    proved *executed* by a `panic!` control redding **75 rows across
    every calling suite** — called and wholly unasserted, which is a
    coverage finding rather than a dead fold.
    (2026-09-20, `dup/shells-of-solid-door` and `dup/viewer-shared-doors`;
    the wording is the implementer's, the null-swing clause the
    reviewer's.)

20. **Briefed reading must be on the branch the lane will cut from.**
    Twice in one sitting the orchestrator pointed lanes at text that
    existed only on `dup/orchestrator`: a log entry, and then method
    items 16–18 themselves, cited in three briefs while `main`'s
    `plan.md` stopped at 15. One lane said so and read it out of the
    orchestrator's checkout; the others said nothing, so whether they
    read it is unknown — **the failure is silent at the lane's end**. A
    dangling citation in a brief is item 15's shape with the
    orchestrator as the hand it passes through: before a brief cites a
    file, section or numbered item, check it is on the branch the
    worktree is cut from, not merely on the branch that wrote it.
    A third instance, a different shape: a brief asserted that
    `crates/viewer/tests/common/mod.rs` states a checkable biconditional
    between each module's list and its markers. That text is in
    **`crates/sweep/tests/common/mod.rs`** — a fact learned from one
    crate's shared module, transposed onto another's by an orchestrator
    who had read it an hour earlier in a review of a different unit.
    Neither string occurs anywhere under `crates/viewer/tests/`. **A
    fact about `<crate>/tests/common` is about that crate**, and a
    brief that generalises one is asserting a census it has not taken.

21. **A lane's scratch path must be lane-private, for the same reason
    its `CARGO_TARGET_DIR` is.** Three concurrent lanes shared one
    scratchpad directory. A sibling's `plant.py` — written for a
    different worktree — **overwrote another lane's harness by name**
    between writing and running it. The run produced **empty output and
    exit 0**, which reads exactly like *no plant reddened anything*. It
    was caught only because the output file was zero bytes.
    This is the ENOSPC hazard's twin and the sharper of the two: a
    truncated run at least fails, while a clobbered harness **succeeds
    at doing nothing**. Both produce a green that is an artefact of the
    apparatus rather than a fact about the tree, which is the one thing
    a plant exists to rule out. So: a lane writes its harness under its
    own worktree or its own target dir, never a shared scratch path;
    and a plant that reds nothing is not read until its output is
    confirmed non-empty and its `test result:` line complete.
    (2026-09-20, `dup/viewer-shared-doors`; found and reported by the
    lane it happened to. The shared path was the orchestrator's
    arrangement, not the lane's.)

22. **A correction inherits the instrument class of the thing it
    corrects.** A lane was told a published count was wrong, re-took
    it — and **its re-take was wrong the same way**: a four-line window
    read a `.to_vec()` six lines below its call as a borrow, exactly
    the window failure that produced the original figure. Its own
    sentence is the item: *the instrument that corrects a count can be
    wrong the same way the count was.* And the sentence proved itself
    in place — the **next paragraph** of that same fix published "31
    reaching within five lines" where the true figure is **33**, two
    sites pushed out of the window by a four-line `unreachable!` string.
    Three distinct corrections were wrong this sitting (a line gap
    corrected from a right number to a wrong one; a case-sensitive grep
    "correcting" a population from seven to six; these two windows), and
    the common cause is that a correction arrives already framed as a
    fix and so is read rather than re-measured. **Change instrument when
    you correct, not just the number** — if a window produced the error,
    the correction is not another window.
    (2026-09-20, `dup/shells-of-solid-door`; the diagnosis is the lane's
    and the second instance the reviewer's.)

23. **After a plant, read the rows you expected to red and did not —
    and treat a passing row that printed a panic as a swallowed
    failure.** A fold's proof plant reddened 22 rows, and the count
    read as complete. Two rows that should have been in it were not:
    under `--nocapture` both print *"the fixture cannot be minted at
    this eps — standing down"* and report **ok**, because a
    `catch_unwind` wrapper cannot tell an unmintable fixture from a
    broken builder. They are the only rows in that crate no mutation of
    the builder beneath them can red. The count alone would have read
    as 22 of 22 expected.
    The cheaper half, which the reviewer supplied: the panic hook still
    prints a **full backtrace to stderr** before the row passes, so the
    hole is not silent — it is loud, and reported green. **A passing row
    that emitted a backtrace is a swallowed failure**, and that is
    greppable where "the rows I expected" is a judgement.
    (2026-09-20, `dup/src-cyl-sheet`; the corollary is the lane's, the
    backtrace instrument the reviewer's.)

24. **A negative result carries the scope of the search that produced
    it.** A lane was asked whether a convention existed in a crate's
    shared test module. It searched that crate's `common/mod.rs`, found
    nothing, and reported *"neither string occurs anywhere in
    `crates/viewer/tests/`"*. True of the file it read; the convention
    lives in a **sibling crate's** `tests/common/` **submodules**. Its
    own diagnosis is the item: *my negative result was correctly scoped
    to the file I searched and wrongly stated as a fact about the repo.*
    The orchestrator made the mirror-image error in the same exchange,
    asserting a fact learned from one crate's `tests/common` about
    another's. Both are item 3 (**re-derive the SCOPE**) in the negative
    direction, where it is harder to see: a positive hit carries its own
    path, and an absence carries nothing at all. **State a negative with
    its fence attached** — "not under `<path>`, with `<instrument>`" —
    never as a bare "there is none".
    (2026-09-20, `dup/viewer-shared-doors`; the wording is the lane's.)

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
| `brick-has-two-constructions-and-two-homes` | **full** | measured 2026-09-16: one fixture, not two. Now **parked** as the third of three links — reconcile the cube sequence, thread `tol: Tol` through the family, then move and unify |
| `topo-tests-geometric-cube-and-cube-into-are-one-sequence-twice` | **full** | claimed 2026-09-16 as the first of what are now **four** links: ninety lines written twice, and `geometric_cube`'s rows assert on the one step that differs |

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
