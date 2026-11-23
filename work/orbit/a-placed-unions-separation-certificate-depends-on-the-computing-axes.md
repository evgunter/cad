---
id: a-placed-unions-separation-certificate-depends-on-the-computing-axes
kind: issue
title: A placed union's separation certificate bounds each copy by an axis-aligned box, so whether it proves the copies apart depends on the computing frame
status: closed
closed: 2026-10-28
opened: 2026-10-26
---


Found by INTENT stage 3 E's test 13
(`crates/editor-core/tests/intent_e_operation_frame_of_reads.rs`,
`every_corpus_document_reads_alike_in_a_second_computing_frame`), which
computes every corpus document in a second frame, turned by an
irrational angle about an irrational axis and translated
(`EvalOptions::computing_twist`).

`topo::Separation::certify` (`crates/topo/src/separation.rs`, `certify`
and `image`, ~L201 and ~L249) proves a placed union's copies apart by
the images of the prototype's axis-aligned hull under each placement's
map. A box is a property of the axes it is drawn in: in the corpus's
`heat_sink_fins`, the fins built in their own coordinates certify, and
the same fins computed in the turned frame refuse
`PlacementsUncertified` — the turned prototype's box is fatter than the
fins, and two neighbouring fins' boxes meet. Every other node of the
corpus reads alike in both frames; this is the one verdict outside the
sliver band that the computing frame moves (D10 `[placement.frame]`:
"every verdict outside the sliver band is the same in any frame").

With E a placed union computes in its prototype's frame, which for a
prototype built in its own coordinates is the frame its boxes are
tightest in, so no corpus document refuses today; the dependence shows
only when the prototype's frame is turned off its own axes (a copy of a
copy placed by a turn, or the second frame).

Recourse: certify in the prototype's own (oriented) box, or by an
oriented-box test between the mapped boxes, so the certificate is a
function of the copies and not of the axes. `PlacedUnion` retires at
INTENT stage 3 G (`patterns-are-index-variables`), whose `∪` over a
family decides its members' meetings the ordinary way; if G lands
first this item closes with it.

## Closed by G (`patterns-are-index-variables`)

`PlacedUnion` retired. The certificate is now the union's fast path (`topo::Separation::apart`, `crates/editor-core/src/eval/wire.rs` `images_apart`): a pair it cannot certify apart is judged by the n-ary union rather than refused, so the union's result is the same in any computing frame; only how many pairs are judged (`EvalCounters::judgements`) depends on the axes the boxes are drawn in.
