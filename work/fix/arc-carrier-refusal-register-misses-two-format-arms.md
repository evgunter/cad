---
id: arc-carrier-refusal-register-misses-two-format-arms
kind: issue
title: PATHS-DESIGN's typed-runtime-refusal register omits both format arms of the 2-D director doors
status: open
opened: 2026-09-12
refs: [underflow-gate-owed-at-five-more-doors, 2359, 2415]
---

Disclosed by the `fix/arc-fillet-underflow` lane (PR 2415) while
fitting the underflow gate at `arc_fillet::carrier_tangent`, and
filed at the moment of disclosure rather than left in that PR's body.

`docs/PATHS-DESIGN.md`'s **Refusals** paragraph (the "Typed runtime
errors, from geometry" register, around `docs/PATHS-DESIGN.md:903-940`)
enumerates the `PathError` arms the §2a/§2c surface can raise —
`NonpositiveCircleRadius`, `ZeroDirection`, `ArcViaCollinear`,
`DegenerateArcChord`, `DegenerateArcSpec`, `ArcCenterNotEquidistant`,
`DegenerateArcCenter`, `FarEndAnchorWithoutFillet`,
`CircleSplitCount`, the two `ArcContinue*`, the two `SeamArrival*` —
and names neither of the two arms that refuse a length the FORMAT
lost:

- **`PathError::NonFiniteDirection`**, landed by PR 2359, which the
  register has never named;
- **`PathError::UnderflowedDirection`**, landed by PR 2415.

Both are raised from inside this document's own surface — the
components director (`PartialPath::toward`) and the arc carrier's
tangent — so the omission is not a scope question. The register is
prose with no check under it, which is exactly why two arms could
land without it noticing: nothing in CI compares that list against
`PathErrorKind`.

**Two things a taker decides**, and they are separable:

1. The cheap half: add the two arms to the register with the
   sentence that distinguishes them from `ZeroDirection` (not a
   coincidence at any tolerance; the recourse is scale, not ε).
2. The half worth arguing: whether that register should be checked at
   all. `PathErrorKind` is a fieldless mirror of `PathError` and the
   `pncad-py` `TAG_INVENTORY` already pins the full arm set at the
   Python door, so a register that goes stale silently while a
   neighbouring list cannot is a shape worth naming — or the register
   is prose by design and should say so, so the next lane does not
   read its completeness as a claim.

**Fence.** `docs/PATHS-DESIGN.md` is not in any open program's
`paths` globs. It is filed here because FIX took the two units that
moved the arms it omits, not because FIX's charter claims the
document; a taker should say which way it read that.
