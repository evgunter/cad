---
id: the-line-cone-reads-a-flanks-side-beside-the-one-bound-side-reading
kind: issue
title: The line cone reads a flank's side of a face plane at first order (link_plane) beside the one bound-side reading (bound_read, with its descent)
status: spec
opened: 2026-11-02
priority: P1
cost: M
refs: [recl-edge-edge-composes-the-other-solids-material-beside-the-cone, the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways]
---


Found at the merge of `resolve-edge-sector-reads-the-other-solids-face-about-the-line-by-table-ii`'s lane with main after TANGRIM's PR 6253 (2026-11-02).

Since PR 6253, a bound's side of a face has one reading in the kernel: `boolean::sectors::bound_read` (TANGRIM's `the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways`), first order charged, and one order down where a bound ties at first order. `resolve_edge_sector` keys its flanks through it (`flank_keys` / `flank_read`).

The line cone at `recl::resolve_edge_edge` (PR 6282, `boolean::sectors::line_fans` read by `link::Cone::reads_tied`) reads each flank's side of the other solid's flank planes through the cone's own `link_plane` distance, at first order: a second reader of the same side, which ties where `bound_read` descends. A held face's side already enters the cone given (`reads_given`, from `PierceDatum::side`); flank sheets do not. TANGRIM's log (2026-11-01/02) already says stage 2's descent "belongs in the line cone" there.

Owed (a design question — weigh before a lane builds it, with TANGRIM): one reading of a bound's side of a face at both recl sites. Candidates include giving every flank sheet its side from `bound_read` (the cone then reads parity over given sides, and `Cone::crossing` must be consistent with a given side, as PR 6247's F5 noted), or making `bound_read` the cone's plane reading for bound points. A witness: an undeclared curved flank pair whose bounds both tie a reference plane at first order (main decides it by the descent at the edge-sector site; the edge-edge site's cone reads it On).

## Decided (2026-11-02, ONEREAD orchestrator, on a converged two-designer weighing)

Decided without Ev: the designers converged after two reconciliation rounds (their mechanisms crossed in round 1 and met in round 2), and no ratified text changes. Draw, reports and rounds: `analysis/design-fork/oneread-line-cone-side`.

Premise corrections: the duplicate is narrower than two readers — the cone's `link_plane` reading of a bound's point and `bound_side::first_order` are the same first-order arithmetic under the same band; they differ in the charge (`BoolSector::datum()` carries `NO_CURVATURE`, so no sector reading is charged today), the descent and the K names. And it reaches a third site the row missed: `vtxfac::cone_classes`' partner fans read by `link_plane` too.

The final state:

1. **The boolean reads every bound's side of a face through the one bound-side reading** (`sectors::bound_read` → `bound_side::read`, with its descent) at all three sites — recl's edge-sector (as now), recl's edge-edge line cone (each sheet of the other solid), and `vtxfac::cone_classes` (each partner corner, and the touched face) — and hands it to the cone; **the cone reads crossing parity** over what it is handed. The cone's own `link_plane` reading of a probe remains for point probes only (the census's `Link` and the fuzz). One owner of a bound's side, one call shape at every site.
2. **One `Reading` for both orders.** `bound_side::read`'s first-order sign and `Tie::descend`'s parting are both decided through `link::metric` on a `link::metric::Reading` the caller passes: at a cone site the cone's own (`Cone` exposes it, e.g. `cone.reading(band)`, so no consumer re-spells `Reading::tied(band, tie)`); the plain band at splitting, the pierce and the edge-sector site. The descent must clear band + slack: worked by both designers, a parting decided without the slack can take the wrong sign at K ≤ 2 (an apex 0.9·zero off, a parting 1.5·zero at K = 1.4), and a slope hidden in a 1e-7 tie can read the wrong side at any K. Vacuous at K ≥ 3 for the apex case; binding below.
3. **The cone admits a given side for any sector, under one rule**: the cone's metric decided the same sign → that sign; Zero, in band or slack-refused → the given side is taken, the crossing placed by the existing arm; decided the opposite sign → a typed refusal (a new `Unsettled` arm, e.g. `Contradicted`), never an invariant. `WholePlane`/`whole_planes` go. A sector read only by a given side is no parity reference base where the metric did not decide it. Where q lies on the plane of exactly one sector, strictly inside it, and that sector's side was given and taken, q's material is that side's (behind the sector In, in front Out), with no arc: near a point strictly inside one face the material is that face's half-space. This is item 4's On(k) read to the orders read. *Amended 2026-11-02 by the orchestrator on lane 2's measurement:* without it, a tangent kiss whose line cone is one plane in two halves has no decided base and refused (13 tests that build on main). Under it all 13 build exactly as main does, and every moved reading agrees with `point_in_solid`. The rejected alternative let a given-taken sector serve as the base where none decided exists; it left one cone, every sector in q's plane, unread. Every sector is read before any answer, as now.
4. **`On(k)` now means "rides the face to the orders read, or unresolved"** (recl's coplanar ladder unchanged); item 2 of `recl-edge-edge-composes-the-other-solids-material-beside-the-cone`'s `## Decided` narrows accordingly. `flank_keys`' lone-tie rule stays local to the edge-sector site (it routes Table II, not a side). The held face keeps `PierceDatum::side`'s lever within the bound's own reach, read on the cone's `Reading`.

Rejected: the cone reading bounds itself (sector datums, descent in `link`) — the same readings, but it puts a door `link` does not otherwise have into its one-door rule and a datum on every census `Link` sector for a reading the census never takes, while the edge-sector site reads `bound_read` directly anyway. Would be revisited if a census bound reader appears or the edge-sector site returns to a cone.

## Lanes

- **Lane 1** (`the-cone-takes-a-bounds-side-from-the-one-bound-side-reading`): items 2 and 3 in `link.rs` and `bound_side.rs` (+ the edge-sector site's `Reading` argument). Dual.
- **Lane 2** (`the-cone-sites-read-every-bounds-side-through-bound-read`): item 1 at recl's edge-edge and `vtxfac::cone_classes`, and item 4. Dual. After lane 1.
