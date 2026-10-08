# MSOLVE-14-SPEC.md

MSOLVE-14, the mate solve runs at the evaluation's own scalar: frames, the coset fold and the solved poses generic over `T`, the structure read at the nominal, `Unpinned` loses its producer (#3986)

Deleted at the unit's merge, 2026-10-04.
Recover with `git show 7e84100de0dc5134b413b32cca297440ec6a3e41:docs/MSOLVE-14-SPEC.md`.
`work/msolve/log.md` records where the build departed from the spec:
- `clocking_about` decides the carried point's radius
  (`mate_clocking_radius`) before it takes an angle, and returns the
  identity when the table already decided the two axis lines coincide.
  The spec assumed `atan2(0, 0) = 0`, which holds only at `f64`.
- The interval `atan2` reads its branch about π across the negative-x
  cut.
- `quoted_residual` cannot fail: an `Under` stays `Under` on every lane.
- `sensitivities` and `stackup` take the resolver;
  `sensitivities_resolved` is gone.
- The full stackup over an assembly waits on the box driver carrying a
  resolver, which is filed to flux.
