---
id: a-same-kind-pairs-canonical-order-reads-arena-keys
kind: issue
title: A same-kind pair's canonical order reads arena keys
status: open
priority: P3
opened: 2026-11-21
---

A `LocusCertificate` compares its surface pair in order, because the
limbs decide operand by operand and a read replays those decisions in
order (`crates/geom-brep/src/locus_certificate.rs`, `reads`). Check 6
puts the pair in one order before it asks the limbs: by `SurfaceKind`,
and, between two surfaces of one kind, in the order the edge's
`SurfacePair` names them (`crates/geom-brep/src/certify.rs`, the
`let locus = match &resolved` arm).

That fallback is arena-key order. A door that names a plane∩plane or
cylinder∩cylinder edge's pair the other way, such as a partner
operand's copy, misses the kept certificate and derives again. It costs
time only: a miss is a derivation, never a wrong read.

The fix is an order on surface content for two surfaces of one kind,
read at check 6 in place of the named order. No such order exists in
geom today (`Surface::same_content_by` is equality only), so this is not
a two-line change. Found in the confirmation review of PR 7193.
