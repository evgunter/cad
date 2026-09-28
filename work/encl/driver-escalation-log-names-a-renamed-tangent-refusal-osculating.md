---
id: driver-escalation-log-names-a-renamed-tangent-refusal-osculating
kind: issue
title: the driver's escalation log still names the second-order cause for a tangent refusal the certificate renamed to TangentParallel
status: dispatched
opened: 2026-09-28
priority: P3
cost: M
---


## Finding

PR 3334 made the tangency certificate name a definite first-order
defect as the refusal's cause even when the second-order margin
refused first. That fixed the typed error, `CertifyError`, but not the
second channel the subdivision driver reads, so there the row
`interval-jet-hulls-kappa-sign-at-a-right-angle-crossing` survives.

**Trace.**

- `crates/geom-brep/src/dihedral.rs`, `tangent_second_order` (~288), decides
  `"tangent_second_order"` through `k_stats::decide`. An in-band
  outcome, for example the `Interval` enclosure `[0, 0.1]` from the
  hulled `σ₂` at a right-angle crossing, is pushed onto the open
  bracket's escalation log by `crates/geom-core/src/k_stats.rs`
  `classify_in` → `record_escalation` (~353).
- `crates/geom-brep/src/certify.rs`, `run_checks`, the `Resolved::Tangent`
  arm, the `renamed` reading (~1952), then reads the parallelism defect
  at the folded arm. If it is definite, it returns
  `ResidualExceeded { TangentParallel }`, but the second-order
  escalation is already on the log.
- `crates/editor-core/src/drive.rs`, `classify_replay`, reads the log
  before the error enum (read (2), ~1635). The FIRST escalation
  speaks, so the node is classified by `indeterminate` (~1716) against
  `tangent_second_order`'s enclosure. Where that enclosure sits wholly
  in the band (`sliver`, ~1739), the leaf becomes
  `SliverTerminal { predicate: "tangent_second_order" }`, which is the
  osculating cause for a defect that is first-order. Where it does not,
  the leaf bisects. That matches the enum read, since a failed node
  with an empty log also bisects (`_ => return LeafVerdict::Bisect`,
  ~1659), so the misnaming is confined to the sliver case.

**Sibling (a), the same PR.** At `Interval`, a node whose second-order
margin is a definite `Zero` and whose first-order fallback reading is
in-band now carries a `tangent_normal_parallel` escalation. Before, its
log was empty and it read `Bisect`. The typed error is still
`NotSecondOrderSeparated`, but the log can now turn the node into
`SliverTerminal { tangent_normal_parallel }`. Both directions are
conservative: every arm is refused mass. What is wrong is the NAME the
driver reports.

## Why it was not fixed in PR 3334

Neither cheap spelling is sound:

- Dropping the second-order escalation from the log when the refusal
  is renamed needs a new `k_stats` door that splices a detached run's
  verdicts and samples but not its escalations. That is a `geom-core`
  API whose effect on the verdict-diff engine's populations needs its
  own measurement.
- Having the driver prefer the typed error means recognizing a definite
  `CertifyError::ResidualExceeded` through the ~40 op-error wrappers,
  which read (2) exists to avoid.

## Fix shape

Either option is its own unit:

- **Rule in the recorder.** A refusal-naming reading that decides
  definitely supersedes, on the log, the escalation it renamed.
- **Rule in the driver.** A node's escalations are read in decision
  order but a later DEFINITE certification verdict for the same sample
  wins.

Either way, the row is the `Interval` cap crossing with the
second-order enclosure narrowed into the band. It asserts the driver's
`SliverTerminal` predicate is `tangent_normal_parallel` (or that the
leaf is refused as a mismatch), not `tangent_second_order`.
