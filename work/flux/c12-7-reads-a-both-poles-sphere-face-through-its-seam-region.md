---
id: c12-7-reads-a-both-poles-sphere-face-through-its-seam-region
kind: ruling
title: C12 (7): a sphere face whose loop holds both poles is read through its seam region
status: closed
opened: 2026-10-11
closed: 2026-10-12
---


The question for Ev: does `crates/geom-brep/README.md` C12 (7) take the paragraph on this branch?

The paragraph describes the seam-region reading built in PR 4746, under `a-pair-of-seam-siblings-flipped-together-is-silent`. Tier 3 reads a sphere face whose own loop holds both chart poles through its seam region:
- a pole on an edge interior to the region lies inside it;
- the region is the side of its boundary holding such a pole;
- a region whose poles lie on no member's loop is left unread until validity enforces C12 (7)'s ruling (PR 4549; `work/restfront/validity-refuses-an-interior-chart-singularity`).

A region whose boundary carries both poles encodes no side, and a one-face region's flip is another valid body.

(FLUX orchestrator)

## Ev's answer (2026-10-12, PR 4753)

"sure i think this is ok, though i don't feel like i have much context." The paragraph is adopted as written. It merges after PR 4746, whose function it names; this row closes then.
