---
id: moves-of-drops-the-third-angle-near-gimbal-lock
kind: issue
title: moves_of loses up to ~3e-8 rad near gimbal lock (cos β ≤ √ε falls back to γ = 0), so a duplicate or a typed frame lands off its copy by more than ε at 5 cm
status: open
opened: 2026-10-25
priority: P2
cost: E
---


Found by PR 5693's review (sure, measured). `moves_of` (`crates/viewer/src/session.rs`) decomposes a rotation into spins. When cos β is in (0, √ε ≈ 1.49e-8], it takes the gimbal fallback and sets γ = 0, dropping the true γ. Measured gaps between the written motion and the copy's: 2.0e-9 at β = π/2 − 1e-9 (γ = 3), 2.0e-8 at π/2 − 1e-8, and 2.4e-8 at −π/2 + 1.4e-8. At 1e-9 m ε, a point 5 cm from the origin is drawn off by more than ε. A copy solved by a mate to within about 1.5e-8 rad of β = ±π/2 is plausible.

**Callers:** `frame_off` and `gesture_moves` (INTENT's `the-viewers-numeric-gestures-read-a-frame-back-off-geometry`), and since PR 5693 the duplicate's `duplicate_seats`, through `combine::copy_motion`. The same decomposition is written out verbatim in `editor_core::test_support::euler_turns`, which `moves_at`-pinned fixtures share; one home for both is part of the fix.

**Fix (the reviewer's):** in the fallback, keep γ = atan2(r10, r00) and solve α from Ry(β)ᵀRz(γ)ᵀR, so the band is exact. Add rows at β = π/2 − 1e-9 and −π/2 + 1.4e-8 that read the drawn copy. Also tell INTENT's row that the duplicate is now a third caller.

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.
