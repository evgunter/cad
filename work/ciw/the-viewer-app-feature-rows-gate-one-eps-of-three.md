---
id: the-viewer-app-feature-rows-gate-one-eps-of-three
kind: issue
title: The viewer's app-feature rows run at the default eps only, per-PR and nightly, and the red that sat in the gap was found by hand, not by CI
status: open
opened: 2026-09-21
priority: P2
cost: E
---


## Finding

Found by `vgeom/sketch-infinity` after one of its own new rows reded
at `eps = 1e-6` in hosted CI. Running the viewer suite at all three
eps rows locally then turned up a SECOND red that hosted CI does not
see at all.

**The gap.** The viewer's app-gated rows (everything behind
`#[cfg(feature = "app")]` — `viewer::pane`, `viewer::app`,
`viewer::gpu`) run in exactly one place: `ci.yml`'s `viewer` job,
step `viewer app-feature rows (chrome + gpu pipeline smoke)`, which is
`cargo nextest run --profile ci -p viewer --features app` with **no
`CAD_TOLERANCE_EPS`**. The nightly's `full-suite` job runs every eps
row but at default features, so none of these rows is in it; its
`viewer-toolkit` job runs clippy and rustdoc at the app feature and no
tests.

So the app-gated half of the viewer runs at **one** of the three eps
rows every other suite runs at, nightly if not per-PR.

## The red that sat in the gap, and what closes it

`viewer pane::profile::tests::drawing_a_locked_split_circle_above_the_cap_leaves_it_alone`
fails under `CAD_TOLERANCE_EPS=1e-6` and passes at default and at
`1e-12`. Measured twice, by `cargo test -p viewer --features app --lib`
and by `cargo nextest run -p viewer --features app -E
'test(drawing_a_locked_split_circle)'`, with `sketch.rs` at
`origin/main` so no branch change is in it.

```
the document admits a split circle above the form's cap:
  Escalated { site: SegmentPair(0/0, 0/2),
    source: Indeterminate { margin: Value(1.1272608421844756e-6),
      band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 },
      predicate: Some("chord_side") } }
```

It is filed against its own owner as
`work/chrome/a-split-circle-fixture-sits-inside-the-1e-6-escalation-band.md`,
and **the change that carries this amendment is what closes it** —
CHROME's `chrome/split-circle-eps`, which ties the fixture's radius to
the run's ε. The two land together or neither does, so this paragraph
describes `main` exactly when it is on `main`.

**That closes the instance and not this row, and it is worth being
exact about why.** This row's claim was never *"a red is live"* — it is
that the viewer's app-feature rows gate at one ε of three, so a red in
the other two is invisible. The closed instance is the evidence for
that claim rather than a competitor to it: the defect sat on `main`
undetected over a green gate, and what found it was a lane running the
viewer suite by hand at all three ε after CI reded one of its own rows.
Nothing in CI found it, and nothing in CI would have.

## What the by-hand sweep actually covered

The CHROME lane's first pass ran the population at the three gated ε
only. The fix pass re-ran it at **nine**, on the fixed head, and the
distinction between the two populations matters because the first
attempt at this paragraph flattened it:

| population | rows | ε executed |
|---|---|---|
| `--lib -- --skip gpu::` | 151 | 1e-5, 1e-6, 1e-7, 1e-8, **1e-9**, 1e-10, 1e-11, 1e-12, 1e-13 |
| `--test all` | 681 | the same nine |
| `--test all -- --ignored` | 1 | the same nine |
| `--lib gpu::` | 12 | none — no WGPU adapter on the box |

Every runnable row is green at every one of the nine, so **no viewer
app-feature fixture is within a decade of a gated band** — the gap is
empty today. That is a measurement with a date on it, not a guarantee:
it was taken by hand, for the same reason the red was, and the next
fixture to drift into a band will be just as invisible.

**One class of fixture is invisible to that instrument by
construction.** The instrument is "run the population at neighbouring
ε, and an absolutely-scaled fixture sitting near a wall reds at one of
them". It is valid for all 833 runnable rows but one: the fixed row's
radius is now `1e8 * Tol::witness().get().eps`, and an ε-RELATIVE
fixture passes at every ε forever, because the figure moves with the
band. A grep for ε-relative literals across `crates/viewer/src` and
`crates/viewer/tests` returns exactly one hit — that line. (The only
other ε read in a viewer row, `docm9_range_vs_probe.rs`'s
`100.0 * resolution.max(tol().eps())`, scales an assertion THRESHOLD
rather than a figure, so it is not in this class: the geometry it
judges stays put when ε moves.) What guards the ε-relative one is its
own `expect` — a multiplier outside the admissible window reds the row
at every ε — not this sweep. A second ε-relative fixture added later
would be just as invisible, and nothing counts them.

## The fix

The eps rows beyond the default belong in the nightly
(`work/ciw/latency-cut.md`): a step in `nightly.yml`'s `viewer-toolkit`
job running `cargo nextest run -p viewer --features app` at
`CAD_TOLERANCE_EPS=1e-6` and `1e-12`. The `gpu::` smoke needs a Vulkan
adapter; either install lavapipe there as `ci.yml`'s `viewer` job does,
or skip `gpu::` at the non-default rows, since the pipeline smoke is not
eps-sensitive.

## Fence

`.github/workflows/nightly.yml` is CIW's. The red row itself is chrome's
and view's, filed there.
