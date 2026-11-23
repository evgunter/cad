---
id: a-wrapper-of-a-payload-only-refusal-never-ends-it
kind: issue
title: A wrapper of a refusal that was payload-only before E1e (CertifyError, SsiError) writes it as payload and never ends it at a door
status: open
opened: 2026-11-03
priority: P3
cost: M
---


## Finding

From E1e's review (PR 6430, review 1, fix list P5). E1e made every refusal whose `Display` spelled an ending payload-only, and gave each wrapper of one a `render(at)` that ends it at the reader's door. A wrapper of a refusal whose `Display` was *already* payload-only before E1e was not in that cascade. It still writes the inner refusal with `Display`, so the inner refusal's ending is never written at any door:

- `geom_brep::edge_nurbs::PlaneNurbsRefusal` (`crates/geom-brep/src/edge_nurbs.rs`, its `Display`, ~582 and ~590) writes `CertifyError::NotTransverse` / `CertifyError::Escalated` through `CertifyError`'s `Display`, the payload. The transversality decision's ending (`CertifyError::render(at)`) never reaches a reader.
- `topo::offset_derive::SectionVerdict::Ssi` and `CornerVerdict::Ssi` (`crates/topo/src/offset_derive.rs` ~83 and ~132) write `SsiError` through its `Display`, the payload. The SSI decision's ending (`SsiError::render(at)`) never reaches a reader.

## The question

Should each become `Ended` and write its inner refusal through `recourse::read_at(e, at)`, so its doors end it? That is a meaning change at those doors: an ending appears where none did. That is why it was not part of E1e, which moved endings and changed none.
