---
id: at-cannot-bind-start-in-the-path-form
kind: issue
title: paths: at(Start) — the path form cannot bind a position at the entry vertex, and does not say where the Start-anchored spelling lives
status: closed
opened: 2026-09-18
priority: P0
cost: D
closed: 2026-10-02
---

Reported by Ev from the viewer (2026-09-18): "`at` seems to not take `Start`, at least not in the gui."

**What is true today.** `at` takes a `Point2` and nothing else (`verb At(Point2<T>)` in `crates/profile/src/path/program.rs`'s `transition_table!`), so the path form's `at` row has point fields and no target picker (`path_step_fields` in `crates/viewer/src/widgets.rs`). The only spelling that anchors a side at the entry vertex is `CloseTo` (`.to(Start)`, shown in the form as `to Start (close)`), which has one row, at `Open`: the seam fillet with a straight first side. So from a bound arrival direction (`Angle`: `.fillet(r).angle(θ)`) nothing reaches `Start`. PATHS-DESIGN §4 names that form (`.fillet(r).angle(θ).to(Start)`, an arrival side ending at the entry vertex) and leaves it **open, deliberately**, until a case turns up. This report may be that case.

**What is owed.**
1. Find out from Ev which tip the `at(Start)` was wanted at and what it was meant to say. At `Open` it is `CloseTo` under another name, and the fix is the form saying where that lives. At `Angle` it is §4's open form. At `RadiusArrival` or `RadiusArrivalDir` (a radius arc arrival anchored on the entry vertex) it is a close the lattice has no row for.
2. If it is §4's form or the radius-arrival close, it is a vocabulary addition to the ratified PATHS-DESIGN, so an `[ev]` PR. Its row lands in `transition_table!` and the viewer offers it with no further edit (`Verb::states`).
3. Whatever the answer, the form should not leave an author looking for `Start` on the `at` row. The `at` row could point to `to Start (close)` where that verb is admitted.

## Ev's answers (2026-10-02)

To question 1: "honestly i don't remember. possibly a fillet? it is
entirely possible that i was just confused about how to write it and i
shouldn't have been using `at`".

And, on how to resolve it: "you can just close it if it seems like
there's no issue in writing a path final fillet".

## Closed: a path's final fillet is written and validates

A fillet at the close is the seam fillet, and the form spells it as
`fillet` followed by `to Start (close)` — the `CloseTo` row at the tip a
fillet leaves (`Open`), which `Verb::ALL` puts in the form's verb menu
with no edit. `a_final_fillet_closes_from_the_form`
(`crates/viewer/src/sketch.rs`) authors the rounded square from
mid-side anchors through the form's own steps, checks the form admits
each step at its tip (`admits_at` over `tip_state_at`), and pins the
preview closed and valid with all four corners rounded. The
lattice-side twin is `geometry_refusals_are_the_path_class_and_are_binding_dependent`
(`crates/profile/tests/path_program.rs`), which replays the same step
shape. So there is no missing vocabulary: §4's
`.fillet(r).angle(θ).to(Start)` stays open, and no `at` hint was added.
