---
id: open-sign-backstop-row-refuses-section-loop-mixed-at-eps-1e-6
kind: issue
title: reach_volume_backstop's open-sign row fails at eps 1e-6 on main: the scaled oblique rod's boolean refuses Join(SectionLoopMixed) before the backstop is reached
status: open
opened: 2026-10-01
priority: P2
cost: M
refs: [3636, role-resolution-interior-tiers-certify-only-planar-region-faces, point-in-solid-ray-denominators-are-not-lengths]
---


## What

`crates/sweep/tests/reach_volume_backstop.rs`,
`an_open_sign_beyond_the_band_at_the_last_round_refuses`, fails at
`CAD_TOLERANCE_EPS=1e-6`:

    reach_volume_backstop.rs:41: the scaled oblique rod: the boolean refuses:
    Join(SectionLoopMixed { face: FaceKey(9v1) })

- Reproduced on `origin/main` `9fb3de684` (the #3737 reviewer's run)
  and on #3737's fix pass (`reach/pxn-envelope-red`), identically. It
  is green at the default ε. It is a nightly row; no PR's gate selects
  it unless the diff seeds `sweep`'s eps rows.
- The refusal is in `topo`'s boolean join, building the fixture
  (`body_of`, `:41`), so the row never reaches the volume backstop it
  is about.
- `work/reach/reach-volume-backstop-fails-off-the-default-eps.md`
  (closed by #3636) restated this row at `s = 1e12·ε` and `1e13·ε`;
  CONTACT's `point-in-solid-ray-denominators-are-not-lengths` notes
  that at ε = 1e-6 those scales build only by their last bits, with a
  different refusal (`bool_point_in_solid_denom`). This one is
  `SectionLoopMixed`, the shape of ZIP's P0
  `role-resolution-interior-tiers-certify-only-planar-region-faces`
  (a chord-midpoint probe on a curved edge reads both loops alike).
  That it is the same defect is **inferred, not shown**.

## What is owed

Find which scale (`1e13·ε` or `1e12·ε`) and which join tier refuses.
If it is ZIP's chord-midpoint tier, ride with that item and note this
row there. Otherwise file the join defect with its owner. Do not move
the row's scales to dodge it.
