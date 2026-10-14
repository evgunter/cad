---
id: boolean-param-near-sites-take-ellipse-edges-with-no-operand-reaching-them
kind: issue
title: the four boolean param_near sites now proceed on an ellipse edge where they refused, and no operand in the tree reaches them
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Filed by SHELL's `shell/lofted-wall-seam` lane (PR 4117), from its
review's Q6. That PR gave `geom::Curve3::param_near` a closed-form
ELLIPSE arm (it answered `None` before). Four boolean call sites read
`None` as a refusal or a skip, so on an ellipse edge they now proceed:

- `crates/topo/src/boolean/carrier_cross.rs` — `BoundaryCrossing::Unread` on `None`;
- `crates/topo/src/boolean/reduce.rs` — the continuation test's `sampled` and `on` closures;
- `crates/topo/src/boolean/reduce.rs` — the point split's `PointSplitCarrierUnsupported`.

Each consumer gates the answer (`on` and the split re-evaluate
`eval(t)` against the point; `carrier_cross` re-checks interiority and
containment), and the arm is exact for an on-carrier point. Off the
carrier it answers the point's SCALED polar angle, not its foot, which
`on` reads as "not on" — the right verdict, by a different route.

**Unmeasured**: the review lane instrumented the ellipse arm over the
geom, topo, sweep and editor-core suites and both demo roots, and no
boolean operand reaches it. So the change is latent: no row says what
an ellipse edge does at these sites now.

## Fix

A boolean operand with an elliptical edge (a plane cut obliquely
through a cylinder) through each of the four sites, pinning what they
now answer; or a decision that the sites keep refusing ellipses, with
an explicit arm.
