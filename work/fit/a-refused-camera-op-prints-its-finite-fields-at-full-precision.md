---
id: a-refused-camera-op-prints-its-finite-fields-at-full-precision
kind: issue
title: A refused orbit, pan or dolly renders its finite fields through plain Display, at up to seventeen digits
status: closed
opened: 2026-10-22
priority: P4
cost: E
closed: 2026-10-22
pr: 5604
---


Found by the sweep of `zoom-to-fit-frames-no-committed-profile`'s
passenger, `a-degenerate-frame-refusal-prints-an-aspect-nobody-can-act-on`.

`crates/viewer/src/camera.rs`, `impl Display for CameraOp`: the
`Orbit`, `Pan` and `Dolly` arms write `{yaw}`, `{pitch}`, `{right}`,
`{up}` and `{factor}` through plain `Display`. The sentence is shown
only beside a refusal (`frame::fold_status`, *"camera: … (from
{op})"*), and `camera::apply` refuses these ops only for a non-finite
field or a non-positive factor — so the refused field reads `NaN` or
`inf`, and its finite sibling reads at full precision:
*"orbit by yaw NaN rad, pitch 0.12345678901234568 rad"*. That is the
sentence the passenger row retired for `Frame`.

**Reachability.** No input path in the chrome is known to produce a
non-finite delta today (`input::InputMap::map` divides by a viewport
`aspect()` has already vetted), so this is latent rather than seen.

**What a fix has to decide.** Whether a refused op's sentence names its
fields at all (the error beside it already names the offending one, by
name and value — `CameraOpError::NotFinite { what, value }`), or names
them at readable precision (`readout::number`, whose caller census in
its doc would then grow).

## Closed

PR 5604. The sentence keeps its fields and spells them through
`readout::number`, whose caller census now names `crate::camera` as its
fourth caller (the one that reaches it with `NaN`/`inf` by design).
A refused orbit reads *"orbit by yaw NaN rad, pitch 0.3 rad"* where it
read `0.30000000000000004`. The sweep of every `Display` arm in
`camera.rs` moved `DegenerateScene`, `FieldOfViewOutOfRange` and
`NonPositiveDolly` (each any finite `f64`; `-1e300` was three hundred
and one characters), and the two `NotFinite` arms and
`SceneRadiusOverflowsZoomBand` for one rule (output unchanged:
`readout::number` gives `NaN`/`inf` and the same shortest scientific
spelling `{:e}` did). `Unfittable` keeps its `{:.0}` ratio.
`tests/error_display.rs` pins each sentence with `assert_eq!` on a
float-noise input that plain `Display` would spell in full.
