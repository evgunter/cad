---
id: main-red-the-meeting-axes-pose-refuses-on-a-ruled-region-span-since-5664
kind: issue
title: Main red: verbs_germarms2_interval's meeting-axes pose refuses on bool_ruled_region_span at Interval since PR 5664
status: open
opened: 2026-10-28
priority: P1
cost: M
refs: [main-red-two-sweep-rows-at-sym-and-the-certified-scalar-since-limb-2, 5664]
---

Handed over by OFFSET (the main-red bisect lane, 2026-10-28), from
`work/offset/main-red-two-sweep-rows-at-sym-and-the-certified-scalar-since-limb-2.md`,
which suspected PR 5728. The bisect says otherwise.

**The row.** `sweep` `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar`
(slow set, so nightly only) refuses at the default ε on `origin/main`
(`96bc411481`):

    the meeting-axes pose builds: Err(Escalated { decision: Containment {
      decision: None, escalation: Margin }, diag: Indeterminate { margin:
      MarginDiag(Enclosure { lo: -1.5707963267956369, hi: 4.712388980385431 }),
      terminal_sliver: false, band: Band { zero: 1e-9, escalate: 1e-8 },
      predicate: Some("bool_ruled_region_span") } })

The margin is `bool_ruled_region_span` — a root's parameter against its
edge's span (`topo::boolean::ruled_region`, the `span(...)` closure near
line 797) — and its enclosure is `[−π/2, 3π/2]`: the root's angle on the
circle carrier is a whole turn wide at `Interval`, so the containment
cannot decide. The `f64` twin (`verbs_germarms2`) is not affected.

**The bisect** (`git bisect --first-parent`, the one row at the default
ε, `-p sweep --test all`):

- green at `8b15425279` (PR 5068, 2026-10-19), `0bbc425d30`, `7af3cdddf6`,
  `ea5ceee5c2`;
- red at `80a6f77af3` (PR 5664, `chart/ringed-curved-doors`, "cylinder
  and cone point-in-face doors read the face's region from its edges",
  which added `ruled_region.rs`), and at every first-parent point
  tested after it, including `b8974faabc^1`, `b8974faabc` (PR 5728) and
  `origin/main`.

**First bad commit: `80a6f77af3`, PR 5664** (CHART, folded into
CHARTTAIL on 2026-10-26). `ruled_region.rs` is also claimed by cleave,
cleavetail, hone, honepair, honepose, onehome and splitdoor; CHARTTAIL
holds it here because the door is the region skeleton CHART landed.

**Owed:** read why the root's parameter enclosure spans the whole turn
at this pose (an angle read through a branch cut, or an `atan2` whose
arguments straddle zero), fix it with a red-capable row at `Interval`,
and turn this row green.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Red at the default ε, 1e-6 and 1e-12, each escalating on
`bool_ruled_region_span` (`Enclosure { lo: -1.5707963267956369, hi:
4.712388980385431 }`). The row is pinned at all three on that predicate;
it goes red the day the row passes.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured. Known behind the 1e-12 pin: possibly `carrier_endpoint_end` (`germ/main-red-the-meeting-axes-pose-refuses-its-endpoint-at-1e-12`).
