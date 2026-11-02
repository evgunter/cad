---
id: a-template-rectangle-has-no-centre
kind: issue
title: A template rectangle is centred on the origin, so a rectangular hole off-centre must be typed as a path
status: closed
opened: 2026-11-02
closed: 2026-11-02
priority: P3
cost: E
refs: [the-gui-cannot-author-a-profile-of-several-loops, the-rectangle-and-circle-templates-reopen-as-paths]
---

Found by the lane that closed
`the-gui-cannot-author-a-profile-of-several-loops`.

`ProfileShape::Rectangle` (`crates/viewer/src/sketch.rs`) carries a
width and a height and is centred on the sketch origin; the add-profile
form's rectangle fields are width and height only. Once a profile can
hold several loops, a rectangle is as likely to be a hole as an outer,
and a hole is rarely on the origin: a rectangular slot beside a bolt
hole has to be typed as a path chain, corner by corner. The circle
template carries a centre; the rectangle does not.

## 2026-11-02 — closed: covered by the template row

Ev's ruling on PR 6143 (fork-log row 234) makes the rectangle a stored
`rectangle(centre, width, height)` form, so corners offset by a centre
is the derived-numbers defect that ruling closes; the centre arrives
with `the-rectangle-and-circle-templates-reopen-as-paths`.
