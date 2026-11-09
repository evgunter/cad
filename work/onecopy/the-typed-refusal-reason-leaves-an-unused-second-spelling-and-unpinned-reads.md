---
id: the-typed-refusal-reason-leaves-an-unused-second-spelling-and-unpinned-reads
kind: issue
title: PR 6614's typed reason left a second carrier-refusal spelling, a stale walk claim, and reads its tests cannot see
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6648
branch: onecopy/carrier-standing-2
---

Found in the FULL review of PR 6614 (`onecopy/typed-reasons`, head 4771c03b). Items 1, 2, 4, 5, 6, 8, 9 and 10, and item 3's `CensusUnsupportedCause::CurvedEdge` part, landed on `onecopy/carrier-standing`. Two remain:

3. **A census meeting refusal still flattens to a string through the contact lane.** Three sites render a `geom_brep::curve_pair::MeetFault` (or a sphere reading's refusal) as `ContactRefusal::NotCertifiable { what }`. That renders as "this contact cannot be certified (…)" and is classed as the contact lane's "the kernel cannot yet check a contact between faces of these kinds":
   - `census.rs` `CrossRefusal::Unread(&'static str)`, filled from `MeetFault` in `meetings` (the `meet` call's `map_err`);
   - `census/curved.rs` `refuse_meeting`, filled from `MeetFault` in `interior_meeting`;
   - `census/point.rs` `refused`.

   `MeetFault`'s refusing arms (`Unsettled`, `Unwalked`, `Unsupported`) have no sentence or recourse of their own. The typed fix is a cause arm that carries the fault (or a census reason over it), and it needs that vocabulary first. `Unwalked` already has its walk's sentence and recourse.
7. **`SpanRefusal { of: Place }` carries the carrier's refusal where both refused** (`curve_pair/span.rs` `Span::read`, `refused: on.err().unwrap_or(refused)`). The struct's `of` says which reading the answer rests on. Its `refused` is then a different reading's. Choosing which refusal a place refusal carries, or carrying both, is a ranking. It is a design question, not a cleanup.
11. **The cross-solid backstop reads a curved-edge touch without its reason** (`census.rs` `TouchUnread::undecided`). It sends `EdgeTangent(_)` and `OffInterior` alike to `Undecided::TouchEdgeTangent`, which renders "There is no way through yet for this shape". That has two consequences:
    - `OffInterior` is an abort, two readings of the touch that disagree, so its honest ending is a kernel defect;
    - `EdgeTangent(Unwalked(Exhausted))` should end with the walk's recourse (`EdgeTangentUnread::recourse`).

    `TouchUnread::sentence` already renders both by their own recourse (PR 6629). The backstop's `Undecided` arm carries no reason, so it can't. Fixing it means either carrying the reason on `Undecided::TouchEdgeTangent`, or moving `OffInterior` out of the undecided touches (`Undecided::is_undecided_touch`). The second changes what the backstop waits on, so it is a semantics question.
