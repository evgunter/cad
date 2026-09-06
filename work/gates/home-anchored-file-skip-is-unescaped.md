---
id: home-anchored-file-skip-is-unescaped
kind: issue
title: signed-zero-one-home.sh's home-anchored file skip interpolates the path unescaped, so a sibling path exempts itself
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
the pattern reads `signed_zero?rs:` and names more than one path.

WHAT IT ACTUALLY EXEMPTS, corrected against the scan (this paragraph
replaces the first reading of it, and #2064's review found the same
thing from the other side). The flat `crates/step-import/src/
signed_zeroXrs` is NOT exempted, because it is never read:
`find … -name '*.rs'` at `:102` does not return it, so the gate passes
on it whether the skip is escaped or not. Every record's FILE
therefore ends in `.rs`, and the pattern's trailing `:` must land on a
real character, so a path this scan reaches and this skip exempts has
to carry a `:` INSIDE it — `crates/step-import/src/
signed_zero_rs:9:x.rs` is one, planted and confirmed exempt under the
unescaped spelling and firing under the escaped one. That is a narrow
set, not an empty one: a colon is legal in a path here and in git.

Population today: zero, and held there by the SCAN'S GLOB and by the
trailing `:` rather than by the skip. Both are one edit away from a
gate that exempts a file nobody ratified, which is the direction that
never cries wolf — a file at such a path is exempt from the
negative-zero-flush rule with nothing red anywhere. `gate_record_anchor`
makes the skip name one path by construction instead of by two
coincidences, which is what `bounds-allowlist.sh`'s and
`viewer-module-kinds.sh`' own anchors were changed toward in that unit.

Not fixed there because it is a different mechanism: this skip exempts
a WHOLE FILE, not one ratified text, so it takes neither
`gate_exact_skip` nor its fixtures. Its subject check is already the
rule that unit landed — `gate_require_file "$HOME_FILE"` at `:100`
reds when the home is gone.

## Fix shape

`| gate_grep -vE "$(gate_record_anchor "$HOME_FILE")"` — `lib.sh`'s
anchor builder, which escapes the path and pins the `FILE:LINE:` shape
— plus the fixtures that hold it. The anchor has three parts and each
one needs a planted path that FIRES only while that part is there: the
escaping (a path carrying a `:`, above), the `^` (a path ENDING in the
home), and the `:[0-9]+:` boundary (a path BEGINNING with the home).
The home itself must still pass, which is `gate_plant_clean`'s job.

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

THREE planted cases hold it, one per part of the anchor, and none of
them is the file this row first named. `crates/step-import/src/
signed_zeroXrs` is not a fixture at all — `find … -name '*.rs'` never
returns it, so the gate passes on it under the old spelling too, which
proves the scan's glob and nothing about the anchor.

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
