---
id: an-interval-endpoint-can-be-read-without-asking-whether-it-certified
kind: issue
title: An interval's endpoint can be read without asking whether it certified, so every reader must remember is_certified() by hand and a census counts who forgot
status: open
opened: 2026-11-22
priority: P1
cost: H
design: true
refs: [ring-refusal-readers-are-spelled-by-hand-at-every-site, census-and-digest-pins-cost-a-fix-a-day-and-caught-eight-real-defects-in-a-month]
---

Filed by the CIW orchestrator, 2026-11-22, at Ev's request in chat ("make
it so you don't have to explicitly remember to call is_certified"). It is
filed on SCALAR's slate because the certified-door family and its readers
are SCALAR's ground.

**The hazard.** Since RING-2 (PR #3032), a refused certification bracket
carries its refusal in its decoration (`dec < Def`) and keeps **ordinary
endpoints**. `.lo()` and `.hi()` read those endpoints with no question
asked, so a reader that compares one takes the certifying branch on a
value that does not certify, unless it calls `Interval::is_certified()`
first. Nothing in the types makes it ask. `Real::is_poison` does not ask
the same question (`crates/geom-core/tests/certified_endpoint_census.rs:1-20`).

**What it costs today.**
- `certified_endpoint_census` pins per-file counts of endpoint reads in
  order to catch a reader that forgot. In the month to 2026-11-22 it went
  red on main 15 times. Three were real: reads that skipped
  `is_certified()` (`along_side`, seven endpoint reads in PR 6040,
  `node_frame`, `deciding_walk`). The other twelve were count re-pins
  (`work/ciw/census-and-digest-pins-cost-a-fix-a-day-and-caught-eight-real-defects-in-a-month.md`).
- The readers that do ask are spelled by hand at every site
  (`ring-refusal-readers-are-spelled-by-hand-at-every-site`: twelve copies
  of two three-line bodies).
- EMBED has two open rows of the same shape, reads that do not ask
  (`work/embed/continues-reads-its-directions-product-at-its-lower-end-without-asking.md`,
  `work/embed/one-plane-reads-a-brackets-ends-as-a-point-without-asking.md`).

**The ask.** A reader should be unable to get a one-sided endpoint of a
bracket that may not certify without having handled the refusal. For
example, the plain endpoint read could go through a certified view
(an `Option`/`Result` or a witness type), with an explicitly named
unchecked escape for the sites that are dispositioned by hand. The shape
is a design question with several viable answers: what the escape is
called, what to do with the endpoint reads that are legitimately
refusal-agnostic (display, widening, hulls), and how many of the call
sites move. So `design: true`, weighed by designers first
(`memories/orchestration-model.md`).

The census then shrinks to the escape's call sites, or goes away. The
hand-spelled readers collapse into the view. Both rows above close or
narrow with it.

What would close it: forgetting the refusal at a new read site is a
compile error or an explicitly named escape, and
`certified_endpoint_census` no longer pins counts of ordinary reads.
