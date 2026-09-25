# ATREST-13 — check 1 and check 6 follow-ups

**Binds one implementer lane.** Deleted at merge; `work/atrest/ATREST-13.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `atrest/13-check1-6-followups`. Four rows carried, all filed by
ATREST-4 and ATREST-6 and all on their ground — read each in full:

1. `work/atrest/surface-frame-directions-are-unchecked-at-rest`
2. `work/atrest/a-poisoned-curve-carrier-datum-is-refused-at-rest-only-by-accident`
3. `work/atrest/an-infinite-nurbs-net-passes-check-1-where-an-infinite-analytic-datum-does-not`
4. `work/atrest/check-6-planar-arm-skips-ellipse-and-nurbs-loops`

## Settled design

**D-1. Surface frame directions** (a zero `axis` or `u_ref` passes check
1; `geom`'s crate docs claim tier 3 certifies frame unit-ness and
nothing does). A zero or non-finite direction is poison — refuse it as
`PoisonedSurfaceDatum`, exactly as ATREST-6 refuses a zero plane
normal, through the same `is_finite_length` → `is_zero_length` doors.
For unit-ness and `u_ref ⊥ axis`: **measure first** what a non-unit or
non-orthogonal frame does at rest (does any consumer read it as a
different locus?) and over the corpus (does import or any verb mint
frames off-unit beyond the band?). If a frame off its convention
changes the described locus, it is a representability convention —
refuse it through ATREST-6's `representability_margins` door with the
band, not an exact compare. If it does not, correct `geom`'s crate doc
to what is checked. Either way the doc must become true.

**D-2. Curve carrier datums.** The same shape ATREST-6 closed for
surfaces: a poisoned or degenerate curve datum (a `Line` with a zero or
non-finite `dir`, a `Circle`/`Ellipse` with a non-finite or non-positive
radius or semi-axis, a zero axis) is refused only by accident, through
check 2's residual. The row says the measurement is owed — take it
first (one body, one edge's curve swapped), then refuse by name at
check 1 with ONE curve-datum variant pair mirroring the surface pair,
reusing the same doors. Do not mint per-kind variants.

**D-3. An infinite NURBS net** passes check 1 while an infinite analytic
datum does not. One posture: a `+∞` control point or weight is poison
for a NURBS net exactly as for an analytic datum. Make the NURBS arm
agree, through the same `is_finite_length` read.

**D-4. Check 6 reaches ellipse-bounded planar loops.** ATREST-4's shared
winding home (`loop_winding.rs`, `Body::planar_loop_winding`) already
answers `Elliptic` reach for the merge assigner; check 6 calls it at
`Circular`. Widen check 6 to `Elliptic`. ATREST-4's instrument measured
the surface already: 279 ellipse-bearing loops per ε, 278 honest, 1
genuinely inverted (the `cut_cylinder` section face under its test's own
`flip_all`), 0 escalations; the demos 8 more, all honest. Re-baseline
the rows that pin the ellipse residue (ATREST-4's
`an_ellipse_bounded_planar_face_stays_outside_the_planar_arm`, and the
m6_6 cut-cylinder row) to the new verdicts and say what moved. NURBS-
and spiric-bounded loops stay the residue: re-state the carried row to
that remainder and leave it OPEN (it is not closed by this unit).

## Stop clauses

If D-1's measurement shows import or a verb minting frames off
convention beyond the band, stop and report before refusing them. If
D-4's widening refuses any body the corpus builds on purpose other than
the known inverted one, stop and report.

## What you owe

Rows that go red without each change; the re-taken measurements; the
sweep per discipline §5 for *a datum read by a check that was asking a
different question*, curves this time; out-of-fence findings filed; the
four carried rows set to `review` (D-4's row re-stated and left open).

## Verification

Hosted CI on the merged head: **six** `test (eps = …, n/2)` jobs and
**five** `k-lint (gate, …)`, read at STEP level. **The disk is nearly
full machine-wide — do not build locally beyond one crate's `cargo
check`; measure through hosted CI on a throwaway branch.** Own
`CARGO_TARGET_DIR` outside the worktree; private scratch; never end a
turn with background work live; never hold the build slot; never
`pkill -f`. Do not touch `work/atrest/log.md`. Do not merge.

## Review tier

**Single, FULL**: new named refusals at check 1 and a widened check 6,
each with a measured refusal surface. Class M / STRUCTURAL.
