---
id: an-open-sign-row-reds-main-at-1e-6-with-section-loop-mixed
kind: issue
title: reach_volume_backstop's an_open_sign_beyond_the_band row reds main at CAD_TOLERANCE_EPS 1e-6 again: the scaled oblique rod's boolean refuses SectionLoopMixed
status: closed
opened: 2026-10-01
priority: P0
cost: E
closed: 2026-10-01
branch: reach/opensign-red
refs: [point-in-solid-ray-denominators-are-not-lengths, 3716]
---

## What

`crates/sweep/tests/reach_volume_backstop.rs`,
`an_open_sign_beyond_the_band_at_the_last_round_refuses`, fails on
`origin/main` at `CAD_TOLERANCE_EPS=1e-6`. It passes at the default ε
and at 1e-12. The failure is at :41 (`build`):

    the scaled oblique rod: the boolean refuses: Join(SectionLoopMixed { face: FaceKey(9v1) })

## Who measured it

- Hosted CI on EDIT's PR 3676, run 36920139603 (the PR merged into
  main at `4c95f504c`) and run 36927574529 (merged into `97d9e3380`),
  eps step at 1e-6. That PR does not touch `sweep`, `topo` or
  `geom-*`.
- EDIT's fix lane reproduced it on a clean `origin/main` (`97d9e3380`),
  in its own worktree and target dir:
  `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep -E 'test(/an_open_sign_beyond_the_band/)'`.

## History

This is the row's third shape at 1e-6:
- `reach-volume-backstop-fails-off-the-default-eps` (closed by
  PR 3636) restated it at s = 1e12·ε and 1e13·ε;
- `work/contact/point-in-solid-ray-denominators-are-not-lengths.md`
  records that it then built by the last bits of its scale.

It now refuses earlier, in the boolean's join, not the point-in-solid
denominator, so something on main since PR 3636 moved the rod's
section. That is not bisected here. Candidates by date are the
convex-step change (#3524) and the LINALG door adoptions (#3725,
#3727).

## Final state

The row holds at every ε the CI runs, or names and pins where its
verdict changes with ε. Main's eps step is green for a diff whose
eps filter is `all()`.

Filed by the EDIT orchestrator. Every PR whose eps filter reaches
`sweep` is blocked by it.

## Closed (2026-10-01)

**Bisected** on main's first parents at ε = 1e-6, from `03411d3699`
(the first first-parent commit holding PR 3636, green) to `6000ec92d8`
(red): the first bad merge is `3cedf18afb`, PR 3716 (CLEAVE, one
cell-dimension witness ladder for section-loop roles). PR 3627's head
`bb36327908` is green only because it predates PR 3716; merged with
`6000ec92d8` it is red the same way.

**The refusal was wrong, and PR 3716 only exposed it.** The rod's
null face 9 has two loops. The cutter-side loop reads `Out` at its
first vertex. The rod-side loop is the disc the cutter's bottom face
cuts inside the rod. Its vertices and edge midpoints all lie on the
rod, so it reaches the face-interior tier, whose witness is the disc's
centre `(0, 0, 3.5e7)`, on the rod's axis. `point_in_solid` read it
`Out`. Before PR 3716 the vertex tier resolved the cutter loop and never
asked the second loop, so nothing compared the two.

The wrong verdict was the ray caster's axis-parallel rung,
`sin²/2r` against the band: at ε = 1e-6 it cannot exceed ε on a wall
of radius 5e5 or more, so every schedule ray skipped the rod's wall
(`point-in-solid-ray-denominators-are-not-lengths`, ledger row F2). It
skipped no wall at the default ε and at 1e-12 for this row's scales.
That is why the row was green there.

**Fixed** in its general form, ledger row F2 entire. Both skip questions
(the plane arm's `d·n̂` and the wall's axis-parallel rung) are levered
by how far from the query the selection reaches, so a skipped ray
provably meets no point of the face. The wall's discriminant takes the
sphere arm's length form. The wall's hit-outward sign is read off the
discriminant's root order. The row is unchanged and reaches its
subject, `VolumeUndecided` at both scales, at the default ε, 1e-6 and
1e-12.
