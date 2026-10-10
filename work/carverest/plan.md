# CARVEREST — the plan

CARVE's P3 and P4 rows

Opened 2026-10-10 by CARVE's third priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**13.5 budget points** of dispatchable work against a ceiling of 30.
PR 4189's state sync adds two more rows, for 17.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `a-walked-chain-closes-at-a-corner-without-a-junction` | M | walk_chains closes a chain at a corner vertex without recording a junction there |
| P3 | `corner-valence-reads-a-self-closed-edge-once` | M | corner_at, cap_incidence and the corner admission count a self-closed edge once at its vertex |
| P3 | `loft-doors-take-a-non-finite-placement` | M | the loft doors take a non-finite placement: an infinite z refuses ReversedStacking and a NaN escalates |
| P3 | `loft-geometry-refuses-a-bad-parameter-vector-as-a-count-mismatch` | E | loft_geometry refuses a NaN, descending or unclamped parameter vector as a count mismatch |
| P3 | `skin-union-mints-a-hairline-span-from-knots-an-ulp-apart` | M | make_compatible's knot union keeps two section knots an ulp apart as two knots |
| P3 | `sweep-places-vanishing-tangent-is-a-bare-f64-compare` | M | sweep_places refuses a vanishing path tangent by a bare n > 0.0 |

Arriving with PR 4189:
`material-jet-readings-take-plus-minus-order` (P3, M) and
`certify-residual-predicates-still-name-a-slot` (P4, E).

## Order

The rows pair up:

- **The chain walk.** `a-walked-chain-closes-at-a-corner-without-a-junction`
  and `corner-valence-reads-a-self-closed-edge-once` are `walk_chains`
  and the valence readers beside it, found by the review of PR 4185.
  One reads a chain that closes at a corner as closed. The others
  count a self-closed edge once where the walk counts it twice. One
  unit, since they must agree on one count.
- **The loft doors' input checks.**
  `loft-doors-take-a-non-finite-placement` and
  `loft-geometry-refuses-a-bad-parameter-vector-as-a-count-mismatch`.
  One unit.
- **One banded decide.** `sweep-places-vanishing-tangent-is-a-bare-f64-compare`
  is a Q1 fix. CARVE's `sweep-frame-is-a-minimal-rotation-from-the-start-tangent`
  rewrites `sweep_places`, so it should land after that row's weighing
  or ride its build.
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
