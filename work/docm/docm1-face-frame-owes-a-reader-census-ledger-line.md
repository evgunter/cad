---
id: docm1-face-frame-owes-a-reader-census-ledger-line
kind: issue
title: main is RED at TIER=all: docm1_face_frame.rs reads Rust source and owes a reader_census ledger line
status: open
opened: 2026-09-04
refs: [1850]
---


## What is red

`test-utils::reader_census every_site_that_reads_rust_source_is_in_the_ledger`
fails on any PR whose merge ref includes current `main`:

```
thread 'every_site_that_reads_rust_source_is_in_the_ledger' panicked at
  crates/test-utils/tests/reader_census.rs:538:5:
the source-reader ledger no longer matches the tree.
  arrived or moved (owe a ledger line): [
    "crates/editor-core/tests/docm1_face_frame.rs",
  ]
  listed but no longer reading source (delete the line): []
```

Seen on run [33905591338](https://github.com/evgunter/cad/actions/runs/33905591338)
(PR 1850, branch `ciw/unsample-klint`), where it reddens **all six
`test (…, 1/2)` shards** — the three ε points on each of the two lanes,
because they all replay a shard of the same archive — and `gate ok` with
them.

## It is not that PR's

`crates/editor-core/tests/docm1_face_frame.rs` does **not exist** on
`ciw/unsample-klint`; it is on `main`, added by `17bb8fb1` ("DOCM-1 fix
pass", 2026-09-04 18:08). The hosted `pull_request` event checks out
`refs/pull/N/merge`, so every open PR's run tests
`merge(its branch, main)` and inherits this. The same branch's two
previous runs — [33902561110](https://github.com/evgunter/cad/actions/runs/33902561110)
and [33903485143](https://github.com/evgunter/cad/actions/runs/33903485143),
both before `17bb8fb1` reached `main` — were green on the same tests.

## Why main's own runs did not catch it

`main`'s push runs are ONE job (`render work/STATUS.md`): runs
33905366880 and 33905855591 each report `total_count: 1`. The gate runs
on `pull_request` and not on `push`, so `main` can carry a red the push
run cannot see. That is the *class* `work/ciw/main-latently-red-at-tier-all`
recorded and closed, and whose class half was handed to
`work/ciw/f3-recosting-on-a-public-repo`. **This is a fresh instance of
it**, filed rather than only mentioned.

The narrower question — why the DOCM-1 PR's own merge-ref run did not
red on the file it was adding — is worth an answer and is not one this
lane can give. `reader_census` carries no `gated_to!` marker (its only
`gated_to!` occurrences are in doc comments about the mechanism), so the
per-file test gate should not have excluded it.

## The fix

One line in `LEDGER` in `crates/test-utils/tests/reader_census.rs`, with
the disposition the file actually earns — `Shared` if it reaches
`test_utils::source`, `Unconverted` (with a track, and
`UNCONVERTED_TODAY` raised) if it hand-rolls its reader. Deciding which
means reading the file, which is why this is filed for DOCM rather than
fixed by the CIW lane that met it: the ledger's `Shared` rows are
CLAIMS, checked by `every_shared_entry_actually_reaches_the_shared_lexer`,
and a guessed disposition is a false claim rather than a stale one.
