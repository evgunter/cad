---
id: lint-passes-front-matter-fields-fallen-into-the-body
kind: issue
title: work.py lint passes a row whose header fields fell below its closing --- into the body
status: closed
opened: 2026-10-13
priority: P3
cost: E
closed: 2026-10-26
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from PR 4976's review.)

## Finding

`scripts/work.py` `parse_front_matter` ends the header at the first `---` line and returns the rest as the body unread. A row whose author appended `priority:` and `cost:` after `work.py new`'s closing `---`, followed by a second `---`, therefore parses as a header without them and a body that opens with `priority: P3` / `cost: S` / `---`. `lint` passed four such rows in PR 4976 (`work/restfront/validate-window-leads-say-no-length-on-a-zero-arm.md` and its three siblings, since repaired). The fields are silently lost: the board renders the rows with no priority or cost, and the fields escape validation too (all four carried `cost: S`, which is not one of `E/M/H/D`, and lint said nothing until they were moved back into the header).

## Repair shape

Have `lint` refuse a body whose first non-blank lines match `KEY_RE` for a known header key and are followed by a `---` line (a header fallen into the body). The narrower alternative, refusing any `---` line in a body, would trip on bodies that use a Markdown rule.

## A live instance, and a patch that would red it (2026-10-26, tracker sweep)

The repair shape above was written and run against `bb25fa4f`: a check
in `parse_front_matter` that refuses a body whose first non-blank lines
are known header keys followed by a `---` line, with a `--selftest`
mutation on `MESH-2`. It passed its selftest and mutation, and on the
real tree it found one live instance:
`work/vseam/notindexed-sentences-are-held-only-by-a-difference-and-a-substring.md`
(its `priority: P3` / `cost: E` sit below the closing `---`, so the
board shows the row unpriced). It was **not** landed, because it would
red `main` on VSEAM's file, which this sweep may not edit. The unit that
takes this row lands the check and that file's two-line repair in one
PR.

## Closed (2026-10-26, tracker sweep)

Fixed: `scripts/work.py` `_refuse_fallen_header` refuses known header keys closed by a second `---` at the top of a body, with a selftest mutation. Its one live instance, `work/vseam/notindexed-sentences-are-held-only-by-a-difference-and-a-substring.md`, had its `priority`/`cost` moved back into the front matter in the same change.
