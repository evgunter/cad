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

The 2026-11-01/02 sitting closed every P0 and P1 row (log, that date):
the takeover rebind (PR 6264), the split takeover (6331), the
clearance assertion's kernel (6292) and viewer (6357) halves, face
words (6262), and the meeting-scope fix (6354); the extrude side was
superseded by D10 and the fillet-on-a-union row by SPACES' 6265. What
remains is P2 and below; the board's computed place says how urgent.

| pri | item | cost | note |
|---|---|---|---|
| P2 | `the-name-gate-is-tested-as-a-table` | M | the property table re-verify 5 specced; stops the cell-by-cell pattern of 6264 |
| P2 | `a-fresh-pick-off-the-older-picture-commits-a-stale-reference` | M | pickcache / viewport picking |
| P2 | `a-clearance-over-a-patterns-members-is-not-certified` | M | per-member certification |
| P2 | `the-mirror-keeps-two-pieces-and-mirrors-across-another-spaces-face-after-dprime3` | M | design: a `⊔` output changes 6209's README sentence |
| P2 | `a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added` | M | design |
| P2 | `the-boolean-door-evaluates-its-boolean-twice` | M | |
| P3 | the takeover residue (fused pattern, union-operand mates, payload names, undo, re-seat a kept pick, chain walks by hand) | — | each its own row |
| P3 | the clearance-consumer residue (third seam copy, seat machinery, undrawn seat, narrower seats, two spellings of `≥`) | — | each its own row |
| P3 | `a-split-half-cannot-be-split`, `a-selections-kept-labels-miss-the-nodes-its-face-words-name` | E / — | |

## Order

For the next sitting: `the-name-gate-is-tested-as-a-table` first — it
is the one row that makes the name gate's correctness checkable as a
whole rather than probe by probe — then the two P2 design rows to their
designers (mirror two pieces; flush union), then the rest by the board.

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
- **A gate rebuilt cell by cell keeps leaking** (DOORS 2026-11-02,
  PR 6264): five fix passes each closed the probe its review named and
  opened the next cell of the same gate. State a gate's invariant as a
  type (provenance carried by value, private constructors) and test it
  as a table, not as instances.
- **Widening a shared kernel query for one caller changes every
  caller** (PR 6331): pass the new caller's extra reads in, never
  rediscover them by walking the document.
- **A door that `unreachable!`s on a document's value waits for its
  first reachable caller** (PRs 6292/6357, D9): answer a typed refusal.
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

## From HELD (merged 2026-11-20)

Cut from DOORS on 2026-11-01. Every row came over with its id and body
unchanged. The four held-pick rows were parked on D′3 (PR 5636) until
the cut and were unparked at it, so re-read each against D′3's
placements and tools before writing its spec.

### The slate

**23 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P2 | `the-viewer-authors-no-pick` | M |
| P2 | `the-viewers-numeric-gestures-read-a-frame-back-off-geometry` | M |
| P2 | `moves-of-drops-the-third-angle-near-gimbal-lock` | E |
| P2 | `a-seated-tools-held-node-is-drawn-nowhere` | M |
| P3 | `a-held-face-stays-marked-while-its-form-is-collapsed` | M (rides the seated-tool row) |
| P3 | `committed-nodes-do-not-light-the-faces-they-reference` | M |
| P3 | `the-drafts-keep-held-slot-ids-across-a-document-replacement` | E |
| P3 | `a-last-leg-no-close-can-follow-is-dropped` | M |
| P4 | `the-mate-and-blend-commit-rows-restate-tool-commit-row` | M |

### Order

1. `moves-of-drops-the-third-angle-near-gimbal-lock` (E): the fix is
   written in the row.
2. `the-viewers-numeric-gestures-read-a-frame-back-off-geometry` and
   `the-viewer-authors-no-pick`: both are a gesture writing something
   other than what the author picked or typed.
3. The held picks: `a-seated-tools-held-node-is-drawn-nowhere` with its
   passenger, then `committed-nodes-do-not-light-the-faces-they-reference`
   (re-state it against D′3's placements, where mates read only
   geometry) and `the-drafts-keep-held-slot-ids-across-a-document-replacement`.
   DOORS' P0 `a-mate-on-a-taken-over-copys-face-goes-stale` replaces
   `Renamed::through` with the takeover's rebind map, which re-seats held
   picks. Land this group after it, or say on DOORS' log which lands first.
4. `a-last-leg-no-close-can-follow-is-dropped`: half of it waits on
   PROFILETAIL's `a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam`,
   and the other half is a design question about what the preview is.
5. `the-mate-and-blend-commit-rows-restate-tool-commit-row`: re-read it
   against D′3's mate tool.

### Lessons carried from DOORS

DOORS' `plan.md` carries AUTHOR's and AUTHTAIL's lessons, and they hold
here unchanged:

- units mint duplicates, so spec each one against that;
- re-verify every fix pass;
- test a gesture that adds or moves a body on a non-identity placement;
- run ε-sensitive sketch rows at `CAD_TOLERANCE_EPS=1e-6` and `1e-12`.

### Review posture

DOORS', unchanged: a style review plus a second correctness reviewer
on any unit whose failure is a confident wrong picture; E rows take
the orchestrator's read.

### Exit shape

Every row merged or re-homed with a reason. No exit criteria are set,
so no walk is owed.
