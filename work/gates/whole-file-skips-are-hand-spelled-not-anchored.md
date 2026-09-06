---
id: whole-file-skips-are-hand-spelled-not-anchored
kind: issue
title: the directory's other whole-file skips are hand-spelled EREs rather than gate_record_anchor
status: open
opened: 2026-09-06
refs: [home-anchored-file-skip-is-unescaped]
---

## Finding

Disclosed by `home-anchored-file-skip-is-unescaped`'s sweep, which
asked whether any other gate reads a whole-file skip by interpolating a
path into an ERE. **One did** (`signed-zero-one-home.sh:109`, fixed
there) and no other does: every remaining whole-file skip in this
directory is a single-quoted LITERAL with its dots hand-escaped, so
none of them is the widening that row was about.

    grep -hoE "gate_grep -v?E '[^']*'" scripts/gates/*.sh | grep -E "[^\\\\]\."

returns nothing — every literal escapes every dot.

What the sweep turned up instead is the same JOB done fifteen times by
hand, in six gates, none of them going through `lib.sh`'s builder:

    scripts/gates/bit-identity-consumer.sh:38-41      (4 homes)
    scripts/gates/bit-identity-punning.sh:23
    scripts/gates/evalscalar-allowlist.sh:42
    scripts/gates/interval-square-allowlist.sh:464-466
    scripts/gates/no-ambient-env.sh:108-110
    scripts/gates/witness-not-ambient.sh:90

each spelled `gate_grep -vE '^<path>\.rs:'` (line numbers as of
`ff90daf22`, the merge of the anchored-skip unit). Two more sites are
adjacent but different questions:

  * `witness-not-ambient.sh:91-93` skip DIRECTORY prefixes and a path
    class (`^crates/[^/]+/src/bin/`), which is not a file skip and has
    no `FILE:LINE:` shape to pin;
  * `gate-roster.sh:238` escapes an outlier gate's path by hand with
    `esc=${outlier//./\\.}` — dots only — where `gate_ere_escape` is
    the whole set, and matches it against ci.yml command text rather
    than against records.

`bounds-allowlist.sh`'s per-file SELECT was on this list when the row
was written and is not any more: the anchored-skip unit converted it to
`gate_grep -E "$(gate_record_anchor "$path")"` at `:553`. It is the one
worked example of what the conversion looks like.

## Why it is worth a row

Two things the hand spelling does not carry, both of which
`gate_record_anchor` carries by construction:

1. **The `FILE:LINE:` shape is unpinned.** `'^<path>\.rs:'` matches
   any record whose FILE begins with that path and a colon, so a file
   at `<path>.rs:x/inner.rs` — legal on this filesystem and in git —
   is exempt from the gate that skips its home. Population zero, and
   it is the direction that never cries wolf.
2. **The escaping is a reviewer's job on every line.** Fifteen literals
   are fifteen chances to write `.rs` for `\.rs`; the builder makes it
   nobody's job. The one site that was NOT a literal is exactly the one
   that had the defect.

`gate-roster.sh:238`'s partial escape is the same argument one level
over: `${outlier//./\\.}` is correct for the paths on `OUTLIER_GATES`
today and silently under-escapes a path carrying `+`, `(` or `[`.

## What the conversion costs

Each converted gate's CLEAN fixture must plant the skipped home, the
way `lib.sh`'s exact-skip contract already requires — otherwise the
skip is dead in every fixture and an over-narrow anchor is noticed by
nobody. Some of these gates skip four homes; that is four planted
files, not one. The conversion is therefore per-gate work with a
fixture each, not a sed.

## Reachability, read against each gate's scanned set

The same reading `home-anchored-file-skip-is-unescaped` ended up
needing, applied to every hit above, because "the pattern over-matches"
and "the scan can hand it a record that over-matches" are different
claims:

  * The fifteen literal skips all end in `:` and all escape their
    dots, and every one of them scans `*.rs` files. So the only record
    they can over-drop is one whose FILE carries a `:` inside it
    (`…/bit_identity.rs:x/inner.rs`), the same narrow-but-not-empty
    set. None of them can over-drop by an unescaped dot and none by a
    longer path.
  * `witness-not-ambient.sh:91-93` are prefix skips on purpose; there
    is no over-match to reach.
  * `bounds-allowlist.sh:553` is converted already and is the shape the
    rest should take.
  * `gate-roster.sh:238`'s under-escape is reachable only by adding a
    path carrying `+`, `(`, `[` or `{` to `OUTLIER_GATES`, which is a
    hand-maintained list; the reachable set is "the next entry", not
    "the next file".

The unit's value is therefore construction rather than a live hole,
and it should be written up that way: the escaping and the boundary
stop being a reviewer's job on fifteen lines.

## Not measured

Whether each of the six gates' subject checks already reds when its
skipped home is gone (`signed-zero-one-home.sh` has
`gate_require_file`; the others were not read for this). A skip whose
home has been deleted exempts nothing and ratifies whatever lands at
that path next, so a conversion that does not also check the subject
leaves the second half of the same defect standing.
