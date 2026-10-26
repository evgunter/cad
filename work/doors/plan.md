# DOORS — plan

Opened 2026-10-02 at AUTHOR's exit, the half of its cut that waited on
kernel rows. AUTHTAIL, the other half, folded back in on 2026-10-26
(Ev, in chat): both were AUTHOR's successors on the viewer authoring
files, and the split had stopped earning its keep. AUTHOR's history is
recoverable from `docs/doc-ledger/author-leaves-the-tracker.md`,
AUTHTAIL's from `docs/doc-ledger/authtail-leaves-the-tracker.md`; the
three rulings' decision records are rows 22–24 of
`docs/DESIGN-FORK-LOG.md` and the `[ev]` PRs #3551, #3571 and #3587.

## The slate

**13.5 budget points** of dispatchable work against a ceiling of 30;
ten rows are parked and count nothing until their triggers fire.

| pri | item | cost | status |
|---|---|---|---|
| P0 | `a-mate-on-a-taken-over-copys-face-goes-stale` | M | parked on PR 5636 (D′3) |
| P0 | `a-split-of-a-drawn-body-is-drawn-unsplit` | M | parked on PR 5636 |
| P1 | `the-create-pane-has-no-extrude-side` | M | open |
| P1 | `face-pick-cannot-name-which-face` | D (legacy; re-price on touch) | open |
| P1 | `the-gui-has-no-clearance-consumer` | H | parked on CLEAR's `clearance-refusal-names-one-face-twice-across-bodies` |
| P1 | `a-fillet-on-a-drawn-unions-operand-is-drawn-nowhere` | M | parked on PR 5636 |
| P1 | `the-mirror-keeps-two-pieces-and-mirrors-across-another-spaces-face-after-dprime3` | M | parked on PR 5636 |
| P1 | `a-combine-deletes-the-placement-it-just-repointed` | E | parked on PR 5636 |
| P2 | `a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added` | M | open |
| P2 | `the-boolean-door-evaluates-its-boolean-twice` | M | open |
| P2 | `moves-of-drops-the-third-angle-near-gimbal-lock` | E | open |
| P2 | `a-seated-tools-held-node-is-drawn-nowhere` | M | parked on PR 5636 |
| P3 | `a-last-leg-no-close-can-follow-is-dropped` | M | open |
| P3 | `a-held-face-stays-marked-while-its-form-is-collapsed` | M | parked on PR 5636 |
| P3 | `committed-nodes-do-not-light-the-faces-they-reference` | M | parked on PR 5636 |
| P4 | `the-mate-and-blend-commit-rows-restate-tool-commit-row` | M | parked on PR 5636 |

One closed row is kept: `the-viewer-cannot-author-a-mirror`, whose
`## Closed` section records the mirror arrival fork that
`the-mirror-keeps-two-pieces-…` is the residue of.

## Order

Dispatchable now:

1. **P1, the rulings' doors.**
   - `the-create-pane-has-no-extrude-side` (M) — Ev's added requirement
     on #3551 rides it: the negative-depth refusal shows the extrude
     written the other way, and the form offers it.
   - `face-pick-cannot-name-which-face` (D) — draw `StableName`'s own
     `Display`; mint no composer of the viewer's own.
2. **P2, the boolean pair** — one unit: RECIPE's
   `declared-pairs-are-a-booleans-own-payload` retired the separate
   Declare node's edge, so both rows land together.
   - `a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added` (M)
   - `the-boolean-door-evaluates-its-boolean-twice` (M)
3. `moves-of-drops-the-third-angle-near-gimbal-lock` (P2 E): the fix is
   written in the row.
4. `a-last-leg-no-close-can-follow-is-dropped` (P3 M): half of it waits
   on PROFILETAIL's
   `a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam`; the
   other half is a design question about what the preview is.

Once INTENT's D′3 (PR 5636) lands — it rewrites placements (one source
each), names (copies carry their source's names) and picks, and edits
`tools.rs`, `world.rs` and `repoint`:

5. **P0, the takeover.**
   - `a-mate-on-a-taken-over-copys-face-goes-stale`: designed (its
     `## Decided`). An editor-core door re-points the copy and writes a
     `Rebind` per carried name.
   - `a-split-of-a-drawn-body-is-drawn-unsplit`: the split's halves are
     taken over as `above ⊔ below`.
6. **P1, also on the takeover.**
   - `a-fillet-on-a-drawn-unions-operand-is-drawn-nowhere`: the takeover
     re-points along the path the body was seen through.
   - `the-mirror-keeps-two-pieces-…-after-dprime3`: the two-pieces `⊔`
     output, and the world-level mirror across another space's face.
   - `a-combine-deletes-the-placement-it-just-repointed`: close it as
     absorbed if it does not reproduce on a one-source placement.

   These three and the two P0s share one generalised takeover rule: a
   takeover re-points whatever reads the picked body along the seen path
   (a placement's source, a `⊔`/`∪` operand slot, or the product slot).
   INTENT owes its spec sentence (INTENT's log, 2026-10-25). Build that
   rule once, not per gesture.
7. **P2–P3, the held picks.** `a-seated-tools-held-node-is-drawn-nowhere`,
   with `a-held-face-stays-marked-while-its-form-is-collapsed` riding with
   it, is designed in its `## Decided` section: one holder; `FaceSeat`;
   `Tools::held()`; one `FLAG_HELD` lane; the open tool's panel always on
   screen. It also folds the mirror review's three items.
8. **P3–P4, re-state first.**
   - `committed-nodes-do-not-light-the-faces-they-reference`: re-state it
     against D′3's placements; mates now read only geometry.
   - `the-mate-and-blend-commit-rows-restate-tool-commit-row`: re-read it
     against D′3's mate tool.

`the-gui-has-no-clearance-consumer` (H) is taken when CLEAR's row
closes.

## Lessons carried from AUTHOR and AUTHTAIL

- **AUTHOR's units always minted a duplicate**: a hand copy of a kernel
  string, a second home for a rule, a parallel test builder — and every
  review caught it. Spec each unit against it, and keep the kernel's
  words the viewer's words. The refuse-then-offer homes
  (`creation_offer`, `declare_offer`, the accept-version offer) are
  three shapes of one thing; OFFER's P1 row holds the general type, and
  a door here that needs an offer uses those homes rather than a fourth.
- **Re-verify every fix pass before merging.** Both of the mirror's fix
  passes changed the design, and each re-verify found something the
  pass had introduced.
- **A gesture that adds or moves a body must be tested on a
  non-identity placement**, by reading the drawn copy. Every early
  mirror fixture sat at the identity, so none could see the arrival
  defect.
- **Viewer-only diffs never select the ε rows in the per-PR gate**: run
  ε-sensitive sketch rows at `CAD_TOLERANCE_EPS=1e-6` and `1e-12`.

## Review posture

AUTHOR's, inherited unchanged by both halves: no duals; a style review
(`docs/prompts/reviewer-style-lane.md`) plus a second correctness
reviewer on any unit whose failure mode is a confident wrong picture or
a node quietly not what the author drew, and a re-verify on each fix
pass. E and prose rows take the orchestrator's read. No A/B row while
the account's Fable budget is spent (`docs/MODEL-AB-LOG.md`); the band
stays claimed and unspent.

## Exit shape

Every row merged or re-homed with a reason. No exit criteria are set,
so no walk is owed.
