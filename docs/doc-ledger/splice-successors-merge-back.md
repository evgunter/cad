# SPLICECARRIER, SPLICEFACE and SPLICESEAM leave the tracker — 2026-11-02

SPLICE was split on 2026-10-28 (Track size; Ev, in chat) when it measured
91 budget points against 30. Each successor took one witness's thread:
SPLICECARRIER took the NURBS carrier's parameter inverse (teapot wall 3),
SPLICEFACE the spline face read on its own sheet (klein wall 3), and
SPLICESEAM the seam reading between spline walls plus the doors that read
a spline by its kind. None was ever dispatched.

On 2026-11-02 the track budget came to count only the dispatchable P0, P1
and unbanded rows (Ev, in chat). The four tracks then measured 1, 11, 6
and 7 points, 25 against 30 together, so the tracker sweep merged the
three back into SPLICE as threads of its charter. Their rows (5, 11 and
14) moved to `work/splice/` by `git mv`, ids and bodies unchanged, each
with a dated Moved note. Their logs held only the opening entry, and
their plans only an order rule (the witness row first, then P1, then by
band), which SPLICE's plan now restates by thread.

None of their plans set `## Exit criteria`, so they close without an exit
walk.

Recover SHA `82080f7c76a1e08644f69622cb64aae2a2a9cea6`.

    git show 82080f7c76a1e08644f69622cb64aae2a2a9cea6:work/splicecarrier/<FILE>
    git show 82080f7c76a1e08644f69622cb64aae2a2a9cea6:work/spliceface/<FILE>
    git show 82080f7c76a1e08644f69622cb64aae2a2a9cea6:work/spliceseam/<FILE>
