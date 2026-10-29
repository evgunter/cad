---
id: main-red-the-meeting-axes-pose-refuses-its-endpoint-at-1e-12
kind: issue
title: Main red: the meeting-axes pose refuses carrier_endpoint_end at 1e-12 since PR 5156
status: open
opened: 2026-10-20
priority: P1
---


**Main red.** Filed by CONTACTHOLD's nightly-reds lane from the 2026-10-20 nightly (run 39375288176, head `37d8dfdecf`), which is the first nightly to carry it; the 2026-10-19 nightly (head `07a57312bf`) was green on it.

`crates/sweep/tests/verbs_germarms2_interval.rs` `the_meeting_axes_pose_builds_at_the_certified_scalar` fails at `CAD_TOLERANCE_EPS=1e-12` only:

```
the meeting-axes pose builds: Err(Join(BooleanJoin(Euler(Certification { error: Escalated { check: EndpointEnd, sample: 8, cause: Indeterminate { margin: MarginDiag(Enclosure { lo: 0.0, hi: 1.19288120443679e-12, tag: None, zero: ZeroWord(0) }), terminal_sliver: false, band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("carrier_endpoint_end") } } }))))
```

**Culprit.** A first-parent bisect of main over `07a57312bf..37d8dfdecf`, building `sweep`'s `all` binary and running this row at 1e-12, names `d5163286c8` — the merge of PR 5156 (`germ/circular-axes-band-gap`). Its first parent `fa9acb4dc9` passes; `d5163286c8` and every first-parent commit after it that the bisect built (`33139ddc9f`, `af05a8f717`) fail. Still red on `cd4243bcfd`. The same PR's `53967a14d2` also turned `geom` `curves::tests::ellipse_constructor_trios` red, which ENCL's PR 5451 has already re-pinned.

The same row refused the same way (`carrier_endpoint_end`, an interval split vertex `[0, 1.76e-12]` wide) on TANG's PR 5192 before it merged (`work/ciw/a-sweep-diff-gates-no-eps-row-of-sweep.md`), so the endpoint's enclosure sits at the zero band's edge for this pose and PR 5156 moved it across.

**Done when** the pose builds at 1e-12 again, with the endpoint's enclosure decided rather than the row's pins loosened, and the row is green on main.

**Now red at the default ε too (2026-10-25).** Re-run on main `663dd78fef`, the row fails at the default ε with a different refusal, a `Containment` margin escalation on `bool_ruled_region_span`:

```
the meeting-axes pose builds: Err(Escalated { decision: Containment { decision: None, escalation: Margin }, diag: Indeterminate { margin: MarginDiag(Enclosure { lo: -1.5707963267956369, hi: 4.712388980385431, .. }), .. predicate: Some("bool_ruled_region_span") .. })
```

PCERT's E1 merge of main (`30477d14e2`) fails the same way, so the default-ε red is main's. The done-when now covers every ε.

## 2026-10-26 (SPLICE nightly triage): now red at every ε

On nightly run 40274924857 (main `38f1a24ed3`) this row is red at
default, 1e-6 and 1e-12. At the default ε it escalates
`bool_ruled_region_span`
(`Containment { decision: None, escalation: Margin }`). Bisected at the
default ε to #5664 (`80a6f77af3`). The widening is filed on CHART's
slate, at `work/charttail/main-red-ringed-curved-doors-reds-four-sweep-rows.md`.
The 1e-12 red this row names predates #5664.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. At 1e-12 the pose now stops on `bool_ruled_region_span`, as at the
other two ε, not on `carrier_endpoint_end`. The row is pinned at every ε
on that predicate
(`work/charttail/main-red-the-meeting-axes-pose-refuses-on-a-ruled-region-span-since-5664`).

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured. Known behind the 1e-12 pin: possibly this row's `carrier_endpoint_end`, which the `bool_ruled_region_span` stop now precedes.
