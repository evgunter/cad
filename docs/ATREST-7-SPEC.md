# ATREST-7 — a solid's shells bound winding number 0 or 1

**Binds one implementer lane.** Deleted at merge; `work/atrest/ATREST-7.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `atrest/7-shell-winding`. Row carried:
`work/atrest/tier-3-does-not-check-shell-roles-per-solid` — read ALL of
it, and read its last section ("Re-stated again, 2026-09-24") as the
governing statement; the earlier sections are the record of two
readings that were wrong.

## The invariant, and what is NOT a violation

A solid's material is the region its closed oriented shells enclose,
read by winding number: an `Outer` shell adds `+1` inside itself, a
`Void` shell adds `−1` inside the cavity it bounds. **Valid means the
winding is 0 or 1 everywhere.**

NOT violations, and each must stay certifying (rows below): several
disjoint `Outer` shells under one solid (`graft_disjoint`'s onto door,
the boolean coplanar split, `editor-core`'s placed union); an `Outer`
island inside a `Void` of the same solid (the hollow-operand
subtraction — winding `+1 −1 +1 = 1`); an ordinary hollow solid.

Violations tier 3 does not see today, each with a positive per-solid
total so the per-solid check 7 passes them: a `Void` lying outside
every `Outer` of its solid (winding `−1`); an `Outer` inside another
`Outer` with no `Void` between (winding `2`); a `Void` inside a `Void`
with no `Outer` between (winding `−1`).

## Settled design

**D-A. One witness point per shell, under the no-crossing premise.**
Tier 3 already assumes shells do not cross (global self-intersection is
an explicit not-yet-checked item — cite it at the check). Under that
premise the winding of the OTHER shells of the solid is constant along
each shell, so one point `p` on shell `s` decides it: that winding must
be `0` if `s` is `Outer` and `1` if `s` is `Void`. Compute each other
shell's contribution at `p` with
`boolean::solid_contain::point_in_solid_faces` over that shell's faces,
read through that shell's ROLE (a `Void` selection's material side is
the complement — derive this, do not assume it, and pin both arms with
rows). Roles come from check 7's per-solid sign walk or from the shell's
own sign — reuse what ATREST-1 built; do not take a reporting-level
read (`classify_shells_of`), which is exactly the coupling
`work/atrest/tier3-prime-…` exists to remove.

**D-B. Silent where the walk cannot answer.** `point_in_solid_faces`
refuses `KindUnsupported` on curved faces its closed forms cannot
certify, and may escalate. There the check is SILENT and the residue is
named in the not-yet-checked list — check 9's posture. An `OnBoundary`
witness means shells touch, which the no-crossing premise excludes;
treat it as silent too and say so at the site. This program must not
add to the false-refusal direction.

**D-C. A new check, numbered 10, with one new variant** naming the
solid, the shell and the measured winding. Solids with one shell skip
it, so the common body pays nothing; say what a multi-shell solid pays.

## Measure first

Run the check over every multi-shell solid the corpora build and list
what newly refuses. The expected surface is EMPTY — a kernel verb
producing a winding-2 or winding-−1 body would be a producer's bug worth
filing. **If any body a verb produces on purpose refuses, stop and
report** before landing.

## What you owe

The check; rows that go red without it (one per violation shape, built
through public doors where possible — say where you had to hand-build
and why); the three NOT-violation shapes above as rows that certify; the
sweep per discipline §5; out-of-fence findings filed. Update
`work/zip/subtract-of-a-hollow-operand-files-the-island-under-one-solid`
with a dated line: its sentence "Tier 3 does not catch the shape (see
TOPO's `tier-3-does-not-check-shell-roles-per-solid`), which is why it
validates" is no longer the reason — the shape is valid at rest and the
grouping is ZIP's output convention. Set the carried row to `review`
when you open the PR.

## Concurrency

ATREST-3 is restructuring check 7's machinery in the same function
(`tier3_local_checks_marked` and `validate_geometric_certified`). Keep
check 10 in its own function called from those sites, and merge
`origin/main` whenever it moves.

## Verification

Hosted CI on the merged head: **six** `test (eps = …, n/2)` jobs and
**five** `k-lint (gate, …)`, read at STEP level. Own `CARGO_TARGET_DIR`
outside the worktree; private scratch; never end a turn with background
work live. Do not touch `work/atrest/log.md`. Do not merge.

## Review tier

**Single, FULL**: a new refusal stating what a valid multi-shell solid
is; the design is settled here and the risk is the refusal surface,
which is measured first. Class M / STRUCTURAL.
