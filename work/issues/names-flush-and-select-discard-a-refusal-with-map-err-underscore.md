---
id: names-flush-and-select-discard-a-refusal-with-map-err-underscore
kind: issue
title: names/flush.rs and names/select.rs discard a typed refusal with map_err(|_| ..), the shape MSOLVE-3 closed in mate/
status: open
opened: 2026-09-06
---


Reported by MSOLVE-3's implementer lane (PR 2081), outside its fence;
filed by the MSOLVE orchestrator. SEAT's ground (`names/select.rs`,
`names/flush.rs` are theirs by DOCM's keep_out).

`crates/editor-core/src/names/flush.rs` and `names/select.rs` carry
`map_err(|_| …)` arms that drop a typed inner refusal and raise a
different kind in its place — the relabeling shape MSOLVE-3 removed
from `mate/*` (`memories/refusal-text-is-not-cause.md`). Whether each
site's replacement kind is the honest one, or the inner kind should be
carried, is the owner's read; the lane's grep is the receipt.
