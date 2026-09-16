---
id: frame-mint-funnel-names-outside-every-sweep-corpus
kind: issue
title: Two frame-mint funnel names, one of them production, reach no k-lint sweep corpus
status: open
needs_ev: true
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

## The question for Ev (2026-09-16)

Reading this row to schedule it turned up something the row did not
know: **`docs/K-REPORT.md` states two different membership rules, and
this row's own table follows the second one.** Separating them is what
leaves a single question worth Ev's time.

**The rule as written.** `docs/K-REPORT.md`'s "inventory method,
restated" says a predicate name is in scope *"if it reaches the
`geom_core::k_stats` funnel — `decide`, `decide_flagged` or
`decide_invariant` — from anywhere **the sweep can execute**, however it
is spelled at the call site."* By that sentence neither name above is
roster material: the sweep cannot execute either one, which is this
row's whole finding.

**The rule as practised**, in the same document, in the
`chart_bound_*` paragraph: *"`chart_boundary` has no shipped caller
until the clearance seam lands, so a `k_probe_sweep.sh` CSV taken at
this merge carries no `chart_bound_*` row — **the roster's code half
reaches them and its behavioural half does not**."* That is a two-half
model: the roster records what the code reaches, and says separately
what today's corpus samples. The frame-mint table's third column
(*"reaches the sweep's corpus?"*) is that model, already applied to
both names here.

**What does not need Ev.** Making the stated rule say what the document
already practises is a reconciliation of two sentences against a
precedent in the same document, and `docs/DESIGN.md`'s companion table
carries `docs/K-REPORT.md` as **Reference**, not Ratified. That half
lands with this unit and does not wait.

**What does.** `tour_frame_axis` is a DEMO's name, rostered and
sampled. `sketch_plane_frame_norm` is PRODUCTION, rostered and
unsampled — plainly roster material under either reading, and the
honest fix is the coverage note or a corpus that reaches the binding.
`fixture_frame_axis` is neither: it is `crates/sweep/src/test_support.rs`,
**test scaffolding**, rostered today and reachable only if a rostered
probe module builds a fixture plane off the world axes, which none
does. So:

**Is a test-support name roster material at all?**

- **(a) Keep it.** The roster records every name that reaches the funnel
  from any code that compiles, scaffolding included, and the coverage
  column carries the rest. This is today's state, and it needs no edit
  beyond the reconciliation above. It also means the roster's size is
  partly a fact about the test suite.
- **(b) Retire it.** The roster is about the kernel's shipped decisions,
  so a name only a fixture can mint leaves the table with a line saying
  why. This makes the roster a claim about the product, and costs a
  sentence of explanation at the seam each time a fixture mints a name.

Both are defensible and neither dominates, which is why it is here
rather than settled in the unit. The rest of the unit — the coverage
honesty for `sketch_plane_frame_norm`, and the reconciliation — lands
either way and does not wait on the answer.
