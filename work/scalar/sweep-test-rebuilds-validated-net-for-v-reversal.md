---
id: sweep-test-rebuilds-validated-net-for-v-reversal
kind: issue
title: review_probes_m8_4.rs rebuilds a validated NURBS net for a v-reversal and unwraps an unreachable Result
status: open
opened: 2026-09-04
refs: [D320, 1782]
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
