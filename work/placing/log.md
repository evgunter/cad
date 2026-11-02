# PLACING log

Newest entries at the bottom; the tail is the program's live status.

- 2026-11-01 — Opened by CHROME's cut (Ev, in chat: three tracks, each
  with a full slate of P0s and P1s). Eight rows moved from
  `work/chrome/` by `git mv`, ids and bodies kept:
  `the-mate-tool-offers-two-of-the-five-mate-kinds` (P0 H),
  `mate-tool-commits-one-mate` (P1 H, design),
  `a-feature-gesture-re-points-every-copy-of-its-target`,
  `deleting-a-placed-feature-unplaces-its-target` and
  `the-transform-gesture-moves-a-placed-body-through-a-transform`
  (P1 M each), and below the line `the-mate-panel-names-no-slide-a-plane-leaves`
  (banded P2 M at the cut), `instance-value-rows-do-not-say-their-axis`
  (P2 E) and the deferred `band-refusal-still-badges-every-row`. P0 and
  P1 load 25. (CHROME orchestrator)
- 2026-11-01 — **Seam (SPACES):** Ev ruled on PR 6209 that no gesture
  writes a world `Place` on the person's behalf, and the picture draws
  bodies (a body is drawn unless an operation reads it as a body). Once
  `spaces/a-new-body-lands-unplaced` lands, this program's re-point,
  delete and transform rows act only on placements a person asked for;
  `deleting-a-placed-feature-unplaces-its-target` largely dissolves for
  unplaced bodies, and the duplicate stand-in leans on
  `a-feature-gesture-re-points-every-copy-of-its-target`. A live lane in
  either program says so on the other's log. (SPACES orchestrator)
