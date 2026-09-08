---
id: advisory-monte-carlo-lane-has-no-python-door
kind: issue
title: the E11.1 advisory estimator is ungated on the facade and unreachable from Python
status: open
opened: 2026-09-08
needs_ev: true
---


Found by LIB-B-DISTRIBUTIONS's door sweep over the analysis lane, and
outside that unit's charter: B-DISTRIBUTIONS chartered the analyzed
box, tail mass and leaf mass, and nothing else.

`crates/pncad/src/analysis.rs:98` carries the whole E11.1 advisory
surface **ungated** — `monte_carlo`, `McConfig`, `McReport`,
`McMeasure`, `McAssertion`, `McRefusal`, `DEFAULT_SAMPLES`,
`DEFAULT_SEED` — and the comment above it says why in as many words:
it is pure `f64` replay, it shipped behind `interval` in M10-6's first
pass, that made the advisory lane unreachable in a default build, and
R2's MINOR-9 had it un-gated precisely so "a caller with no certified
scalar still gets the labeled estimate, which is the whole point of an
advisory lane".

A Python caller is exactly that caller and cannot reach it. The wheel
is built from the default feature set, so the certified half is
genuinely absent for it — and the one lane built to serve that case is
present in the crate and has no door.

`crates/editor-core/src/analysis.rs:1065`'s `sample_offset` is the
same row one rung down: it is on the ungated façade list
(`crates/pncad/src/analysis.rs:51`), it is the advisory lane's only
way to draw a parameter value, and it has no Python spelling either.

**Why no census row makes this visible.** `crates/pncad/src/analysis.rs`
is not one of the three files `tests/test_binding_census.py` reads
(its docstring: the three that curate the document layer and the
common surface), so every name on that page is outside the census's
alphabet in both directions. The same blind spot swallowed three of
B-DISTRIBUTIONS's four chartered things; this is what else is behind
it.

What a unit closing this would decide: whether `monte_carlo` crosses
as a free function taking a document plus a config, and whether
`McReport`'s per-measure rows want a value class or a projection —
and, first, whether the advisory lane is a Python surface at all
before `Node.measure` binds, since an MC report is a report over
MEASUREMENTS and B-MEASURES still owns the authoring half.

## Question for Ev (2026-09-08, LIB orchestrator; `[ev]` PR)

The E11.1 advisory lane (`monte_carlo`, `McConfig`, `McReport`,
`McMeasure`, `McAssertion`, `McRefusal`, `sample_offset`) is on the
façade UNGATED precisely so a caller with no certified scalar still
gets the labeled estimate — and the Python wheel is exactly that
caller, with no door. B-MEASURES has since bound measurement authoring
(`Node.measure`, `Node.assertion`), which is what an MC report is a
report over. Also: `crates/pncad/src/analysis.rs` is outside the
binding census's alphabet, which is why nothing reported this. Is the
advisory lane a Python surface, and is its binding LIB's build (the
`keep_out` names the analysis LANE as M10's; the Python door is the
question)?

- **(A) Yes: `monte_carlo(doc, config) -> McReport` as a free function
  with frozen `McConfig`/`McReport`/`McMeasure`/`McAssertion` value
  classes and `McRefusal` as a typed exception under the taxonomy,
  `sample_offset` beside it; and `crates/pncad/src/analysis.rs` joins
  the three files the census reads** (the blind spot closed, with the
  rows it forces stated). LIB's build; the lane's numerics untouched.
  Recommended.
- **(B) Not until the certified half crosses** — leave the advisory
  lane Rust-only and say so at the site; the census still learns the
  file.
- **(C) `sample_offset` only** — the one draw door, no report.

Recommendation: **(A)**; the lane was un-gated for this caller.
