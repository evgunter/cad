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
