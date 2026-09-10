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
| `view/censuses` (seven censuses in four hats, and an eighth) | #2103 | style + fix pass |
| `view/all-gate` (the `const ALL` gate, filed by #2046) | #2106 | **correctness** + fix pass |
| `view/summarised` (a summarised field renders as a summary) | #2148 | style + fix pass — **merged** |
| `view/labelled` (two of the four bare vocabularies, and the corrected rule) | #2143 | style + fix pass — **merged** |
| `view/gate-bullets` (the vocab gate's kind scan, anchored) | #2172 | style + fix pass — **merged** |
| `view/wasm-dead-items` (CIW's §6: two items dead at wasm32) | #2272 | style + fix pass — **merged** |

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

**Green CI on an old head is not a merge criterion.** #2106 had
`gate ok` green at `77505a340` and the merge was still refused: `main`
had moved and #2103 had rewritten a neighbouring section of
`crates/viewer/README.md`, the file that gate parses its allowlist
from. Resolve by merging the base IN — never rebase, never
force-push — re-run the receipts on the MERGED tree, and keep BOTH
halves of an append-only file in merge order. `work/view/log.md`
conflicted on both of the day's merges and will conflict on every day
two units land.

**Escalate to a correctness lane when the failure mode is a confident
wrong answer, not when the diff is large.** #2106 was the wave's one
non-style review, and the trigger was that a gate which silently never
fires is indistinguishable from a gate that passes. It found the gate
printing `OK` and exiting 0 over two planted breaches whenever its
roster table was empty — a docs-tier edit away, and the #1953 class
again. A style lane reads what the code says; only an adversarial one
asks what it does when a reader dies.

**Namespace what you write in the shared scratchpad.** #2143's reviewer
generated `base.rs`, `head.rs`, `base.txt` and `head.txt` for its own
word proof and overwrote four files of those exact names that another
lane had left there. Nothing was lost that mattered, and the collision
is silent by construction — a lane cannot tell whether a scratchpad
file is its own. Dispatches carry a per-lane prefix now.

**Ask what would have shown the OTHER answer, and the answer that
matters is often one control over.** #2272 had to choose between
`#[cfg]`-ing two dead items and hunting a lost caller. The brief asked
for the choice AND for what had been checked that would have shown the
other shape — and that second half is what found the live defect: not
in either item, but in the preferences store beside them, where a
`usable()` guard makes the palette picker apply and silently not
persist against prose claiming it is disabled with a reason. A
justification written to be falsifiable searches; one written to be
sufficient stops at the first sound argument.

**A sweep shaped like the symptom finds the symptom.** That same unit's
first framing came from a `cfg(target_family = "wasm")` sweep, so the
defect it found was filed as a browser defect. It is not: `usable()` is
`path.is_some()`, and the native build takes the identical silent
return when neither `XDG_CONFIG_HOME` nor `HOME` is set. A
`target_family` fix would have repaired the browser and shipped the
other half. **Re-derive the population from the PROPERTY that fails,
not from the pattern that surfaced it.** The same unit shows the other
direction too: a dead symbol in a doc comment in `src/` was the cause
of a wrong citation in `work/`, and the tracker-shaped sweep found only
the symptom.

**The orchestrator's own heartbeat is a single point of failure, and a
one-shot timer is the wrong shape for it.** The check-in was a
`run_once_at` trigger re-armed by hand at the end of every turn. On
2026-09-08 it fired, the turn ended without re-arming it, and VIEW sat
idle for a day — no lane, no PR, 796 commits of drift, one row filed
onto its slate by another program and unread. None of the rules above
covers it, because they are all about lanes and evidence and this is
about the orchestrator's own liveness. It is now a recurring cron
(`57 */6 * * *`) whose prompt says not to make it a one-shot again, and
deliberately rare rather than a tight poll: lanes notify this session
directly when they finish, so the timer is only the backstop for a
failure to dispatch after a report, and a sub-cache-TTL cadence would
cost a cold context every firing to buy nothing (Ev, 2026-09-09).

**A measurement can be an artifact of the instrument, and the first
number out is the one to distrust.** Sizing the tracker's `file:line`
citations, the first pass reported 49% of them pointing at a file that
does not exist. It was counting the repo's own `session.rs:1517`
shorthand as a missing path. Resolved against `git ls-files`, the
figure is 2.1%. The corrected pass is in
`work/issues/tracker-file-line-citations-measured`, along with the
method, so the next reader can see why the numbers differ rather than
picking whichever they meet first.

**A hold is not a hold until the diagnosis's own repair is tried.**
#2172 argued that putting the count word inside `KIND_ANCHOR` bought a
hold the gate lacked: the section could previously say "Four kinds…"
over three bullets unread. Its reviewer took the resulting red's OWN
second suggested repair — change `KIND_ANCHOR` to match — and got
**green, exit 0**, with the `OK` line printing `3 kinds read from
"Four kinds of list stay hand-written"`. The hold cost one extra edit
and the error message named that edit. **A guard whose diagnosis offers
a way around it is a speed bump**, and the only way to find that out is
to follow the repair the tool prints, not to read the code that prints
it.

**A `^`-anchored pattern over markdown is a claim about column zero
that markdown does not make.** The same PR's new reader escaped its
paragraph state on `/^- /` and swallowed anything indented as a
continuation. CommonMark lets a list marker sit at one to three spaces
and interrupt a paragraph, so `  - **A fourth kind**` renders to every
human as a ratified kind and the gate read three and printed OK — the
gate's own thesis inverted, in the gate written to prevent it. Four
spaces is a code block, so the boundary is exact and worth encoding
rather than approximating; and the reviewer settled the rendering with
a CommonMark parser rather than by reasoning about it.

**A §6 report filed against a tree a lane is still changing owes a
re-derivation after that lane lands.** `gate-selftest-cannot-observe-
the-identity-a-gate-names` was filed from a review report while #2106's
fix pass was in flight; that fix pass then took the very finding as its
MINOR-8 and added the name-bearing cases, so the report's worked
example was false before it reached `main` — twelve rows where there
were twenty, a line range pointing at a different function, and a
universal about name-independence that four cases already contradicted.
The mechanism claim survived; the example did not. **A report is a
claim about a tree, and naming the tree it is true of is part of making
it.**

**File to `main` BEFORE dispatching a lane against the file.** The
`view/gate-bullets` lane could not see its own item: it existed only on
the orchestrator branch, so the lane had to merge that branch into its
own to read the row it was closing. This is the second instance —
#2106 cited two `work/issues/` paths that were likewise orchestrator-
only, and needed #2107 landed ahead of it to resolve. **The orchestrator
files and then sits on the file**, which is the same *disclosed but not
scheduled* shape §Q6 forbids a lane, one level up. State-syncs land
before the work that depends on them.

**Asserted-somewhere is not asserted-here, and only a MUTATION tells
them apart.** #2148 shipped a test file whose stated job was to hold
seven renderings to their spellings. Its reviewer did not read the file
and agree — it changed one rendering and ran the suite, which stayed
green. Six were held; the seventh was asserted nowhere, because the row
covering it pinned the field's absent arm. The counterpart to #2103's
*rendered-and-unasserted is not unrendered*: there the compiler
answered, here only a perturbation could, and in both cases reading the
code would have confirmed the wrong thing.

**An item's own menu of options is a claim like any other.** Both of
2026-09-08's forks went to Ev with the tree re-read rather than the
item summarised, and in both the re-derivation moved the question
before he ruled: `finish-marker` offered three candidates and the tree
held a cheaper fourth already in use one impl away; `bare-vocabularies`
framed all-four-or-none and the readers gave two. An item is written at
a moment and reasons from the tree of that moment; costing its options
against the tree of today is what a fork costs, and skipping it is how
a ruling gets made on a question nobody still has.

**A count fixed in ONE place contradicts itself, which is worse than
one uniformly stale.** #2148 re-derived `1,779` to `1,780` at
`four-debug-walks-are-spelled-and-placed-two-ways.md:42`, wrote the
rule that produces it beside it, and left the file's own `title:` and
line 68 at 1,779 — so the file is now wrong by a rule it states. That
is #2103's retitle defect re-minted five days later by a different
lane. A citation fix is class-wide over the file or it makes the file
worse: a uniformly stale number is at least consistent, and a reader
who spots one instance distrusts all of them; a half-fixed one invites
belief in whichever copy they read first.

**Re-derive means find the SUBJECT, and a delta is not a subject.**
The same PR re-pointed `crates/viewer/README.md:787-797` to `804-814`
— `787+17` — where the true shift was +22 and the subject sits at
`809-819`; the words the citing row quotes are at `816`, outside the
range it now names. `citation-repoint-shifted-a-number-the-lane-knew-
was-wrong` closed that exact defect at #2083, two days earlier. Two
re-mints of two-day-old closures in one PR is the measure of how weakly
a closed row holds: **a closed item is a record, not a guard**, and the
only instrument that has caught either class is a reviewer re-deriving
the citation by hand.

**The rule a unit states is the first rule to check the unit against.**
#2143 wrote *a universal in prose owes the sweep rule that produces its
population* into `crates/viewer/README.md` and then stated a population
its own rule does not produce: the sweep says every loop over a
vocabulary's `ALL` under `src/` read for what it asks each entry for,
and says **two** ask for the word — there are **seven**, and all seven
do. "Two" was the count of newly converted vocabularies, a restriction
no longer expressible once `PathVerb` and `ArcMode` joined the labelled
five. It was also a regression: the rewrite deleted the one paragraph
that had accounted for the other five. A reviewer who runs the stated
rule catches this in one command; a reviewer who reads the sentence
does not.

**A base worktree needs its OWN target dir, not the lane's.** The rule
above says export `CARGO_TARGET_DIR` everywhere; #2148's lane did, and
exported the SAME one in the throwaway base worktree it made to measure
against — so the base build clobbered the lane's own test binaries. It
caught this from a backtrace naming the base path, cleaned and
re-measured, but the near-miss is the point: two trees sharing a target
dir is the failure the private target dir exists to prevent, and a base
worktree is a second tree. Dispatches say *its own target dir* now.

**Operational, the box's `awk` is now gawk.** #2106's fix pass
`apt-get install`ed gawk to test a regex on two implementations, and
Debian alternatives moved `/usr/bin/awk` from mawk to gawk. **Left in
place deliberately**: `scripts/gates/loop-boundary-discards.sh:222-234`
records that a backslashed metacharacter made *"the gate's own clean
fixture fail there and pass here"* — mawk shrugged where the hosted
runner's awk warned and died — so a lane testing a gate under mawk is
testing an awk CI does not run. A shared-box change other lanes did not
ask for, hence this line. The repo convention stands whichever awk is
installed: a metacharacter is a one-member bracket expression (`[(]`,
`[|]`), never a backslash, because a backslash must survive both bash's
escape pass and awk's `-v` processing and does not.

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

**A diff that shifts a file owns the citations that shift broke.** #2278
deleted 61 lines of `app.rs` and grew two `frame.rs` doc comments by one
line each, re-derived every citation in the five rows it edited, and
left every citation in every row it did not — five confirmed wrong by its
style review, one of them (`cursor-projection-is-f32-…:46`) landing 61
lines off onto a test body, on this program's own slate. The
re-derivation obligation as it was written scoped itself to rows the
branch touches, which is the wrong population: the population is every
open row citing into the bands the diff moved. So a unit that shifts
line numbers owes the **census** of those bands, not a sample, and owes
it in its own PR. The alternative is what happened for a week — a
general item absorbing damage particular branches did, which is how
`stale-file-citations-after-the-split` reached four classes without
fixing one.

**An unsubstituted matrix placeholder in a job name means the matrix
never expanded, so that row is not the row it appears to be.** #2282's
lane read a run as green on sixteen successes including the three
wide-tier rows, then caught that its own push had cancelled seven jobs
before expansion: the names still read `test (eps = ${{ matrix.eps }},
${{ matrix.shard }}/2)` and two such rows stood where twelve belong. A
skipped matrix job legitimately shows the placeholder too, so the
placeholder alone is not the tell — **the count is**. Read the twelve
`test (…)` and five `k-lint (gate, …)` rows explicitly, and treat
`gate ok` as the verdict rather than a tally: it is the summarising job
that asserts every other job reached a terminal state, so it reds or
stays pending precisely when a narrowed matrix would otherwise read as a
pass. `neutral` is a passing conclusion, not a failure —
`render drift (…)` is a check run posted by the rebaseline action and
designed to be neutral (`ci.yml:4964-4976`).

**A receipt is a citation and gets no exemption.** #2278's log entry
offered two perturbations as evidence and named the wrong line for one
of them: a multi-line `assert_eq!` panics where the invocation STARTS,
not at its closing `);`, so the receipt said `frame.rs:2170` where the
macro opens at `:2167`. The perturbation was real and the proof stands;
what was wrong was the evidence's own address. A receipt wrong about its
own file is worse than no receipt, because it is offered as the thing a
successor would re-run.

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

### The two 09-07 forks, and what re-deriving them changed

Both went to Ev in chat with the tree re-read rather than the item
summarised, and in both cases **the re-derivation moved the question**
before he ruled. That is the argument for costing a fork against the
tree instead of against its own file.

**`finish-marker-cannot-say-summarised`** listed three candidate
spellings and called none obviously right. The tree held a fourth,
cheaper than all three: `LandedRun` already summarises `checks` through
`&format_args!(…)`, which reads as a summary, while every other
summarised field uses `is_some()`, which renders `false` and reads as a
`bool` field. So the fix is to follow the precedent already in the file
and leave `std`'s markers alone — the marker answers *are all fields
shown*, and the question a reader needs is *is this value the whole
field*, which belongs at the FIELD. **Ev: "sounds good"**, 2026-09-07.

**`bare-vocabularies-declare-their-words-a-second-time`** framed a
dichotomy: either the labelled arm absorbs all four bare vocabularies
and the README's two-shape rule is DELETED, or it does not. Tracing
every reader gives neither. `PathVerb` (`pane/create.rs:727`) and
`ArcMode` (`widgets.rs:300`) have a PRODUCTION loop that iterates `ALL`
and asks each option for its word; `ToolKind` and `Seat` have no
word-reading iteration anywhere — their `ALL` is read only by
`crates/viewer/tests/combine_ops.rs`, which maps kinds to bools and
never asks for a word. **So it is two of four, and the rule is
corrected rather than deleted**: the current test asks *is there a
single-value reader?*, which sends `PathVerb`/`ArcMode` to the bare arm
despite a loop wanting their words. The test that sorts this tree is
*does anything iterate the table FOR ITS WORDS?* — a rule about whether
the words are table data rather than about how many readers exist, and
falsifiable by grep where the old one was not. **Ev: "sure"**,
2026-09-07.

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
