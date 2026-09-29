---
id: contact-verify-logs-a-second-order-escalation-its-outcome-overruled
kind: issue
title: contact_verify's tangency ladder leaves its early second-order escalation on the node log when a definite parallelism defect refuses, or a declaration bridges it
status: open
opened: 2026-09-28
priority: P3
cost: M
---


## Finding

This is the same shape as the renamed refusal in `geom_brep::certify`'s
tangent arm, which is fixed there through
`geom_core::k_stats::splice_superseded`. It was found by the sweep of
that fix and has not been fixed here.

`crates/topo/src/boolean/contact_verify.rs`, the tangency ladder
(steps (3) and (4), ~386–455), decides
`"contact_tangent_second_order"` through the logged funnel FIRST and
acts on it LAST. An in-band reading is pushed onto the innermost open
bracket's escalation log right away (`geom_core::k_stats`
`classify_in` → `record_escalation`). Three outcomes then disagree with
the log, which keeps the second-order escalation first:

- **A definite parallelism defect.** `contact_tangent_parallel`
  decides `Positive | Negative` and the ladder returns
  `ContactRefusal::Contradicted`, which is definite. The log's first
  escalation is still the second-order one.
- **A declared contact.** An in-band second-order reading is the
  bridged residue. With `declared` the ladder returns `Ok(Bridged)`,
  but the escalation stays on the log.
- **An in-band parallelism reading.** With the second-order reading in
  band and `contact_tangent_parallel` in band too, the ladder returns
  `ContactRefusal::Escalated { diag }` carrying the PARALLEL reading.
  The log's first escalation is still the second-order one, so the
  driver names `contact_tangent_second_order` for a refusal whose error
  names `contact_tangent_parallel`.

`crates/editor-core/src/drive.rs` `classify_replay` read (2) lets the
node log's FIRST escalation speak, for a node that built as much as
for one that failed. Where the ladder runs inside a node bracket, the
leaf reads as `SliverTerminal { "contact_tangent_second_order" }` when
the enclosure sits wholly in band, and as a bisect otherwise. A
bridged (successful) declared contact then never certifies a leaf.
The contradicted case names an osculating cause for a first-order
defect.

## Not yet measured

- Whether a driven document reaches this ladder at `Interval` inside a
  node bracket. The boolean contact lane is reached from recipe
  booleans, but no row here drives one.
- Whether a bridged node SHOULD refine. A declaration carries the
  residue by design (CONTACT-DESIGN, the #175 clause). If the driver is
  meant to treat a bridged node as undecided, the second bullet above
  is intended and only the first and third are defects.

## Fix shape

The same fix as the certificate: take the second-order reading
`detached`, and use `splice_superseded` for it once a definite outcome
overrules it, or once the parallel reading's own escalation becomes the
refusal. A red-first row brackets `tangent_locus_relation` at `Interval` on
a pair whose second-order enclosure is in band and whose defect is
definite, then asserts that the log is empty.
