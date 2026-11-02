# CARVEREST — the plan

CARVE's P3 and P4 rows

Opened 2026-10-10 by CARVE's third priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**21.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `a-walked-chain-closes-at-a-corner-without-a-junction` | M | walk_chains closes a chain at a corner vertex without recording a junction there |
| P3 | `corner-valence-reads-a-self-closed-edge-once` | M | corner_at, cap_incidence and the corner admission count a self-closed edge once at its vertex |
| P3 | `loft-doors-take-a-non-finite-placement` | M | the loft doors take a non-finite placement: an infinite z refuses ReversedStacking and a NaN escalates |
| P3 | `loft-geometry-refuses-a-bad-parameter-vector-as-a-count-mismatch` | E | loft_geometry refuses a NaN, descending or unclamped parameter vector as a count mismatch |
| P3 | `material-jet-readings-take-plus-minus-order` | M | tier 3, rim_wedge and contact_verify read the tangent jet in (plus, minus) order, a third order beside the certificate's key order |
| P3 | `skin-union-mints-a-hairline-span-from-knots-an-ulp-apart` | M | make_compatible's knot union keeps two section knots an ulp apart as two knots |
| P3 | `cap-plane-refusals-end-as-typed-kernel-defects` | E | CapPlaneError's Orientation and EdgeOn, and LoftError's SeamStructure and SectionStructure, name the kernel alone for states a validated input never reaches |
| P3 | `corner-config-indeterminate-has-no-real-body-witness` | E | `CornerConfig::Indeterminate` has no real-body witness once a plane–plane end reads its end face first |
| P3 | `open-rim-end-corner-of-a-partial-revolve-does-not-classify` | M | a partial revolve's open neck-flare rim ends in a corner `corner_at` reads as Indeterminate |
| P3 | `blend-an-edge-in-another-shell-moves-predicate-4-at-a-cavity-corner` | M | one island edge added to a sealed cavity's request makes predicate 4 refuse `ChainNotG1` at a cavity corner |
| P3 | `skinned-wall-weights-drift-an-ulp-along-the-stacking` | E | a skinned wall's weights differ by an ulp along the stacking |
| P4 | `certify-residual-predicates-still-name-a-slot` | E | certify's residual predicate names still name a slot of the surface pair |

## Order

The rows pair up:

- **The chain walk.** `a-walked-chain-closes-at-a-corner-without-a-junction`
  and `corner-valence-reads-a-self-closed-edge-once` are `walk_chains`
  and the valence readers beside it, found by the review of PR 4185.
  One reads a chain that closes at a corner as closed. The others
  count a self-closed edge once where the walk counts it twice. One
  unit, since they must agree on one count. Three rows from BAND's
  2026-10-26 split read the same code:
  `blend-an-edge-in-another-shell-moves-predicate-4-at-a-cavity-corner`
  asks which two links `walk_chains` paired at a cavity corner
  (measure first), and `corner-config-indeterminate-has-no-real-body-witness`
  and `open-rim-end-corner-of-a-partial-revolve-does-not-classify` are
  `corner_at`'s lossy fold of a neighbour's refusal into
  `Indeterminate`. The walk unit's taker reads all five.
- **The loft doors' input checks.**
  `loft-doors-take-a-non-finite-placement` and
  `loft-geometry-refuses-a-bad-parameter-vector-as-a-count-mismatch`.
  One unit.
- **The kernel-defect endings.** `cap-plane-refusals-end-as-typed-kernel-defects`
  is a child of ENCL's census of typed kernel-defect endings, and is
  repaired in the shape that census gives.
- **The knot union.** `skin-union-mints-a-hairline-span-from-knots-an-ulp-apart`
  is unmeasured. Its first step is whether a real loft reaches it.

`material-jet-readings-take-plus-minus-order` is not mechanical. The
(plus, minus) order carries material-side meaning, so its unit says
which order each reader means before it moves any.
`certify-residual-predicates-still-name-a-slot` is a rename that
re-pins the ledgers counting those predicate names.

## The D10 hold

None of these rows reads declared pairs, declared contact, placement
vocabulary or the node vocabulary. The loft doors' placement checks
are input validation on the frame the caller passes, not a placement
vocabulary.

## Review posture

Protocol v7 (`docs/DUAL-REVIEW-PROTOCOL.md`), with the triage question
OPEN for the first dispatch, as CARVE's was.
