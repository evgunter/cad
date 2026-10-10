# PIPE log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/pipe/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Ten rows moved in by `git mv`,
each with a `## Re-homed` record: one from `work/issues/`, nine from
`work/code-quality/`.

Every row here is on ground a live program owns, which is why none was
claimed: the finding in each case is that two owned halves disagree, and
the disagreement belongs to neither half. The plan says what that costs —
no `paths`, an announcement per unit, and the two structural rows not
dispatchable without S-BOOL's and CURVED's agreement.

## Seam note from TOPO (2026-09-12): `S79` waits on one row now

`S79`'s `blocked_on` edited by the TOPO orchestrator in its state-sync:
`758` dropped (closed with TOPO's census door, PR 2131) and `759`
dropped (its item closed 2026-09-08; `work.py lint` was already warning
on it). The row waits on `#757` alone. No other change; PIPE decides
whether it re-homes or closes. Signed (TOPO orchestrator).

## 2026-10-10 — the described-net routing pass is done (orchestrator's read)

The routing lane re-found the thirteen sites, and a second pass shaped at
the first one's blind spot found nine more. Its table is in the row's
`## Closed (routing pass)` section.

- **One site is wrong, and latent.** `transform.rs`'s NURBS arms map a
  poisoned net instead of refusing it, and a rotation can turn it into
  the placeholder (probe-confirmed). Filed on SHELF as
  `transform-rigid-maps-a-poisoned-net-into-the-placeholder` (P3, E).
- **The `±∞` note is a real but narrow gap.** Filed on FLUX as
  `net-state-reads-an-infinite-net-as-described` (P3, M, design).
- **Every other site refuses or escalates downstream.** Two exceptions:
  the census reach arm, which is S350's lane, and census arm 1, which is
  correct once S350 lands.
- `r2_probes.rs`'s stale header was fixed as a drive-by.

The lane's stated blind spots:
- 137 `Surface::Nurbs` arms were triaged, not traced one by one.
- Curve-side nets were not covered (`NurbsCurve3` has no `net_state`).
- `props/quad_lane` was judged by reading only.

The curve-side gap is a class, so it is filed on FLUX as
`nurbs-curve-has-no-net-state-door` (P3, M). The other two are
triage depth, not known instances.
