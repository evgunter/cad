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

## Re-homed to WIRE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

WIRE is the evaluation seat's successor. `docs/DOC-LEDGER.md` sweep 9
parked three of these rows in `work/issues/` as "the successor's opening
slate" when EVAL closed on 2026-09-08, naming two more already there;
this program is that successor, and it takes the rest of the seat's
ground with them.

Its class at the cut was **E** — two `map_err` sites; may add one
variant to carry inner kind. The class is a dispatch estimate made by
reading the row against the tree on 2026-09-11, not a verdict on the
finding, and a lane that finds it wrong says so in its PR. The id, the
`track:` letter where the row carries one, and the body above are
unchanged by the move.
