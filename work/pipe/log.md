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
## 2026-10-10 — picked up; the D10 hold checked; the slate re-read

A PIPE orchestrator holds the track (`status: active`).

**The D10 hold.** I read every live row against the hold's covered
ground (TOPO's log, 2026-10-03), and none stands on it; `plan.md` gives
the reasons. The collision is in files, with INTENT's
`booleans-glue-on-zero` (`intent/s4-e-glue-on-zero`, not yet a PR, 56
`crates/topo/src` files):
- in `census.rs` it touches only test fixtures (`GeomSource` stamps);
- in `chart_region.rs` it touches the declared-chart path, away from
  `WITNESS_BUDGET`;
- `boolean/boxes.rs` has a 34-line hunk.

The local rows go ahead and merge main before landing.

**Re-read against main:**
- `S14` and `S70` were already closed (PR 4006 ruling, PR 4022 build).
- `S79` is closed. `#757` is BOXES's
  `boolean-declarations-has-no-geometric-producer`, and D10's
  `declared-pairs-retire` deletes `BooleanDeclarations`. I left a seam
  note on BOXES's log.
- `lane-keeping-at-rest-doors-…` is closed on ATREST-10's evidence:
  LANE-1 took option 2, and the named callers reach certified doors.
- `S350`, `D291` and the witness-budget split are live as filed, at
  moved line numbers.
- Re-priced: `S350` M and `D291` M (legacy `D`), and
  `topo-shared-cores-…` H with `design` set. `S5` also gets `design`.

**Sequencing call:** `topo-shared-cores-hosted-in-one-half` and `S5`
are parked on `booleans-glue-on-zero` and `declared-pairs-retire`.
- The alternative was to run the layering designers now, but the
  pipeline they would weigh is losing its declared rungs this week.
- Extracting a core from a file another P0 lane is halving costs both
  sides a three-way merge.

That leaves 14.5 points dispatchable, against a budget of 30.

**Wave 1** (all four in parallel):
- `S350`: single FULL review. It changes what a census arm answers, a
  permissive wrong answer today.
- `D291` and the witness-budget split, one lane and one PR: single
  STYLE review. One deletes two arms that are unreachable by
  construction and rewrites a comment; the other is a refusal-vocabulary
  split with ordering rule 5 as the claim to falsify.
- `described-net-…` routing pass: orchestrator's read. It files rows
  and changes no code.
