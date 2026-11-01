---
id: the-link-splits-into-a-cone-and-a-link
kind: unit
title: topo::link splits into a Cone (sectors, closes, reads) and a Link (a Cone of real face points, plus ball, offset, lowest_on, judge), and the parity reads On on a shared bound and wide past an in-band plane outside its sector
status: spec
opened: 2026-11-01
priority: P1
cost: H
parent: the-booleans-cone-reader-is-a-second-link
branch: oneread/link-cone
---


Items 1 and 2 of `the-booleans-cone-reader-is-a-second-link`'s `## Decided`, which is the spec. In `crates/topo/src/link.rs` and the census's use of it:

- The `Cone` / `Link` split: `Cone` holds the point and its sectors (`closes`, `reads`, the apex `tie`); `Link` is a `Cone` whose wedges are real face points, plus ball, `offset`, `lowest_on`, `rest::judge`, `Round`, `Sector::cleared`. The census and `Gap` keep their readings through `Link`.
- `Link::parity` reads `On` where a direction is decided on a face's plane and on a bound two sectors share (`Place::Bound`).
- The wide parity for every reading: a plane the direction lies in band of, decidedly outside its sector, is not crossed. This is item 2 of ATREST's `the-census-reads-its-link-without-the-wide-parity`; item 1 there (reading at the point's own distance, `Reading::own`) is not this unit's.

Review tier: dual (an architectural split of a public type four consumers read; broad and hard to change later).
