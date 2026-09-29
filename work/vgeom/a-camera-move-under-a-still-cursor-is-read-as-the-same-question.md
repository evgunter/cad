---
id: a-camera-move-under-a-still-cursor-is-read-as-the-same-question
kind: issue
title: A camera move under a still cursor is read as the same pick question, so the hover and the id answer go stale
status: open
opened: 2026-09-25
priority: P2
cost: D
---

Found by the review of `vnews/ray-refusal-is-not-a-disagreement`
(PR 3221). That PR drives the wiring in
`pane::viewport::tests::a_ray_refusal_the_pick_path_did_not_ask_for_is_said_by_the_comparison`:
after a `CameraOp::Orbit` under a still cursor, `IdQueryLog::step`
answers `IdStep::Hold`.

## The finding

`crates/viewer/src/idpass.rs`: `IdQueryLog::step` re-asks only when
the cursor or the `IdSubject` changes, and `IdSubject` is `{ revision,
generation }` (the picture and the index). The camera is in neither.
`crates/viewer/src/pane/viewport.rs`, `viewport_ui`, reads the same
verdict for the CPU ray: `skips_the_ray` drops a `Hover` on `Hold`,
on the premise that *"a hover over an unchanged picture at an unmoved
cursor asks a question whose answer the session already holds"*.

A camera move with the pointer still breaks that premise. A scroll
`Dolly` moves toward the target, not the cursor, so a wheel turn over
a still pointer changes the ray under it while the log says `Hold`.
(Any other camera operation that lands without the pointer moving does
the same. The dolly is the one a user reaches without trying.) What
follows:

- **The hover goes stale.** The session's hover (and so the
  highlight and the probe mark) keeps naming the entity under the OLD
  ray until the pointer moves.
- **The id query is not re-asked.** The GPU answer on hand describes
  the picture from the old camera. The cursor comparison still asks the
  ray afresh through the new camera, so the two can disagree about two
  different cameras, and the status line reads that as *"picking paths
  disagree"*, which issue #1097 §4 tells an operator to read as an
  `R32Uint` clear fault.
- **A ray refusal the new camera brings on** (the hit test's unnamed
  arm) is not asked by the pick path. PR 3221 makes the comparison say
  it in that case (`cursor_news`, keyed on the pick loop's own
  `ray_asked_at`), so that half no longer depends on this row. The
  first two do.

## What a fix would have to decide

Whether the camera joins `IdSubject` (every camera move becomes a new
question, and a new blocking readback: the cost the movement gate
exists to avoid during a drag), or whether a camera change re-asks only
once it has settled. Either way, when it lands, PR 3221's wiring test
has to find another way to reach a frame the pick path skips. Its
`Hold` assertion documents the behaviour this row reports.

## Fence

`crates/viewer/src/idpass.rs` (`IdQueryLog`, `IdSubject`) and the skip
rule in `crates/viewer/src/pane/viewport.rs` (`skips_the_ray`).
