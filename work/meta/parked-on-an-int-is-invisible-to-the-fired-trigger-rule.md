---
id: parked-on-an-int-is-invisible-to-the-fired-trigger-rule
kind: issue
title: A parked row whose blocked_on is a PR/issue number is invisible to the fired-trigger rule
status: open
opened: 2026-09-06
---


**Filed by the LIB orchestrator, 2026-09-06, onto META's slate per
Ev's 2026-09-04 ruling (file straight onto the owner).**

`work/README.md`'s fired-trigger rule ("every blocker closed — a lint
ERROR") resolves item ids only; `blocked_on` also admits ints, which
are PR or issue numbers and "are not checked". A row parked on an
int therefore never reds when its trigger fires. The instance:
`work/lib/LIB-G17.md` sat `parked` on `blocked_on: [1202]` for two
days after `work/shell/shell-needs-shellnaming-birth-channel.md`
(the tracker file for that number, `github: 1202`) closed on
2026-09-04, and was found by a SEAT lane reading by eye
(`work/lib/lib-g17-is-parked-on-a-fired-trigger.md`, now closed).

Two shapes of fix, META's to choose: (a) lint resolves an int in
`blocked_on` against every item's `github:` field and applies the
fired-trigger rule when the match is closed, treating an int that
matches nothing as unchecked as today; (b) lint refuses an int in
`blocked_on` on a `parked` row when an item with that `github:`
exists, telling the author to name the item. (a) catches the case
silently-later, (b) at filing time.
