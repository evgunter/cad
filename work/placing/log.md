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
