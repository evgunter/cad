# WIRE — the plan

the evaluation seat: wiring, the placement lift and the verb vocabulary

Re-cut 2026-10-17 by a triage of every live row against the tree at
`fd218851f9`: twelve rows closed as overtaken (most by INTENT stage 4
F, which deleted the declare surface), eleven parked on the D10 hold
and moved to WIREHOLD, one re-homed to CONTACTHOLD, and the P3/P4 tail
moved to WIRETAIL (`log.md`, same date).

## The slate

**30 budget points** against a ceiling of 30. Nothing here stands on
ground D10 is still rebuilding; a lane that finds it does stops and
says so.

| pri | item | cost | note |
|---|---|---|---|
| P0 | `a-merged-face-with-several-same-side-constituents-has-no-chord-rule` | H | closed by PR 5263; its chord descent retired by PR 5313 (N3, Ev on PR 5279) |
| P1 | `a-split-through-a-pinch-names-both-tip-copies-alike` | M +design | one weighing with the next row: how a pinch vertex is named |
| P1 | `a-pinch-vertexs-name-depends-on-the-unions-member-order` | M +design | measure first (one name table across the six orders) |
| P1 | `a-split-circles-phase-edit-re-aims-its-piece-names-silently` | M +design | an N1 reading; ratified text, so a fork goes to Ev |
| P1 | `the-designation-road-resolves-before-it-asks-the-kind` | E | one unit with the next row (`entity_door::Found`) |
| P1 | `the-entity-doors-key-comes-from-its-caller` | M | FORK-ENTITYDOOR converged: the kind moves to the edit and load doors |
| P1 | `part-cache-lock-held-across-a-nested-rayon-join` | M +design | latent while `opts.parallel` defaults off |
| P1 | `need-count-spells-every-failure-as-a-pattern-count` | E | unparks VGEOM's count-cast row |
| P2 | `a-union-fold-publishes-no-pinch-row` | M | |
| P2 | `a-prism-crossing-a-wedge-about-a-contact-line-names-a-crossed-edge-both-ways` | M | after the merged-chord unit (same file) |
| P2 | `wire-refusals-short-of-the-shape-guard` | M | the unheld families only |
| P3 | `lane-profile-defaults-a-missing-replay-record-to-empty` | E | rides with need-count |
| P4 | `eval-kernel-bug-refusals-end-in-a-tag` | E | rides with need-count |
| P4 | `wire-profile-spells-a-missing-precompute-as-a-missing-slot` | E | rides with need-count |

## Order

1. **In parallel, no file overlap:** the merged-chord rule (P0,
   `names/emit_topo.rs`); the refusal-endings bundle (`need-count`,
   `eval-kernel-bug`, `wire-profile-spells`, `lane-profile`, plus the
   stale `M6` citations S40 names and the declare-door prose the
   triage found, `eval/mod.rs` and `eval/wire.rs`); the union fold's
   pinch row (`wire_combine`).
2. **Designer pairs**, while wave 1 builds: the pinch-naming pair of
   rows together; the entity-door pair; the part-cache lock; the
   circle-split phase (N1).
3. The prism crossing once the merged-chord unit has merged, then the
   recourse census and whatever the designers settle.

## Review posture

ANSWERED 2026-10-17 at the first dispatch: protocol v7, and no unit on
this slate is triaged into the dual — none is an architectural decision
whose impact is broad or hard to reverse. Single reviews: a FULL review
(claims plus style) for the naming units, whose failure mode is a
confident wrong name; a STYLE review for the refusal-text bundles. A
unit the designers turn into a representation change is re-triaged at
its dispatch.
