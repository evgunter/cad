---
id: decoration-plane-mint-doc-names-the-c9-ring
kind: issue
title: decoration_plane_mint.rs's module doc reads a plane normal into the C9 ring, the retired name for certification arithmetic
status: closed
opened: 2026-09-29
priority: P4
cost: E
closed: 2026-11-22
---


## What

`crates/geom-brep/tests/decoration_plane_mint.rs`'s module doc says
`ssi::certify`'s chart tube "reads the three components of a
`Surface::Plane` normal into the C9 ring". "The C9 ring" named the
certification scalar `RingInterval`, which RING-3 dissolved into
`Interval`; C9 now says certification arithmetic. The sentence is
present tense, so it states the retired type as current.

## Proposed

"into certification arithmetic (C9)". Comment only; nothing reads it.

## Found by

SCALAR-HYGIENE's second-pass sweep for the retired ring vocabulary
(`ring-3-residue-outside-its-fence`), 2026-09-29. That row's own list
did not name this file; its past-tense siblings ("re-measured when the
C9 ring became a newtype") are true as written.

Moved from TINT to CIW on 2026-11-20, by `git mv`, when TINT merged into CIW: TINT held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CIW carries.

## Closed

`decoration_plane_mint.rs`'s module doc now says "into certification arithmetic (C9)".
