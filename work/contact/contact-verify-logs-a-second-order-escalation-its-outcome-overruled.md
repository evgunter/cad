---
id: contact-verify-logs-a-second-order-escalation-its-outcome-overruled
kind: issue
title: contact_verify's tangency ladder leaves its early second-order escalation on the node log when a definite parallelism defect refuses, or a declaration bridges it
status: open
opened: 2026-09-28
---


## Finding

This is the same shape as the tangency certificate's renamed refusal,
which the encl branch `encl/driver-log-renamed-refusal` fixed there
through `geom_core::k_stats::splice_superseded`. It was found by that
branch's sweep and has not been fixed.

`crates/topo/src/boolean/contact_verify.rs`, the tangency ladder
(steps (3) and (4), ~386–455), decides
`"contact_tangent_second_order"` through the logged funnel FIRST and
acts on it LAST. An in-band reading is pushed onto the innermost open
bracket's escalation log right away (`geom_core::k_stats`
`classify_in` → `record_escalation`). Two outcomes then overrule it
without taking it off the log:

- **A definite parallelism defect.** `contact_tangent_parallel`
  decides `Positive | Negative` and the ladder returns
  `ContactRefusal::Contradicted`, which is definite. The log's first
  escalation is still the second-order one.
- **A declared contact.** An in-band second-order reading is the
  bridged residue. With `declared` the ladder returns `Ok(Bridged)`,
  but the escalation stays on the log.

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
  is intended and only the first is a defect.

## Fix shape

The same fix as the certificate: take the second-order reading
`detached`, and use `splice_superseded` for it once a definite outcome
overrules it. A red-first row brackets `tangent_locus_relation` at `Interval` on
a pair whose second-order enclosure is in band and whose defect is
definite, then asserts that the log is empty.
