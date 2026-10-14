---
id: a-pair-walk-end-at-a-domain-corner-refuses-undecided
kind: issue
title: ssi/four: a pair-walk carrier ending at a domain corner refuses Undecided, because face_roots resolves no corner root
status: open
opened: 2026-10-14
---


(Found at the FULL review of PR 5094, `nurbs/pair-walk-domain-side-end`.)

## What

`ssi/four.rs` `one_arc_pair` asks each box for exactly two boundary roots (`face_roots`) before it reads either end (`reaches_end`). A locus that enters the domain through a corner of a chart has its root on two of the box's faces at once, on their shared edge:
- a face walk can never isolate that root by Krawczyk on either face;
- the walk runs to `EXIT_DEPTH` and returns `None`;
- `one_arc_pair` therefore refuses `Shortfall::Undecided` before `reaches_end`'s root reading (PR 5094) is reached.

The refusal is conservative, not unsound.

**Probe**, measured on the PR 5094 branch:
- the walls are the `four::tests` bilinears `(u, v) ↦ (0, u, v)` and `(u, v) ↦ (u − ½, 2v − ½, 2v − ½)`;
- they cross on the diagonal `u₁ = v₁`, from the first chart's corner `(0, 0)` to its corner `(1, 1)`;
- the ends are `[0, 0, ½, ¼]` and `[1, 1, ½, ¾]`, at axis 1.

Over the one box that is the whole domain, `face_roots` gives `None` and `one_arc_pair` gives `Err(Undecided)` at ε = 1e-9.

## What is open

A corner root needs a reading of its own: the edge where the two faces meet, held at two coordinates, is two equations short of the root's three. Equivalently, the walk can count a root on a shared edge once rather than failing on both faces. Whether this is worth building waits on a consumer; no current row reaches a corner end.
