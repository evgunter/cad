---
id: at-cannot-bind-start-in-the-path-form
kind: issue
title: paths: at(Start) — the path form cannot bind a position at the entry vertex, and does not say where the Start-anchored spelling lives
status: open
opened: 2026-09-18
priority: P0
cost: D
---

Reported by Ev from the viewer (2026-09-18): "`at` seems to not take `Start`, at least not in the gui."

**What is true today.** `at` takes a `Point2` and nothing else (`verb At(Point2<T>)` in `crates/profile/src/path/program.rs`'s `transition_table!`), so the path form's `at` row has point fields and no target picker (`path_step_fields` in `crates/viewer/src/widgets.rs`). The only spelling that anchors a side at the entry vertex is `CloseTo` (`.to(Start)`, shown in the form as `to Start (close)`), which has one row, at `Open`: the seam fillet with a straight first side. So from a bound arrival direction (`Angle`: `.fillet(r).angle(θ)`) nothing reaches `Start`. PATHS-DESIGN §4 names that form (`.fillet(r).angle(θ).to(Start)`, an arrival side ending at the entry vertex) and leaves it **open, deliberately**, until a case turns up. This report may be that case.

**What is owed.**
1. Find out from Ev which tip the `at(Start)` was wanted at and what it was meant to say. At `Open` it is `CloseTo` under another name, and the fix is the form saying where that lives. At `Angle` it is §4's open form. At `RadiusArrival` or `RadiusArrivalDir` (a radius arc arrival anchored on the entry vertex) it is a close the lattice has no row for.
2. If it is §4's form or the radius-arrival close, it is a vocabulary addition to the ratified PATHS-DESIGN, so an `[ev]` PR. Its row lands in `transition_table!` and the viewer offers it with no further edit (`Verb::states`).
3. Whatever the answer, the form should not leave an author looking for `Start` on the `at` row. The `at` row could point to `to Start (close)` where that verb is admitted.

