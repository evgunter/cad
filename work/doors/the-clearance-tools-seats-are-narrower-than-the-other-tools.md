---
id: the-clearance-tools-seats-are-narrower-than-the-other-tools
kind: issue
title: The clearance tool has no split-half chooser, and a seat whose body is not drawn lights no witness face
status: open
opened: 2026-11-02
priority: P3
cost: M
---

Disclosed by PR 6357 and placed here by its review.

- **No split-half chooser.** A body seat holding a split picked in the
  tree is refused at the commit (`Refusal::WrongNodeKind`), where every
  `Seats`-held seat offers the two halves (`create::half_rows`). The
  half is pickable as a face in the picture.
- **A seat whose body is not drawn lights no witness face**
  (`viewer::marks::witness` through `drawn_patch`): the body read is not
  the drawn one when a later operation consumed it.

Both follow from the tool holding its seats outside `Seats`
(`the-clearance-tool-copies-the-seat-machinery`).
