---
id: registrations-sealed-inside-frozen-compounds
kind: issue
title: A registration cannot reach an identity sealed inside a frozen compound; discharge depends on which side of the term cap a spelling lands
status: open
opened: 2026-09-30
---


Found by PATHS 5a (`retire-the-stored-bulge`, PR 3527): the rounded pad
(`m10_9_pins_interval::m10_9_no_registrant_lies_on_any_measured_document`)
reads `registered` 122 and `numeric` 1072 there, against main's 128 and
1066. `symbolic_zero` is 885 on both.

**The six decisions.** Shapes #331/#333, #551/#553, #1584 and #1625 of
the pad's shape report at its certifying box are
`carrier_endpoint_start` (four) and `carrier_matches_mapped_source`
(two). Each is Zero on both sides: `Registered` on main,
`NumericZero` at 5a. The residual is ‖carrier(0) − q_from‖, and the
q_from node is the same node on both (`#c784c98c`).

**The cap.** `Poly::mul` in `crates/geom-core/src/sym/form.rs:219-229`
returns `None` (a freeze) when `terms(a)·terms(b)` exceeds
`budget.max_terms`, which the replay sets to
`editor_core::drive::DEFAULT_SYM_MAX_TERMS = 4096`
(`crates/editor-core/src/drive.rs:289`).

**Main.** The arc's centre comes from `arc_carrier`
(`center: frame.mid + frame.normal * apothem`, `crates/profile/src/seg.rs`)
over the REVERSED chord: main's `ProfileLoop::reversed` re-lowered the
arc. Its x is Add(mid.x 53/11 terms, normal·apothem 272/96). Over one
denominator that is 53 × 96 = 5088 pairs, past the cap, so centre.x
freezes to the atom `?#d1542898`. The carrier start is then
atom + r·u_ref (13 terms), and the rim registration discharges it.

**5a.** `ProfileLoop::reversed` copies the forward lowering's carrier
(`Segment::Arc(arc) => Segment::Arc(arc.reversed())`,
`crates/profile/src/lib.rs:731`; the centre is built at
`crates/profile/src/seg.rs:222`). Its mid.x is 37/5 terms, so the
largest product is 37 × 96 = 3552 pairs, under the cap, and centre.x
stays live (1774/204). The freeze lands one level up, on
carrier(0).x = Add(1774/204, r·u_ref.x 1722/231), node `#4355cef4`.
The c + (q − c) cancellation and the registered rim sit inside that
frozen node, where no registration reaches.

**The observation.** The identity still holds and every verdict is
unchanged. What decides whether the registry discharges these six is
which side of the term cap the spelling lands on, and here the SMALLER
spelling lost the discharges.
