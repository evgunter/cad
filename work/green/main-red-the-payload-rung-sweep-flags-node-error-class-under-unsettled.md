---
id: main-red-the-payload-rung-sweep-flags-node-error-class-under-unsettled
kind: issue
title: Main red: the payload-rung sweep flags NodeErrorClass under drive's Unsettled, since PR 7458
status: open
opened: 2026-11-23
priority: P0
cost: E
---


Lint's `gates (scripts/gates)` step runs `scripts/payload-rung-sweep.py --check`,
and on main it exits 1 with one line:

```
NEW payload rung, curated on no list that carries its carrier: NodeErrorClass at crates/editor-core/src/eval/class.rs:41 under Unsettled (crates/editor-core/src/drive.rs:636). Decide it — carry it where its carrier is carried, or argue the non-carriage beside that carrier — and add it to CROSS_LIST_DISPOSITIONS with the home of the argument.
```

`Unsettled::Failed { class: crate::NodeErrorClass, cause: String }`
(`crates/editor-core/src/drive.rs`, `enum Unsettled`) came in with
`581b4e1090` ("split-reads fix pass"), merged to main by PR 7458
(`boxfinish/split-reads`) at `50c8275810`. `gate ok` is red on every PR
whose test merge holds that main.

Re-run here on main `75c8ac8d4b` alone (the script, locally): still
red, same line. Also seen on PR 7400's run 45168956268, and reported in
`work/boxfinish/log.md` (from EMBED, on main `dc263f8553`),
`work/formwords/log.md`, `work/germ/log.md` and `work/green/log.md`
(from PROFILE), none of which is a row.

The fix is a disposition, BOXFINISH's to decide (`drive.rs` is
BOXFINISH's and SCALAR's ground): carry `NodeErrorClass` on the list
that carries `Unsettled`, or argue beside `Unsettled` why it is not
carried, and add the entry to `CROSS_LIST_DISPOSITIONS` in
`scripts/payload-rung-sweep.py` naming where the argument lives.
