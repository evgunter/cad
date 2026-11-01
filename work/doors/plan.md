# DOORS — plan

Opened 2026-10-02 at AUTHOR's exit. AUTHTAIL folded back in on
2026-10-26. On 2026-11-01 HELD was cut out (`work/held/`) once D′3's
fired triggers took the track over budget. AUTHOR's history is
recoverable from `docs/doc-ledger/author-leaves-the-tracker.md`, and
AUTHTAIL's from `docs/doc-ledger/authtail-leaves-the-tracker.md`. The
decision records for the three rulings are rows 22–24 of
`docs/DESIGN-FORK-LOG.md`, together with the `[ev]` PRs #3551, #3571
and #3587.

## The slate

**29 budget points** of dispatchable work against 30. Four rows are
parked on SPACES' `[ev]` PR 6209.

| pri | item | cost | status |
|---|---|---|---|
| P0 | `a-mate-on-a-taken-over-copys-face-goes-stale` | M | open; designed |
| P0 | `a-split-of-a-drawn-body-is-drawn-unsplit` | M | parked on PR 6209 |
| P1 | `the-create-pane-has-no-extrude-side` | M | open |
| P1 | `face-pick-cannot-name-which-face` | M | open; ruled #3571 |
| P1 | `the-gui-has-no-clearance-consumer` | H | open; design fork |
| P1 | `a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product` | M | parked on PR 6209 (to SPACES) |
| P1 | `a-fillet-on-a-drawn-unions-operand-is-drawn-nowhere` | M | parked on PR 6209 |
| P1 | `the-mirror-keeps-two-pieces-and-mirrors-across-another-spaces-face-after-dprime3` | M | parked on PR 6209 |
| P2 | `a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added` | M | open; design |
| P2 | `the-boolean-door-evaluates-its-boolean-twice` | M | open |
| P3 | `a-takeover-could-re-seat-a-pick-its-result-keeps` | M | open; design |
| P3 | `a-takeovers-rebind-cannot-reach-part-spelled-names` | M | open; design |

Two closed rows are kept here:
- `the-viewer-cannot-author-a-mirror`: its `## Closed` records the
  mirror arrival fork;
- `a-combine-deletes-the-placement-it-just-repointed`: absorbed by
  D′3.

## Order

The sitting runs P0 and P1. Three lanes and one designer weighing run
concurrently, because their ground barely overlaps.

1. **P0, the takeover's rebind**:
   `a-mate-on-a-taken-over-copys-face-goes-stale`. Its `## Decided`
   is the spec. An editor-core door, `refactor::take_over`, re-points
   the copy and writes a `Rebind` for each carried name in one undo
   step. Its carrier query lives in `names/`. The viewer's
   `feature_over`, `add_mirror`, `sketch_on_face` and `add_transform`
   call it. It replaces `Renamed::through` for held picks. CHROME's P1
   `a-findings-face-reference-reads-as-stale` is parked on this row.
   **Coupled with SPACES**: its "gestures stop placing" unit rewrites
   the same gestures. The door is independent of which readers a
   re-point reaches, so it lands first and SPACES calls it. Both logs
   say so.
   Tier: style review plus a correctness reviewer. A stale or
   wrongly-rebound reference is exactly "a node quietly not what the
   author drew".
2. **P1, `the-create-pane-has-no-extrude-side`**: a side control, an
   op that carries it, the property row, and the negative-depth
   refusal's recourse made one click through an existing offer home
   (lessons, below). Tier: a style review.
3. **P1, `face-pick-cannot-name-which-face`**: one face-in-a-sentence
   composer beside `tree::node_number`, spoken through
   `Speaker::of(landed).within(evaluation)`, across five sites, with
   `BlendTarget::of_face` removed and a hover mark on each line of the
   declare offer. Tier: a style review.
4. **P1, `the-gui-has-no-clearance-consumer`**: the designer pair
   first (`docs/DESIGN-FORK-PROTOCOL.md`), then an `[ev]` PR only if
   the fork is Ev's.

The tail is the boolean pair (P2) and the two INTENT follow-ups (P3).
It is handed back with the track unless the P0 lane's context makes
`a-takeover-could-re-seat-a-pick-its-result-keeps` cheap; that row
re-seats exactly on what the rebind map already computes.

Once Ev answers PR 6209, re-read the four parked rows against the
answer. Each one dissolves (close it with the reason), moves to SPACES
(`git mv`), or opens here.
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
