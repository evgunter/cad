---
id: a-plane-section-along-a-fits-window-edge-proves-no-one-arc
kind: issue
title: A plane x fit section lying on the fit's window edge refuses its tube's one-arc proof undecided, so a moved fitted cap whose face is its whole fit cannot re-certify its edges
status: open
opened: 2026-10-09
priority: P2
refs: [a-moved-fitted-faces-corners-have-no-root-on-a-derived-spline-section]
---

Found by SHELL's fitted-corners unit. `crates/sweep/tests/r1_lane0_e2e.rs:41`'s
`the_f64_seam_answers_every_public_door` moves the unit box's cap, swapped
for a bilinear NURBS patch over exactly the face (`[0, 2]²` at `z = 1`),
by 0.05 through `topo::replace_faces_offset` and through
`topo::replace_face_offset`. The cap's offset fit covers the same window,
so each side plane's section with it is the fit's own boundary row
(`crates/topo/src/offset_derive.rs:692`, `level_row`). The edges derive and the corners solve; the
surface swap then refuses re-certifying the first one:

`Op { edge: None, error: RechartFalsifies { door: SetFaceSurfacesDescribing,
edge, error: PlaneNurbs(TubeNotOneArc { rungs: 20, cause: Undecided(
Indeterminate { margin: MarginDiag(Invalid), predicate: Some("ssi_tube_one_arc"), .. }) }) } }`

at ε = 1e-9, for d = +0.05 and −0.05. The `Undecided` arm is
`limb_three`'s (`crates/geom-brep/src/ssi/certify.rs:1130`, the
`Shortfall::Undecided` arm at :1175): every rung was a
graph whose one-arc walk read no margin it could classify. The same pair
one window wider (the patch over `[-1, 3]²`, so each section crosses the
fit's interior) certifies every edge and builds
(`crates/sweep/tests/encl_curved_loft_shell.rs:719`,
`a_moved_fitted_cap_stands_its_corners_on_the_held_sides`), so the
refusal reads as the tube's walk leaving the fit's chart along a
section that lies on its window edge. Unmeasured beyond that: which rung
and which box the walk stopped in.

Owed: a one-arc proof for a section on the fit's window edge (the
row is exact structure, `interior_iso_u`'s boundary case), or a refusal
that names the window edge rather than an undecided margin. Either makes
the box with a fitted cap that is its whole fit the first fitted face to
move and re-certify, and the row above re-baselines.

## 2026-10-12 — also red at 1e-6 on main

`r1_lane0_e2e::the_f64_seam_answers_every_public_door` fails at
`CAD_TOLERANCE_EPS=1e-6` on clean main (`8cd31ca1d4`, again on
`852d68bb1d`) with the same refusal shape (`RechartFalsifies` …
`TubeNotOneArc`, `Undecided`), seen by the PR 4286 and PR 4797 lanes
while re-running their eps rows on main. At 1e-9 it passed there.

## 2026-10-13 — still red at 1e-6 on main

The FLUX cylinder-band lane (PR 4839) re-ran
`r1_lane0_e2e::the_f64_seam_answers_every_public_door` alone by name on clean
main `e71d719f1`. It is red at `CAD_TOLERANCE_EPS=1e-6` and green at 1e-12.

## 2026-10-14 — still red at 1e-6 on main (NURBS, fresh target)

Re-run alone with a fresh `CARGO_TARGET_DIR` per commit: red at `a71403e3` and still red on main at `307ed404`. Not moved by NURBS PR 4984 (red on both sides of it). (NURBS build A)

## 2026-10-15 — still red at 1e-6 on main

Re-run by name at `CAD_TOLERANCE_EPS=1e-6` on main `8dc4a60767` by the FUSE curve-pair reader lane, after the dual review of PR 5154 found it red there: red with the same refusal on main and on the PR head, so it is not the PR's.

## 2026-10-19 — the 1e-6 red dates from PR 4472's merge

CONTACTHOLD's nightly-red lane built `sweep`'s `all` binary on both sides of `108b06955d`, the merge of PR 4472 (`shell/fitted-corners`). At ε 1e-6, `r1_lane0_e2e::the_f64_seam_answers_every_public_door` passes at its first parent `cb4172aa9f` and fails at the merge, with this row's `TubeNotOneArc { rungs: 15, cause: Undecided(..) }` refusal. It is still red in the 2026-10-19 nightly (run 39205007024).

## 2026-10-28 — measured on the side, left undecided

SPLICE's tube-tripwire lane
(`splice/the-two-chart-tube-refuses-a-carrier-along-a-chart-side-undecided`)
measured this section's image in the fit's chart within 1e-17 to 2e-16
of the fit's side, so it lies along the window edge as the two-chart
tube's new `AlongSide` refusal reads one. The chart tube does not take
that name: past its side arm, an `Undecided` may come from the arm's
own in-band readings (`Reading::Refused | Beyond`, a missing side
reader, `Near::Unknown`), which are indeterminate, and naming it would
report them as sign-certain. The payload here stays `Undecided`.
"A refusal that names the window edge" therefore needs the side arm to
say which of its readings refused; the one-arc proof is still open.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. `r1_lane0_e2e::the_f64_seam_answers_every_public_door` is still red
at 1e-6 (`PlaneNurbs(TubeNotOneArc { rungs: 15, cause: Undecided(… MarginDiag(Invalid) … ssi_tube_one_arc) })`).
It is pinned at 1e-6 on that stop (`EdgeKey(10v1)`, `rungs: 15`,
`MarginDiag(Invalid)`, `ssi_tube_one_arc`).

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured.
