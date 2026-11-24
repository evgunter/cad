---
id: a-body-past-the-session-box-refuses-at-certifications-residual-first-by-its-spelling
kind: issue
title: A body past the session box refuses at certification's residual before the box gate, by the luck of its spelling
status: dispatched
opened: 2026-11-14
priority: P2
cost: M
branch: oneround/box-certify-ask
parent: the-session-box-reach-misses-a-curved-faces-bulge
---


Found by ONEHOME's rim-span lane (PR 6953), whose line-rim spelling exposed it. The defect predates that PR: main passed only by a coincidence of spellings.

**What happens.** A 1e8 m cube is past the session box (`crates/geom-core/README.md`, "The session box": the finished-body gate asks it, and the refusal names the operation). Two rows pin that it refuses there, `OutsideSessionBox`:

- `crates/mesh/tests/newell_probes.rs` `probe_d_huge_offset_tiny_face`;
- `crates/mesh/tests/r2_cert9_probes.rs` `p6_reproduce_the_huge_offset_red_digit`.

But certification runs at each edge's mint, before the finished-body gate. A line rim's carrier is checked against its description (`MappedSource::PlacedSegment`, evaluated as `place(a.lerp(b, s))`, `crates/geom-brep/src/mapped.rs`). At 1e8 m one ulp is `1.49e-8`, already past the ε band, so a carrier spelled any other way than the description misses it by rounding. On main the carrier was `q_from + (q_to − q_from)/|q_to − q_from| · t`, which happened to round as the description does at the sample the check reads, so the residual read zero and the body reached the box gate. With PR 6953's spelling (`q_from + L·(b − a)/|b − a| · t`, the span and direction from the sketch segment) the same check reads `ResidualExceeded { check: MappedSource, sample: 4, margin: 1.4901161193847656e-8 }`, one ulp at 1e8, and the body refuses there instead.

**Why it matters.** A body past the session box should refuse with the box's typed story (the operation, the coordinate past it), not with a certification residual that says nothing about scale, and which refusal it gets should not depend on whether some residual rounds to zero first. At that scale no two spellings of a point can be held within ε (geom-core README W1: width scaling with the distance from the origin is a defect), so every certificate the mint reads there is a coin toss.

**Likely fix.** Read the session box before the mint's certification (the box of an operation's inputs, or of the placed vertices, ahead of the first edge certificate), so a body past it refuses with the box's story whatever its residuals read. Other shapes may serve; this row records the defect, not a choice.

The two probes accept the certification refusal at 1e8 since PR 6953, each with a comment pointing here.

**The plate's walk ledger** (`editor-core` `m10_sym_profile_interval` `PLATE_LEDGER`), re-pinned by PR 7272 and measured on its head after its merge of main through the carrier frames.

What moved, from main's pin:
- Each edge's certification asks the session box of the edge first: one assertion call per certified edge on both assertion walks (`1047 → 1095`) and on `Door/Decision` (`550 → 574`).
- With those calls come the forms of each edge's cover: `Plain/Assertion` `12236 → 15845`, `Early/Assertion` `12686 → 16094`, `Door/Decision` `12378 → 17689`.
- `Plain/Decision` keeps its 979 calls over fewer forms (`33088 → 32611`), and `Early/Decision` keeps its 308 (`22928 → 22700`). The box's ask runs before the checks, so the reads that built a cover's forms before now find them in the walk's memo, charged to the assertion that built them first.
- Frozen does not move (36), and every digest moves.

**The digest moved between the PR's re-baseline (`77c3ed8d`) and its head `34c966f6` with no count moving.** Bisected:
- green at `63df6e8c` (the base merged), red at `2455e2e5` (main merged, at `61403c47`);
- each form's digest logged in build order on both sides. The 38,997 `Plain/Decision` forms are the same multiset (a sorted diff is empty), and two short runs of about 40 forms are built in a different order.

No form changed, only the order in which those reads build theirs, so the new digest is a right answer.

**Re-pinned again after merging the base's fix pass and main (`1329e397`).** The base re-pinned main's move (PR 7434) at `Plain/Assertion` 1051. Each edge's ask then adds its 48 assertions and 24 door decisions, with the forms of the covers:
- `Plain/Assertion` `1051 → 1099` (forms `12382 → 15991`), `Early/Assertion` `1051 → 1099` (`12832 → 16240`), `Door/Decision` `554 → 578` (`12524 → 17835`);
- `Plain/Decision` and `Early/Decision` keep their calls over fewer forms (`33088 → 32611`, `22928 → 22700`), the covers' forms built first and shared;
- every digest moves.
