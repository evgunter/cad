---
id: an-instance-value-carries-its-copies-only
kind: issue
title: An instance's value carries its fused part product beside its copies; it should carry the copies only, with contact records re-keyed per copy
status: open
opened: 2026-10-16
priority: P1
refs: [mates-declare-no-contact, a-placement-is-the-bundle-of-mates]
---


An instance's value (`ValuePayload::Copies`, `crates/editor-core/src/eval/mod.rs` `Copies`) holds two readings of one part. `Copies.bodies` is the part's whole product fused into ONE body, and `Copies.members` holds the per-copy bodies, each keyed by the part placement it is (`CopyKey::Part`). The value is built in `crates/editor-core/src/eval/wire.rs` (`wire_instantiate_part`), and its members in `crates/editor-core/src/eval/parts.rs` (`keyed_copies`).

The fused body is what the root gather grafts, what `read_body` hands a selection on the instance, and what the part's contact records (`NodeValue::contacts`) and declaration rows are keyed against. That keeps the product's records and bits as they were before stage 3 B.

The intent model (`docs/INTENT-STAGE3-SPEC.md` §3, D10) has an instance deliver its copies and nothing else: a construction reads one copy by key. The fused body survives only because the part's contact records are keyed by the fused product's arena, not per copy. That is a named interim of stage 3 B (PR #5232).

Done when:

- the instance value carries the copies only;
- each copy carries the contact records and declaration rows of its own part body, re-keyed to that copy;
- the gather grafts the copies one by one, and a selection on an instance reads its copy;
- the fused body and the single-body acceptance in `read_body` retire.

The re-keying belongs with the stage 4 contact work (`mates-declare-no-contact`), which records every contact between copies at the door.
