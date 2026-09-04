# VIEW — viewer architecture (plan)

**STATUS: OPEN, and dispatching (2026-09-04).** Opened 2026-09-03 from
`docs/WORK-TRACKS-2026-09.md` (VIEW section). Orchestrator handed over
2026-09-04 after the first orchestrator exited with unit 1 closed and
nothing dispatched. Live state is `work/view/log.md`'s tail and the
item files beside this plan, never this file.

Branch prefix (the #396 convention): **`view/`** — unit branches
`view/<unit>-<slug>`, orchestrator branch `view/orchestrator`. The
2026-09-04 handover session drives the orchestrator half from
`claude/view-orchestrator-exit-9sjdmh` instead, because its harness
pins that branch; unit branches are unaffected and keep the `view/`
prefix. Away-channel tag `(VIEW orchestrator)`.

**Review posture (Ev, in-chat, 2026-09-04).** This program runs **no
A/B duals and writes no row in `docs/MODEL-AB-LOG.md`**, whatever
review a unit gets. The A/B band **1900–1999** stays claimed and empty
and the band table says so. The default is a **style review** against
`docs/prompts/reviewer-style-lane.md`; a second correctness reviewer is
added where a unit's failure mode is a **confident wrong answer rather
than a refusal**, and the dispatch says which it chose and why. Under
this posture the dispatcher's own exposure is the live risk rather than
a formality: unit 1's chain produced **seven** dispatcher corrections,
two against decisions rather than details, so every brief this program
issues states its claims AS claims and says so in as many words
(`docs/REVIEW-STYLE-DISPATCH.md` §3).

## Charter

Decide the viewer's shape before more units accrete into a
3,224-line `session.rs` and a 5,696-line `app.rs`. One conversation
gates the rest; the builds after it are mostly E with one hard
concurrency unit.

## Order

Unit 1 is closed. What follows is the going-forward order set at the
2026-09-04 handover; items 3–6 keep the numbers the opening plan gave
them so the log's references still resolve.

1. `viewer-session-god-module-split` — **DONE, 2026-09-04.** Four PRs:
   #1801 ratified the boundary rule, #1816 made gesture safety data
   (`SessionOp::permitted_during_value_gesture`, one exhaustive match
   in `perform`, the 23 call-site guards deleted, no operation's
   answer changed), #1830 split both files, #1832 made the
   one-of-seven tool invariant unrepresentable. `session.rs`
   3,260 → 1,500 and `app.rs` 5,696 → 1,752, thirteen new modules,
   **no test file touched and no assertion changed** across the chain.
   Representation changes were kept separate from the move on purpose:
   the move's whole safety property is that the compiler checks it.
   Residue: `session-shims-and-test-imports` (the `pub use` shims, now
   un-parked — see below) and
   `tool-kind-all-and-ordinal-have-no-production-reader`.

2. `pick-priority-filter-vocabulary` — **not dispatchable, and `open`
   only for want of a truer status.** A per-kind admission set replaces
   the three-variant `PickKinds` when a third asymmetric tool (vertex
   pick) exists; none does or is scheduled, and `crates/viewer/README.md`
   GQ7 ratifies the deferral. The blocker the opening plan gave it was
   **false** — `ToolKind::pick_kinds` was already an exhaustive match,
   so unit 1d had nothing to collapse — and the correction is in the
   log. The status question is now
   `tracker-has-no-status-for-an-unscheduled-trigger`, an `[ev]`
   ruling.

3. `camera-fold-clears-status-line` — **DONE, 2026-09-04** (#1849).
   The status line carries per-frame
   NEWS and `frame::frame_status` owns its ranking; a fact that stays
   true after the frame ends is not news. So `land` stops clearing,
   its refusal reaches the line through the existing ranking, and the
   product fault gets a home with a standing lifetime. The rules land
   in `frame` as values with rows. The ~15 further direct writers of
   the line are censused and filed, **not** refactored — those files
   are shared with CHROME.

4. `focus-marking-is-per-node-not-per-segment` — **blocked, and the
   blocker is not this program's to clear.** The authored-step to
   canonical-segment map door straddles two globs: the authored `step`
   is `ProfileProgram::step_args` (DOCM) and the canonical `segment` is
   `crates/profile`'s canonicalization (S-BOOL). Where the map lives is
   a question neither this program nor either owner can answer alone.
   Two announces are owed before this can start.

5. `layer3-recipenodeid-aliases-across-rewinds` — DI1's build, ruled:
   a hold carries its id plus its minting entry, descent is checked
   before liveness, tools clear on history replacement. **Parked** on
   `next-id-has-no-layer3-door` — DI1's walk needs to ask *could this
   document have minted this id*, `Doc::next_id` is `pub(crate)`, and
   the door is DOCM's to shape. The free-move half was never this
   program's: DI5 hands it to CHROME
   (`no-persistent-setplacement-session-op`).

6. `pick-index-built-on-ui-thread` — **6a RULED (Ev, #1843,
   2026-09-04); 6b is the build and 6c is gone.**

   The ruling: the inventory gains **no third shape** and GQ6 gets no
   staleness sentence, because an index never READ while stale is
   *current or absent* rather than a shadow — and "absent, showing an
   older picture" is a state GUI-3 already ratified and built chrome
   for. Picks **refuse with a visible loading state, not silence**:
   every pick path guards on `Option<&PickIndex>` and is silently inert
   today, and a spinner over inert picks is fail-quiet. A δ change
   **restarts without cancel**, the in-flight build running to
   completion and its result discarded, with the doubled-window cost
   (~26.8 s on the fine-δ row) stated rather than left to a reader.

   **6c collapses into 6b**: with no stale read there is no staleness
   rule to express as frame data, so what is left is the indicator's
   own state, which 6b carries.

   **6b, the whole remaining build**: tessellation and
   `PickIndex::build` onto the `EvalService` worker keyed by
   `Generation`; `PickCache`'s retry policy travelling with it;
   restart-without-cancel on δ change; the indexing state added to the
   existing busy chrome (`app.rs:1095-1112`, three states already); the
   pick path made to say *not indexed yet*; and the GQ6 sentence —
   which lands HERE and not before, because the README states the
   present and there is no off-thread index until this unit exists.
   `mesh` and `bvh` grow no cancel points, so no other program gates
   it. **Waits only on `view/superseded-reaches-the-user`**, which is
   live in `app.rs`.

### Beside the numbered order

Findings that accreted during unit 1 and are not part of its chain.
They are dispatchable independently, and two are running:

- `set-param-prechecks-what-the-door-refuses` — **DONE, 2026-09-04**
  (#1846). The pre-check is gone and the door's typed refusal
  surfaces; the sweep ran over all 41 `OpOutcome::refused` sites with
  a per-site disposition. It also corrected a ratified README clause
  that named a fact the door refuses, and added the sentence that
  qualifies an entry in that list. Residue:
  `self-boolean-precheck-duplicates-the-doors-duplicate-input`,
  `refusal-edit-arm-doubles-a-prefix-and-splits-one-mistake`,
  `sweep-blind-spots-the-precheck-sweep-could-not-see`,
  `session-clearing-walk-is-hand-maintained-three-times`.
- `boundary-rule-has-no-mechanical-check` +
  `loud-skip-marker-says-two-modules-and-there-are-six` — **DONE,
  2026-09-04** (#1848). `scripts/gates/viewer-module-kinds.sh` reads
  every module's `use` block against a kind it declares in its own
  header; the forbidden-crate set derives from `Cargo.toml`'s `app`
  feature and the driver roster from the README's own table, so each
  list is an input to a comparison rather than a second copy. Sited in
  `mirror`, not `discipline`, so it fires on a README-only change
  (`ci.yml:804-812`). **The rule it enforces was already false at five
  sites** — `pick-and-parts-name-the-session-driver`, with a
  site-granular exception that reds on a sixth.
- `stale-file-citations-after-the-split` — VIEW's own five files are
  paid (2026-09-04); what stays open is the general case, and the
  finding that a machine resolving line NUMBERS would have passed the
  one file whose CLAIM had gone stale.
- `two-gestures-can-be-in-flight-together`,
  `opoutcome-superseded-has-no-production-reader`,
  `tool-kind-all-and-ordinal-have-no-production-reader`,
  `revolve-tool-unreachable-no-axisinplane-form`,
  `save-is-not-gesture-guarded`, `session-shims-and-test-imports` —
  open, undispatched. `opoutcome-superseded` waits on item 3's
  vocabulary by preference, not by rule; the `tool-kind` and
  `session-shims` rows each have a half in `crates/viewer/tests/`,
  which is CHROME's glob.
- `tracker-has-no-status-for-an-unscheduled-trigger` — an `[ev]`
  ruling, not work.

### The standing hazard this program keeps hitting

Five prose claims outran the tree in one day during unit 1, four of
them the orchestrator's, and every one was caught by a reader with the
tree open rather than by a gate. Two more were found at the handover:
a row parked behind an item that had closed, and a file whose
citations were corrected while its mechanism sentence stayed false.
The countermeasures are items — `boundary-rule-has-no-mechanical-check`
and `stale-file-citations-after-the-split` — and until one lands, the
only instrument is a reader. Dispatches are written accordingly.

## Exit shape

The README states the module map and every item above has landed or
been ruled out; the walk convention applies.
