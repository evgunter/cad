# CONTACT-6 — no false Out inside a tilted-cut cylinder cavity

**Binds one implementer lane.** Deleted at merge; `work/contact/CONTACT-6.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `contact/6-cut-cavity`, cut from `contact/land-4` (CONTACT-4's
landing head, PR #3345). Merge `main` in once #3345 lands. The row this
unit carries is
`work/contact/point-in-solid-reads-out-inside-a-tilted-cut-cylinder-cavity`.
Read all of it.

## What is wrong

`point_in_solid` answers `Out` for points inside the lower half of this
body:
- a brick with a cylindrical rod subtracted (a cylinder cavity, whose
  wall has reversed sense);
- split by a plane at tilt 1.0.

`validate_geometric` passes, and the volume is exactly half.
A CONTACT-3 reviewer measured 298 false `Out` at CONTACT-3's head (353
at its base), on a 9³/11³ grid clear of the boundary at 6 rigid poses.
Example: `(−1.422, 1.113, 0.128…1.628)` at the identity pose.

Forcing CONTACT-3's `Unsupported` wall arm to refuse every hit left the
count unchanged, so the cause is not the wall arm. The reviewer
suspected the planar arm on the cut face, which is bounded by lines and
ellipse arcs.

## The work, in order

**1. Measure first, on CONTACT-4's head.** CONTACT-4 moved `contfp`
onto the carrier walk and made ellipse readings two-sided. Rebuild the
fixture (build it through public doors: a subtract, then a split) and
count false answers against an independent ground truth, at the six
poses, at base (`main` before #3345) and at CONTACT-4's head. Ground
truth: analytic membership in (brick minus rod) intersected with the
cut half-space, on points clear of every boundary by at least 1e3·ε.
- If the head has 0 wrong, the row is fixed by CONTACT-4. Add the grid
  as a row, say which change fixed it (bisect it by reverting the
  pieces), and stop.
- Otherwise go on.

**2. Trace one false `Out` to its cause.** Follow the ray lane
(`boolean::solid_contain`): which rays are cast, which faces each hits,
what each hit reads, and where the parity goes wrong. Name the
defective site with `file:line` and the invariant it breaks. Do not
guess from the population; trace a concrete probe.

**3. Fix it at its home, then sweep its class.** The fix must never
answer where the evidence is short: refuse typed and confined where
the true answer cannot be decided. State the argument at the site.
Then look for the same shape in the sibling sites (other face kinds,
other doors reading the same construction) and measure or file each.

**4. Report back before landing** with the trace, the fix, and
head-vs-base counts, so the orchestrator can set the review tier.

## Rows

- The cavity grid at the six poses, at all three eps rows: 0 wrong.
  Report refusals against base.
- A row pinned to the concrete probe you traced.
- An upper-half control, and an uncut cavity control, answering as
  before.

## Discipline

- Your own clone and your own `CARGO_TARGET_DIR` at
  `/home/user/contact-6-target`, with `CARGO_INCREMENTAL=0`.
- Every cargo command goes through
  `local-scripts/with-build-slot.sh -- <cmd>`.
- Check `df` first; stop if under 4 GB.
- No doc-gate script; rustdoc with the flags the implementer discipline
  names.
- Foreground only, or `setsid` with polling to completion. No pattern
  kills and no process listings. Never list the shared scratchpad.
- Push the branch only; no PR.
- Before hand-back, run the `editor-core` concision rows and
  `-p test-utils` if you add a refusal variant, user-facing text, or a
  source-reading test.
