---
id: a-flat-rung-pair-is-read-as-flat-below-it
kind: issue
title: The display fit reads a body flat across one rung step as flat at every finer delta
status: open
opened: 2026-09-12
---


## The residue

`scene::fit_delta`'s ladder stops on `ProbeStop::Flat` when two rungs
in a row count the same: the body did not subdivide across a factor of
at least two in δ, so the 1/δ law is refuted for it over that span and
the fit takes the measured count as the prediction rather than running
a constant through a law the body just disproved. That is what makes
an all-planar document's prediction EXACT — `heat_sink` predicts 152
against 152 drawn, where the law read 1216 — and it is what stops the
ladder crawling down in halvings on a body whose count never moves.

What it takes on trust is the step below the last rung: flat across
[δ₁, δ₀] is read as flat below δ₁ too. The last rung is
`PROBE_FACTOR` × the request, so the extrapolation is one factor of 8,
the same leap the 1/δ law makes in the other direction.

**Measured, it holds everywhere it is used**: all 26 flat rows of
`crates/viewer/tests/display_budget.rs`'s table predict the drawn
count exactly, and the row asserts that equality per row rather than
in aggregate. A body that would break it has to be saturated across a
whole rung step and then start subdividing below it — on the numbers,
a floor mesh over ~15 000 triangles on a document whose answer is
still near the application's δ. The corpus has nothing like it (the
largest floor here is `die_composed_tour`'s 1510).

## Why it is filed rather than fixed

Closing it means either a confirming rung at a δ finer than the
request's own (unbounded in the requested δ — the defect the ladder
exists to remove) or a second model for "how a count responds to δ",
which is the budget law's own question and fenced out of the unit that
found this. A taker should start from whether the budget is a cap at
all (`work/view/the-budgets-predicted-count-is-not-always-an-over-
count.md`): if it is, both questions are the same question.

## Where it came from

The PERF side unit on `fit-delta-probe-can-exceed-the-picture-it-
sizes` (PR "the display probe is never larger than the picture"),
where the flat rule replaced a crawl that paid the same count at every
rung.
