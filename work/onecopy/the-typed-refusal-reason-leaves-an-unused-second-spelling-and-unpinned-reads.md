---
id: the-typed-refusal-reason-leaves-an-unused-second-spelling-and-unpinned-reads
kind: issue
title: PR 6614's typed reason left a second carrier-refusal spelling, a stale walk claim, and reads its tests cannot see
status: open
opened: 2026-11-09
---


Found in the FULL review of PR 6614 (`onecopy/typed-reasons`, head 4771c03b), which merged with these left open. Line numbers are at that head.

**Copies:**
1. `Refusal::of_carrier` (`geom-brep` ladder.rs:150) spells the carrier-refusal standing a fourth time. The rule is escalation→Margin, Exhausted→Unsettled, Unbounded→Limit. The other three spellings:
   - `MeetFault::from(CarrierRefusal)` (curve_pair.rs:652) with `MeetFault::refusal` (curve_pair.rs:180);
   - `curve_face::carrier` (curve_face.rs:553) with `BoundaryRefusal::refusal`;
   - `SiteRefusal::from` (pair_section.rs:1373).

   Nothing calls `of_carrier` or `SpanRefusal::refusal()` (span.rs:192). The standing belongs on `CarrierRefusal` itself, with each wrapper delegating to it.
2. `span::window_near`'s 32-piece cover (span.rs:323-355) is a fourth reading of a point against a span's pieces, outside `Curve3::point_walk`. `point_walk`'s doc (geom on_carrier.rs:355) says it is "the one walk behind `clear_of` and every other reading", which is now false. Route it through `point_walk`, or have it own a stated schedule. The 32 has no stated reason.
3. `CensusUnsupported::CurvedEdge` renders and classifies through a temporary `ContactRefusal::NotCertifiable` (validate.rs:895, 4310), so the typed-then-flattened shape is still there. The siblings `CrossRefusal::Unread(what)` (census.rs:4719) and `census/point.rs:225` still flatten to strings.

**Reads no test sees** (each shown by mutation in the review):
4. The sentence text of `EdgeTangentUnread` (link.rs:589) and the cause's own Display arm (validate.rs:895) can both be rewritten with topo ci green. `a_curved_edge_refusal_reads_back_as_its_reason` (census/curved.rs:524) pins only the classify string.
5. In `passes` (curved_edges.rs:315-333), mapping a place refusal to `Met(InBand)` (graze read as wedge) stays green across topo ci. `a_refusal_names_the_reading_it_rests_on` (span_rows.rs:436) is red only for the span's tagging, not at the door, although its doc says the door. The walk+Past→Clear cell is undisclosed.
6. Dropping the ball radius from `window_near`'s `reach` stays green across geom-brep and topo ci. `a_refused_members_window_on_an_arc_is_its_stretch` uses only point members. Ulp caveats:
   - `radius + esc` rounds;
   - `at(32)` need not equal `hi`.

**Smaller:**
7. `SpanRefusal { of: Place }` can carry the carrier's refusal (span.rs:589-592).
8. `EdgeTangentUnread::OffInterior` is an unreached arm with copy.
9. validate.rs:707 says "four different lanes", but the enum now has five arms.
10. validate.rs depends on `crate::link::EdgeTangentUnread`, so the reason's home sits inside one of its consumers.
