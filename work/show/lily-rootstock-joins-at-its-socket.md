---
id: lily-rootstock-joins-at-its-socket
kind: issue
title: The lily's corm and foot union through their declared socket now; the scene still shows them threaded and apart
status: closed
opened: 2026-10-02
priority: P3
cost: E
closed: 2026-10-03
pr: 3910
---


Found by the REACH lane that closed
`work/reach/full-turn-bore-rest-mate-does-not-union.md` (branch
`reach/fullturn-bore-mate`).

Lily wall probe 12 — "thread the corm onto the stem's foot at their
shared cylinder wall (declared cylindrical Rest, no planar contact
anywhere on the mate)" — no longer refuses: `union_with(corm, foot,
flush_declarations(corm, foot))` builds one shell, valid at tier 3,
its volume the two parts' sum. The probe panicked on success as it is
built to, and that lane retired it (`demos/tour/src/lily.rs`'s
`wall_probes`), moving its claim into
`review_probes::the_curved_rungs_declare_the_socket_and_leave_the_stem_glue_alone`,
which now asserts the union.

The probe's retirement text asks for the rest, which is this file:

- **give the plant a joined rootstock**: the `lily_corm` and
  `lily_foot` pieces become one body through that union. That moves
  the lily's rendered frames (one body, one colour where there were
  two), so it renders itself (`local-scripts/render-hosted.sh`);
- **re-derive the two-peg cell's claim about what a cylindrical mate
  needs beside it**: `demos/tour/src/twopeg.rs` was named by the probe;
  no sentence there was found saying a planar `Rest` is required
  beside a cylindrical one, so this half may already be moot — read
  the cell's narration and say so.

## Closed

By #3910. `lily_rootstock` is the corm and foot unioned through their
socket (three cylinder/cylinder `Rest` declarations, no planar contact),
validated at tier 3′; its volume is asserted against the parts' sum and
an independent closed form. The two-peg narration never claimed a
planar `Rest` is required beside a cylindrical one; the false sentence
was the README's lily row, corrected.
