# COMB — the roll-ups and the sweeps that go last (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

Branch prefix: **`comb/`**. Away-channel tag `(COMB orchestrator)`.
A/B ordinal band **COMB = 4300–4399**.

## Charter

Two kinds of row that share one property: **no fence can hold them.**

- **The sweeps.** `L1`–`L5`, `S36`, `S37`, `S38`, `D79` — comb-and-rename
  over ~230 suites, comment trimming over every crate's `src`, the
  milestone-coded rustdoc in four kernel crates, this program's own
  audit of its struck rows and its one-sided dispositions. Each is
  document-wide or workspace-wide, operates on files whose fate earlier
  rows have not settled, and was written from the start as
  *not takeable while a track is open on those files*.
- **The roll-ups.** `S35` (62 live sub-rows), `S11`, `S19`, `S43`. These
  are not units and must never be dispatched as units. **A roll-up is
  PARTITIONED**: a member whose ground a live program owns is filed
  there as its own row and struck here; what is left when the tracks
  empty is what this program actually carries, and `L3` is the name of
  that remainder.

## Territory — none, by construction

This program claims **no paths** and cannot. `L2`'s population is every
`crates/*/src/**/*.rs`; `L1`'s is every suite; `S35`'s spans eight
crates. Claiming ground for them would put this program in conflict with
every other program in the tree simultaneously, which is precisely the
fact that made them "last, deliberately".

