# ATREST-11 — check 9's contact half sees a crossing and a tangency

**Binds one implementer lane.** Deleted at merge; `work/atrest/ATREST-11.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `atrest/11-check9-crossing`. Row carried:
`work/atrest/check-9-contact-half-misses-a-crossing-and-a-tangency` —
read it in full, and read `validate::ring_nesting`'s doc, which states
the no-crossing premise the nesting arm rests on (ATREST-5).

## The defect

Check 9's contact half (`validate::ring_outer_contact`) matches a
shared vertex, a vertex on an edge, and an edge running along an edge.
It does not see a ring CROSSING its outer loop at a point that is a
vertex of neither, nor a one-point TANGENCY. The nesting arm places a
whole ring from one vertex on the premise that no crossing exists, so
on an arc-bearing loop that premise is assumed, not checked.

## Settled design

**D-A. The Disc × Disc case is exact and comes first.** Where both
loops are `loop_shape`'s `Disc` class they are whole circles, so
`|c_o − c_r|` against `R − r` and `R + r`, each through one `decide`,
classifies nested / internally tangent / crossing / externally tangent /
disjoint. A tangency or crossing reports `RingMeetsOuter` with a
contact shape naming the two LOOPS (a crossing need not have a ring
vertex outside to name) — add the shape if none fits, one variant.

**D-B. The general case: edge pairs of `Line` and `Circle` carriers.**
Line×line, line×arc and arc×arc intersection points, each tested for
lying inside both edges' windows with the arc-window test
`boolean::contain::point_on_arc` already spells (call it, do not
re-spell it; if it must move to be shared, move it, and name the seam —
`contain.rs` is CONTACT's ground). An interior crossing or tangency
point refuses; a decision inside the band escalates, never guesses.

**D-C. What stays silent.** `Ellipse` / spiric / NURBS carriers: named
as residue at the banner and in the not-yet-checked list. Pairs sharing
a vertex are the existing arms' business — do not double-report.

**D-D. Then the nesting arm's premise is CHECKED for the carriers D-A
and D-B cover** — say so at `ring_nesting`, and narrow its "assumed"
sentence to the carriers still outside.

## Measure first

Run the new arms over the corpus's planar faces with rings (every
suite, the demos, the STEP corpus) before landing: the expected refusal
surface is empty, because profile validation refuses crossing loops
and the shell verb's door refuses ahead. **If any body a verb produces
on purpose refuses, stop and report.**

## What you owe

The arms; rows that go red without them — the line case
`a_ring_crossing_its_outer_loop_passes_as_the_banner_says` flips to a
refusal, plus a Disc×Disc crossing, an internal and an external
tangency, and a line×arc crossing (hand-built where no public door
mints them; say which); controls that certify (concentric annulus,
off-centre hole near the rim, a ring close to but not touching the
outer loop at ε = 1e-12); the sweep per discipline §5; out-of-fence
findings filed. Set the carried row to `review` when you open the PR.

## Verification

Hosted CI on the merged head: **six** `test (eps = …, n/2)` jobs and
**five** `k-lint (gate, …)`, read at STEP level. Own `CARGO_TARGET_DIR`
outside the worktree; private scratch; never end a turn with background
work live; **never hold the machine-wide build slot for a battery**.
Do not touch `work/atrest/log.md`. Do not merge.

## Review tier

**Single, FULL**: new refusals from exact intersection arithmetic.
Class M / NUMERIC.
