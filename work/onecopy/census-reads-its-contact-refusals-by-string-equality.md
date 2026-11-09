---
id: census-reads-its-contact-refusals-by-string-equality
kind: issue
title: The census identifies its edge-tangent refusals by string equality on ContactRefusal::NotCertifiable's what
status: dispatched
priority: P3
opened: 2026-11-06
---


Found by `onecopy/spline-walks` (PR 6571), which gave the spline-walk reasons the payload's own sentence (`geom::PieceWalkRefusal::sentence` / `of_sentence`).

`topo::census::curved` pushes each reason a curved edge's reading refused with as `ContactRefusal::NotCertifiable { what: &'static str }` (`refuse`). It then identifies the reason again by matching `what` against its constants (`edge_tangent_refusal`): `NO_CURVE`, `NO_REACH`, `NO_LENGTH`, and the walk's sentence. So a reason travels as text and is parsed back into `link::EdgeTangentUnread`, though `NotCertifiable` renders no `what`.

A typed `what` (an enum the census owns, or `EdgeTangentUnread` itself carried on the refusal) would end the round trip and the string match with it.
