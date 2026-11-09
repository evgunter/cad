---
id: arc-trim-margin-stayed-in-containment-when-arc-trim-moved-beside-check-9
kind: issue
title: Span membership is spelled two ways beside check 9 (the arc-trim move's S4); the pair is unnamed
status: open
opened: 2026-11-09
---


Found by the delta review of PR 6609 (`onecopy/loop-walks`, head 894d822f), finding F2.

PR 6609 moved `arc_trim` and `ArcTrimRows` beside check 9 in `topo/src/validate.rs`. Two loose ends remain:

1. `arc_trim_margin` stayed at `topo/src/splitting/containment.rs:~1049`, although its only consumer is `validate.rs:~9998`. Move it beside its consumer.
2. Its doc says it is "for a caller that has decided the ends by another metric first (an ellipse's exact end distances, [`ConicArc::hit`])". `hit` does not call it, so the sentence is stale. Correct it.

Also still open from that review (S4, style): span membership is spelled two ways. Name both, and route one through the other.

## Narrowed (2026-11-09, PR 6641)

Items 1 and 2 are done. `arc_trim_margin` is a private fn beside
`arc_trim` in `topo/src/validate.rs`, and the stale caller is gone from
its doc. What remains is S4. It does not name the two spellings of span
membership. The likely pair is `validate.rs` `window`'s line arm (two
signed spans under `ring_outer_meet_span`) and `arc_trim` (end distances,
then the trim margin). Those are different constructions, so whether one
can honestly route through the other is the open question. Name the pair
first.
