# ATREST-6 — check 1 names an analytic surface whose datum describes no locus

**Binds one implementer lane.** Deleted at merge; `work/atrest/ATREST-6.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `atrest/6-check1-datums`. Row carried:
`work/atrest/quadric-datums-unchecked-at-rest` — read it in full, the
measurement table especially.

## The defect

A face whose analytic surface carries a poisoned or degenerate datum —
a `Plane` with a zero or non-finite normal or a non-finite origin, a
`Cylinder`/`Sphere` with a non-finite or non-positive radius, a `Cone`
with a non-finite half-angle or one outside `(0, π/2)` — is refused by
tier 3 today, but only BY ACCIDENT: checks 3, 4 and 5 read the surface
for other questions, get poison, and escalate with margin `Invalid`.
None of them says what is wrong. S330 closed exactly this shape for
`Nurbs` (a named check-1 refusal) and left every analytic kind open.

## Settled design

**D-A. The poison half: check 1 refuses by name** any analytic surface
whose stored datum is non-finite, or whose `Plane` normal is zero — on
`geom`'s totality-and-poison rule (`crates/geom/src/net.rs`, *"must
reach each consumer's described arm and fail there"*), which is about
no particular surface kind. **Reuse the variant S330 minted for the
`Nurbs` case if its shape fits**; if it does not, say why at the site
and mint ONE variant for the family, not one per kind.

**D-B. The convention half: refuse on REPRESENTABILITY, the torus's
reason.** A cylinder or sphere of radius `≤ 0`, or a cone whose
half-angle is outside `(0, π/2)`, describes no 2-manifold surface a
face can bound. `geom`'s conventional-and-unchecked rule promises such
values give "well-defined garbage", which is `geom`'s contract with its
callers and not a statement that the face is valid; the torus arm
(`validate.rs`, check 1) already refuses a horn or spindle torus on
representability. One posture across the analytic kinds: extend that
arm's reason to the other three, and name the datum and the kind in
the refusal. Put the convention's bound in ONE place per kind — if
`geom` already exposes it, read it; do not restate `0 < θ < π/2` in
`topo`.

**D-C. Report order.** Check 1 runs before checks 3–5, so after this
unit the named refusal is what a caller sees first. The downstream
escalations may still fire (the coarse gate is tier 1–2 only); say in
the PR whether they do, and do not suppress them to make the output
tidy.

## What you owe

The arms; the row's measurement table re-taken on the same fixture
(`coplanar_pillow`, one surface swapped through
`Body::set_face_surface`) with the new first finding per case, as rows
that go red without the change; the named no-op arms' comments
re-worded to what now happens; the sweep per discipline §5 for *a
surface datum read by a check that was asking a different question*,
hit list and blind spot; out-of-fence findings filed. Set the carried
row to `review` when you open the PR.

## Verification

Hosted CI on the merged head: **six** `test (eps = …, n/2)` jobs and
**five** `k-lint (gate, …)`, read at STEP level. Own `CARGO_TARGET_DIR`
outside the worktree; private scratch; never end a turn with background
work live. Do not touch `work/atrest/log.md`. Do not merge.

## Review tier

**Single, FULL**: new named refusals at check 1, and a posture decision
extended from one kind to three. Class M / STRUCTURAL.
