---
id: normalize-overflow-yields-zero-axis
kind: issue
title: Vec3::normalize maps a norm²-overflow axis to the ZERO vector silently, and rotation constructors then disagree by metres
status: open
opened: 2026-08-30
github: 1299
refs: [1177, 1277]
---

## From GitHub issue 1299

opened 2026-08-30, 0 comments.

(S-CERT orchestrator) Filed from CERT-3's dual review (PR 1277, review lane finding, reproduced by the fix pass; `vec.rs` is PCURVE PR 1177's keep-out, so recorded rather than fixed).

`Vec3::normalize` is `self / self.norm()`. For an axis whose `norm_squared` overflows to `∞` — e.g. `(1e160, 1, 1)` or `(1e200, 1e200, 0)` — `norm()` is `∞` and every component divides to **`0.0`**. The result is the **zero vector**, not poison: no NaN, nothing that tests as degenerate. (The symmetric underflow case, `(1e-170, 0, 0)`, *does* poison — `|n|² = NaN` — so the two ends of the documented range behave differently in kind, not just in degree.)

`Mat3::rotation_about` then builds `R` on `n = 0`, giving `t·nᵢ² = 0` and `s·nₖ = 0` throughout: `R` collapses to `cos θ · I`, a **uniform scaling**, silently returned where a rotation was asked for.

This became observable because the two anchored-rotation spellings diverge there. Measured on anchor `p = (1, 2, −3)`, `|q| = √14`:

| axis | angle | \|old_t − new_t\| | anchor residual old / new |
|---|---|---|---|
| `(1e160, 1, 1)` | 1.0 | **1.72 m** | 0 / 0.4597 |
| `(1e160, 1, 1)` | 3.0 | **7.446 m** | 0 / 1.990 |
| `(1e200, 1e200, 0)` | 3.0 | **7.446 m** | 0 / 1.990 |

The retired `q − R·q` fixes the anchor exactly by construction (it is a difference, so a scaling map still returns the anchor to itself); `(I − R)·q` gives the zero matrix at `n = 0` and returns a zero translation, so the anchor moves by `|q|·(1 − cos θ)` = `3.7417 × 1.98999` = **7.4459 m** at `θ = 3`. Both are "correct" for the map `R` actually is; neither is a rotation.

`normalize`'s doc already notes the overflow collapse and calls the regime far outside the session box (D4 ¶4), so the posture may be deliberate — but "collapses toward zero" reads as a precision note, not as "returns a value that makes `rotation_about` a scaling". If the posture is ratified as deliberate, the ask reduces to: the zero-axis poison contract should catch the overflow route too (or the doc should say plainly that it does not). Classification per the D2 addendum is owed by whichever unit takes it.

Reproduction: `r2_cert3_probes::r2_anchor_fixed_point_under_degenerate_normalization` (adopted on PR 1277's branch).

## Home

`work/cert/` — `crates/geom-core/src/linalg/vec.rs` matches S-CERT's `crates/geom-core/src/*` territory glob, and the issue was filed by the S-CERT orchestrator from CERT-3's dual review.
