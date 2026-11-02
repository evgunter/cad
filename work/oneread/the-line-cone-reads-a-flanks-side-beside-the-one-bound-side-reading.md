---
id: the-line-cone-reads-a-flanks-side-beside-the-one-bound-side-reading
kind: issue
title: The line cone reads a flank's side of a face plane at first order (link_plane) beside the one bound-side reading (bound_read, with its descent)
status: open
opened: 2026-11-02
priority: P1
cost: M
design: true
refs: [recl-edge-edge-composes-the-other-solids-material-beside-the-cone, the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways]
---


Found at the merge of `resolve-edge-sector-reads-the-other-solids-face-about-the-line-by-table-ii`'s lane with main after TANGRIM's PR 6253 (2026-11-02).

Since PR 6253, a bound's side of a face has one reading in the kernel: `boolean::sectors::bound_read` (TANGRIM's `the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways`), first order charged, and one order down where a bound ties at first order. `resolve_edge_sector` keys its flanks through it (`flank_keys` / `flank_read`).

The line cone at `recl::resolve_edge_edge` (PR 6282, `boolean::sectors::line_fans` read by `link::Cone::reads_tied`) reads each flank's side of the other solid's flank planes through the cone's own `link_plane` distance, at first order: a second reader of the same side, which ties where `bound_read` descends. A held face's side already enters the cone given (`reads_given`, from `PierceDatum::side`); flank sheets do not. TANGRIM's log (2026-11-01/02) already says stage 2's descent "belongs in the line cone" there.

Owed (a design question — weigh before a lane builds it, with TANGRIM): one reading of a bound's side of a face at both recl sites. Candidates include giving every flank sheet its side from `bound_read` (the cone then reads parity over given sides, and `Cone::crossing` must be consistent with a given side, as PR 6247's F5 noted), or making `bound_read` the cone's plane reading for bound points. A witness: an undeclared curved flank pair whose bounds both tie a reference plane at first order (main decides it by the descent at the edge-sector site; the edge-edge site's cone reads it On).
