---
id: pierce-ring-has-no-join-arm
kind: issue
title: A pierce RING has no join arm on any carrier: three typed doors, one missing lane
status: open
opened: 2026-08-30
github: 1291
refs: [347, 1068]
---

## From GitHub issue 1291

Opened 2026-08-30; 0 comments.

A **pierce ring** — the empty loop `vtxfac` mints inside a pierced face,
carrying only null-edge scaffolding — has no join arm on any carrier.
Three typed doors are the same missing lane wearing three names, and
none of them is a bug in the layer that reports it.

## The three doors, measured

| carrier | fixture | door |
|---|---|---|
| planar cap | a box driven through a cylinder CAP (`verbs_pierce.rs`, shipped by #1068) | `SplitJoinError::SectionLoopMixed` |
| cylinder wall | a bar driven through a cylinder WALL (`verbs_germarms.rs`) | `SplitJoinError::SectionArcWindow { case: NoChartedRun }` |
| cylinder wall, asymmetric pose | an off-centre bar, same half-wall (R2's probe) | `SectionArcWindow { NeitherContained }` *(see the diagnosis below — this pose now stops one layer earlier)* |

**Why they differ, and why neither reporting layer is at fault.**
`NoChartedRun` fires because `run_azimuth_window` skips null
scaffolding by contract ("zero-length, no azimuth extent"), so a run
made of nothing else leaves the divided face windowless — every time,
by construction. `SectionLoopMixed` fires because a ring's section loop
has no above/below-paired boundary to join at all; its variant doc says
"kernel bug, loudly", and that sentence is correct for every other way
of arriving there. Both docs have been amended in place to name the
pierce ring as a legitimate typed destination pending this unit, rather
than leaving a falsified sentence standing (VERBS-GERMARMS PR-1).

## The `NeitherContained` diagnosis

Asked for by the PR-1 review: is the asymmetric pose's
`NeitherContained` mis-bookkept run/chord pairing, or honest
ill-conditioning?

**The site already answers it in prose** (`chord_join.rs`, the
azimuth-window rows): *"The chord's start lying in the window is a
consequence of the run's own geometry, not an assumption: a run that
does not actually end where this chord starts fails the x₁ rows and
lands in `NeitherContained`."* So on that pose the run handed to the
arc-side rule does not co-bound the face with the chord it was paired
with — a PAIRING question, not a degenerate window.

**It is not currently reproducible**, and that is itself the finding:
after PR-1's curvature charge landed, the same off-centre fixture stops
one layer EARLIER, at `CurvedSectorSideUnsupported` — the sector-side
verdict on that pose cannot be certified against the wall's curvature,
so the crossing layer never hands the join anything. The pairing
question is therefore parked with its evidence rather than answered:
this unit owes it a fixture that reaches the join on an asymmetric
pose, and the run/chord pairing is the first thing to read there.

## Scope

- Give a pierce ring's run its own chord lane, on both carriers — a
  planar face's ring and a wall face's ring are the same topological
  object and should not be two separate arms if the walk can be shared.
- Re-read the run/chord pairing on an asymmetric pose (above).
- Retire the two amended doc paragraphs when the arm lands.

## Consumers waiting

- #347's union half: a wall pierce reaches the join and stops there.
- VERBS-GERMARMS PR-2 (the cyl×cyl germ arm) — its own chord lane is a
  different question, but a Steinmetz union that mints ring vertices
  needs this one too.
- The planar cap row in `verbs_pierce.rs`, which has been refusing at
  the join since #1068.

## Home

`work/verbs/` — the missing lane is the germ-arm/pierce ground VERBS' charter claims (Wave 2's curved boolean breadth), and S-BOOL's keep_out cedes the germ arms to VERBS explicitly.
