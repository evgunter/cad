---
id: the-graph-separator-accepts-a-side-past-its-edges-piece-by-membership
kind: issue
title: Under the graph separator, a corner presentation's side along a cylinder's parallel may run past the edge's piece, where the graph does not vanish by construction, and membership accepts it
status: open
opened: 2026-11-13
priority: P1
cost: M
---



## Finding

Found by CLEAR's lane B (`clear/own-separators`, PR 6905), and not
fixed there.

The graph separator vanishes on its one shared edge's piece by
construction, and only there. Under it, a `Zero::Side` from a corner
presentation on a cylinder's parallel can run past that piece. Past
the piece, the graph does not vanish by construction.

PR 6905 kept the existing "as now" handling, as its brief asked:
membership accepts that side. If the side really runs past the piece,
`sign_on` treats it as a zero side where `ψ` is not zero, and the sign
argument across it would be unproven.

No instance has been shown.

## What is needed

1. **Probe.** Find whether any presentation hands the graph separator
   a side that runs past its edge's piece:
   - build a fixture: a cylinder's parallel meeting a corner;
   - count such sides over the injective test suite and the tour walk.
2. **If none can occur,** argue it in `sign_on`'s doc and close.
3. **If one occurs,** re-band to P0, since a hole in the certificate's
   soundness takes the gate's band. Then make membership require the
   side's parameters to lie in the graph's piece, as the curve arm
   already does.
