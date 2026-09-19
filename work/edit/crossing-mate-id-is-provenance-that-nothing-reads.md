---
id: crossing-mate-id-is-provenance-that-nothing-reads
kind: issue
title: An interface crossing's mate id is provenance nothing reads: checked at insertion today, deletable tomorrow
status: open
opened: 2026-09-19
---


## The finding

`InterfaceCrossing::Mate { mate, class, outer, inner }`
(`crates/editor-core/src/node.rs`) carries the id of the mate that
crossed the cut. Since PR #2872's fix pass it is a PROVENANCE reference
the insert door checks live (`Node::payload_read_sites` lists it, so a
dangling id refuses `ReadSiteMissingNode`), and a later delete of the
mate is not reported. Nothing READS the id: `eval/wire.rs` carries it
into `NodeErrorKind::CrossingUnverified`'s prose (`mate.0` is printed)
and `crates/pncad-py/src/py/refactor.rs` republishes it as a `NodeId`;
no door looks the node up, no re-verification resolves against it, and
the two references that do the work are `outer` and `inner`.

## Why it may matter

A field that is checked but never read is a check with no reader to
protect. The more harmonious shape may be to DELETE the field: the
crossing then carries exactly the two references the seam needs, the
read-site arm and its fixtures go, and the refusal prose names the
crossing by its `outer` instead of by a mate id. What keeps it today is
the wire: `InterfaceCrossing` is persisted (`serde`, `deny_unknown_fields`),
so removing `mate` is a format change — a decision of its own, and the
persist schema is contended ground per `work/paths/program.md`'s
keep-out convention. Put to Ev on `[ev]` #2869's thread (2026-09-19)
as the one item where a nicer shape may exist.

## What closing it would decide

Whether a crossing's provenance is worth a persisted field; if not,
the wire change (a file that carries `mate` refuses or ignores it —
say which), the deletion of the read-site arm and its rows, and the
refusal prose re-worded to name the crossing by `outer`. Ground:
`crates/editor-core/src/{node.rs, edit.rs, eval/wire.rs, refactor.rs}`
(EDIT, FIX), `crates/pncad-py/src/py/refactor.rs` and `pncad.pyi`
(LIB, by announcement).
