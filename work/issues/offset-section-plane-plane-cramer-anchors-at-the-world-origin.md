---
id: offset-section-plane-plane-cramer-anchors-at-the-world-origin
kind: issue
title: offset_derive::plane_plane solves its three-plane Cramer system from offsets to the world origin, so the section line sits about u·|d|/s off both planes (past ε at small tilts)
status: open
opened: 2026-10-19
---


## The finding

Found in review of PR 5190 (`cleave/circle-pair-touch-on-both`, closed
unmerged: FUSE's curve-pair reader replaced `carrier_cross::common_line`,
which had the same class).

`offset_derive::plane_plane` (`crates/topo/src/offset_derive.rs:632-669`)
places the section line of two planes by Cramer's rule on three planes:
the two given and one across `dir = n₁ × n₂` through the old middle. Each
plane enters as its offset from the world origin, `dᵢ = nᵢ·(qᵢ − o)`, and
the solve is `x = (n₂×dir·d₁ + dir×n₁·d₂ + n₁×n₂·d₃)/det` with
`det = |n₁ × n₂| = s`.

- The numerator's three terms are each about `|d|`, so they round by
  about `u·|d|`, and dividing by `s` leaves the line about `u·|d|/s` off
  both planes.
- **Measured (the reviewer's figures), `|d| ≈ 10`:** `1.1e-14` at
  `s = 0.1`, and `9.5e-9` at `s = 1e-7`, which is past ε at `1e-9`.
- The parallel check above it reads `s·extent` against the band, so any
  tilt past the band reaches the solve.

## What it needs

Anchor the solve at a point of the problem rather than at the world origin:

- take offsets from the old middle `mid`, so `d₃ = 0` and `d₁`, `d₂` are
  the middle's heights over the two planes;
- solve for the displacement from `mid`;
- add it back.

The line is then off each plane by `u` of those heights over `s`. Those
heights are near zero wherever the old section was nearly on both planes,
which is the offset case. A ci row should pin a pair of planes about
1 km from the origin at a small tilt, at three ε.

Ownership: `offset_derive.rs` is in no program's territory. Its section
lane is read by SHELL's replace-face
(`shell/replace-face-trusts-route-pose-for-pairs-section-closed-refuses`).

**A sibling with the same shape:** `offset_together::cramer`
(`crates/topo/src/offset_together.rs:645-647`). Before it solves, it
decides `|det|·arm` against the band (`:596-602`), which bounds the
amplification at about `u·|d|·arm/(Kε)`, but not at `ε`. Not measured.
