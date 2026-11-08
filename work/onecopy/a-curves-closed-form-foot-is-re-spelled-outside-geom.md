---
id: a-curves-closed-form-foot-is-re-spelled-outside-geom
kind: issue
title: A point's foot on a line or circle carrier is re-spelled outside geom, and param_near reads a line's parameter as if dir were unit
status: closed
opened: 2026-11-02
priority: P1
cost: E
branch: onecopy/foot-and-angle
closed: 2026-11-03
pr: 6449
---

Found by the sweep of the `onehome/geom-readings` lane (straight carriers and axis feet), against main `44577259fb`.

`[one-way.readings]` puts a point's foot on one carrier in `geom` (`Curve3::foot`, named in DESIGN's crate table). Three closed-form copies of it for lines and circles live outside:

- `crates/geom-brep/src/chart_singularity.rs` `foot` (line and circle in closed form, `None` otherwise; its doc calls itself "the one spelling of the foot"), cited by `crates/step-import/src/normalize/singularity.rs` `param_at`.
- `crates/sweep/src/blend/reach.rs` `foot_on` (line and circle; the circle arm normalizes `axis` and reads the angle about it).

The line arms now read `geom::Line::param`/`foot`; the circle arms are still their own closed forms beside `Curve3::foot`'s polished reading, and nothing holds them to one pick where the pick is free (a point on the circle's axis, where every point of the circle is a foot).

Inside `geom` itself, `Curve3::param_near`'s `Line` arm reads `(p − origin)·dir`, taking `dir` as unit, while `Curve3::foot` and `geom::Line::param` divide by `dir·dir`. They agree on a unit `dir` up to rounding and disagree on any other; `geom::Line::param` is the one that holds for every nonzero `dir` (pinned by `geom` `curves::tests::a_lines_foot_is_square_to_it_at_any_dir_length`). Routing `param_near` through it moves the last bits of every line edge's endpoint parameter, which is why it was left for its own review.

Fix: one foot reading in `geom` (a closed-form arm in `Curve3::foot` for the circle, if the polish is the cost these copies avoid), with the copies repointed and `param_near`'s line arm reading `Line::param`.

## Slate triage (2026-11-03, ONEHOME cut)

- Moved to ONECOPY (`work/onecopy/`) by `git mv`, id and body unchanged. ONEHOME measured 70 budget points against 35 when its orchestrator ran out of usage; the rows it had not started (no lane, no design fork, no in-flight PR riding them) were cut onto ONECOPY, and what is in flight or staged stayed on ONEHOME.
