---
id: props-face-start-reads-a-record-key-through-proven
kind: issue
title: props::face_start reads face.outer through proven although its key comes from a record, so a torn record panics where linked would name the hop
status: dispatched
opened: 2026-11-03
priority: P2
cost: E
refs: [6460]
branch: onecopy/face-view
rides_with: a-face-is-read-through-a-resolved-handle
---


Found by the delta review of PR 6460. `props::face_start` (`props.rs`; called from `reference_of`, `References::of`, `resolution_runs` and `measure_to`) reads the face's outer loop through `proven`. The key comes from a record, so by `proven`'s own doc it belongs to `linked`. This is the panic the sweep row's `before_any_seam` list (`Loop(1v1)`) admits. Fold it into `a-face-is-read-through-a-resolved-handle`, which makes the source one choice per face, if that unit lands first.
