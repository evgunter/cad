---
id: bounds-allowlist-select-cuts-at-the-first-colon
kind: issue
title: bounds-allowlist.sh's per-file SELECT cuts the FILE column at the first colon, so a colon-carrying path rides an allowlist entry
status: review
opened: 2026-09-06
refs: [whole-file-skips-are-hand-spelled-not-anchored]
branch: gates/bounds-select-anchor
pr: 2157
---

## Finding

Turned up by `whole-file-skips-are-hand-spelled-not-anchored`'s sweep,
which asked for the whole CLASS — a whole-file exemption that is not
pinned to the `FILE:LINE:` shape — rather than for the one spelling that
row's grep matches. This is the same defect in a spelling the anchor
does not fit, and it is the LAST one in this directory — which is a
claim with a grep behind it. The row's own grep finds the literal
spelling; the widened one finds every anchored path skip:

    grep -nE "gate_grep -v?E ['\"]\^" scripts/gates/*.sh

and what it returns is three comment strippers (`'^[[:space:]]*#'`) and
`witness-not-ambient.sh`'s two directory prefixes plus its
`^crates/[^/]+/src/bin/` path class, none of which is a file skip.
Neither grep can see a skip that does not go through `gate_grep -vE`
with a `^`-anchored pattern, which is exactly this one, so the rest of
the directory was read by hand: every other `gate_grep -v` in it
(`grep -nE "gate_grep -v" scripts/gates/*.sh`) is a blank-line filter,
a comment stripper, `viewer-module-kinds.sh:489` (already
`gate_record_anchor`), or the two lines below.

`scripts/gates/bounds-allowlist.sh:940`:

    hits=$(printf '%s\n' "$records" | gate_grep -v '^$' |
      cut -d: -f1 | sort -u |
      gate_grep -vxF -f <(gate_allowlist_paths))

`cut -d: -f1` takes the FILE column as "everything before the first
colon". A record from a file whose own path carries a colon —
`crates/geom-core/src/real.rs:x.rs:12:…`, legal on this filesystem and
in git — therefore arrives as `crates/geom-core/src/real.rs`, matches
that allowlist entry exactly under `-vxF`, and is dropped. A compound
`Bounds` bound written there is exempt on a ratification that was
granted to a different file.

**Measured, not read.** The same file, twice, over the live tree:

    printf 'pub fn f<T: Decide + Bounds>(_t: T) {}\n' \
      > 'crates/topo/src/boolean/boxes.rs:x.rs'
    scripts/gates/bounds-allowlist.sh          # OK, 183 occurrences

    printf 'pub fn f<T: Decide + Bounds>(_t: T) {}\n' \
      > 'crates/topo/src/boolean/zzz_probe.rs'
    scripts/gates/bounds-allowlist.sh          # ERROR, names the file

`crates/topo/src/boolean/boxes.rs` is an allowlist entry pinned at 4;
the colon-carrying sibling rode its ratification and the pinned count
did not move.

**The count check does not catch it.** `gate_allowlist_counts` at
`:553` selects the same entry's records with
`gate_grep -E "$(gate_record_anchor "$path")"`, which is anchored and
does NOT match the colon path — so the pinned count for `real.rs` is
unmoved and the gate stays green. The two halves of the same gate read
the file column two different ways, and only one of them is pinned.

## Why it is worth a row

Population zero and the direction that never cries wolf, exactly as the
sibling row argued for the six literal skips — the value is
construction. What makes it worth its own row rather than a line in
that PR is that the repair is NOT the sibling's: `gate_record_anchor`
builds a record pattern, and this select works over a de-duplicated
column of paths, not over records. The two candidate repairs are to
drop the `cut` and select with the anchor alternation the counts
already build, or to read the FILE column the way `lib.sh`'s
`GATE_RECORD_PREFIX_RE` defines it. Either is a change to how the scan
picks its hits, in the file D102 and D103 both landed in, and it wants
its own fixture: a compound bound planted at `<entry>.rs:x.rs` must
fire while the entry itself stays exempt at its pinned count.

## Landed

**The select is the count check's own predicate, negated.** The `cut`
and the `-vxF` set membership are gone; the scan drops a record when
`gate_record_anchor_any` over `gate_allowlist_paths` claims it, which is
`gate_record_anchor "$path"` — the pattern `gate_allowlist_counts`
already attributes a record to an entry with. One builder, one reading
of the FILE column: GIVEN A NON-EMPTY LIST a record is exempt from the
scan if and only if it is attributed to some entry's pin. The quantifier
is not decoration — an empty list builds `gate_grep -vE ''`, which drops
every record; `gate_record_anchor_any` refuses no homes for exactly that
reason, but the refusal prints from a command substitution and is NOT
terminal at the caller, so a gate whose list came out empty would print
OK. `lib.sh` is where that refusal becomes terminal (asked of the lane
holding it, with `GATE_MATCHER_FAILED` as the marker); what holds the
quantifier here is that the list is a literal array in this file.

What the equivalence still does not buy is the entry NAMING the file it
exempted: `…/boxes.rs:12:x.rs` satisfies the `boxes.rs` anchor, so it is
exempt from the scan AND counted into that entry's pin — the pin moves
and reds, naming `boxes.rs` rather than the file that gained the bound.
Measured, and registered as KNOWN GAP 8.

The row's second candidate (parse the column through
`GATE_RECORD_PREFIX_RE`) was not taken. It fixes the drop — that RE is
`^[^:]*:[0-9]+:`, which a colon-carrying record does not match at all,
so nothing is extracted and the membership fails — but it leaves TWO
readings of the column in the file, agreeing only where the record is
ordinary, and it decides an exemption on a string a record may not
yield. The anchor decides it on the record.

**The FILE column survives for the DIAGNOSIS only**, as
`gate_record_file_column`: the record up to its first `:LINE:`, so the
colon-carrying file is named whole rather than as the entry it merely
begins with. A path carrying a `:LINE:` shape of its own
(`foo:12:bar.rs`) is ambiguous in a `FILE:LINE:TEXT` record at any
reader and is named short; that costs a misnamed file in a diagnosis
and never an exemption, which is why the selection does not come
through it. There is no `lib.sh` helper for this reading —
`GATE_RECORD_PREFIX_RE` is the first-colon one — so it is spelled in
this gate; a shared `gate_record_file_column` beside the prefix RE is
the natural home and is reported rather than taken here.

**Fixture**: `plant_colon_path_beside_entry` writes the compound bound
at `crates/topo/src/boolean/boxes.rs:x.rs` while the clean fixture
leaves `boxes.rs` itself at its pinned 4, so the count check is silent
and the SCAN is what fires. Asserted on the PATH: firing at all is the
harness' own assertion, so the want pins the other half — the
diagnosis names the colon-carrying file whole.

**Mutation-proved.** Restoring `cut -d: -f1 | sort -u | gate_grep -vxF
-f <(gate_allowlist_paths)`: the selftest fails at
`plant_colon_path_beside_entry` ("the gate PASSED on a planted
violation") and, with that one case removed, is green again — only the
new case reds. Reverting `gate_record_file_column` to `cut -d: -f1`
reds the same one case, on the message. Dropping the anchor filter
altogether reds the clean fixture, which is the filter being
load-bearing.

**Live output is byte-identical** to the merge base, `cmp` on stdout
and on stderr: 26 ratified files, 183 occurrences, 440 source files.

**Residue, filed**: `record-file-column-read-by-first-colon-split`,
widened to the CLASS with its own grep — every reader in the directory
that answers "which file is this record from" by splitting at the first
colon, `viewer-module-kinds.sh:469`'s dedupe key and the four
`GATE_RECORD_PREFIX_RE` PARSER uses in `lib.sh` (`:844`, `:1041`,
`:1150`, `:1195-1196`) among them.

## Fix pass (style review)

**The gate's own reader was a third parser of the same column**, in this
file: `sub(PFX, "", line)` stripped the prefix with
`GATE_RECORD_PREFIX_RE`, which matches nowhere in a record from a
colon-carrying path, so the path stayed in the text the bound walk
reads. `crates/topo/src/boolean/a:Bounds.rs` holding one SOLE bracket
bound red as a compound one (`a` keyed with `Bounds` and `rs` beside
it) — pre-existing, cry-wolf, population zero. The split is now the
first `:LINE:` on both sides, and the THREE spellings of that reading in
this file are one constant, `BOUNDS_RECORD_LINE_RE`, read by the
diagnosis column and by both halves of the reader's split; the constant
says that `lib.sh`'s parser use of `GATE_RECORD_PREFIX_RE` is the
residue row's repair site.

Planted in both directions on one path: `plant_colon_path_sole_bound`
must PASS (the whole guard for the cry-wolf) and
`plant_colon_path_compound_bound` must FIRE, named whole. Restoring the
old strip reds exactly the first, and with that case removed the
selftest is green again.

**KNOWN GAP 8** registers what the column reading still cannot do: a
path carrying a `:LINE:` SHAPE of its own is ambiguous at any reader,
this one takes the shorter split, and the two costs are a diagnosis
naming the entry rather than the file and a tail of the path reaching
the walk as code. Both need a path that spells a line number between
two colons; population zero.
