---
id: a-symmetric-parts-seam-is-not-walked-as-one-construction
kind: unit
title: The coincidence walk stops at a mirror, so a symmetric part's seam is glued by value where D10 says it is one construction read twice
status: review
opened: 2026-10-22
priority: P1
cost: M
branch: intent/mirror-seam-walk
pr: 5616
refs: [a-mirror-is-a-construction]
---


Found by `a-mirror-is-a-construction` (2026-10-22). D10 `[repetition]` says a symmetric part, `union(body, Mirror { body, plane })`, is "one construction whose seam is the plane's face read twice". Today the union glues the seam by value: DM4's pairwise pass meets two coplanar faces and fuses them (`crates/editor-core/tests/a_mirror_is_a_construction.rs`, `a_body_and_its_mirror_across_its_own_face_union_to_one_solid`; the `symmetric_plate` corpus document). The coincidence door's walk does not reach it: `coincide::construction` (`crates/editor-core/src/coincide.rs`) follows `Part` and `Split` from a name to its minter, and stops at a mirror (`Unwalked::Through`), since a mirror carries its source's names but reflects their geometry (`names::VerbatimEdge::Reflected`).

The open question is what the walk should say through a mirror. The image of a face lying in the mirror plane is that face with its normal reversed, so a mirror across a face's own plane carries that face onto itself, and the two reads meet as one construction. Every other face's image is a new construction. Weigh it with COINCIDE, whose door it is.

## Decided (2026-10-23, designer pair; INTENT orchestrator)

A blinded designer pair weighed this; the record is on `analysis/design-fork/intent-mirror-seam`. Both recommend the same final state (both "likely"), and it changes no ratified decision. `[coincidence.structural]`'s first rung already reads cells "directly or as projections of one construction", and D10 `[repetition]`'s sentence stands (Ev approved it on PR 4341). So this is built, not put to Ev.

- **The walk follows a mirror.** A mirror is one more link on the walk's chain of maps, keyed by its node, beside placements. Rename the chain from "placements" to the maps the read passes. A cell read through `Mirror { body, plane }` is its source's construction with that link appended. Two image cells of one mirror compare equal; an image cell and its source do not.
- **A reflection fixes its plane.** Where the carried cell's construction is the plane's own construction, the mirror's link drops out. The image of the plane's face is that face read again, as an opposite-sense pair. The same holds for edges and vertices in the face's closure, as the source built it: generalise `on_its_cap` to "lies on the mirror's plane". Every other image is a new construction, which the symbolic rung decides as it decides any coincidence of two constructions.
- **A plane read off a face is a projection of that face's construction.** `Plane { face }` walks through to the face's construction, and `Flip` is transparent. Planes built otherwise (a frame's `Xy`, a `Slide`) keep a pose origin. A split's `Origin::Tool` uses the same function, which closes the gap where a split's section face and the face its tool was read from compare unequal at rung 1.
- **Exhaustive over the names.** `coincide::construction` becomes an exhaustive reader of `names::VerbatimEdge`, which today it matches with `_ => Through`. That catch-all is how the mirror slipped past silently. The `VerbatimEdge::Reflected` doc ("no walk that reads a pose follows this edge"), the names README's N1 table row and N6 ("placed or reflected") are re-worded with the change. No clause changes what it decides.
- **The corpus fixture is wrong.** `test_support::plane_on` takes the first planar face with the requested normal and slides its flipped plane to the point. For the `symmetric_plate` document it reads the +x wall slid by 2, not the −x face its doc comment names. So its seam is a value coincidence even after the fix. Prefer the face at the point (slide zero), and have the corpus read its seam face plainly.
- **Measured today:** on the symmetric plate, the seam face pair and its 4 seam vertex rows are Unproven, with every image cell `Unwalked::Through(mirror)`. The seam's replay is `Numeric`, with recourse `Budget`.
- **Rows:**
  - (a) the plate mirrored across `Plane { face: its own −x cap }`: the seam pair and its vertices are proven by rung 1, with no replay;
  - (b) the corpus document after the fixture fix;
  - (c) a C-shape whose second prong is coplanar with the plane by value only: it stays a finding;
  - a split across a body's own face plane, whose section face is one construction with that face.
- **Not in this unit:** (c)'s recourse naming the wrong lever (`Budget`) is COINCIDE's `the-door-names-its-residual`. Perpendicular walls in the slid-plane scene that do not prove at `Sym` are COINCIDE's too, if they still fail after the fixture fix: the implementer measures and files.
