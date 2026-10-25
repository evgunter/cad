# AUTHTAIL — plan

Opened 2026-10-02 at AUTHOR's exit, the unblocked half of its cut.
AUTHOR's history is recoverable from
`docs/doc-ledger/author-leaves-the-tracker.md`.

## Where the track stands

Most of the slate waits on INTENT's D′3 (PR 5636,
`assemble-is-an-operator-and-the-product-is-a-slot`). D′3 rewrites
placements (one source each), names (copies carry their source's names)
and picks, and it edits `tools.rs`, `world.rs` and `repoint`, which most
of these rows sit on. Once it lands, take the parked rows in this order:

1. **P0, the takeover.**
   - `a-mate-on-a-taken-over-copys-face-goes-stale`: designed (its
     `## Decided`). An editor-core door re-points the copy and writes a
     `Rebind` per carried name.
   - `a-split-of-a-drawn-body-is-drawn-unsplit`: the split's halves are
     taken over as `above ⊔ below`.
2. **P1, also on the takeover.**
   - `a-fillet-on-a-drawn-unions-operand-is-drawn-nowhere`: the takeover
     re-points along the path the body was seen through.
   - `the-mirror-keeps-two-pieces-…-after-dprime3`: the two-pieces `⊔`
     output, and the world-level mirror across another space's face.
   - `a-combine-deletes-the-placement-it-just-repointed`: close it as
     absorbed if it does not reproduce on a one-source placement.

   These three and the two P0s share one generalised takeover rule: a
   takeover re-points whatever reads the picked body along the seen path
   (a placement's source, a `⊔`/`∪` operand slot, or the product slot).
   INTENT owes its spec sentence; see INTENT's log, 2026-10-25. Build that
   rule once, not per gesture.
3. **P2–P3, the held picks.** `a-seated-tools-held-node-is-drawn-nowhere`,
   with `a-held-face-stays-marked-while-its-form-is-collapsed` riding with
   it, is designed in its `## Decided` section:
   - one holder;
   - `FaceSeat`;
   - `Tools::held()`;
   - one `FLAG_HELD` lane;
   - the open tool's panel always on screen.

   It also folds the mirror review's three items.
4. **P3–P4, re-state first.**
   - `committed-nodes-do-not-light-the-faces-they-reference`: re-state it
     against D′3's placements; mates now read only geometry.
   - `the-mate-and-blend-commit-rows-restate-tool-commit-row`: re-read it
     against D′3's mate tool.

Dispatchable now, not waiting on D′3:

- `moves-of-drops-the-third-angle-near-gimbal-lock` (P2 E): the fix is
  written in the row.
- `a-last-leg-no-close-can-follow-is-dropped` (P3 M): half of it waits on
  PROFILETAIL's
  `a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam`; the
  other half is a design question about what the preview is.

## Lessons

- **AUTHOR's units always minted a duplicate**: a hand copy of a kernel
  string, a second home for a rule, a parallel test builder. Spec every
  unit against it.
- **Re-verify every fix pass before merging.** Both of the mirror's fix
  passes changed the design, and each re-verify found something the pass
  had introduced.
- **A gesture that adds or moves a body must be tested on a
  non-identity placement**, by reading the drawn copy. Every early mirror
  fixture sat at the identity, so none could see the arrival defect.
- **Viewer-only diffs never select the ε rows in the per-PR gate**: run
  ε-sensitive sketch rows at `CAD_TOLERANCE_EPS=1e-6` and `1e-12`.

## Review posture

- A full single review (claims plus `docs/prompts/reviewer-style-lane.md`)
  on any unit whose failure mode is a confident wrong picture, and a
  re-verify on each fix pass.
- E and prose rows take the orchestrator's read.
- No A/B row while the Fable budget is spent; the band stays claimed and
  unspent.

## Exit shape

Every row merged or re-homed with a reason. No exit criteria are set, so
no walk is owed.
