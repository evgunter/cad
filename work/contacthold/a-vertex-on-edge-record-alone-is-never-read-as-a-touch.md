---
id: a-vertex-on-edge-record-alone-is-never-read-as-a-touch
kind: issue
title: A cross-solid vertex-on-edge record is read only when another meeting at its point is, so a pair whose one meeting is such a record can clear at the gate unread
status: open
opened: 2026-10-19
priority: P2
cost: M
---

Filed by RESTREAD's link-reading lane (branch `restread/link-touch-reading`).

`census::sweep_cross_solid_backstop`'s `recorded` maps v-on-f and v-v
contact records to touch sites (`Recorded::Site`), not v-on-e records
(`declared.ve`). Since the link reading, a v-on-e record ties its edge
into its solid's link (`census::Ties`, `Link::at_point`), so it is read
whenever another finding, record or shared key at the point is. Alone,
it is not a site: `declared_only` does not count it, and a pair whose
only meeting is a recorded vertex on an edge is gated as if it met
nowhere.

Adding it to `recorded` was tried in the unit and reverted: a STALE
v-on-e record (its vertex off the edge,
`vertex_on_edge_records::a_vertex_on_edge_record_without_its_event_is_stale`)
then ties an edge that is not at the point, and the link built from it
reads `MixedTouch` beside the stale finding. The record has to be
confirmed (`confirm_vertex_on_edge`) before it ties anything, for v-on-f
records too, which `recorded` reads today on their word.

Owed: read a confirmed v-on-e record as a site, and tie only confirmed
records into a link.

Placed on CONTACTHOLD (RESTREAD orchestrator's fix pass on PR 5331): the
question is how a record is read when nothing else names its point, which is
the D10 hold's ground (`declared-only-meetings-clear-at-the-census-gate-unread`),
not the census's touch reading.
