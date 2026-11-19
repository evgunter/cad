---
id: a-joins-periodic-far-end-is-read-off-a-foots-uncertified-parameter
kind: issue
title: edge_join::read_far takes a periodic cubic's whole-carrier foot parameter as a far end's parameter, which no reading certifies
status: open
opened: 2026-11-17
priority: P2
cost: E
---


Found merging main into fork 275's step 3b (`onehome/span-foot`), which retires the sampled whole-carrier foot search.

`crates/topo/src/boolean/edge_join.rs` `read_far` (from SECTARMS' `d48d7ed1ce`, "a join reads a periodic far end by its foot") inverts a far end on a periodic cubic carrier as `turn.near(carrier.foot(q, from).seed(), from)`, twice: the middle of the gone edge, seeded at `tw`, then the far end seeded there. It returns that parameter as the far end's.

A foot's parameter certifies nothing (`geom::Foot::seed`, the `on_carrier` module docs): it is a polished stationary point, which on a spline can be another branch's. The parameter is the point's only where `OnCarrier::On` certifies it (the gap in the zero) or the kind makes the foot nearest, and a periodic cubic is neither. Before step 3b the sampled search made a far branch less likely; after it, the foot is Gauss–Newton from `from` alone.

**Owed.** Read the far end's parameter off a certificate: `Foot::on_carrier`'s `On`, which reads the gap and restarts once from the Off's first kept piece where the seed's gap does not decide zero, and refuse where it does not read `On`; or hold the foot to the gone edge's span (`Curve3::foot_within`). The census row in `geom` (`a_foots_seed_is_read_only_where_it_seeds_a_search_or_a_bound`) lists the read until then.
