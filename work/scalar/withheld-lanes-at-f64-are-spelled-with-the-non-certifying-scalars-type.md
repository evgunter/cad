---
id: withheld-lanes-at-f64-are-spelled-with-the-non-certifying-scalars-type
kind: issue
title: None::<NoLane> in production at f64 spells a withheld lane with the type a non-certifying scalar holds
status: open
opened: 2026-11-20
priority: P3
cost: E
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


From the review of PR 7190, 2026-11-20.

## What

geom-brep's passes take a lane as `Option<impl SsiLaneOf<T>>` or `Option<impl FittedLaneOf<T>>`. A caller that withholds the lane writes `None::<geom_core::NoLane>`, because a bare `None` no longer names a type. In production at `f64` that is four sites: `crates/geom-brep/src/intersect.rs` (`route_pose`'s torus arm), `crates/geom-brep/src/pcurve_cache.rs` (`PcurveCache::certify`'s projected arm), `crates/topo/src/boolean/node.rs` and `crates/topo/src/boolean/section_cert.rs` (`classify`); about fifty more are in tests.

`NoLane` is documented as "what a scalar that does not certify holds". At `f64` the caller certifies and chooses not to pass a lane, so the type says something untrue of the call.

## What it asks

A spelling for "withheld" that is not the non-certifying scalar's type: a named constant per lane (`SsiLane::<T>::WITHHELD: Option<…>`), or the passes that never use a lane on that route split from the ones that do, so nothing is withheld. topo's passes take `Option<T::…Lane>` and a bare `None` reads there.
