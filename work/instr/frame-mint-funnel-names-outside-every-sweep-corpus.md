---
id: frame-mint-funnel-names-outside-every-sweep-corpus
kind: issue
title: Two frame-mint funnel names, one of them production, reach no k-lint sweep corpus
status: open
opened: 2026-09-15
---

## What

Disclosed by FRAME-WITNESS (`docs/FRAME-WITNESS-SPEC.md`, PR 2675) and
scheduled here at its fix pass, on the review finding that a disclosed
gap owes a schedule.

`geom_core::OrthoFrame`'s mints take the K funnel name as a PARAMETER,
so every layer that mints a frame names its own decisions. `docs/K-REPORT.md`'s
roster-change section tables the names that now reach the funnel, and
two of them reach no corpus `scripts/k_probe_sweep.sh` runs:

- **`sketch_plane_frame_norm`** — `crates/pncad-py/src/py/doc.rs`, the
  Python `SketchPlane.from_frame` door. This is PRODUCTION code on the
  binding's value road, and it now decides TWO lengths per call where
  it decided none before: the authored `u`'s own length, and the
  authored `v`'s residual perpendicular to `u`. Both go through
  `OrthoFrame::gram_schmidt` under `Tol::witness()`'s band. No K sweep
  sees either, so the distribution of a shipped door's margins is
  invisible.
- **`fixture_frame_axis`** — `crates/sweep/src/test_support.rs`, the
  fixtures' frame mint. Reached only if a rostered probe module builds
  a fixture plane off non-world axes; none does today.

`tour_frame_axis` is the contrast that makes the gap visible: the
demo-scenes leg runs the scenes that mint it, so a demo's frame
decisions ARE in the distribution while a binding's are not.

## Why it is INSTR's

The roster and the sweep's corpus are this program's ground
(`docs/K-REPORT.md`'s "inventory method, restated" — the roster is
hand-maintained and nothing mechanical catches an omission). The
question is not whether the binding should decide (it should, and
does); it is whether a name the roster carries is one the sweep can
ever read.

## The shapes a fix could take

1. **Bring the binding into the corpus** — a Probe-lane leg over the
   Python guide's own scenes, or a rostered Rust probe module that
   calls the same door under the same site.
2. **Say in the roster that a name is code-reachable and
   behaviourally unreached**, as the `chart_boundary` paragraph
   already does for its own hole, and keep the sweep's coverage claim
   honest rather than widening it.
3. **Retire `fixture_frame_axis` from the roster** if test-support
   names are not roster material at all — which is a question about
   the roster's membership rule, not about this unit.

## Not this

`work/props/exact-frame-mints-cover-three-of-the-world-frames.md` is a
different friction with an overlapping cast: there the complaint is
that a caller has to INVENT a name for an exact frame that needs no
decision. Here the names are earned and the corpus cannot see them.
