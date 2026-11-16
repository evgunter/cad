---
id: a-lens-of-the-wrong-kind-for-its-region-refuses-instead-of-crossing
kind: issue
title: rest::lensed refuses a lens of the wrong kind for its region, a decided overlap, instead of crossing
status: open
opened: 2026-11-12
priority: P3
cost: M
---


Found by the remainder-refuses lane (PR 6854). Priority P3: it is a
refusal that could be a crossing. It is sound either way (nothing
passes), and no measured body reaches it.

## What

`rest::lensed` (`crates/topo/src/link.rs:5227`) reads each lens's or
lune's middle against the remainder's material. A void lens must read
inside the material, and a sliver outside it. A pair of the wrong kind
for its region returns `Curved(Unreadable)` (`_ => return unread`,
`crates/topo/src/link.rs:5328`).

The reading has decided an overlap there. A void cut out of a region
that is already void (or a sliver added to material) winds a third
value beside the remainder's two. That is the same argument PR 6854
writes for a thorn read inside its sectors' material and for a nappe on
the wrong side (`crates/topo/README.md`, "A thorn read inside the
material is a crossing").

## The fix

Where the remainder reads one material and the middle's region is
decided, report `TouchVerdict::Crossing`. Write the argument as a README
clause beside the lens's, with a refusing counterpart row, as PR 6854
does for the thorn.
