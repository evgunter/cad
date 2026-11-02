---
id: a-template-rectangle-has-no-centre
kind: issue
title: A template rectangle is centred on the origin, so a rectangular hole off-centre must be typed as a path
status: open
opened: 2026-11-02
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

A fix gives the template a centre (lowered as corners offset by it,
in `sketch::loop_program` and `sketch::shape_steps`) and a centre
field in the form. It touches the same template the parked
`the-rectangle-and-circle-templates-reopen-as-paths` is about, so the
two may want to land together.
