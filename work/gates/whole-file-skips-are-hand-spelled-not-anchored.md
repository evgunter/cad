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
path into an ERE. **One did** (`signed-zero-one-home.sh`, fixed there)
and no other does: every remaining whole-file skip in this directory is
a single-quoted LITERAL with its dots hand-escaped, so none of them is
the widening that row was about.

    grep -hoE "gate_grep -v?E '[^']*'" scripts/gates/*.sh | grep -E "[^\\\\]\."

returns nothing — every literal escapes every dot.

What the sweep turned up instead is the same JOB done by hand, in six
gates, none of them going through `lib.sh`'s builder. The class IS its
grep, so run it rather than trusting a count here:

    grep -nE "gate_grep -v?E '\^crates[^']*\\\\.rs:'" scripts/gates/*.sh

It answers one line per skip, in `bit-identity-consumer.sh`,
`bit-identity-punning.sh`, `evalscalar-allowlist.sh`,
`interval-square-allowlist.sh`, `no-ambient-env.sh` and
`witness-not-ambient.sh`. **A line is not a home**, and the
conversion's unit of work is the home: the lines that the same grep
piped through `grep '|'` returns each name two or three in an
alternation (`(mod|parts)`, `(svd|lsq)`, `(jet|march|system)`), so read
the expanded set off those. At `ff90daf22` it was 13 lines naming 18
homes, 4 of the lines carrying the 9 that are alternated.

Two sites are adjacent but different questions:

  * `witness-not-ambient.sh` also skips DIRECTORY prefixes and a path
    class (`^crates/[^/]+/src/bin/`), which is not a file skip and has
    no `FILE:LINE:` shape to pin;
  * `gate-roster.sh:238` escapes an outlier gate's path by hand with
    `esc=${outlier//./\\.}` — dots only — where `gate_ere_escape` is
    the whole set, and matches it against ci.yml command text rather
    than against records.

`bounds-allowlist.sh:553` is the one worked example of the conversion:
its per-file SELECT reads `gate_grep -E "$(gate_record_anchor "$path")"`.

## Why it is worth a row

Two things the hand spelling does not carry, both of which
`gate_record_anchor` carries by construction:

1. **The `FILE:LINE:` shape is unpinned.** `'^<path>\.rs:'` matches any
   record whose FILE begins with that path and a colon, so a file at
   `<path>.rs:x/inner.rs` — legal on this filesystem and in git — is
   exempt from the gate that skips its home. Population zero, and it is
   the direction that never cries wolf. `signed-zero-one-home.sh`'s
   `plant_colon_after_the_home_that_is_not_a_line_number` is that path
   planted, in the one gate that pins the shape.
2. **The escaping is a reviewer's job on every line.** Every literal is
   another chance to write `.rs` for `\.rs`; the builder makes it
   nobody's job. The one site that was NOT a literal is exactly the one
   that had the defect.

`gate-roster.sh:238`'s partial escape is the same argument one level
over: `${outlier//./\\.}` is correct for the paths on `OUTLIER_GATES`
today and silently under-escapes a path carrying `+`, `(` or `[`.

## What the conversion costs

Each converted gate's CLEAN fixture must plant the skipped home, the
way `lib.sh`'s exact-skip contract already requires — otherwise the
skip is dead in every fixture and an over-narrow anchor is noticed by
nobody. One of these gates skips four homes; that is four planted
files. The conversion is per-gate work with a fixture each, not a sed.

## Reachability, read against each gate's scanned set

"The pattern over-matches" and "the scan can hand it a record that
over-matches" are different claims, and `gate_record_anchor`'s header
in `lib.sh` argues the second one once. Applied to the hits here:

  * The literal skips all end in `:`, all escape their dots, and all
    scan `*.rs` files, so the only record any of them can over-drop is
    one whose FILE carries a `:` inside it
    (`…/bit_identity.rs:x/inner.rs`) — the same narrow-but-not-empty
    set. None can over-drop by an unescaped dot; none by a longer path.
  * `witness-not-ambient.sh`'s prefix skips are prefixes on purpose;
    there is no over-match to reach.
  * `gate-roster.sh:238`'s under-escape is reachable only by adding a
    path carrying `+`, `(`, `[` or `{` to `OUTLIER_GATES`, which is a
    hand-maintained list; the reachable set is "the next entry", not
    "the next file".

The unit's value is therefore construction rather than a live hole, and
it should be written up that way.

## The subject half, measured

None of the six gates checks that the file it exempts still exists.

    grep -c 'gate_require' scripts/gates/{bit-identity-consumer,\
    bit-identity-punning,evalscalar-allowlist,interval-square-allowlist,\
    no-ambient-env,witness-not-ambient}.sh

is 1 for each, and each of those is `gate_require_crate_sources` —
which proves the CRATE has sources, not that the skipped path is one of
them. `gate_require_file` and `gate_exact_skip`'s subject check appear
in this directory only in `bit-identity-debug-only.sh`,
`bounds-allowlist.sh` and `signed-zero-one-home.sh`.

So a skip here whose home is renamed or deleted exempts nothing, stays
green, and ratifies whatever lands at that path next — the second half
of the same defect, and the half that has an actual live route (a
rename is ordinary; a colon in a path is not). A conversion that pins
the anchor and leaves the subject unchecked closes the smaller half.
