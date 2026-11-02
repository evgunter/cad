---
id: the-cones-parity-arc-refuses-where-the-flanks-side-is-decided
kind: unit
title: Cone::reads_given's parity arc requires each end to clear the q-w bisector, costing cos(θ/2) over the side reading: a fin t in (Kε, √2·Kε) refuses where its side is decided
status: closed
opened: 2026-11-02
priority: P1
cost: M
branch: oneread/link-arc-half-turn
pr: 6337
closed: 2026-11-02
---


Found by `resolve-edge-sector-reads-the-other-solids-face-about-the-line-by-table-ii`'s lane (2026-11-02), at its decision diff.

`crates/topo/src/link.rs` `Cone::reads_given`, the reference loop: the reference w is carried to q's radius (`w = p + c·|q−p|/|c|`), and each arc end must clear `LINK_ARC` = (x−p)·half, `half` the bisector of q̂ and ŵ, i.e. |q−p|·cos(θ/2). Where the cone's only wedges are one plane's two halves (a line cone of a face the line runs inside), every reference stands 90° from a q square to that plane, so the arc's margin is t·cos 45° while q's side of the plane clears with margin t.

Witness: `crates/topo/tests/sliver_shell_role.rs`, `fin(t)` under Subtract, read through `resolve_edge_sector` on the line cone: t = 10.5, 11, 12, 13, 14 ε refuse `Sectors` with `link_arc` margin exactly t/√2 (7.4246e-9 at t = 10.5ε, ε = 1e-9; and at 1e-6), where main builds them (the test's ABOVE_K requires a build); 14.5, 15, 19 ε build.

The lane's observation: for a reference in the base sector (w on sector t's plane) with q's side of that plane decided, q and w cannot be antipodal, so the half-turn condition follows from q's decided side; the check may be stricter than the reading's soundness needs.

Owed: the arc check no stricter than its soundness argument (e.g. the half-turn condition taken from the base sector's decided side, or each end read at its own reach), argued at the site; `cone_fuzz` 0 wrong at three ε with read-nothing counts reported before/after; a decision diff over every reader of `Cone::reads_given`/`reads_tied` (the census's touch analysis, `vtxfac::cone_classes`, recl's edge-edge) at three ε, each changed decision explained; the fin witness pinned. Tier: dual (it changes the one material reader every consumer reads).

## Closed (2026-11-02)

Merged as PR 6337: `link::ArcReach` reads every reference at q's distance first (main's reading) and, only where none reads, again at its wedge's reach, so the change is never stricter by construction; `winds` alike. The fin witness reads. Dual review, adjudicated; record `docs/dual-review-log/rows/PR-6337.md`.
