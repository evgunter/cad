---
id: radius-arrival-has-no-start-anchored-close
kind: issue
title: paths: whether a Radius arc arrival takes the .at(Start) close that arc_onto()'s builder takes
status: open
opened: 2026-10-19
priority: P3
cost: E
design: true
---


A residual of the arrival `[ev]` design (`arc-closer-constructed-from-arrival-tangent`): both designers named it in their last round and left it outside that question.

`arc_onto()`'s arc-arrival builder takes `.at(Start)` with a director as its sharp seam (`docs/PATHS-DESIGN.md` §2a item 6). That is the shape of the `Via { q, p: Start }` close (`ViaArrivalStart`, `crates/profile/src/path/family.rs:845`), written as a binder. The `Radius { r, side }` arc arrival has the same builder states (`RadiusArrival` / `RadiusArrivalAt` / `RadiusArrivalDir`), but its `.at` takes only a `Point2` (`verb At(Point2<T>)` in `program.rs`'s `transition_table!`), so a radius arc arrival cannot be anchored at the entry vertex.

The closed `at-cannot-bind-start-in-the-path-form` named this "radius-arrival close" as a vocabulary addition. It closed without one because Ev's report turned out to be the seam fillet. No case asks for it today.

The open question is whether `Radius` arrivals should take the same `.at(Start)` close, for uniformity with `arc_onto()` and `Via`, or stay without it until a case appears. Because the answer is a vocabulary addition to ratified text, it goes through designers and then an `[ev]` PR, and only once the `arc_onto()` ruling has landed.
