# REFS — plan

## Slate and order

1. `a-name-is-scoped-by-the-variable-holding-its-body` (DM4 unit 2,
   PR 4674): green, holds for Ev on PR 4691 (N5's `ReadGone` arm and
   the reference's path). Its lane reports to the INTENT orchestrator
   until it merges; the ruling item moves here after.
2. `typing-a-value-mints-or-offers-a-variable` (in review).
3. The indexed-read rows together (`an-indexed-read-of-rank-two-has-no-family`,
   `a-selects-body-takes-an-indexed-read` (closed 2026-11-10, PR 6555),
   `python-cannot-re-point-an-indexed-reads-index`): one representation
   question, likely one designer pair first.
4. The rest by band and cost.

## Review posture

INTENT's, unchanged (the repo-wide tiers; rows in
`docs/dual-review-log/rows/`; short readouts at each M-tier miss).

## Exit criteria

None beyond the slate.

## From WIRETAIL (merged 2026-11-20)

WIRE's P3 and P4 tail: the selection vocabulary's missing atoms, label and kind-mirror hygiene, and at-rest gating repeated per consumer.

Opened 2026-10-17 by WIRE's priority-seam cut. Nothing dispatched.

### The slate

**24 budget points** against 30 (re-weighted tiers, 2026-10-28).

| pri | item | cost |
|---|---|---|
| P3 | `selector-has-no-sharp-edge-atom-so-a-kind-pair-takes-tangent-seams` | M +design |
| P3 | `adjacent-kinds-cannot-tell-a-crease-from-a-co-surface-seam` | M |
| P3 | `a-name-pattern-cannot-say-descends-from-this-node` | M |
| P3 | `id-lowerings-to-u64-tokens-drop-the-ordinal` | M +design |
| P3 | `attr-label-has-no-reader` | M |
| P3 | `a-body-value-consumed-by-k-nodes-is-gated-at-rest-k-times` | M |
| P4 | `on-tool-vertex-may-be-a-segment-n1-already-makes-redundant` | M +design |
| P4 | `split-half-side-words-are-spelled-at-three-sites` | E |
| P4 | `sentence-labels-is-a-second-meaning-of-label` | E |
| P4 | `the-seated-tools-label-nouns-are-not-pinned-to-the-nodes-they-mint` | E |

Parked: `a-verdicts-other-nodes-are-said-by-tag-once-deleted` (P3), on
DOCTAIL's `no-docedit-splices-a-deleted-node`, which is itself deferred.
Deferred: `S40` (the witness stubs; its body cites the ratification,
and its prose half is done).

### Order

The refusal-recourse rows moved to WIREREFUSE on 2026-10-28 (`work/doctail/plan.md`).

The two GS-Q2 rows are one designer weighing (a crease atom changes
SELECT-DESIGN, so a fork goes to Ev). The E rows are one unit.

### Review posture

Inherits WIRE's (WIRE's plan, recoverable by `docs/doc-ledger/wire-leaves-the-tracker.md`): protocol v7, single reviews.

**From EMIT's close (2026-10-19).** The four label rows above came from
`work/emit/` by `git mv` (+7 points). So did the parked
`a-verdicts-other-nodes-are-said-by-tag-once-deleted`, a selection
verdict's diagnosis that says deleted nodes by tag; it waits on its own
`blocked_on`.

## From NAMECHECK (merged 2026-11-20)

N0's checks: the denotation verifier, the coverage fuzzer, the `RoleSeg` form map, the storage-order perturbation, and the clauses that point at N0.

Opened 2026-10-20 by WIRE's cut. Nothing dispatched.

### The slate

**17.5 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P2 | `a-denotation-verifier-checks-every-name-against-the-finished-body` | H |
| P2 | `a-coverage-fuzzer-finds-legal-bodies-that-refuse-naming` | H |
| P2 | `every-role-seg-variant-maps-to-one-n0-form` | M |
| P2 | `topo-has-no-arena-permuting-helper-for-a-storage-order-name-check` | M |
| P3 | `n0-the-duplicated-naming-clauses-point-at-n0` | M |

### Order

1. **The form map first.** It is small, and the verifier reads it: each `Made` variant's doc lists the structural fields the verifier confirms.
2. **The storage-order helper next.** It is independent of the map and can run in parallel.
3. **The fuzzer.** Every `Naming(*)` refusal it finds on a legal body is a WIRE naming-defect row. File it there; do not fix it here.
4. **The pointer rewrite last,** once the checks it points at exist. It is a re-wording of ratified clauses with no change of substance, so it lands with its own change under CLAUDE.md's re-wording rule.

### Review posture

Inherits WIRE's (WIRE's plan, recoverable by `docs/doc-ledger/wire-leaves-the-tracker.md`): protocol v7, single reviews; FULL where a check could pass vacuously.

**Class note from WIRE's 2026-10-20 sitting:** a check that cannot go red is the commonest failure here. PR 5460's VACUOUS lists, and its admission lists that red in both directions, are the precedent. Every new check is shown red by a mutant before it merges.