**The gate on dispatch is therefore not a fence but a question**: does
any live program have an open unit on the files this row would touch? If
yes, the row is not takeable. That question is asked at dispatch, not
written down here, because the answer changes weekly.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `S43` | **M** | Rule ratified as D2 addendum; leftover is one crate's conversion with per-site proofs. | 5 idioms, 4 discharged; residue = `crates/mesh/src/{tessellate.rs,walk.rs,chords.rs,curved.rs,planar.rs,trimmed.rs}` (idiom 2), plus S14-owned `docs/DESIGN.md` + `crates/topo/tests/review_m1_pr5_internal.rs` |
| `S19` | **H** | ~260 refusal sites over 5 crates; downstream of S43's addendum; placed as D36/D48/D366. | 3 table rows + 5 postmortem rows: `crates/mesh/src/*.rs` (`MissingEntity`, 49 sites), `crates/geom-brep/src/pcurve_cache.rs` (22 sites), `crates/topo/src/validate.rs` (59 variants), `crates/pncad-py/src/tags.rs`, `crates/editor-core/src/{edit.rs,program.rs}` |
| `S11` | **H** | Roll-up over 8 crates; several rows still await an unasked delete-or-keep call. | ~14 table rows + 12 sort rows + 4 still-tabled: `crates/geom-brep/src/{props/quad/,pcurve.rs,ssi.rs}`, `crates/geom-core/src/{spline/hull.rs,linalg/vec.rs}`, `crates/geom/src/curves/boxes.rs`, `crates/topo/src/boolean/mod.rs`, `crates/editor-core/src/{eval/wire.rs,resolve/vdiff.rs,mate.rs,mate/{coset,solve}.rs}`, `crates/step-import/src/recognize.rs`, `crates/profile/src/validate.rs`, `crates/sweep/src/blend/open/planar.rs` |
| `S35` | **H** | Sixty-two independent findings spanning every track; partitioned out row by row. | roll-up of 63 table rows (1 struck closed 2026-09-04, 62 live) over `crates/editor-core/src/{meta,persist,mate,names,resolve,eval}/**`, `crates/topo/src/{validate.rs,euler*.rs,boolean/,splitting/,null.rs,body.rs}`, `crates/geom-brep/src/{ssi/,certify.rs,pcurve_cache.rs,blend/,props/}`, `crates/geom/src/curves/**`, `crates/geom-core/src/{linalg,spline}/**`, `crates/step-import/src/**`, `crates/profile/src/**`, `crates/mesh/src/trimmed.rs`, `crates/bvh/src/lib.rs` |
| `L3` | **H** | Residue of a 60+ row table spanning every track's territory; scope unknown until tracks empty. | whatever S35 rows no track claimed: `crates/{editor-core,topo,geom-brep,step-import,profile,geom,mesh,bvh}/src/**` |
| `S36` | **H** | Comb-and-fixup ~230 suites plus lift `review_*` out of src | `crates/topo/tests/review_m*`, `crates/topo/src/review_m1_pr*.rs`, `crates/step-import/src/cr_r1_probes.rs`, `crates/*/tests/*.rs`, `crates/*/src` stale `docs/M*-LOG` refs |
| `L1` | **H** | Per-suite comb-then-rename over hundreds of files; collides with every fence | `crates/*/tests/*.rs` (~230 suites), `crates/topo/src/review_m*`, `crates/step-import/src/cr_r1_probes.rs` |
| `S37` | **H** | Cross-cutting rustdoc/comment sweep over four kernel crates; collides with every track | `crates/topo/`, `crates/editor-core/`, `crates/geom-brep/`, `crates/geom-core/` rustdoc (~1115 lines); 473 `//` comments workspace-wide |
| `S38` | **H** | Judgement-heavy prose trim across many crates; every track's files | `crates/pncad/src/{closure.rs,select.rs}`, `crates/mesh/src/{lib.rs,walk.rs}`, `crates/profile/src/fillet_select.rs`, `crates/geom/src` (`speed_lower_bound`), `crates/topo/src/boolean/ops.rs`, `crates/geom-brep/src/props/quad/`, `crates/sweep/src/blend/surgery.rs`, `crates/geom-brep/src/{planar.rs,curved.rs}` |
| `L2` | **H** | Workspace-wide sweep colliding with every track; ordering rule 1 holds it. | workspace-wide comments: `crates/*/src/**/*.rs` (S38 population plus S37's 473 milestone-coded `//`) |
| `D79` | **M** | One file, but what to cut is pure judgement; waits behind L2's sweep | `demos/tour/src/lily.rs` (118-line `//!` header, 40% comment body) |
| `L4` | **H** | Register-wide re-audit of one-sided gate reasoning; collides with every track | no code fence — the register itself: `work/code-quality/*.md` dispositions (and their cited sites when re-opened) |
| `L5` | **H** | Program-wide walk of every struck row on every track; no fence, citations re-derived | `work/*/logs/*`, `work/code-quality/*.md` (new rides-along files), `docs/DOC-LEDGER.md` |

## Order

Partition before sweep, and within that, **decide before you delete and
delete before you polish** — the ordering these rows were written under,
and the reason every one of them says it goes last: they operate on
files whose fate earlier rows have not settled.

1. **Partition the four roll-ups.** `S43` first — four of its five
   idioms are already discharged and the residue is one crate's
   conversion. Then `S19`, `S11`, `S35`. Each partition PR files rows on
   live programs and strikes the members it filed; none of them fixes
   anything. This is most of the program's actual output.
2. **`L3`** is by definition what step 1 leaves, so it is not readable
   until step 1 finishes.
3. **The comb pass** — `S36` then `L1`, per-suite review before
   comb-and-rename, which is the order both rows already state.
4. **The comment passes** — `S37`, `S38`, `L2`, `D79`. Last, because
   every earlier deletion changes what there is to comment on.
5. **`L4` and `L5`** are audits of this program's own record and can run
   at any point; they produce findings and rides-along files, never edits
   to another program's live item.

## Review posture

Infra-and-prose for the sweeps: one style review per unit, no A/B row.
The partition PRs take a second reader whose only question is **was a
member filed or was it quietly dropped** — a struck row with no new home
is the exact loss `L5` exists to walk back.
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

## From REROUTE (merged 2026-11-20)

**Charter.** Each row names a door (or, for the rustdoc row, the rule in
method item 13 below) that is already on main; the unit routes the
remaining members onto it, one PR per batch of rows. The method items
below bind every lane here, and briefs cite this file by path. They are
S-DUP's, numbers kept, moved here when S-DUP closed on 2026-09-28
(`docs/doc-ledger/dup-leaves-the-tracker.md`).

**What differs from S-DUP.**

- **A member that cannot reach the door is not a routing failure.** A
  `src` test module cannot see `tests/common`; a demo is written from the
  public API's seat and does not import `test_support`. The row says which
  members are out of reach and why, and closes without them rather than
  minting a second home. A second home is a new door, and a new door is
  a design question: the row goes by `git mv` to the program that owns
  the crate the door would live in.
- **Rows are cheap and many; batch them.** One lane takes several rows in
  one crate, and a batch lands as one PR (Ev, 2026-09-26: combine units to
  spare CI).
- **Hosted CI gates.** The local-CI-while-hosted-queues arrangement
  (Ev, 2026-09-26) ended when hosted CI recovered: no `[skip ci]` on a
  code change.

No exit criteria are set: the program closes when its slate is empty.

### Method — what SUITE learned (S-DUP items 1–6)

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

### Method — what S-DUP added (items 7–29)

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
    hold the measurement, dated, with its instruments named. **And no
    pointer to the row either**: a closed program's directory is
    deleted (`work/README.md`, the closing rules), so a rustdoc line
    naming a `work/` file dangles the day the program closes, and "re-
    measured by a lane" is a promise nobody keeps once the row is closed.
    Three doors carried one until #3284 removed them (2026-09-26).

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

25. **"Done" means present at HEAD, not reported by the editor.** A
    lane's edit script checked every anchor before writing and, on one
    stale anchor, wrote nothing and exited cleanly; it had lost two
    corrections that way in an earlier pass, and the lane had reported
    both as done. Only the orchestrator's re-check against HEAD, item
    by item, found the gap. This is item 21's failure seen from the
    reporting side: a tool that refuses to act looks the same as one
    that acted. **Confirm each claimed change by reading HEAD (`git
    show`, a grep for the new text) before you report it**, and a
    brief that asks for a list of fixes gets back a line per fix with
    the sha that carries it.
    (2026-09-24, `dup/shells-of-solid-door`; the lane found it itself
    on re-check.)

26. **Read EVERY `test result:` line, not the last one.** The standing
    brief line said "read the LAST `test result:` line", which fixed a
    panicking shard's early line. It is wrong under `--no-fail-fast`: the
    last line printed is the integration binary's, green, while the lib
    binary above it failed. A reviewer following the brief literally would
    have read a red plant as green. Read each binary's line and the panic
    lines under them; the row-sum-to-baseline check (the second sitting's
    log) still applies per binary.
    (2026-09-25, caught by `#3151`'s reviewer in the orchestrator's brief.)

27. **A CI roster is the tree's, not the orchestrator's.** The
    orchestrator verified runs against "39 jobs, 12 `test`, 5 `k-lint`"
    and nearly reported a correct lane as narrowed: RING-4 had deleted the
    kernel `interval` feature the day before, so six `test` jobs was the
    whole matrix. Establish green the way implementer-discipline §2 says
    — the `change filter` job's own `EPS`/`KLINT_ROW` lines and the
    run's conclusion — and take the roster from `scripts/ci-filter.py`
    when a count is wanted. A memorised count is item 22's shape: a
    measurement that was right once and is now an instrument.
    (2026-09-25, `dup/owner-index-divergence`.)

28. **A local mirror can agree with its twin on paper and still
    differ at runtime.** `check-ci-mirror-parity.py` compared every row,
    flag, env var and directory of `ci-local.sh` against `ci.yml` and
    passed, yet the viewer app row reddened locally: hosted installs a
    software Vulkan adapter (`mesa-vulkan-drivers`, lavapipe) as a
    package, and a package is not something either script spells. The
    first local run of a mirror is itself the parity check, row by row
    against a hosted run on the same commit; read its failures for an
    environment the hosted half provisions before reading them as the
    change's. (2026-09-26, `#3285`'s local run; after the install the
    row went 978/978 on `#3284`.)
29. **A gate that outlives the session's attention must be a task the
    harness tracks.** The remote container is recycled when the session
    sits idle. A `setsid`-detached `ci-local.sh` has survived hours
    while lanes kept the session busy, but it died twice in #3302's
    gate once only the hourly check-in woke the session: once in the
    1e-12 row, and once three minutes in. The same run, launched as a
    harness-tracked background command, held the container for its
    full 3 h 50 min and reported on exit. Launch a long local gate that
    way. A run that dies still leaves its log of finished rows; restart
    on the head it was meant to gate, not on a patched-up tail. Merge
    main first if main has moved. (2026-09-26, #3302.)
