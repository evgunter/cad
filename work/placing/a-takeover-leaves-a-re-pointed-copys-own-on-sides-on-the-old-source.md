---
id: a-takeover-leaves-a-re-pointed-copys-own-on-sides-on-the-old-source
kind: issue
title: The kernel's takeover re-points a copy's source but leaves the copy's own mates' on sides reading the old source, which the mate door then refuses NotOnTheCopy
status: open
opened: 2026-11-08
priority: P2
cost: E
---


Found by `placing-copy-reads` (the 6353 build), building the re-point offer.

**What.** `Recording::take_over` (`crates/editor-core/src/take_over.rs`) sets each copy's `Source` slot to the result and rebinds the selections that read the copy (and copies of it). A copy's own mates read their `on` side off the body the copy places (`mate::solve::on_reads_the_copy`), so after the re-point every `on` side still reads the old source: the mate is `NotOnTheCopy` at the next evaluation. The accepted re-point meets it on every Place-made copy that has a mate (Designer A's worked example: `C2` rested on `C1`). Traced, not run: whether the retired feature takeover met it too.

**Worked around, not fixed.** The viewer's re-point moves the `on` sides itself after the takeover (`on_sides_follow` in `crates/viewer/src/session.rs`): each `on` selection in the old source is read in the result under the result's successor name. A Python or API caller of `take_over` still gets the stale mates.

**Done when** `take_over` moves a re-pointed copy's own `on` sides with its source (the successor query it already runs answers the names), and the viewer's `on_sides_follow` goes.
