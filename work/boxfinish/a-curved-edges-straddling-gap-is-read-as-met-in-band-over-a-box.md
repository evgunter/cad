---
id: a-curved-edges-straddling-gap-is-read-as-met-in-band-over-a-box
kind: issue
title: A curved edge whose carrier reading straddles the band is read as met in band; over a box that reading may hold members that pass it clear
status: open
opened: 2026-11-14
priority: P1
cost: M
---


Found by the aimed-ray unit's fix pass (PR 6958), sweeping for an
undecided reading turned into a fact.

**The shape.** `solid_contain::curved_edges::passes` reads where a ray
passes an edge a curved face bounds. Where the edge's carrier refuses
with an escalation it returns `Passes::Met(s, Gap::InBand(diag))`: the
ray meets the edge in band. A met edge stands for its faces' crossings
(`ray_crossings`), half a crossing each, and its faces offer none of
their own at that crossing.

**At `f64` this is a ruled, pinned reading.** A carrier's distance
reading can bracket the truth between a lower bound in band and a gap
past it (`geom::CarrierEscalation::Straddle`), and
`curved_edges::straddle_rows::a_straddle_is_in_band_and_past_the_span_is_clear`
pins that such a straddle is in band ("red when a straddle reads
undecided"). The bracket there is a few bands wide, so the half rule's
error is of the band's order.

**Over a box the same arm takes any enclosure.** The line-edge twin of
this arm (`edge_meets`, `Err(gap) => Gap::InBand(gap)`) read a gap
enclosed as `[0, 0.4]` as an edge met in band, dropped a cap's entry
and read `Side(In)` on a brick every member of which reads `Out`
(`side_reader::boxed_brick_rows::a_ray_beside_a_moving_silhouette_edge_reads_no_wrong_side`).
That arm now sets the ray aside unless the whole enclosure lies in the
band (`geom_core::ladder::in_band`). This one was left as it is:
applying the same test here turns the pin above red, because the
`f64` straddle is itself an enclosure not wholly in band, and which of
the two holds for a curved edge is a ruling on FLUSH's curved-edge ray
(PR 5697), not a repair.

**Not demonstrated.** No wrong side was found: the sweep doors refuse
every curved body over a box that
`sweep` `an_aimed_ray_over_boxed_door_bodies` draws (a filleted prism,
a disc, cylinders and a frustum of boxed radius or height), so no ray
reached this arm over a box in the unit. It becomes reachable the day
a door admits a curved body over a box.

**What would close it.** Either a width test on the escalation (met in
band only where the bracket's upper end is within a stated multiple of
the band, the ray set aside past it), with the `f64` pin kept and a
boxed curved row beside it; or the pin's rule re-argued for an
enclosure of any width.
