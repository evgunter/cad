---
id: code-citations-of-work-rows-go-stale-when-rows-close-or-move
kind: issue
title: a third of the work/ row citations in code name a row that has closed, moved or gone, and only pin citations are checked
status: open
opened: 2026-10-30
priority: P3
cost: E
---


## What

Found by GREEN while writing `scripts/gates/pin-cites-an-open-row.sh`,
which checks only the citations a pinned or disabled red makes. A survey
of every `work/<program>/<id>` path in `*.rs` under `crates/`, `demos/`,
`tools/`, `interval-transcendentals/` and in `.github/workflows/*.yml`
(main at `7a63a39bcc`, grep `work/[a-z0-9-]+/[A-Za-z0-9_.-]*[A-Za-z0-9]`)
resolved 1114 citations:

- 682 name an open (or otherwise not closed) row at the cited path;
- 177 name a row that is closed;
- 188 name an id that now lives under another program;
- 67 name an id that exists nowhere under `work/`, most of them rows of
  programs whose directories were deleted on close.

Not every one is a defect: a closed row cited as the record of how a
behaviour came to be is history, and history is allowed to cite what is
done. But a citation that tells a reader where an open question lives
(`geom-core/src/k_stats.rs`'s module doc: "`topo`'s contact ladder does
not yet (`work/contacthold/contact-verify-logs-a-second-order-escalation-its-outcome-overruled.md`)",
a closed row) sends them to an answer that is not there, and a moved
row's old path is a dead link however it is read.

## Owed

Decide which citations a check should hold (a pin's owner is held by
the gate above; a "does not yet" or "until … closes" clause is the next
candidate), and re-point or retire the moved and missing ones, which
need no judgement: a moved id's new path is the file of that name
elsewhere under `work/`.
