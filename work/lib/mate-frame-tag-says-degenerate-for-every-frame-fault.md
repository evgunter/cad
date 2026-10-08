---
id: mate-frame-tag-says-degenerate-for-every-frame-fault
kind: issue
title: MateFault::Frame tags mate_frame_degenerate for every FrameError, a non-finite or underflowed frame included
status: open
opened: 2026-09-29
---


## What

Found by D366's review (PR 3469), pre-existing on `origin/main`.
`MateFault::Frame` (`crates/editor-core/src/mate.rs`) carries the frame
ladder's whole `FrameError`: `Degenerate`, `NonFiniteLength`,
`UnderflowedLength` or `Band`. Its outer Python word is one constant,
`mate_frame_degenerate`, written in `node_error_tag`'s `MateFrame` arm
(`crates/pncad-py/src/tags.rs`), and `mate_fault_tag` reads that word
through `NodeErrorClass::of_mate`. So a mate frame with a non-finite or
underflowed axis, or one refused by the band constructor, reaches
Python as `kind == "mate_frame_degenerate"`: the word says zero length
when the fact is overflow, underflow or a bad band.

`inner_variant` does carry the right fact (`mate_payload.rs`'s
`with_frame` writes `frame_error_tag(error)`: `non_finite_aim`,
`underflowed_tangent`, `band`, ...), so the fact is not lost. The
outer word is still wrong for three of the four arms, and it is the
word a caller branches on first. `frame_error_tag`'s own doc gives the
reason these differ: a degenerate direction is a coincidence at this
tolerance, and a non-finite or underflowed one wants scale.

## The fix this wants

A decision on the outer word, which is stable API: either a neutral
word (`mate_frame`) with the fact on `inner_variant`, or a split of
`MateFrame` by `FrameError` arm (the way `FrameDirection` splits by
`UnitVec3Error`). Either moves a published word, so it is LIB's call
and a re-baseline of `node_error_tags_are_the_published_words`.
