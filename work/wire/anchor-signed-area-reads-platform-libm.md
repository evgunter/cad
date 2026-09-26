---
id: anchor-signed-area-reads-platform-libm
kind: issue
title: The replay naming anchor's signed area computes through std's atan/sin, not libm
status: open
opened: 2026-09-25
---


`crates/editor-core/src/eval/anchor.rs::signed_area` (≈ l.360) computes each arc's
θ as `4.0 * b.atan()` and the circular-segment term through `sin` on bare `f64`,
which resolves to std's inherent methods — the platform libm — rather than
`geom_core::Real`'s `libm` crate. `geom-core/src/real.rs`'s
`libm_vs_std_divergence_census` documents why D9 mandates the `libm` crate: std's
transcendentals differ across platforms in the last ulp. The area decides
`replay_naming`'s loop order (largest area, ties refuse) and each loop's
orientation, so a last-ulp platform difference is a naming difference in a
measure-zero but reachable tie.

Found by PATHS `canonical-segment-type-in-profile`, which could not route the θ
through the stored sweep (libm) without moving these bits, and left it on std.
`work/paths/store-constructed-carriers.md` deletes this hand copy; whichever lands
first should read the stored carrier and sweep through `Real`.

## EMIT note (2026-09-25)

EMIT's PR 3223 deletes `signed_area`, `replay_naming` and `naming_of`,
along with the anchor code this row cites. A profile's names no longer
depend on its loops' signed areas. Whether this row is closed is WIRE's
call.
