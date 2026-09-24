# ATREST-4 — check 6's planar arm reaches arc-bearing loops

**Binds one implementer lane.** Deleted at merge; `work/atrest/ATREST-4.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `atrest/4-check6-arcs`. Row carried:
`work/atrest/sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts`
— read its `## Measured` and `## What is left` sections in full; they
are this unit's opening evidence (the 2×2 isolating the `Circle`
carrier as the whole discriminant; the four sense-reading gates).

## The defect

Tier 3's check 6 planar arm is the only at-rest reader that falsifies a
planar face's stored sense against its loop winding, and it runs only
on loops whose carriers are ALL `Line` (`all_lines`). Every planar loop
carrying an arc is skipped, so an arc-bounded planar face can carry an
inverted sense bit through the public `Body::set_face_sense` and
certify — measured, pinned in `crates/sweep/tests/m5_s10_face_sense.rs`.

## Settled design

**D-A. The arm's domain widens from `Line` to `Line`-or-`Circle`
carriers on a `Plane`.** An arc's contribution to the loop's signed
area is its chord plus its circular segment (the bulge), so a loop of
lines and arcs has an exact winding the Newell sum over vertices alone
does not give. Ellipse and NURBS carriers stay exempt and are named as
the residue at the arm's banner and in the not-yet-checked list.

**D-B. One home for the winding of an arc-bearing loop.**
`merge_faces::loop_winding` already states the bulge decomposition, and
`work/zip/verbs-1031b-assigner-checker-divergence.md` records that the
assigner and this checker already disagree in spelling. Do NOT write a
third. Either call the existing one or move it to a home both call; if
it moves, it moves out of `merge_faces.rs` (TOPO's and ZIP's ground) —
announce that seam in the PR body and add your evidence to the ZIP row.

## Measure first — the refusal surface is the risk

Before the arm lands, run the widened arm over every body the corpora
build (the tier-3 suites, the sweep suites, the tour and wild scenes,
the STEP fixtures) and list every body that NEWLY refuses. For each:
**genuinely inverted** (a real defect — file it on the producing
program's slate) or **a false refusal** (the arm is wrong). ATREST-1
settled a check as a decision and then measured 36 pinned rows of
bodies kernel verbs make on purpose refusing under it; that is this
program's recorded lesson. **If any body a verb produces on purpose
newly refuses and you cannot show that it is genuinely inverted, stop
and report the list before landing the arm.** An empty surface is a
result; say it plainly with what you ran it over.

## Rows that move

ATREST-2's pins in `crates/sweep/tests/m5_s10_face_sense.rs`
(`only_the_line_bounded_cap_refuses_…`,
`every_sense_reading_gate_shuts_on_the_arc_loft`,
`the_public_sense_door_builds_an_inverted_arc_loft_tier_3_accepts`)
were written to go red on exactly this change. Re-baseline them to the
new verdicts and say in the PR body what moved and why (discipline §3);
keep a row that pins the ellipse/NURBS residue so its silence stays
visible. `Body::set_face_sense`'s rustdoc (`crates/topo/src/attach.rs`,
TOPO's ground) states check 6 skips conic-bearing loops — re-word it
with the change. `work/tess/planar-mesher-posture-rests-on-a-refusal-check-6-does-not-make.md`
and residual 4 of `work/verdict/m6-sense-gate-recorded-residuals.md`
cite the old gap: add a dated line to each saying what is now checked.

## What you owe

The widened arm; the refusal-surface table; rows that go red without
the change (an inverted arc-bounded planar face refuses
`LoopRoleInverted` naming its face and loop; a ring carrying an arc
too); the sweep per discipline §5 for *a winding computed over a
loop's vertices where the loop carries a curve*, hit list and blind
spot; out-of-fence findings filed. Set the carried row to `review`
when you open the PR.

## Verification

Hosted CI on the merged head: **six** `test (eps = …, n/2)` jobs and
**five** `k-lint (gate, …)`, read at STEP level. Own `CARGO_TARGET_DIR`
outside the worktree; private scratch; never end a turn with background
work live. Do not touch `work/atrest/log.md`. Do not merge.

## Review tier

**Single, FULL** (correctness claims plus the style lane): a new
refusal on the door every program reads as proof, whose risk is the
refusal surface — more than reading the diff settles, but the design is
settled here. Class M / NUMERIC.
