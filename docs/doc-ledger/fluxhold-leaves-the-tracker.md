# FLUXHOLD leaves the tracker — 2026-10-26

FLUXHOLD (FLUX's rows on the D10 hold: assertions, the driver over an
assembly, `measure.rs`) was cut from FLUX on 2026-10-10, `blocked`
with nobody on it. By the 2026-10-26 tracker sweep the hold had
released most of it, and two live rows were left. Its plan said each
row moves to the live program whose charter fits and the track then
closes, so:

- `the-box-driver-carries-no-part-resolver` (P2, still parked on
  `intent-stage3-is-built`) moved to `work/scalar/` (`drive.rs` is on
  its paths);
- `assertion-verdict-derives-partialeq-alone` (P4) moved to
  `work/polish/` (the viewer's `tree.rs`).

Its two closed rows were deleted with it. `crates/editor-core/src/measure.rs`,
its one claimed path, is left unclaimed.

Its plan set no `## Exit criteria`, so it closes without an exit walk.

Recover SHA `72a3110e` (reached from main through this change's merge):

    git show 72a3110e:work/fluxhold/<FILE>