- 2026-11-01 — (CHROME orchestrator) From the review of CHROME's PR 6203, unverified: `MateFreedom::offered_beside(kind)` is called without the freedom in `pane/create.rs`, around the mate form's freedom rows, so every freedom may be offered or none. Read it before the mate-tool rework builds on that panel.
- 2026-11-01 — (CHROME orchestrator) Correction to the note above: CHROME's mate-banner lane read `MateFreedom::offered_beside(kind)`, and it is by design, per its doc. Each freedom is offered beside a plane or an axis mate; which slides apply is the copy's geometry's to say, and the evaluation refuses the rest. Nothing to file.
- 2026-11-01 — Track claimed. Read against `origin/main` at `36c9eca7f`.
  Decisions, taken unilaterally as sequencing:
  - **The P0 is cut along the line the design question cannot reach.**
    Unit `placing/mate-kinds` widens the existing one-mate draft to every
    kind whose pose one pick per side reads: `Plane` and `Axis` off faces
    and now edges (a straight edge's line, a circular edge's axis),
    `Point` off a vertex or a carrier's centre, `Direction`. The
    `Frame`–`Frame` kind (corners) is NOT in it: a frame side is an edge
    and a vertex, two picks per side, which is the shape of the draft
    `mate-tool-commits-one-mate` asks about, so it goes to the designers
    with that row. Tier: single FULL review — reading poses off edges and
    vertices and the freedoms each new kind leaves carry real chances of
    a wrong answer. The P2 `instance-value-rows-do-not-say-their-axis`
    rides along if it fits (same placement value rows).
  - **`mate-tool-commits-one-mate` goes to the designers** (design fork,
    protocol `d05675f9a`, draw recorded on
    `analysis/design-fork/mate-tool-bundle`; two designers).
  - **The three gesture P1s are parked, not dispatched.** Each rewrites
    code another program has a live lane on, with semantics that move
    under it: `a-feature-gesture-re-points-every-copy-of-its-target` on
    DOORS' `a-mate-on-a-taken-over-copys-face-goes-stale` (its
    `refactor::take_over` door re-points "the copy" from `feature_over`,
    `add_transform` and siblings: exactly this row's question);
    `deleting-a-placed-feature-unplaces-its-target` and
    `the-transform-gesture-moves-a-placed-body-through-a-transform` on
    SPACES' `a-new-body-lands-unplaced`, after which a gesture acts only
    on placements a person asked for (SPACES' seam note above). The
    alternative, dispatching now, builds on gestures two lanes are
    rewriting. The transform row is also stale in its premise:
    `add_transform` now inserts a `Place` over the input and re-points,
    not a `Node::Transform`; re-read it when it unparks.
  (PLACING orchestrator)
- 2026-11-01 — **`mate-tool-commits-one-mate` decided** (fork row 232, no `[ev]` PR). Two designers reconciled over two rounds to one recommendation: a placement editor whose draft is the whole bundle, with derived value rows, seats routed by body, and a live verdict. The decision is written into the row. Why no `[ev]` PR: no ratified text changes, the answer elaborates Ev's direction on the row, and the designers rejected the only alternative they found. The row is parked on the P0 `the-mate-tool-offers-two-of-the-five-mate-kinds`: it rewrites that unit's state, so it is sequential, not coupled. Filed: `the-viewers-mate-remove-takes-the-structural-door` (P1 M, unverified, rides with the editor), and in DOCTAIL `removing-a-value-constraint-is-a-second-spelling-of-setting-it` (P3 E). `the-mate-panel-names-no-slide-a-plane-leaves` now rides with the editor. Premise correction for Ev: the multi-mate draft by itself does not make two plane mates compose. That is MSOLVE's `a-consistent-second-mate-after-a-rest-is-refused`, the row that decides whether Ev's stacked-corner case works by two planes; `Frame`–`Frame` answers it in one mate. (PLACING orchestrator)
- 2026-11-01 — SPACES' `a-new-body-lands-unplaced` closed (PR 6269) sooner than expected, so the two rows parked on it are dispatched together on `placing/gesture-placements`: `deleting-a-placed-feature-unplaces-its-target` and `the-transform-gesture-moves-a-placed-body-through-a-transform`. The lane re-reads each against `Takeover` first; a row the landing made moot closes as such. Tier: single FULL review, because a gesture's effect on the world and its undo identity are easy to get subtly wrong. (PLACING orchestrator)
- 2026-11-01 — `placing/gesture-placements` reported on PR 6280, green at `78bd26d2`. **Delete row: still true on main, and fixed.** The delete hands each re-pointed reader back up the chain in the same action. Review tier: single FULL, dispatched. The lane filed `a-combines-delete-hands-back-to-its-first-operand`. **Transform row: still true** (a transform of a copy commits a copy of the copy), and it is a fork: compose the move into the copy's bundle, overwrite the bundle, or refuse on a copy. The README sentence "on a copy it edits the copy's placement" is agent-written (38d2ab30ba, with `[ev]` 6209, whose answers did not cover Transform). The row goes back to `open` with `design: true`. Three designers are dispatched (draw on `analysis/design-fork/transform-a-placed-copy`, bytes 58/190). (PLACING orchestrator)
- 2026-11-02 — **Transform fork reconciled** (fork row 233; three designers, two rounds). All three recommend the same final state: Transform retires into the placement editor (Edit door on a copy's bundle, Create door with no mates on an unplaced body; the axis-and-angle input goes). One split remains, on clarifying `docs/DESIGN.md` `[placement.placement]`, and it goes to Ev. Round 1 found the sentence's general reading came from an agent-written edit, `e48cb88cad`, which dropped "(today's `Transform`)". B and C recommend the clarification; A offers it as optional. The `[ev]` PR states it by editing the sentence. **Interim: nothing is built before the editor.** The alternative, refusing Transform on a copy now, stops the stacking on main but is a stopgap thrown away one unit later, and nothing is released. (PLACING orchestrator)
- 2026-11-02 — Ev answered `[ev]` 6281. The Transform decision stands and DESIGN.md does not change ("the current state seems pretty clearly in violation of what's already written there"). Ev asked "can you see if anything similar also got missed?" by the INTENT refactor. A read-only sweep lane is dispatched for consumers (gestures, API doors, Python, demos, tests) where something INTENT retired or ruled survives under another name; its findings are filed as rows. (PLACING orchestrator)
- 2026-11-02 — PR 6280's review (single FULL) found it not mergeable. Its probe rows are on `placing/gesture-placements-review-6280`. **Class finding:** the delete's hand-back infers "what took a body over" from node shape (`taken_body`, a second census of the takeover gestures), so it splices readers no takeover touched (M1: a feature on a transform of a copy deletes the transform; m1: deleting a duplicate re-points a hand placement). **Coupled:** DOORS' `take_over` door writes rebinds, which an inverse must undo too (m3). Decision, sequencing: `deleting-a-placed-feature-unplaces-its-target` is parked on DOORS' `a-mate-on-a-taken-over-copys-face-goes-stale` with `design: true`, and PR 6280 goes back to draft. The question (where a takeover's inverse lives and what it reads) goes to designers once that door's shape is fixed. The alternative, a fix pass now that narrows the shape census, would leave a census in place and still owe the rebind inverse. **Friction:** the session's disk allowance ran out with three lanes' targets (13 + 7 + 5 GB) live; the two finished lanes' targets were reclaimed. (PLACING orchestrator)
- 2026-11-02 — **The INTENT sweep Ev asked for on 6281 is in.** The report is kept at `~/.local/share/cad-work/placing-intent-sweep-report.md` on this box; what matters is filed.
  - **New rows:**
    - `a-duplicate-freezes-a-mated-copy-into-literal-values` (P1 M, design): Duplicate snapshots a copy's solved motion as literals and drops its mates.
    - `the-mate-snap-overwrites-held-values-with-measured-literals` (P1 M, rides with the editor): committing a mate unlinks values that read variables.
    - `a-linear-patterns-direction-components-land-in-its-slides` (P1 M): direction components stored in slides, unnormalized.
    - CHROME's `the-at-rest-badge-shows-only-for-documents-that-instantiate-a-part` (P1 M).
    - `work/issues/intent-retired-machinery-is-still-described-as-live` (P4 C, with a P3 Python `Frame` half for LIB).
  - **Evidence added** to HELD's `the-viewers-numeric-gestures-read-a-frame-back-off-geometry` (a third caller, plus a Python twin), to SPACES' `the-projection-tool-writes-a-member-read-a-pick-already-seats` (`Node::Part` rebuilt as an identity `Place`; P1 suggested) and to the Transform row (`in_one_space`'s `self_placed` branch).
  - **Class:** INTENT landed its retirements in the kernel's node vocabulary, and the consumers (gestures, the Python helpers, prose) were never re-read against the principle. Every finding is a gesture or helper that reads geometry or an evaluation back as literals, or rebuilds a retired node's job.
  - **Blind spots the sweep names**, with a second pass owed:
    - `crates/viewer/src/pane/` and `app/`: forms prefilling from an evaluation.
    - editor-core `refactor.rs` extract: lands in `VarId::WORLD` from a non-world space?
    - stacked moves in editor-core and pncad tests: needs the doc API, not grep.
    - pncad-py `src/py/*.rs` evaluate-then-edit paths.
    - `snap.rs`'s least turn against `editor_core::mate::value::Chart`: a semantic duplicate.
    - the unbuilt `Node::Sweep`'s two profiles' planes.

    The second pass is a later dispatch; it is logged here so it is not lost.

  (PLACING orchestrator)
- 2026-11-02 — **The second INTENT sweep** covered the first sweep's six blind spots.
  - **Clean:** editor-core `refactor.rs` (a cut from a non-world space refuses, and no refactor writes a number its caller did not write) and the pncad-py door sources.
  - **Filed:**
    - `the-snap-respells-the-charts-value-read-back` (P3, rides with the editor);
    - DOCTAIL's `a-sweep-states-no-relation-between-its-profiles-planes` (P3, design);
    - the split seed as a rider on HELD's numeric-gestures row;
    - two test-prose shapes added to `work/issues/intent-retired-machinery-is-still-described-as-live`.
  - **Remaining blind spot:** stacked moves were checked by a static scan only. The doc-API pass needs a build, which waits until the disk allows it.

  Taken together, the two sweeps found the Transform shape again only in Duplicate, the snap and the pattern direction, all filed. Nothing else in the kernel or Python doors. (PLACING orchestrator)
- 2026-11-02 — The P0's lane reported: PR 6284, green at `eda000ca0e`. The mate tool now offers plane, axis, point and direction, read off faces, edges and new vertex picks. Which entity a pose reads comes from the kernel (`SlotKind::selection_kind`), and what a kind leaves comes from `SubgroupFamily::leaves`. Frame–Frame is out, with its reason in the `partial_mirror!` list. The P2 ride-along was not done; the axis lives in the bundle, not in the slot id. Disclosed: a point mate has no snap arm, proven only on untranslated copies. Filed: `a-direction-mate-on-a-curved-face-reads-its-plane` (P3), CHROME's `a-checks-cell-for-a-vertex-selects-its-node` (P3), and a note on VSEAM's bare-name row. Coupled with DOORS' `take-over-rebinds` through `HeldPick::following`: whichever lands second makes it generic. Review is dispatched (single FULL, the tier set at dispatch); the lane's 13 GB target was reclaimed. (PLACING orchestrator)
- 2026-11-02 — PR 6284's review (single FULL): mergeable, no MAJOR. Nine probe rows passed and none was pushed. They cover: the point and direction freedoms against the solve, a slid and turned copy (so the point-snap disclosure is narrower than feared), vertex occlusion, and a held vertex across a fillet degrading typed. A fix pass has gone to the same lane:
  - MINOR-1, the panel's plane-rest flip default turning edge mates over;
  - S1, `pick_kinds` restating the kernel's reads (a class: a second table of one rule);
  - S3, the `member_of`/`member_of_ref` duplicate;
  - the stale names and prose.

  S2, a third bare-name selection struct, goes as evidence on VSEAM's row. MINOR-2 is noted on the editor row. (PLACING orchestrator)
