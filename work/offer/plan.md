# OFFER — the plan

what the viewer offers a person to do about what it shows, and what it
asks of the platform beneath it

Opened 2026-09-22 by CHROME's priority-seam cut (`work/README.md`,
Track size). Re-cut 2026-11-22 at its first sitting (`log.md`).

## The slate

**30 budget points** of dispatchable P0/P1 work against a ceiling of 30.

| pri | item | cost | design | title |
|---|---|---|---|---|
| P1 | `a-refusal-offers-no-action-in-the-viewer` | H | yes | a kernel refusal is prose with no action the viewer can take on it (Ev's report) |
| P1 | `a-check-offers-no-way-to-add-the-assertion-it-asks-for` | M | yes | a check asks for an assertion and no door adds one (Ev's report) |
| P1 | `a-node-name-the-viewer-shows-is-not-a-link-to-it` | M | yes | a node's name on screen is not a link to it (Ev's report) |
| P1 | `a-placement-and-its-part-do-not-link-to-each-other` | M | | a placement and its part do not link (Ev's report) |
| P1 | `a-made-body-does-not-show-the-profile-it-was-made-from` | E | | a made body does not show its profile (Ev's report) |
| P1 | `certify-affordance-on-the-bounds-panel` | H | yes | the bounds panel has no certify control |
| P1 | `cascade-delete-shows-the-strand-count` | M | | the delete button shows what a delete would strand (DM7) |
| P1 | `the-viewer-has-no-guide-to-its-controls` | E | | the viewer has no guide to its controls (Ev's report) |
| P3 | `chooser-probe-misses-dbus-autolaunch` | E | | the chooser probe misses D-Bus autolaunch |
| P3 | `viewer-expresses-no-gpu-adapter-preference` | M | yes | the viewer expresses no GPU adapter preference |

## Order

Three design questions are weighed first, concurrently, while the two
rows whose fix is written are built:

- **Built now**: `cascade-delete-shows-the-strand-count` (its body is
  the spec: the pre-click count is the END state over
  `cascade_delete_order`, the post-click panel lists the door's rows)
  and `the-viewer-has-no-guide-to-its-controls`.
- **An offer** — `a-refusal-offers-no-action-in-the-viewer` and
  `a-check-offers-no-way-to-add-the-assertion-it-asks-for` are one
  question: what a recourse the viewer can act on is, where it is
  composed, and how it reaches the screen as a control. Three
  refuse-then-offer instances already exist in three shapes.
- **A node named to a person** — `a-node-name-the-viewer-shows-is-not-a-link-to-it`,
  with VNEWS' `the-viewer-spells-a-node-as-its-kind-before-its-name`
  (the same sites, the same value). The two Ev-reported instances,
  `a-placement-and-its-part-do-not-link-to-each-other` and
  `a-made-body-does-not-show-the-profile-it-was-made-from`, are built
  on whatever answer lands.
- **The certify control** — `certify-affordance-on-the-bounds-panel`:
  a long-running query's panel, its budget, and the three readings its
  body says a panel can get wrong.

The two P3 platform rows are tail.

## Review posture

The review tiers in `memories/orchestration-model.md` apply, and every
unit sent to a review is a row of the model comparison
(`docs/MODEL-COMPARE-PROTOCOL.md`). CHROME's 2026-09 "no AB protocol"
posture concerned the concluded model A/B and does not carry over.
Each unit's letter, tier and reason are logged at dispatch.
