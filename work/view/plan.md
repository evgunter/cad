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
   ranking` (19 sites) and `four-badges-five-spellings` are its
   residue, both open.

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
| `view/scene-gathers` (the double gather) | #1908 | style — **fix pass in flight** |

Plus #1912, this session's orchestrator state-sync, merged separately
because it is a session's worth of adjudication across five units and
should be visible on its own.

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
  `save-is-not-gesture-guarded`, `stale-file-citations-after-the-split`
  (general case only), `sweep-blind-spots-...`,
  `gesture-drags-have-no-cancel-door`,
  `two-hand-written-copies-of-the-g1-gesture-machine`.

### The three design forks, going to Ev as one PR

`the-news-vocabulary-has-no-expiry`,
`pick-and-parts-name-the-session-driver` and
`four-badges-five-spellings` are decisions, not builds, and they
interlock: the news vocabulary decides what
`status-line-writers-bypass-the-ranking` sweeps *to*, the badge family
decides what its standing-fact half sweeps to, and the boundary rule is
ratified text of Ev's that #1848 proved false of the tree. One `[ev]`
decision document carries all three while the build lanes run
(Ev, in-chat, 2026-09-04).

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
