---
id: home-anchored-file-skip-is-unescaped
kind: issue
title: signed-zero-one-home.sh's home-anchored file skip interpolates the path unescaped, so the anchor is exact only by accident
status: review
branch: gates/file-skip-anchor
pr: 2065
opened: 2026-09-06
---

## Finding

Turned up by the sweep for `anchored-exact-text-skip-has-three-homes`,
which converted the two exact-text skips and left this NEIGHBOUR of the
class in place.

`scripts/gates/signed-zero-one-home.sh:109` drops every record at the
sanctioned home with

    | gate_grep -vE "^$HOME_FILE:" \

where `HOME_FILE=crates/step-import/src/signed_zero.rs` (`:66`) is
interpolated into an ERE **unescaped**. The `.` is a metacharacter, so
as a pattern the skip also names `crates/step-import/src/signed_zeroXrs`.

**THAT PATH IS UNREACHABLE, and this row's first reading of it was
wrong in both directions.** The gate's scan set is
`find "${SCAN_DIRS[@]}" -type f -name '*.rs'` (`:102`), so no record it
reads carries a path whose extension is not `.rs`: a flush planted at
`signed_zeroXrs` does not fire, before the fix or after, and a fixture
built on it proves the scan's glob rather than the anchor. Confirmed by
planting it, and #2064's review found the same from the other side.

**What does not follow is "zero for any tree this gate can scan."**
Every record is `FILE:LINE:TEXT` and the pattern's trailing `:` has to
land on a real character, so an exempted path must end in `.rs` AND
carry a `:` inside it — legal on this filesystem and in git.
`crates/step-import/src/signed_zero_rs:9:x.rs` is one, planted and
confirmed EXEMPT under the unescaped spelling and firing under the
escaped one. The reachable set is narrow, not empty.

Population today: zero, and held there by the SCAN'S GLOB and the
trailing `:` rather than by the skip.

What it is instead: **an anchor that is exact by accident rather than
by construction**, in a gate whose whole claim is "the flush lives in
THIS file and nowhere else". The pattern says something wider than the
gate means, and what keeps it nearly harmless is a property of a
DIFFERENT line — the `-name '*.rs'` in the scan — that nothing ties to
this one.

## Fix shape

`| gate_grep -vE "$(gate_record_anchor "$HOME_FILE")"` — `lib.sh`'s
anchor builder, which escapes the path and pins the `FILE:LINE:` shape
that every view emits — plus the fixtures that hold it. The anchor has
three parts and each needs a planted path that FIRES only while that
part is there: the escaping (a path carrying a `:`, above), the `^` (a
path ENDING in the home), and the `:[0-9]+:` boundary (a path
BEGINNING with the home). The home itself must still pass, which is
`gate_plant_clean`'s job.

This row earlier said the second half could not be planted as a `.rs`
file in the scan and would have to be a unit assertion over hand-built
records, `gate_exact_skip_escaping_case`'s shape. It can be planted;
that paragraph came from the same wrong premise as the one above.

## What was not measured

Whether any other gate reads a whole-file skip this way. The sweep that
found it (`grep -n '\-E "\^\$' scripts/gates/*.sh`) matched the three
exact-text skips, this one, and nothing else, but it cannot match a
skip built by a helper or spelled with the anchor in a variable.

## Landed

`scripts/gates/signed-zero-one-home.sh`'s home skip is
`gate_grep -vE "$(gate_record_anchor "$HOME_FILE")"`: the path escaped,
the `FILE:LINE:` shape pinned. Live output byte-identical, stdout and
stderr.

THREE planted cases hold it, one per part of the anchor:

  * `plant_sibling_the_raw_anchor_exempted` — the ESCAPING.
    `crates/step-import/src/signed_zero_rs:9:x.rs`, the reachable
    sibling: it ends in `.rs` so the scan reads it, and it puts a `:`
    where the anchor's `:` sits so the raw interpolation exempted it.
    `:9:` rather than `:x` so the case reds for the escaping alone and
    not for the `FILE:LINE:` shape.
  * `plant_nested_path_ending_in_the_home` — the `^`, which had NO
    witness anywhere in `scripts/gates/`: dropping it left all nineteen
    selftests green, the shared `gate_exact_skip_escaping_case`
    included, whose four records all begin at the path. A flush at
    `crates/step-import/src/crates/step-import/src/signed_zero.rs` is
    exempt from an unanchored skip and fires under this one.
  * `plant_longer_path_beginning_with_the_home` — the `:[0-9]+:`
    boundary, without which the skip is a prefix match over every
    longer path the home opens. `signed_zero.rs.generated.rs`.

The home's own exemption needed nothing: `gate_plant_clean` already
writes the home in the wrapped flush form, so the skip is live in every
fixture and an over-narrow anchor reds the clean case.

## What the sweep found

The row's open question — whether any other gate reads a whole-file
skip this way — is answered no: this was the only skip built by
interpolating a path into an ERE. Fifteen other whole-file skips in six
gates are single-quoted literals with their dots hand-escaped. They do
not pin `FILE:LINE:` and they escape by reviewer rather than by
construction, which is
`whole-file-skips-are-hand-spelled-not-anchored`.
