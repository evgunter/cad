---
id: the-guided-derived-frame-extrude-escalates-as-a-kernel-defect
kind: issue
title: Under the guided profile lift at Sym<Interval>, the derived-frame document's extrude refuses its endpoint certification as a kernel defect (enclosure [0, 0.32])
status: open
opened: 2026-10-17
priority: P2
cost: M
---


Found by INLANE's pinned-lift unit (ruling A, branch `inlane/pinned-lift-derived`, f9a5e0b65f). It is filed here because `crates/sweep/src/extrude.rs` is carve's and strut's territory.

`crates/editor-core/tests/m10_derived_frame_interval.rs`, `m10_the_derived_frames_refusal_is_not_a_freeze`, evaluates the derived-frame document at `Sym<Interval>` over its analysed box with every rule off. Under the pinned profile lift (main's default), the row's own account holds. The gate residual is a plain theorem, and what refuses is the boss's side plane, where the value channel's `newell_plane_residual` is `Invalid` (clause 1).

Under the GUIDED lift, the same document refuses earlier, at the boss extrude's Euler-op certification:

> node 50 — the extrude op refused: an Euler operation refused: geometry attachment gate: the start-endpoint residual at sample 0 escalated: enclosure [0e0, 3.2025529370240113e-1] cannot be classified against the ambiguity band (1e-9, 1e-8). There is no way through: this is a kernel defect; report it — `Extrude(Op { source: Certification { error: Escalated { check: EndpointStart, sample: 0, cause: Indeterminate { margin: MarginDiag(Enclosure { lo: 0.0, hi: 0.32… } …`

Under the guided lift the profile is elaborated at the lane, so the boss's profile is a function of the box's variables rather than its nominal `f64` form. The guided lift is what every analysis takes once ruling A lands (`work/inlane/the-pinned-profile-lift-launders-at-sym.md`).

Two things are wrong:
- **The enclosure.** The start-endpoint residual's enclosure is a third of a unit wide over a box whose variables span `±5e-2` at most. That width is the dependency problem or a lost correlation in the curve-on-surface endpoint residual (`crates/geom-brep/src/certify.rs`, `EndpointStart`), not the box.
- **The wording.** An escalation reachable by input is worded as a kernel defect ("There is no way through").

To reproduce on main: set `profile_lift: ProfileLift::Guided` in that row's `sym_failures_in` options (`crates/editor-core/tests/m10_derived_frame_interval.rs`).

Owed:
- Find where the endpoint residual's enclosure loses the box: is it the frame's landed placement, or the boss profile's carrier at the lane?
- Make it decide, or refuse with recourse.
- Pin the row under the guided lift.

## Measured on the rungs (pinned-lift unit, after PR 5247)

The row now pins this refusal under the guided lift. The two rungs differ:

- **No rules (`SymRules::none()`):** the escalation above, with **12 decisions frozen** on the way (SymCounts: symbolic 480, numeric 314, checks_frozen 2).
- **A0 alone (`const_fold`):** **nothing frozen**. The document reaches the boss's side plane and refuses at `newell_plane_residual` (clause 1, an invalid margin), as the row always said.

So the [0, 0.32] enclosure appears only on the rung that freezes. The first thing to read is whether the endpoint residual's enclosure comes from a frozen form's numeric fallback. If so, the defect is the freeze reaching an attachment gate as an enclosure this wide, not the frame.

## No longer reached under INTENT E

Since stage 3 E (`intent/e-operation-frame-of-reads`) a copy shares its
source's body under its frame, so the derived-frame document's boss is
never rebuilt over the box and the endpoint certification that
escalated over `[0, 0.32]` does not run:
`m10_derived_frame_interval::m10_the_derived_frames_refusal_is_not_a_freeze`
now reads no refusal on the plain tier. The kernel's escalation itself
is unmeasured by any document now.
