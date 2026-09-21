---
id: the-viewer-app-feature-rows-gate-one-eps-of-three
kind: issue
title: The viewer's app-feature rows run at one eps row of the three the matrix gates, and a live red sits in the gap
status: open
opened: 2026-09-21
priority: P2
cost: D
---


## Finding

Found by `vgeom/sketch-infinity` after one of its own new rows reded
at `eps = 1e-6` in hosted CI. Running the viewer suite at all three
eps rows locally then turned up a SECOND red that hosted CI does not
see at all.

**The gap.** `.github/workflows/ci.yml` has two ways viewer rows run:

- the `test (eps = …)` matrix, which runs the nextest ARCHIVE. The
  archive is built at DEFAULT features, deliberately and with the
  argument written out in the job's own comment ("THE APP-FEATURE
  TEST ROW, AND WHY IT IS NOT `--features app` ON THE ARCHIVE"), so
  everything behind `#[cfg(feature = "app")]` — `viewer::pane`,
  `viewer::app`, `viewer::gpu` — is absent from all twelve of them.
- the `viewer app-feature rows (chrome + gpu pipeline smoke)` step in
  `rustfmt + rustdoc (gate) + wasm32`, which is
  `cargo nextest run -p viewer --features app` and carries **no
  `CAD_TOLERANCE_EPS`**, so it is the default row and only that.

So the app-gated half of the viewer is gated at **one** of the three
eps rows the run otherwise gates everywhere. That is the residue of
`work/chrome/viewer-chrome-not-in-nextest-archive.md` (closed, #1755):
that row bought the rows a seat, and the seat is single-eps.

## The live red in the gap, executed

`viewer pane::profile::tests::drawing_a_locked_split_circle_above_the_cap_leaves_it_alone`
fails on `origin/main` under `CAD_TOLERANCE_EPS=1e-6`, and passes at
default and at `1e-12`. Measured twice, by `cargo test -p viewer
--features app --lib` and by `cargo nextest run -p viewer --features
app -E 'test(drawing_a_locked_split_circle)'`, with `sketch.rs` at
`origin/main` so no branch change is in it.

```
the document admits a split circle above the form's cap:
  Escalated { site: SegmentPair(0/0, 0/2),
    source: Indeterminate { margin: Value(1.1272608421844756e-6),
      band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 },
      predicate: Some("chord_side") } }
```

Filed against its own owner as
`work/chrome/a-split-circle-fixture-sits-inside-the-1e-6-escalation-band.md`;
this row is the reason nobody knew.

## What a fix would have to decide

The eps rows exist because a tolerance is a runtime parameter and a
row can be true at one value and false at another — which this red is
a live instance of. Adding `CAD_TOLERANCE_EPS` to the app-feature step
three times costs three runs of a row whose slowest member builds
every GPU pipeline on a real device, in a job that already holds the
codegen. Cheaper shapes exist: run the two non-default rows without
the `gpu` smoke, or shard the app rows into the `test` matrix behind a
second archive. **Not adjudicated here**, and the measurement the
closed row above refused to pay (a second archive at `--features app`,
+179 MB per leg) is the one number a decision needs.

## Fence

`.github/workflows/ci.yml` is CIW's. The red row itself is chrome's
and view's, filed there.
