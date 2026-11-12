---
id: a-cups-fixture-boolean-is-unfinished-at-eps-1e-6
kind: issue
title: The cups-mouth fixture's own boolean returns an unfinished body at eps 1e-6 (a face's area not certified positive), so band_strip_mouth_cups' trimline row is red on main
status: open
opened: 2026-11-12
priority: P1
cost: M
---


Found while bringing PR 6520 up to date with main. CI on its head
`2fe187ca58` (job `test (kernel)`) went red here, and the test fails
identically on origin/main `9ee1c046ef` and `6c70512339`, so it is not
6520's. Not yet in nightly 42965378060's failure list.

`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --test all -E
'test(a_cups_mouth_a_hair_across)'` fails
`band_strip_mouth_cups::a_cups_mouth_a_hair_across_the_trimline_crosses_and_one_inside_goes`
(`crates/sweep/tests/band_strip_mouth_cups.rs:444`) at the fixture's own
boolean, `crates/sweep/tests/band_concave_bar.rs:74`, before any blend code
runs:

> the fixture's boolean builds, got the Boolean's result is not a finished
> body … a face's area could not be certified positive at this tolerance.
> Recourse: widen the face well past the tolerance, or … tighten the
> tolerance below 9.9999999995104e-7 m

The "hair across the trimline" geometry leaves a face about one tolerance
wide at eps 1e-6. The fixture either scales its hair with the eps or states
the eps it is meant for; or, if the sliver face is the boolean's to merge
away, the row goes to the boolean's owner.
