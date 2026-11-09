---
id: the-ring-lanes-island-is-not-checked-in-its-face
kind: issue
title: the boolean's ring lane reads an island's winding off its face's outer-loop points (chord_join::IslandReading::read_paths, Answering::Any) on the premise that the island lies in the face, which only the construction guarantees mid-operation
status: open
opened: 2026-11-09
priority: P3
cost: E
---


## Finding

Filed by `onecopy/path-parity` (PR 6607's review).
`chord_join::IslandReading::read_paths` reads a ring-lane island's
winding at the face's outer-loop points, through an `Answering::Any`
ladder, on the premise that every one of those points lies on the same
side of the island. The premise is that the island lies in the face.

Mid-boolean it is read at `boolean::join::ring_run_ccw` (the
`IslandClosing::Quadric` and `Carried` arms). The island there is a run
of a ring of the operand's face, which check 9 placed in the face at
rest, closed by a section piece that the reduction kept where the face
holds it. So the construction guarantees the premise, but nothing checks
it there. An island whose closing piece leaves the face is read by its
first decided reference.

PR 6607 found exactly that shape in a test fixture (`cone_ring_rows`'
corner sheet). The fixture builders now assert the ring lies in its face.
Production has no such assertion.

**What closes this row:**
- a debug check at `read_paths` that the outer-loop references agree:
  the agreeing check, `a-ladder-that-checks-its-answers-agree-has-no-home`,
  which waits on `[ev]` PR 6592;
- or a reason the section pieces cannot leave the face, cited at the site.
