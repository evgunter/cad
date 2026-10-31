---
id: lies-on-beyond-the-extrude
kind: issue
title: The coincidence walk's lies-on reads a wall's struts and cap vertices only for an extrude of the document itself: not for a revolve, loft or sweep, nor inside a part
status: open
opened: 2026-10-23
priority: P3
cost: M
---

Found by `a-symmetric-parts-seam-is-not-walked-as-one-construction` (PR 5616, 2026-10-23), raised by its review.

`coincide::lies_on` / `on_face` (`crates/editor-core/src/coincide.rs`) decide whether an edge or a vertex lies on a face as their minter built it. Two places use that answer:
- the mirror link: a reflection fixes the cells on its plane's face;
- the mate statement: `stated_by_a_mate`'s `other_on`.

**What it decides by name alone, for any minter:**
- a rim edge or cap vertex lies on its cap;
- a rim edge lies on the wall over its run.

**What needs the profile.** A strut (`LateralEdge`) or a cap vertex lies on a wall whose run it bounds. That is read off `ProfilePieces::bounds`, through the profile of an `Extrude` of the document itself. So the walk answers `false`, a new construction, for:

- **a revolve, a loft or a sweep.** Their walls, bands, seams and meridians each have their own closure rules (`Band`/`BandRim`/`Meridian`, `LoftWall`/`LoftSeam`, the sweep's laterals). A mirror across the plane of a revolve's wedge cap, or of a loft's planar wall, leaves those seam rows to the replay.
- **any entity inside a part** (`InPart`). The part's profile lives in the part's document, which the walk does not hold; only the name-alone relations apply there. A mate whose wall face should state the struts and cap vertices on the other copy's wall therefore states only its rims.

All of this is sound, since nothing is proven falsely, but it leaves rows unproven. Extending the cases needs closure rules per minter, and, for parts, a way to reach the part's pieces: the resolver, or pieces published with the instance.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to COINCIDE's charter (the coincidence door and its lint). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
