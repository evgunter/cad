# VIEW — viewer architecture (plan)

**STATUS: OPEN, and dispatching (2026-09-04).** Opened 2026-09-03 from
`docs/WORK-TRACKS-2026-09.md` (VIEW section). Orchestrator handed over
twice; the third session took it 2026-09-04 evening. Live state is
`work/view/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`view/`** — unit branches
`view/<unit>-<slug>`, orchestrator branch `view/orchestrator`. Sessions
whose harness pins a branch drive the orchestrator half from that
branch instead; unit branches are unaffected and keep the `view/`
prefix. Away-channel tag `(VIEW orchestrator)`.

**Review posture (Ev, in-chat, 2026-09-04, reaffirmed 2026-09-04
evening).** This program runs **no A/B duals and writes no row in
`docs/MODEL-AB-LOG.md`**, whatever review a unit gets. The A/B band
**1900–1999** stays claimed and empty and the band table says so. The
default is a **style review** against
`docs/prompts/reviewer-style-lane.md`; a second correctness reviewer is
added **only** where a unit's failure mode is a *confident wrong
answer* rather than a refusal, and the dispatch says which it chose and
why. Under this posture the dispatcher's own exposure is the live risk
rather than a formality: unit 1's chain produced **seven** dispatcher
corrections, two against decisions rather than details, so every brief
this program issues states its claims AS claims and says so in as many
words (`docs/REVIEW-STYLE-DISPATCH.md` §3).

**Territory, as of 2026-09-04 evening.** `paths` now covers
`crates/viewer/tests/*` (Ev, in-chat): CHROME's slate landed and that
program has been dormant since 07:00, so the *"CHROME goes first"*
clause is discharged. The glob is still S-TCOST's and Track W's by
declaration, so test-MECHANISM changes are announced, not assumed.
`crates/editor-core` stays DOCM's with **one narrow amendment** Ev
authorised in-chat: `EditError`'s user-facing `Display` wording — the
`edit: ` prefix and the `{:?}`-quoted payloads — because the layer that
raises it has no reason to know the viewer renders it verbatim to a
person, and VIEW cannot fix that sentence from its own side. No variant
is added or removed and no edit semantics are touched.

## Charter

Decide the viewer's shape before more units accrete into a
3,224-line `session.rs` and a 5,696-line `app.rs`. One conversation
gates the rest; the builds after it are mostly E with one hard
concurrency unit.

## Order

Unit 1 is closed. Six further PRs merged on 2026-09-04 and are on
`main`; the sessions that merged them left no log entry, so the entry
that records them is this plan's Order and the log's 2026-09-04-evening
section, written after the fact from `git log`.

1. `viewer-session-god-module-split` — **DONE, 2026-09-04.** Four PRs:
   #1801 ratified the boundary rule, #1816 made gesture safety data
   (`SessionOp::permitted_during_value_gesture`, one exhaustive match
   in `perform`, the 23 call-site guards deleted, no operation's
   answer changed), #1830 split both files, #1832 made the
   one-of-seven tool invariant unrepresentable. `session.rs`
   3,260 → 1,500 and `app.rs` 5,696 → 1,752, thirteen new modules,
   **no test file touched and no assertion changed** across the chain.
   Residue: `session-shims-and-test-imports` and
   `tool-kind-all-and-ordinal-have-no-production-reader`, both open.

2. `pick-priority-filter-vocabulary` — **deferred**, ratified by
   `crates/viewer/README.md` GQ7. The status vocabulary that could not
   spell it is settled: Ev ruled `deferred` into `work/README.md` and
   made `lint` refuse a `parked` row whose blockers have all closed
   (#1857).

3. `camera-fold-clears-status-line` — **DONE, #1849.** `land` stopped
   clearing, its refusal reaches the line through `frame_status`'s
   ranking, the product fault became a badge. The census of the other
   writers was filed, not swept: `status-line-writers-bypass-the-
   ranking` and `four-badges-five-spellings` were its residue. Both are
   now **closed** — the badge family at #1957, and the sweep at #2026,
   which routed seventeen of the eighteen writers through the ranking
   and added `frame::deliver` as the door for a policy that may or may
   not have news. The eighteenth is the startup initializer and has its
   own file, `startup-notices-need-holding-to-badge`, open.

4. `focus-marking-is-per-node-not-per-segment` — **blocked, and the
   blocker is not this program's to clear.** The authored-step to
   canonical-segment map straddles DOCM's `program.rs` and S-BOOL's
   `crates/profile`; the siting question has its own file,
   `work/issues/authored-step-to-canonical-segment-map-has-no-home.md`.

5. `layer3-recipenodeid-aliases-across-rewinds` — DI1's build, ruled.
   **Parked** on `next-id-has-no-layer3-door`, which is DOCM's door to
   shape. Announce standing; nothing in VIEW clears it.

6. `pick-index-built-on-ui-thread` — **DONE, #1888, merged 2026-09-05.**
   6a was ruled by Ev at #1843; **6c collapsed into 6b** under that
   ruling, as the item predicted. The index and its tessellation are on
   their own seam, keyed by `(Generation, DisplayTolerance)`, with no
   `cancel` door at all — Ev's restart-without-cancel answer made
   structural so a later lane cannot wire a token through without
   meeting the argument. Three reviews: correctness, a delta round, and
   style. The correctness lane found a **MAJOR** the whole 483-row
   suite was green over, and the fix removed the shape rather than the
   instance. Seven residues filed as items, none left in prose.

### The 2026-09-04/05 wave — all four units landed

| unit | PR | reviews |
|---|---|---|
| `view/prune-report` (both `prune` discards) | #1886 | style |
| `view/clearing-walk` (the four-site reset) | #1885 | style |
| `view/pick-index-offthread` (6b) | #1888 | correctness + delta + style |
| `view/scene-gathers` (the double gather) | #1908 | style |

Plus #1912, that session's orchestrator state-sync, merged separately
because it is a session's worth of adjudication across five units and
should be visible on its own.

### The 2026-09-06 wave

| unit | PR | reviews |
|---|---|---|
| `view/edit-door-wording` (`EditError`'s `Display`) | #1932 | style |
| `view/module-kind-gate` (the gate's own clean-tree bug) | #1953 | style |
| `view/axes-and-badges` (Ev's provenance rule, made structural) | #1957 | style |
| `view/status-line-sweep` (seventeen of eighteen writers) | #2026 | style + fix pass |
| `view/const-all` (the `vocabulary!` declaration) | #2046 | style + fix pass |
| `view/refusal-all` (`Refusal` has no `ALL`) | #2053 | style + fix pass |
| `view/progress` (the swappable bool pair) | #2055 | style + fix pass |
| `view/index-seam` (Ev's (d): the seam cycle broken) | #2079 | style + fix pass |
| `view/marks` (the second split, and the rename) | #2083 | style + fix pass |
| `view/homes` (`cursor_projection` to `camera`; `Generation::get` deleted) | #2089 | style + fix pass |
| `view/debug-walk` (five field censuses made exhaustive) | #2093 | style + fix pass |
| `view/censuses` (the remaining seven, in four hats) | #2103 | style — **in flight** |
| `view/all-gate` (the `const ALL` gate, filed by #2046) | #2106 | **correctness** — in flight |

**Fifteen units on main. Two rules this wave earned**, both about
evidence rather than code:

- **A sweep owes a TRACKER pass as well as a tree pass** (#2053, below).
- **A grep over a signature is a grep over one line of it** (#2055).
  `rg` cannot see a multi-line `fn` header, so a receipt built on one
  missed a second instance of its own defect a hundred lines away in the
  same file. Parse the parameter list. A receipt offered as evidence and
  wrong about its own file is worse than no receipt, because a reader
  stops looking.

**Prove a claim by COMPILING, not by grepping, when the compiler can
answer it.** #2093's reviewer was asked whether any test observed a
`Debug` dump; instead of grepping for `{:?}` it deleted both impls and
built the workspace — zero errors answers the question completely,
where a grep answers to the limit of its pattern. It also tested a
rejected alternative by writing it and compiling it. Counterpart to
*a grep over a signature is a grep over one line of it*.

**Take a base measurement in a SEPARATE worktree.** #2089's lane ran
`git checkout HEAD -- crates/viewer` to measure its base and clobbered
its own uncommitted edits; it caught and re-applied them, but the safe
shape is a throwaway worktree with its own target dir, which is what
#2079's and #2083's lanes used.

**A §6 report is not a durable artifact.** The same out-of-fence stale
citations were reported through §6 twice — #1848 and #2089 — with
nothing a later reader could find either time, which is the case §6
itself warns about. When a report is a REPEAT, file it in
`work/issues/` instead.

**Re-derive a citation, never shift it — and verify every one by
reading the line.** The class cost this program five instances in one
day, including two lanes each shifting a number by a delta computed
correctly somewhere else, and one orchestrator propagating a lane's
miscount into a check-in. No grep finds it: a citation pointing at the
wrong line still parses. The instrument that works is #2083's —
enumerate every `file:line` in every row a branch touches, `sed -n Np`
each, and read whether the subject is there (39/39 there). Re-run it
after the LAST edit, because a header rewrite moves every line under
it.

**A merge criterion is a TEST, never an absolute.** #2079's fix brief
said the test counts "must still be 24/1 and 501/0/1"; they came back
503 because a concurrent merge of `main` brought two new rows in. The
invariant that survives is *the count must not move against your own
merge base* — checked here as `#[test]` at 533 on both sides. A number
in a brief goes stale the moment anything else lands.

**A universal in prose owes the sweep rule that produces its
population, written at the sentence.** #2103's style review found no
broken code and four broken sentences, and the two that mattered were
both universals: *"Every place in this crate that lists a value's
fields by hand destructures the value instead"* (false —
`BlendTool::clear`, two fields named by hand, in a file the PR edited)
and *"every other one is over an enum and is exhaustive by its `match`
already"* (false as an argument, because a `match` is exhaustive over
VARIANTS and the class is FIELDS — `Display for CameraOp` drops
`bounds` behind an unargued `..`). A sentence that certifies a
population is where the next defect hides, because it tells the reader
to stop looking. The sweep rule is what makes it falsifiable, and it
belongs beside the claim rather than in the PR body that gets thrown
away.

**Settle a CI-scope question by RUNNING the filter, not by reading a
manifest.** The same review reported `prose_census` as possibly sited
where it cannot fire on its own inputs — its subject is every `Display`
in the tree, its trigger looked like `pncad-py`'s dependent closure.
The coupling was described exactly and the disposition was wrong:
`crates/pncad-py/src/prose_census.rs:168-170` walks from the repository
root, and `scripts/ci-filter.py:624-627` pins any crate whose read
lands at the root into every non-docs closure, by construction. Two
commands answered what the dependency graph could not. Counterpart to
*prove a claim by compiling, not by grepping*.

**Check the arithmetic, including the orchestrator's.** Five counts
came back wrong across this wave — four from lanes (a receipt's `60`
with no enumeration rule, "six move by -13" for five, "twelve lines"
for eight, "two public items" for three) and one from here, when a
lane's miscount of the VIEW citations was propagated into a check-in
before the fix pass established the real figure. A count is a claim
like any other: it carries its enumeration rule, and it is re-derived
rather than copied forward.

**Operational, lane isolation**: `CARGO_TARGET_DIR` must be exported in
EVERY command that can reach cargo, including one that only invokes a
SCRIPT which shells out to it. Each Bash call is a fresh shell and
inherits no earlier export, so a brief saying *before any cargo
command* is read as being about the word `cargo` in the command line.
`view/all-gate` reported this against itself: its `scripts/doc-gate.sh`
run built into the worktree's own `target/` rather than the lane's
private one. Harmless there — exclusive to that worktree, gitignored,
nothing reached the branch — but the same slip under two lanes building
at once is how disk reached 1.7G free on Sunday. Dispatches say
*including a script that shells out to cargo* now.

**Operational, for whoever reads the log's CI notes**: the slow interval
shard is not a FIXED shard. `1/2` was slow on #2026/#2046 and `2/2` on
#2055's final run; nextest moves the heavy tests between runs. One
shard carries the tier, not a particular one.

**Twelve units on main. The wave produced nineteen new items** — ten
from the sweep and its review, nine from `const-all` and its review —
every one a file rather than a sentence in a merged PR body.

**Every sweep this program runs owes a TRACKER pass as well as a tree
pass.** #2053's sweep fixed four sites that
`work/fix/verb-and-dimension-render-through-debug.md` — open on FIX's
slate — had already enumerated, and missed the fifth that item names,
because a tree-grep cannot tell you an instance is already filed
somewhere else. Half-completing another program's item without saying
so is how two programs come to disagree about what is done. The
dispatch did not ask for a tracker pass either, so this is the
orchestrator's rule now, not the lane's mistake.

**The census failure has three distinct members now**, and they are
three different mistakes: a count inherited stale from another program,
a membership test that admitted the wrong things (#2026), and a scan
narrower than the test it claimed to implement (#2046, where the regex
required `[` to follow `=[(,` and so could never see an array literal
introduced by a keyword). A census owes BOTH halves in writing — the
test and the method — and the closed items say so.

Plus the `[ev]` PR carrying the three design forks, and this session's
orchestrator state-sync — which had again gone 32 commits behind main
before it was merged, the same failure #1912 repaired. See the log's
2026-09-06 tail.

**What the wave produced beyond its four diffs: eighteen new items**,
every one of them a file rather than a sentence in a merged PR body.
That is the rule `work/README.md` states and the thing this program had
been failing at; it is now the wave's largest single output.

### Beside the numbered order

- `boundary-rule-has-no-mechanical-check` +
  `loud-skip-marker-says-two-modules-and-there-are-six` — **DONE,
  #1848.** `scripts/gates/viewer-module-kinds.sh` runs on every CI
  pass. It found two sites the ratified rule is false about, filed as
  `pick-and-parts-name-the-session-driver` — a design fork, and one of
  the three going to Ev (below).
- `set-param-prechecks-what-the-door-refuses` — **DONE, #1846.** Its
  sweep's blind spots are `sweep-blind-spots-the-precheck-sweep-
  could-not-see` (two of three still open) and its one other hit is
  `self-boolean-precheck-duplicates-the-doors-duplicate-input`.
- `opoutcome-superseded-has-no-production-reader` — **DONE, #1872.**
  Residue: the two `prune` items now dispatched, plus
  `rank-one-discards-the-frames-other-news` and
  `frame-module-has-eight-concerns-and-no-holds-row`.
- `two-gestures-can-be-in-flight-together` — **DONE, #1873.** Residue:
  `gesture-drags-have-no-cancel-door` and
  `two-hand-written-copies-of-the-g1-gesture-machine`.
- `tracker-has-no-status-for-an-unscheduled-trigger` — **DONE, #1857**,
  Ev's ruling.
- `session-gesture-guard-spelled-thirteen-times` — claimed from CHROME
  and **closed as dissolved**: VIEW-1b answered both questions it said
  a fix had to answer.
- Claimed from CHROME and held:
  `viewer-const-all-tables-have-no-exhaustiveness-guard` (takes with
  `tool-kind-all-and-ordinal-have-no-production-reader`) and
  `no-persistent-setplacement-session-op` (DI5's build, which
  `two-hand-written-copies-of-the-g1-gesture-machine` waits on).
- Open and undispatched:
  `revolve-tool-unreachable-no-axisinplane-form`,
  `stale-file-citations-after-the-split` (general case only),
  `sweep-blind-spots-...`, `gesture-drags-have-no-cancel-door`,
  `two-hand-written-copies-of-the-g1-gesture-machine`.
  **`save-is-not-gesture-guarded` was on this list and is CLOSED** —
  answered rather than fixed, 2026-09-04, with its one residue filed
  as `save-permitted-row-argues-only-half-of-save` and taken by #1932.
  It was still listed here two days later, which is this section's own
  instance of `stale-file-citations-after-the-split`'s expensive half:
  the CLAIM going stale rather than the number. Read `scripts/work.py
  status --program view` before believing any list in this file.

### The three design forks, going to Ev as one PR

`the-news-vocabulary-has-no-expiry`,
`pick-and-parts-name-the-session-driver` and
`four-badges-five-spellings` were decisions, not builds, and they
interlocked: the news vocabulary decided what
`status-line-writers-bypass-the-ranking` swept *to*, the badge family
decided what its standing-fact half swept to, and the boundary rule is
ratified text of Ev's that #1848 proved false of the tree. One `[ev]`
decision document carried all three while the build lanes ran
(Ev, in-chat, 2026-09-04). **All three are answered**, and the sweep
they gated closed at #2026; the boundary rule's mechanical half is
still `boundary-rule-has-no-mechanical-check`.

### The standing hazard this program keeps hitting

Seven prose claims outran this tree in two days, every one caught by a
reader with the tree open rather than by a gate. **The eighth is this
plan's own**: six PRs merged on 2026-09-04 with no log entry, so for
most of a day `work/view/log.md`'s tail described three lanes as still
running that had already landed — the tracker asserting the past
tense's opposite. The countermeasures for the citation half are items
(`boundary-rule-has-no-mechanical-check`, landed; `stale-file-
citations-after-the-split`, open); for the log half there is none, and
the only instrument is a successor reading `git log` before believing
the tail. Dispatches are written accordingly.

## Exit shape

The README states the module map and every item above has landed or
been ruled out; the walk convention applies.
