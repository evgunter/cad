---
id: a-part-resting-on-a-gauge-cannot-follow-a-part-edit
kind: issue
title: a part set on another part's gauge cannot follow that part's edit: a mate across gauges only declares, and a nested gauge restates the solved face by hand
status: closed
opened: 2026-10-02
priority: P3
cost: H
branch: place/mate-frame-offset
pr: 3961
closed: 2026-10-03
---


Found by SHOW's `bench-on-a-gauge` (PR 3840). It is pinned live as
walls 1 and 2 in `demos/tour/src/assembly.rs` (`update_door`). MSOLVE
should read it too: whether a mate across gauges may ever place is
A11 (2)'s question, and that clause is ratified, so changing it is an
`[ev]` conversation.

## What

The tour's bench stands on a turntable gauge. A crate set on the shelf
stands on a shelf-top gauge nested on the turntable, at `SHELF_TOP`,
which restates in numbers where the solve puts the shelf's top. Its
one contact with the shelf is a `Rest` mate. The crate and the shelf
sit on two gauges, so that mate DECLARES (A11 (2)) and solves for
nothing. When the shelf part changes, the crate cannot follow:

- **Wall 1.** A thicker shelf (`thickness` × 1.5, same underside)
  grows up into the crate. `assemble` refuses `AtRest` with 6 findings:
  - the crate's rest refuted;
  - four undeclared edge-through-face contacts at the crate's corners
    (z = 0.56);
  - one `InstanceInterference`.

  The posts' seats name the shelf's underside and hold.
- **Wall 2.** The same shelf on posts 40 mm shorter. The shelf comes
  down with the posts (their mates name the caps' faces), and the crate
  is left in the air. `AtRest`: the crate's rest refuted, and nothing
  else.

## Knobs varied before pinning

1. **Crate mate primitive.** `PlanarRest` and `FrameCoincidence`:
   both refused alike.
2. **Shelf-side frame.** Authored numbers and `MateFrame::from_face`
   of the shelf's top cap: refused alike. A declaring mate solves for
   nothing, whatever its frames.
3. **Shelf-top gauge's height as an expression over the shelf's
   `thickness`.** `parse_expr` refuses `UnknownParam`: an assembly's
   scope cannot name a part's parameter.
4. **Re-gauge the crate onto the turntable and mate it, placing, with
   the shelf side authored.** Still refused: authored numbers stay
   behind.
5. **Re-gauge onto the turntable and mate it, placing, with the shelf
   side as the shelf's top FACE.** CERTIFIES. But the crate is then
   no longer on a gauge of its own, and the nested gauge is gone.

So the gap is the nested gauge itself. A gauge can't be anchored to a
part's face, or read anything a part edit moves, so "a frame on this
shelf's top that things are set in" can't be said as a gauge.

## Shape of a fix (a design question)

Some candidates, none ruled:

- a gauge whose parent is an instance's face frame (`FromFace` as a
  gauge placement);
- a gauge placement that reads a part's parameter or a solved pose;
- a mate across gauges allowed to place the gauge itself.

Each touches A11 (2)'s "contact between groups on different gauges is
declared and verified, never placed".

## RULED (2026-10-03, Ev on `[ev]` #3920)

A mate frame is a base composed with an offset, a `Placement` written in the base's frame. In general form, `MateFrame { base: Part | Face, offset: Placement }`:
- today's authored vectors become the part base plus one literal step;
- `FromFace` is the face base, with the empty chain by default.

The crate sits on the shelf's gauge, with a placing mate whose shelf side is the shelf's top face offset in that face's frame. It follows any edit of the shelf. Gauges are unchanged, and A11 (2) is unchanged. The offset can be any rigid motion; the mate's contact class decides which offsets are legal. A3 and A11 (5) state this, and the design-fork log records it as row 53. The tour's shelf-top gauge goes when this is built.

## Closed

Built on `place/mate-frame-offset` (PR 3961). A mate frame is a base
composed with an offset (`MateFrame { base, offset }`,
`crates/editor-core/src/mate.rs`). The tour's crate goes on the stand's
turntable through `regauge_then_mate`, its shelf side being the shelf's
top face with an in-face offset (`demos/tour/src/assembly.rs`, `bench`).
Walls 1 and 2 are now positive asserts: a thicker shelf lifts the crate
and shorter posts lower it, and the gate certifies both (`update_door`).
The residue is filed: the face base's local-+Y convention is
`work/msolve/a-face-base-puts-its-reference-on-local-y.md`, and the
`PlanarRest` fold is
`work/msolve/planar-rest-offset-is-a-second-spelling-of-a-frame-offset-step.md`.
