---
id: the-bare-rounding-list-rises-in-the-diff-that-adds-its-site
kind: issue
title: bare-rounding-allowlist: the list 'only shrinks' but a diff may raise a row beside the site it adds
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Filed by `onecopy/gate-blind-spots`, from its sweep of the gates whose list says it only falls.

## What

`scripts/gates/bare-rounding-allowlist.sh` says its list "only shrinks", but it compares the tree only with `bare-rounding-sites.tsv` as the same diff leaves it. So a change that adds a charge and raises its row's count (or adds a row) passes. The only check is a reviewer reading the tsv diff.

`engine-leaves.sh` holds the same rule against the merge base. `engine_leaves.py` `raised` reads `gate_base_file` (`lib.sh`) and reds a pin above the base unless its line cites a row. Of the gates whose header claims a shrink-only list, `engine-leaves.sh` is the only one that does this.

## Owed

Read the base's tsv through `gate_base_file` and red any (file, scope) row above the base. One exception: a `test` row with reason `oracle`, since a test's oracle is not a charge. Alternatively, take the claim out of the header.
