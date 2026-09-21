---
id: blend-spells-the-half-edge-to-face-walk-five-times
kind: issue
title: sweep/src spells the half-edge to face walk five times where topo now exports one door
status: open
opened: 2026-09-19
priority: P1
cost: E
---


## Finding

- **Where**: `crates/sweep/src/blend/build.rs` (`vertex_faces`, ~:230),
  `crates/sweep/src/blend/surgery.rs` (`loop_of_half` ~:3852 +
  `face_of_half` ~:3856), `crates/sweep/src/blend/battery.rs`
  (`face_of`, ~:453), `crates/sweep/src/test_support.rs` (~:397 and
  ~:940), plus `crates/sweep/examples/r2_p2_consumer.rs` (~:100, ~:104,
  ~:196).
- **Importance**: medium
- **Confidence**: sure — every site above was read, not matched.
- **Raised by**: PR #2857's fix pass, 2026-09-19, announced by seam from
  `work/dup/half-edge-to-face-walk-is-spelled-once-per-suite.md`.

PR #2857 exported `pncad::topo::Body::face_of_half_edge(he) ->
Option<FaceKey>` — `&self`, total, `None` on either stale key. Five
spellings of that walk remain in `sweep/src`, all `Option`-returning and
so all foldable onto it with no refusal lost:

| site | shape |
| --- | --- |
| `blend/build.rs` `vertex_faces` (~:230) | `body.get_loop(body.get_half_edge(h)?.parent_loop)?.face` — byte-identical to the `topo` pair #2857 folded |
| `blend/battery.rs` `face_of` (~:453) | a named private door with exactly the new door's signature |
| `blend/surgery.rs` `face_of_half` (~:3856) | the same door, over a second private door `loop_of_half` (~:3852) |
| `test_support.rs` (~:397) | `Option` closure |
| `test_support.rs` (~:940) | `unwrap` chain inside a `map` |

**`surgery.rs` also holds the twin of `topo`'s
`replace_face::edge_faces`** (~:3862): same edge-key argument, same two
half-edge walks, composed over `face_of_half` instead. `replace_face`'s
doc comment used to disclose a twin "in `merge_faces`" that has not
existed for some time; #2857 corrected it to name this one. Whether the
two `edge_faces` become one is a crate-boundary question and is the
second half of this row.

`test_support.rs` is BLEND's by the announced seam
(`work/dup/program.md`'s `keep_out`), which is why both of its sites are
here rather than on a suite program's slate.

## What the instrument could not see

The count comes from `git grep parent_loop` over `crates/sweep/`, read
site by site. It cannot see a walk split across a helper boundary wider
than one function, and it says nothing about `sweep/tests` — those 72
sites are `work/tint/the-half-edge-to-face-walk-is-spelled-per-test-file.md`.
