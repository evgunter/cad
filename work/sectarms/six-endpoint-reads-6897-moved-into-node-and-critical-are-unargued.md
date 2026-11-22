---
id: six-endpoint-reads-6897-moved-into-node-and-critical-are-unargued
kind: issue
title: Six endpoint reads PR 6897 left in ssi/node.rs and sweep/critical.rs neither ask is_certified() nor carry an argument; the census is pinned at main's counts
status: open
opened: 2026-11-17
priority: P2
cost: E
---


Filed by GREEN from nightly 44023697569 (main `cdee7461`, 2026-11-17), where
`geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
was red at every ε. Re-measured on main `8466ca3d`.

    cargo nextest run --profile ci --ignore-default-filter -p geom-core -E 'test(every_production_endpoint_read_is_counted_and_dispositioned)'

**What moved, and where.** PR 6897 (`sectarms/spline-node-carry`, merge
`dfa288a92`) moved the node machinery out of
`pair_section/spline_implicit/sweep/critical.rs` into a new
`crates/geom-brep/src/ssi/node.rs`. The census (reads, asking) went:

| file | pinned before | main |
|---|---|---|
| `pair_section/spline_implicit.rs` | 6, 4 | 9, 7 |
| `pair_section/spline_implicit/sweep/critical.rs` | 37, 28 | 11, 2 |
| `ssi/node.rs` | (none) | 28, 22 |

Read per fn (production lines only):
- `ssi/node.rs`: `krawczyk_split` 12 and `decide_split` 10 ask by name.
  `read` 2 and `mid` 2 are the two fns whose arguments `critical.rs`'s
  note already gave (the value's ceiling, decided only after
  `at.value.is_certified()`; the Krawczyk box's centre, a seed), so
  they moved with their argument. **`hessian_over`'s 2** (`x[k].lo()`,
  `x[k].hi()`, the grid of `x`'s parts) are new and unargued.
- `sweep/critical.rs`: `node_frame`'s 2 ask; `grown`'s 4 and
  `along_line`'s 1 keep their argument. **`decide`'s 4** (`b[0].lo(),
  b[0].hi()`, `b[1]`'s, the box handed on as `u`/`v` spans) sit in a fn
  that no longer asks `is_certified()` itself, since the asking moved
  to `node.rs`'s `decide_split`.
- `spline_implicit.rs`: the three new reads ask.

So six reads neither ask nor carry an argument. The census's rule
(its message): each asks `is_certified()` by name, or is argued safe at
the site and its file's note extended.

**Pinned.** GREEN set the roster to main's counts, with the six named
unargued in the two files' notes and this row cited beside them. The
census goes red again on the next move either way.

## Done when

Each of the six reads asks by name or carries its argument in the
roster's note, and the note's "unargued" clause and this citation are
gone.
