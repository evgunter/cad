---
id: the-curve-pair-reads-point-feet-over-the-span-and-the-pose-pair-held-to-the-spans
kind: unit
title: Fork 275 steps 3b and 3c: point questions read their foot over the span, the sampled search retires, and the pose's polished pair is read as a pair held to the spans
status: open
opened: 2026-11-17
---


Fork 275's build order (PR 7025, decided; C5 in `crates/geom-brep/README.md`) after step 3 (PR 7048, the walk's stop reads its polished pair as a pair held to the spans). Two steps remain before step 4 (the ring rule), each its own PR, held for review. The orchestrator's rulings below cite C5.

## Step 3b: the point foot over the span

**Scope.**
- `Curve3::foot_within(q, span, near)`, with `Curve3::foot` that same search over the whole carrier; the sampled whole-carrier search (`SPLINE_SAMPLES`, `spline_starts`) retires. A non-nearest kind (ellipse, spiric, spline) is polished from its seed clamped into the span, each iterate held to it.
- `Span::read` and `Span::place` read every kind's foot over the span (`Span::foot`). A foot held at an end reads past the span by its chord and the span's walk (`clear_of` over the span), not by a side at `m = 0`.
- Off the carrier stays the whole-carrier lower bound (`clear_of` over the domain).
- `Reading::Carrier` retires from the span reading: whether a point past the span lies on the carrier is the whole carrier's question, which topo's conic containment (`ConicArc::hit`) asks itself.

**Rulings** (orchestrator, 2026-11-17).
- **Span foot for every kind (b), not spline-only (a).** Two exceptions, each a reading of another question, not an exception to C5:
  - a line's and a circle's foot stays the whole carrier's nearest point, unclamped. With the place it is the span's nearest point already, and clamping it in `geom` would be a second reading of the place (an `Interval` box astride an arc's gap: `span_rows::a_box_astride_a_near_full_arcs_gap_is_not_read_past`);
  - the witness and end-candidate feet (`walk::polished_feet`, `pose::foot`) stay whole-carrier: they name where a place lies along the carrier, past the span too (`meet_rows::two_segments_touching_on_far_carriers_are_never_read_apart`).
- **B1, with one restart.** Where the foot's gap does not decide zero, the polish restarts once, from the first piece the Off's whole-carrier walk keeps, where that walk already stops; `On` where the restart's gap decides zero. The certified walk is the one home for where on the carrier the point can be near. A restart from every kept piece ran the walk to its budget on an `Interval` box (`a_tie_across_a_hairpins_branches_is_the_certificates_to_read`: 5,852 escalations against ≤ 64), so it is out.
- **O1 via `TubeRun`.** `pair_section::CarriedComponent::reads` reads a site's foot over the carrier's run of each tube piece the site lies in (`CarriedComponent::runs`, a marched loop's chain boxes and a fitted arc's chart windows, each with its span), the least gap kept (`Foot::least`, F32). The site's parameter is the certificate's place for it, so the sites' order follows from the pieces, not from which stationary point a search lands on. Seeded at `span.0` on the whole carrier, a site within a 0.08–0.13 m padded zero read `On` short of its nearest point and the sites' order broke.
- **Not (c)**: fixed extra starts at the span ends are a second spelling of where to start.

**Stop conditions.** Stop and report where routing containment's carrier flag becomes a design choice (more than one call site, or a change to what containment decides), where the probe or the suite shows the span foot costing verdicts, or where the kept pieces multiply.

## Step 3c: the pose held to the spans

**Scope.** The closed-form pose's polished pair (`pose::Polish::closest`) is read as a pair (`Found::read_pair`) with its polish held to the two spans, as C5 reads a pair candidate. Its window is restated as the region about the plane crossing intersected with the spans; a clamped pair reads the one-sided window the walk reads (half-tangents at the held ends, the curvature over that side), and `Span::past_by`'s reach and run arms retire for the pose. `Span::contact`'s two feet retire for the pose's closest pair. New pose doc text and rows, each red under its mutation; each pose-oracle row that moves gets its cause, and the 30-row refuse probe is reported.

**Ruling** (orchestrator, 2026-11-17): (p2), the option that conforms to C5. (p1), polishing on the whole carriers and narrowing by `past_by`, would revise ratified text to fit the code and answers a wider question than the edges' closest pair. (p3), two polishes, is out.

**Stop conditions.** Stop and report where (p2) needs a decision C5 does not answer (for instance where region ∩ spans is not one connected piece, so the one-sided window's premise fails), or where the probe or the pose oracle shows holding costs verdicts. Either would make (p1) a real `[ev]` question, to be asked with the numbers.
