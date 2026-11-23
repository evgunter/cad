---
id: the-bare-rounding-list-rises-in-the-diff-that-adds-its-site
kind: issue
title: bare-rounding-allowlist.sh says its list "only shrinks", but it holds the list only against itself
status: open
opened: 2026-11-09
priority: P4
cost: E
---

`scripts/gates/bare-rounding-allowlist.sh` reds a fallen count (`STALE`,
`:548-551`, "so the list only shrinks") but compares the tree only with
`bare-rounding-sites.tsv` as the same diff leaves it, so a diff that adds a
site and raises its row in the same change passes. The only check on a rise is
the reviewer reading the tsv hunk.

A base ratchet (`gate_base_file`, as `engine_leaves.py`'s `raised` does) is not
worth its friction here. The base-ratchet gates have cost this month: PR 7143's
`# from` split credit in `cover-kind-match`'s pins passed its own PR and turned
main's lint red until PR 7238 (2026-11-20/21,
`a-split-credit-in-a-pin-file-reads-as-a-rise-once-its-pr-merges`), and PR 7183
shaped `Ellipse`'s API to `cover-kind-match`'s `RAISED` rule because the ratchet
had no way to record a re-baseline (`work/germ/log.md`, 2026-11-20). No rise in
this list has been recorded as wrong, and every new row already carries a
one-word reason a reviewer reads.

The repair is the wording: the `STALE` message at `:550` and any header text
that says "only shrinks" say instead that a fall must be recorded and a rise is
a reviewed diff to the tsv, with its reason.

What would close it: the gate's message and header state what it enforces (falls recorded, rises reviewed), with no base comparison added.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): Asks for a merge-base ratchet. Base ratchets cost friction this month: PR 7143's split credit turned main's lint red until PR 7238 (11-20/21).
PR 7183 also shaped an API to `cover-kind-match`'s `RAISED` rule (11-20).
No incident of a wrong rise in this list is recorded. The grain worth keeping is the overclaim in the message `bare-rounding-allowlist.sh:550`.
See `reform/the-bare-rounding-list-rises-in-the-diff-that-adds-its-site.md`.
