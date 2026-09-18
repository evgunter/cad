---
id: sweep-test-rebuilds-validated-net-for-v-reversal
kind: issue
title: review_probes_m8_4.rs rebuilds a validated NURBS net for a v-reversal and unwraps an unreachable Result
status: closed
opened: 2026-09-04
refs: [1782]
branch: scalar/vrev-surface-door
pr: 2627
closed: 2026-09-15
---

## Finding

`crates/sweep/tests/review_probes_m8_4.rs:110-121` takes an
already-validated `NurbsSurface<f64>` off a body, rebuilds its control
net and weight vector by hand to reverse the `v` direction (both knot
vectors carried verbatim, `control` and `weights` permuted by
`j -> nv - 1 - j`), and pushes the result back through
`NurbsSurface::new(...).unwrap()`.

That is `D320`'s smell in a test: the constructor's only check is
`validate_counts`, and a permutation of an already-validated net changes
no count and no weight value, so the `Result` is unreachable on this data
and the `unwrap()` announces nothing. Unlike `D320`, there is **no door
to delegate to** — `geom` carries no v-reversal (nor u-reversal) door on
`NurbsSurface`, so the fix is either minting one in `geom` or a
`from_validated_parts`-style admission that the permutation preserves the
invariants.

**Out of Track T's fence.** `crates/*/tests/` is Track W's except the
files Track T's own rows name, and no T row names this one — hence a file
rather than an edit.

## Was

Filed by the style review of PR 1782 (`D320`/`D321`), which swept
`crates/sweep/` for hand-rebuilt control nets.

## Re-homed to SCALAR (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

SCALAR collects the scalar lane: the lift doors, the newtypes that would
carry an invariant, and the generic-scalar questions the kernel has been
answering by hand. This row is one of them.

Its class at the cut was **M** — fixing the test needs a new geom door
or a validated-parts admission decided. The class is a dispatch estimate
made by reading the row against the tree on 2026-09-11, not a verdict on
the finding, and a lane that finds it wrong says so in its PR. The id,
the `track:` letter where the row carries one, and the body above are
unchanged by the move.

## Refs at code-quality's sweep (2026-09-11)

`work/code-quality/` left the tracker (`docs/DOC-LEDGER.md`, sweep 11)
and its closed rows went with it. `D320` is now cited by its closing PR
1782.

## Closed (2026-09-15) — PR 2627

`NurbsSurface::reversed_u` (native) and `reversed_v` (by the module's
transposition conjugation) in `crates/geom/src/surfaces/nurbs.rs`: the
same point set with one direction reversed, defined only when that
direction's knot vector is mirror-symmetric under an EXACT sum test
(`geom_core::exact::two_sum`, one home), refusing `KnotMirrorError`
otherwise — which means decimal-symmetric pairs (thirds, 0.1/0.9) refuse
and the kernel's own loft accepts up to six equally spaced sections
(BLEND's row `interpolate-columns-averaged-knots-could-be-mirror-symmetric`
is the upstream fix). The door's doc says what it does not do: a
reversed chart re-attached to a body leaves that face's pcurves stale
(a row pins the hazard). `transposed` routes through
`from_validated_parts`; the structural-map count argument has one home
in `scalar_lift.rs`. `seam_on_chart` calls the door. Reviews: dual, both
APPROVE WITH FIXES, no MAJOR; twelve fix-pass items taken. Rows filed:
BLEND, PROPS ×2, TINT (extended), CIW (folded into
`gate-ok-summarised-a-run-with-a-k-lint-row-still-in-progress`).
