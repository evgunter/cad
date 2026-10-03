---
id: a-from-face-mate-side-cannot-cross-the-split-or-inline-seam
kind: issue
title: a FromFace mate side refuses at the split and inline seam even where its frame provably does not move; how it crosses is a design question on A4
status: closed
opened: 2026-10-02
priority: P1
cost: H
parent: placement-split-and-inline-at-a-gauge-are-refused-until-p2-split
branch: place/p2-face
closed: 2026-10-03
pr: 3934
---


## What

`MateFaceFrameCrosses` (split and inline, `crates/editor-core/src/refactor.rs`) refuses every `FromFace` mate side across the seam, because the face name is a row of the member's part and crosses verbatim. `docs/EDIT-PLACEMENT-SPEC.md` P2-split (rulings 7 and 8, rows F1–F5) is the survey's design: re-spell the name (wrapped `InPart` at split, `FaceName::part_local` at inline) through a new recorded edit, `DocEdit::SetMateFrame`, since no edit writes a mate's alignment except its insert.

Two questions are open, both on ratified A4 text, so they go to a designer pair and then an `[ev]` PR:
- **The frame rule.** A4 lets a side cross "only when its coordinates do not change (the instance it reads is its group's root, on the part's world, at the empty chain)". A face frame resolves in the member's own part, so a re-spelled face frame provably does not move even off root-at-the-empty-chain. Does that condition bind a `FromFace` side?
- **The representation.** Re-spelling through a new edit, or a face frame spelled in the host's naming so the existing `Rebind` carries it (which reaches MSOLVE's `FaceFrame`, the wire, and M1, `work/msolve/a-mate-frame-is-written-in-the-reading-instances-coordinates.md`).

The same `[ev]` PR carries the spec's D1: A4's round trip against the gauge hoist and inline's sugar, which fold a gauge with an empty placement, or one holding exactly one group at the empty chain, into the root's offset.

## RULED (2026-10-03, Ev on `[ev]` #3888)

- **A gauge in the cut moves into the part as it is** ("i agree with (i)"). Split has no hoist and inline no sugar. `Promote`/`Fold` carry the convenience. A4's acceptance (inline-of-split is the identity up to node ids) holds on every shape. This retires the spec's D1 and D3.
- **A `FromFace` side names no face.** Its frame is its head's own face, so it crosses the seam with its head. This was taken with the designers' lean, under Ev's delegation in the same thread. `MateFaceFrameCrosses` and the planned `SetMateFrame` go.

A4, A3 and A11 (5) state it. The design-fork log records it as row 51.

## Closed

Built on `place/p2-face` as ruled. `MateFrame::FromFace` stores no face;
`resolve_side` reads the face its head names in the member's part
(the member walk, the strip's one home; the viewer's mate tool reads
the same walk).
`MateFaceFrameCrosses` is gone at both seams, and `frame_survives`
holds a face side to condition (b) alone. A face side reading a
non-root member, or a pattern copy, crosses split and inline with its world frame
unchanged bit for bit, and the round trip is exact under R1's
comparator (`crates/editor-core/tests/p2_face.rs`). The spec's
§ P2-split rulings 7 and 8 and rows F1–F5 record what is built.
