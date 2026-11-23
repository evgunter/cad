---
id: pr-7203-left-the-seat-and-name-slow-pins-red-with-no-row
kind: issue
title: PR 7203 renamed Circle.frame to at and added Ellipse.held, which moved every curve-bearing seat4/7/8 and name-table slow-set digest; it merged with those pins red and no row
status: open
opened: 2026-11-23
priority: P3
---


Found by CIW's census lane (PR 7501) and its review, 2026-11-23.

**What happened.** PR 7203 (`oneround/anchored-conic`, merge `2b9cd8b24c`) renamed a field (`Circle.frame` became `at`) and added one (`Ellipse.held`). The seat digests (`fixture::digest`, which feeds every curve's `Debug`) therefore moved with no body moving: seat4's `die_fillet`, seat7's and seat8's `cut_cylinder` and `kitchen_sink`, and the name-table digests. These pins are in the slow set, so the PR's gate did not run them. The PR merged with the pins red, and no row was filed. The keyed feed main used, run on PR 7501's merge of `2b9cd8b24c`, reads seat4's `die_fillet` as `0x14af7ff18165cee3` against main's pin `0xa968a45c16ee6213`. PR 7501's review traced the move to this rename.

**Repaired by PR 7501.** That PR reads curves and surfaces by their values (`fixture::digest::canonical`), not by their field names, so a renamed field, or an added field holding nothing, no longer moves these pins. It re-cuts the pins on its tree.

**Owed by ONEROUND:** nothing beyond this record. The class is "a representation-only PR runs none of the slow digest pins its spelling moves". It is CIW's `a-diff-that-moves-corpus-ids-runs-none-of-the-slow-set-id-pins`.
