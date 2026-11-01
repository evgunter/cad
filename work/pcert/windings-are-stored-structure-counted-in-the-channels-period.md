---
id: windings-are-stored-structure-counted-in-the-channels-period
kind: issue
title: Unit B2 of the [ev] PR 5142 answer: a harmonic row's windings stored as integers counted in its channel's Period, the chart-pcurve door and check 4's integer gone, and every 'k periods as a length' read through Period::times
status: closed
opened: 2026-10-15
priority: P2
cost: M
closed: 2026-10-20
---


Unit B2 of the `[ev]` PR 5142 answer, after B1 (`Azimuth` and placement).
Ev's constraint on PR 5113: windings are integers counted in the
channel's `Period` (`geom_brep::Period`, PR 5126), in the same home as
`Deck` and the half-period marks, not a separate enum.

## The design

- A harmonic row stores its windings `β` and `σ` as `i32` beside the
  image, structure that `certify` compares exactly. Today `β` is
  `chart_windings`' selection in `{−1, 0, 1}`
  (`crates/geom-brep/src/pcurve_cache.rs`, `fn chart_windings`), read
  per certificate rather than stored.
- A winding becomes a length only through the chart's
  `Period::times`: the image advances `β` periods per period of its
  carrier.
- A joint's `Deck` element is the same kind of integer, decided by
  `Period::decide`.
- So `Period` is the one place a channel's whole periods become
  lengths. `chart_windings`, its `_affine` gates and the chart-arm
  period gate go in its favour, and check 4's whole-period integer
  (`fidelity`'s `branch`) goes once placement is form-identical (B1).

## The second home to sweep: "k periods as a length"

Found by PR 5126's second review (S1). Each of these spells `k·period`
itself rather than asking the channel's `Period::times`:
- `topo::joint::Deck::apply` (`crates/topo/src/joint.rs:151`, `:154`),
  through `shift_polar_branch` and `Pcurve::shift_branch` with
  `u_period(surface)`;
- `topo::pcurves::decide_joint`'s lift
  (`crates/topo/src/pcurves.rs:4213`, `sheet.shift_branch(whole(ku), …)`),
  and `chart_u_period` (`pcurves.rs:1838`) and its callers
  (`:2502`, `:3868`, `:4282`, `:4735`);
- `topo::chord_join::run_azimuth_images`' joint shift
  (`crates/topo/src/chord_join.rs:3237`);
- check 4's `fidelity`, `less_branch`'s `k * tau`
  (`crates/geom-brep/src/pcurve_cache.rs:5176`).

## Closed

Landed in three units:
- B1 (PR 5225): placement over the span in the chart's principal window;
- B2a (PR 5244): windings stored as `i32` counted in the channel's `Period`, compared exactly;
- B2b (PR 5343): check 4 decides no whole period on any lane, `pcurve_fidelity_branch` retired, and every "k periods as a length" above goes through `Period::times`.

What B2b's sweep found outside its fence is filed:
- `whole-period-integers-read-off-a-value-outside-the-s1-sweep.md`;
- `chart-channel-periods-are-spelled-in-several-places.md`;
- `a-lifted-copy-on-a-closed-spline-chart-has-no-placement-term.md`;
- `a-sub-half-period-drift-names-envelope-on-the-focal-and-spiric-lanes.md`, all here;
- `work/foreign/step-import-unwrap-reads-a-whole-period-off-a-value.md`.
