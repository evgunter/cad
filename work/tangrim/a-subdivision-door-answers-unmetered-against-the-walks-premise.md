---
id: a-subdivision-door-answers-unmetered-against-the-walks-premise
kind: issue
title: certified_subdivision's doc says every door that answers meters its walk, but the coplanar curve-pair subdivision answers unmetered
status: open
opened: 2026-11-01
priority: P3
cost: E
---


Found by the TANGRIM review of PR 6257 (`tangrim/coplanar-conic-touch`).

## The premise

`crates/geom-brep/src/half_angle.rs`, `certified_subdivision`'s docs, "Without a meter". Without a `RootSlack` meter, a root is only a scalar whose residual reads ON beside a sign change. The docs add that "no answer rests on such roots: every door that answers meters its walk".

## Where it does not hold

`crates/geom-brep/src/curve_pair/pose.rs` `subdivided` answers on the walk's roots with no meter, so the coplanar circle × ellipse and two-ellipse poses do. On `main` before PR 6257 it called `certified_subdivision(…, None, band)`. Since 6257 it calls `certified_subdivision_touching`, which takes no meter. Its roots become candidates, and the meeting reader reads each one on both spans (`Span::contact`). That reading may be what makes the roots safe, but the docs do not say so.

## Places to check

- `crates/topo/src/boolean/ellipse_torus.rs` `torus_walk`: takes the meter as an `Option`. Check that only its rows pass `None`.
- `crates/topo/src/boolean/section_cert/ruling.rs`, the `certified_subdivision` call: the docs call it the ruling charts' uncharged reading, which only chooses a refusal's row. Check that no answer rests on it.

## What is owed

Either the docs name the curve-pair reader's `Span::contact` reading as what stands in for the meter, with the argument, or `subdivided` meters its roots. Not fixed in 6257.
