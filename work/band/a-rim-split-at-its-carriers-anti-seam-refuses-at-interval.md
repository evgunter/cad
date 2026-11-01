---
id: a-rim-split-at-its-carriers-anti-seam-refuses-at-interval
kind: issue
title: blend: a foot at a curved rim's anti-seam reads the whole turn at Interval, so split_param_in_span refuses it as off-span
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by the class sweep of
`an-interval-fillet-cut-off-at-an-oblique-end-reads-its-arc-backward`.
Suspected, not measured: no row reaches it yet.

`crates/sweep/src/blend/surgery.rs` `split_param_in_span` reads a
foot's parameter on its rim's carrier at the seam,
`param_near(target, T::zero())`, deliberately (its docs: the answer
must be a function of the point and the carrier alone). On a circle
or ellipse rim at `Interval`, a foot half a turn from `u_ref` puts the
`atan2` box across its branch cut, which reads the hull `[−π, π]`;
neither that nor its `±τ` neighbours is strictly inside the stored
span, so the plan refuses the foot as off its rim
(`FOOT_INSIDE_A_FACE`) where `f64` builds.

`surgery.rs` `arc_window` is the shape of a fix that keeps the
function-of-point-and-carrier contract: re-read at the half turn when
the seam's read spans half a turn or more. The measure-first step is a
witness: a cut-off whose foot lands on a cylinder's or a sheared
prism's curved rim at its anti-seam, at `Interval`.

`surgery.rs` already has three ways of moving a parameter read off the
wrong branch: `arc_window`'s second anchor at the half turn,
`split_param_in_span`'s `±τ` shift into the stored window, and
`seam_split_param`'s branch shift. The fix here should share one
helper with `arc_window` rather than add a copy of its re-read.
