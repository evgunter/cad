---
id: touch-sites-read-no-side-for-an-edge-on-a-ruled-face
kind: issue
title: Arm 2's touch sites read no side for a line edge one of whose faces is a cylinder or cone it is a ruling of
status: open
opened: 2026-11-03
priority: P3
cost: M
---


Found by PR 6415's sweep (`restread/edge-on-ruling-in-face`).

## What

PR 6415 reads an edge lying in a planar face of its **own** solid, where a face of the edge is a cylinder or cone the edge is a ruling of, by that face's chord across the edge (`census::Link::chord_on_edge`, `link::FaceSector::chord`, `link::chords_clear`). The census's touch sites between two solids (`census.rs` `TouchSite::links`) still build such an edge's link with `Link::touch_on_edge`. That refuses `TouchUnread::Unreadable` for the ruled face, in three arms:

- **`EdgeInFace`** has the same shape, a line edge against one planar face. The chord argument applies as it stands.
- **`EdgeEdge`** and **`EdgeCross`** hold the other link as an edge, two sheets. The lens would have to lie on one side of each.

**Measured** (default ε, on the PR's branch): a tip of a separate solid, its wall an arc leaving the floor across it or tangent to it, resting on a slab (`union` builds 2 solids). The census returns `Ok(())`: the touch is answered before the link reader reads it, so no refusal from these arms has been observed. A crossing across two solids is no Boolean result, and no body has been built that reaches the link reader here.

## Fix

First find a body that reaches these arms (a refusal measured, `memories/refusal-text-is-not-cause.md`). Then read `EdgeInFace` through `chord_on_edge`, and give the edge-edge arms the lens held against both sheets, or a typed refusal naming it.
