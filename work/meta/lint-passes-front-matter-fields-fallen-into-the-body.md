---
id: lint-passes-front-matter-fields-fallen-into-the-body
kind: issue
title: work.py lint passes a row whose header fields fell below its closing --- into the body
status: open
opened: 2026-10-13
priority: P3
cost: E
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from PR 4976's review.)

## Finding

`scripts/work.py` `parse_front_matter` ends the header at the first `---` line and returns the rest as the body unread. A row whose author appended `priority:` and `cost:` after `work.py new`'s closing `---`, followed by a second `---`, therefore parses as a header without them and a body that opens with `priority: P3` / `cost: S` / `---`. `lint` passed four such rows in PR 4976 (`work/restfront/validate-window-leads-say-no-length-on-a-zero-arm.md` and its three siblings, since repaired). The fields are silently lost: the board renders the rows with no priority or cost, and the fields escape validation too (all four carried `cost: S`, which is not one of `E/M/H/D`, and lint said nothing until they were moved back into the header).

## Repair shape

Have `lint` refuse a body whose first non-blank lines match `KEY_RE` for a known header key and are followed by a `---` line (a header fallen into the body). The narrower alternative, refusing any `---` line in a body, would trip on bodies that use a Markdown rule.
