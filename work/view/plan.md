# VIEW — viewer architecture (plan)

**STATUS: OPEN, RE-SCOPED 2026-09-17, NOT DISPATCHING.** Opened
2026-09-03 from `docs/WORK-TRACKS-2026-09.md` (VIEW section).
Orchestrator handed over twice; the third session took it 2026-09-04
evening. Live state is `work/view/log.md`'s tail and the item files
beside this plan, never this file.

**What the re-scope did, and what it did not.** The six units of the
`Order` below are all done, deferred or handed off — that verification
is in `log.md`'s 2026-09-17 entry, row by row. What remained was
ninety-four live rows of review accretion on one crate, and four
successor programs were opened for them per `work/README.md`'s
re-homing clause (Ev, 2026-09-06): **`vnews`** (the news vocabulary),
**`vgeom`** (geometry, camera and the numeric renders), **`vseam`**
(the seams and the session vocabulary) and **`vdoc`** (prose,
citations and censuses, dispatching last). Fifteen further rows went
to seven live programs. **This program did not close and does not
dispatch**: its exit walk is a separate ratified step and
`docs/DOC-LEDGER.md` records the sweep when it happens.

**The eight rows that stayed, and why.** Six are in `review` with their
lanes in flight (#2622, #2662, #2665, #2666, #2670, #2672) — a rename
mid-review is a merge conflict for nothing, and each row's successor is
named in that program's `plan.md` §Inbound.
`a-dead-seam-worker-reads-as-an-ordinary-idle-state` is parked on a
ruling at PR #2762 and nothing touches it.
`startup-notices-need-holding-to-badge` is the last of the status-line
sweep's residue. Plus
`the-lane-register-has-no-home-after-views-directory-goes`, filed by
the re-scope: **the register below binds four live programs by
reference and dies with this file, so re-homing it is a precondition of
this program's exit walk rather than a follow-up to it.**

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
   `crates/viewer/GUI-DESIGN.md` GQ7. The status vocabulary that could not
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

**The rule register that sat here is deleted (2026-09-21, Ev's
ruling).** Roughly 1,330 lines and 87 rules accreted under this heading
over eighteen days. The test for keeping any of them: does it report an
actual problem, AND would an advance warning have prevented it rather
than merely named it afterwards? Seven were offered. On checking, three
were already written down — `docs/prompts/implementer-discipline.md`'s
verification section (twelve `test (…)` rows, five `k-lint (gate, …)`),
its §5 Sweeps (*a pattern with no hits recorded is a claim; a hit list
is a receipt*), and `memories/agent-lane-operations.md`'s
conflicting-PR bullet. The rest were retrospective categorisation: true
after the fact, useless before it.

**One amendment came out of it** (#3019): that bullet said a
conflicting PR gets NO CI run, and its third face is a run that
COMPLETES with every job dead in 2-3 seconds — a healthy docs tier
except for `gate ok` — diagnosed with `git merge-tree --write-tree`,
never from the logs.

**The register is recoverable at `66d7357417`**, the last commit that
carried it, which is the convention `docs/DOC-LEDGER.md` uses for a
deleted exit walk. An item file that cites a rule of it cites it at
that tree; merge-only means the history is intact.

**The lesson is worth more than the file was.** The failures it
recorded were not caused by missing rules. Three of the last day's were
covered by text that is read at the start of every session and was not
applied. Eighty-seven more would have made that worse, and the four
successor programs inheriting it by reference would each have carried
the cost of reading it before every dispatch.

## What is left of this slate

**Nothing.** The two rows that were left — both residue of Ev's *panic
on crash* ruling of 2026-09-17 —
`the-quiet-seam-half-of-pickcache-indexing-has-no-shipped-producer`
and `the-dying-seam-fakes-mirror-a-machine-they-do-not-share` — are
both **closed** at `view/seam-residue`, so neither had to go to VSEAM.
Two findings went onto VSEAM's slate from that lane; `log.md`'s entry
for the day has them.

The other five were re-homed on 2026-09-21 (Ev, in chat) against each
receiving program's own charter test — one to VNEWS, two to VSEAM, two
to CHROME, each `git mv` with its id, body and history unchanged and
each announced in the receiving program's `log.md`. `log.md`'s entry
for that day has the table and the reasoning.

**This program does not dispatch.** A row that lands on its ground
from here on goes to the successor whose charter covers it, not onto
this slate.

## Exit shape

The README states the module map and every item above has landed or
been ruled out; the walk convention applies.
