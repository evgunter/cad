#!/usr/bin/env bash
# signed-zero-one-home.sh — the negative-zero flush has exactly ONE
# home. ONE home for the gate too; ci.yml's "signed-zero one home" step
# and local-scripts/ci-local.sh's discipline row both call this file.
#
# `crates/step-import/src/signed_zero.rs` asserts, in its own module
# doc, that it is the importer's only flush of `-0.0` to `+0.0`. That
# claim went unenforced through FOUR copies of the same three lines in
# one crate (S18's roll-up row), each landing green: the reader's
# `as_real`, two private `flush_zero`/`flush_zero_point` pairs in the
# recognition modules, and the home itself. A claim nothing checks is
# how copy five lands too, so this is the check.
#
# WHY IT IS NOT LINE-BASED. rustfmt's canonical form for the branch
# spelling is FIVE LINES once the identifier is long enough to cross
# `single_line_if_else_max_width` (50), and that wrapped form is what
# rustfmt PRODUCES — the common case, not an edge case. A line matcher
# passes it green, so whether the guard fired would depend on how long
# the author's variable name was. This is S56's shape one level over
# (a matcher that saw `T: Bounds + Decide` and not the other order),
# and the ruling there was that the matcher must see both forms rather
# than carve one out. So each file is stripped of comments and scanned
# as overlapping SIX-LINE WINDOWS with whitespace collapsed, which
# makes the one-line and rustfmt-wrapped spellings the same string.
#
# WHAT FIRES IT, anywhere under `crates/step-import/{src,tests}` except
# the home — the tests are in scope because four of this crate's suites
# hand-construct frames, so a re-derived helper there is a plausible
# copy and a suite is exactly where one would be written to avoid
# touching `src`:
#
#   * `if <e> == 0.0 { 0.0 } …`  — branch-and-substitute, either the
#     one-line or the rustfmt-wrapped form;
#   * `<e> + 0.0`                — add-to-flush, the spelling both
#     deleted copies used, including `*value + 0.0`;
#   * `0.0 + <e>`                — the same technique with the operands
#     reversed, where `<e>` opens with an identifier, a deref or a
#     paren (so `0.0 + 1e-12`, a real offset, does not fire).
#
# `0.0` is matched as a WHOLE literal: `+ 0.05` and `+ 0.0125` are
# ordinary constants and stay green. A gate that cried wolf on those
# would teach the next author that it is noise.
#
# WHAT IT STILL CANNOT SEE (stated because a sweep whose blind spot is
# unstated is an unverified claim, §C15):
#
#   * a flush with no literal `0.0` on either side — an
#     `f64::from_bits` sign-bit mask, `x - x.min(x)`, `x.abs() * s`;
#   * one spelled across MORE than six lines, or split so that no
#     window holds the whole expression (a `+` and its `0.0` seven
#     lines apart, a `match` arm ladder standing in for the `if`);
#   * one behind a generic, a trait method, or a macro that expands to
#     any of the above — the scan reads source text, never expansions;
#   * a same-shaped guard that is NOT a flush (`if den == 0.0 { 0.0 }
#     else { num / den }`) fires here on purpose: it looks identical
#     and wants a human, not an allowlist;
#   * anything outside `crates/step-import/{src,tests}`. The claim
#     guarded is the one the module makes — one flush in THIS importer
#     — not a workspace-wide uniqueness that is false by design
#     (`pncad-py/src/py/doc.rs` folds `-0.0` for `__hash__`/`__eq__`
#     consistency, a different rule with a different reason).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/step-import/src/signed_zero.rs
SCAN_DIRS=(crates/step-import/src crates/step-import/tests)
GATE_SCAN_NOUN='step-import source file'

# THE WINDOW COMES FROM `lib.sh`. This gate used to carry the second
# hand-rolled Rust reader in this directory — an awk that knew `//` and
# `/* */` and nothing else, so a `//` inside a string literal truncated
# its line and a char literal could desynchronise it. S63 praised it for
# having a real stripper, which was true relative to the leading-`//`
# filter and misleading relative to a lexer. `--window 6` is the same
# six-line join over the shared code-only view, which also knows string,
# raw-string, byte-string and char literals, and lifetimes.
#
# Six lines, because the needle spans a construct rather than ending at
# a delimiter: `== 0.0 { 0.0 }` has braces in it, so the statement view
# would cut it in half.
# Three spellings of one technique. `0.0` as a whole literal each time.
PAT_BRANCH='== 0\.0 \{ 0\.0 \}'
PAT_ADD='\+ 0\.0([^0-9]|$)'
PAT_ADD_REVERSED='0\.0 \+ [A-Za-z_*(]'

gate() {
  # The home must exist — a renamed subject would make this gate green
  # forever (see bit-identity-debug-only.sh's header for the live
  # instance) — and the trees it guards must actually hold sources.
  #
  # THE TWO ASK DIFFERENT QUESTIONS, and the gap between them is the
  # second guard's live route. `gate_require_file` is `[ -f ]`, which
  # FOLLOWS a symlink; `find … -type f` does not. So a home file that is
  # a symlink to a real `.rs` outside the scanned trees clears the first
  # guard and leaves the scan with nothing to read — planted as
  # `plant_home_symlinked_out_of_the_scan`, where the second guard fires
  # with its own diagnosis. The predicates are left disagreeing
  # deliberately; the reasoning is at that planter.
  gate_require_file "$HOME_FILE"
  local files hits
  mapfile -t files < <(find "${SCAN_DIRS[@]}" -type f -name '*.rs' 2>/dev/null | sort)
  GATE_SCAN_FILES=${#files[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no .rs files under ${SCAN_DIRS[*]} in $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
  # THE HOME'S EXEMPTION IS ONE PATH, by construction rather than by
  # coincidence: `gate_record_anchor` escapes the path and pins the
  # `FILE:LINE:` shape, so `signed_zero.rs` cannot also read as
  # `signed_zero?rs`. Planted at `plant_sibling_the_raw_anchor_exempted`.
  hits=$(gate_rust_code --window 6 "${files[@]}" \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")" \
    | gate_grep -E "$PAT_BRANCH|$PAT_ADD|$PAT_ADD_REVERSED" \
    | cut -c1-140)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a negative-zero flush outside the sanctioned home ($HOME_FILE) — call crate::signed_zero instead of re-deriving it"
    exit 1
  fi
  gate_ok "the negative-zero flush lives only in $HOME_FILE"
}

# This gate's subject is one crate's src+tests, not `crates/*/src`. The
# clean fixture puts the home in rustfmt's WRAPPED form — the exemption
# has to hold for the shape the formatter actually produces — beside a
# caller, two innocent literals that begin `0.0`, and a comment that
# spells a flush out. Every one of those was a way to make this gate
# lie, so the negative control carries all of them.
gate_plant_clean() {
  mkdir -p "$1/crates/step-import/src" "$1/crates/step-import/tests"
  cat > "$1/$HOME_FILE" <<'RS'
pub fn plus_zero_scalar(component_value: f64) -> f64 {
    if component_value == 0.0 {
        0.0
    } else {
        component_value
    }
}
RS
  cat > "$1/crates/step-import/src/recognize.rs" <<'RS'
// A mint whose flush is `x + 0.0` would be a copy; this one calls the home.
pub fn mint(x: f64) -> f64 {
    crate::signed_zero::plus_zero_scalar(-x)
}
pub fn pad(t: f64) -> f64 {
    t + 0.05
}
pub fn nudge() -> f64 {
    0.0 + 1e-12
}
RS
  printf 'pub fn probe() -> f64 { super::mint(1.0) }\n' \
    > "$1/crates/step-import/tests/probe.rs"
}

# Each planted case is one way the previous matcher was evaded. They are
# separate cases, not one fixture, so a fix that closes three of them
# and reopens the fourth fails visibly (the S56 lesson).

# The one that mattered: rustfmt's own output for the branch spelling.
plant_rustfmt_branch() {
  cat > "$1/crates/step-import/src/adopt.rs" <<'RS'
fn flush_component(component_value: f64) -> f64 {
    if component_value == 0.0 {
        0.0
    } else {
        component_value
    }
}
RS
}

plant_oneline_branch() {
  printf 'fn flush(x: f64) -> f64 { if x == 0.0 { 0.0 } else { x } }\n' \
    > "$1/crates/step-import/src/adopt.rs"
}

plant_add() {
  printf 'fn flush(x: f64) -> f64 { x + 0.0 }\n' \
    > "$1/crates/step-import/src/adopt.rs"
}

# `*value + 0.0` — the shape any `.map(|v| ...)` over a `&f64` takes,
# and the exact idiom at the site this row's fourth copy lived on. The
# old comment filter discarded it as a block-comment continuation.
plant_deref_add() {
  cat > "$1/crates/step-import/src/adopt.rs" <<'RS'
fn flush(value: &f64) -> f64 {
    *value + 0.0
}
RS
}

plant_reversed_add() {
  printf 'fn flush(x: f64) -> f64 { 0.0 + x }\n' \
    > "$1/crates/step-import/src/adopt.rs"
}

# THE SUBJECT EXISTING AND THE SUBJECT BEING SCANNABLE ARE TWO
# QUESTIONS, and only the second one decides anything. `[ -f ]` follows
# a symlink and `find … -type f` does not, so a home file symlinked to a
# real `.rs` outside `SCAN_DIRS` passes `gate_require_file` while the
# scan reads zero files. The gate must refuse, and the refusal it owes
# is the empty-scan one, not the missing-file one.
#
# THE MISMATCH IS NOT REPAIRED, and that is a decision rather than an
# omission. Giving `find` a `-L` would align the two by widening every
# scan in this directory to symlink targets outside its own tree — a
# behaviour change with no instance in this repo, and one that would
# make this guard dead again. Tightening `gate_require_file` to reject a
# symlink would align them by making it say "does not exist" about a
# file that does. Both spellings are correct for the question they ask;
# what was missing is a fixture proving the scan-side one still fires.
plant_home_symlinked_out_of_the_scan() {
  rm -f "$1/$HOME_FILE" "$1/crates/step-import/src/recognize.rs" \
        "$1/crates/step-import/tests/probe.rs"
  mkdir -p "$1/vendor"
  printf 'pub fn plus_zero_scalar(x: f64) -> f64 { x }\n' \
    > "$1/vendor/signed_zero.rs"
  ln -s "$1/vendor/signed_zero.rs" "$1/$HOME_FILE"
}

# THE SKIP IS A PATTERN, NOT A PATH, and this is the fixture that holds
# it to one path. The home's `.` is an ERE metacharacter, so the skip
# interpolated raw read `signed_zero?rs:` and exempted a sibling that
# differs from the home only where the metacharacter sits.
#
# WHY THE PLANTED PATH CARRIES A COLON, rather than being the flat
# `signed_zeroXrs` the widening is usually described by. That file is
# not a fixture at all: `find ... -name '*.rs'` never returns it, so the
# gate passes green on it whether the skip is escaped or not — a case
# that proves the SCAN's glob and says nothing about the anchor. To
# reach the widening through this gate's own scan a path must end in
# `.rs` AND put a `:` where the anchor's `:` sits, which means the colon
# is inside the path. That the two coincidences did not line up in this
# tree is why the population is zero; it is not why the skip is right,
# and a skip that is right by coincidence is one glob away from being
# wrong.
#
# WHY `:9:` AND NOT `:x`. It makes the case discriminate the ESCAPING
# alone: with the `FILE:LINE:` shape pinned but the path still
# interpolated raw, `...signed_zero_rs:9:x.rs:1:` matches and the flush
# is still exempt, so this case can only go green on an anchor that
# escapes. The home's own exemption is the clean fixture's job — it
# plants the home in the wrapped flush form, so an anchor that
# over-narrows reds `gate_selftest_clean` rather than waiting to be
# noticed.
plant_sibling_the_raw_anchor_exempted() {
  printf 'fn flush(x: f64) -> f64 { x + 0.0 }\n' \
    > "$1/crates/step-import/src/signed_zero_rs:9:x.rs"
}

# THE ANCHOR'S OTHER HALF, and until this case it was unwitnessed —
# `gate_record_anchor`'s `^` could be deleted with every `--selftest` in
# this directory still green, including the shared escaping case, whose
# four hand-built records all begin at the path. A skip is a claim about
# ONE path, so an unanchored one exempts every path that ENDS in the
# home, and this is the nearest such path a real tree could grow: a
# vendored or generated sub-tree under `src` whose tail repeats the
# crate layout. The gate must read it as an ordinary file.
plant_nested_path_ending_in_the_home() {
  mkdir -p "$1/crates/step-import/src/crates/step-import/src"
  printf 'fn flush(x: f64) -> f64 { x + 0.0 }\n' \
    > "$1/crates/step-import/src/crates/step-import/src/signed_zero.rs"
}

# THE ANCHOR'S THIRD PART, the trailing `:`. Without it the skip is a
# PREFIX match and exempts every longer path the home opens — a backup,
# a generated `<home>.<something>.rs`, anything the tree grows beside
# the file it names. `gate_record_anchor` spells the boundary as
# `:[0-9]+:`, the shape every view's records carry, so the skip stops at
# the end of the path rather than wherever the path happens to stop.
plant_longer_path_beginning_with_the_home() {
  printf 'fn flush(x: f64) -> f64 { x + 0.0 }\n' \
    > "$1/crates/step-import/src/signed_zero.rs.generated.rs"
}

# A suite is where a copy gets written to avoid touching `src`.
plant_in_tests() {
  printf 'fn flush(x: f64) -> f64 { x + 0.0 }\n' \
    > "$1/crates/step-import/tests/probe.rs"
}

# GREEN cases. A gate that fires on these is a gate people route
# around, so they are asserted, not assumed.
# A `//` INSIDE A STRING LITERAL. The hand-rolled reader this gate used
# to carry truncated the line there, so everything after it — including
# a real flush — was invisible. The shared reader lexes the literal.
plant_flush_after_a_string_with_slashes() {
  cat > "$1/crates/step-import/src/adopt.rs" <<'RS'
fn note() -> &'static str {
    "http://example.invalid/a//b"
}
fn flush(y: f64) -> f64 {
    y + 0.0
}
RS
}

plant_innocent_literals() {
  cat > "$1/crates/step-import/src/adopt.rs" <<'RS'
fn pad(t: f64) -> f64 {
    t + 0.05
}
fn scale(x: f64) -> f64 {
    x + 0.0125
}
fn offset() -> f64 {
    0.0 + 1e-12
}
RS
}

plant_comment_only() {
  cat > "$1/crates/step-import/src/adopt.rs" <<'RS'
// The old copies spelled this `x + 0.0`, and one wrote
// `if x == 0.0 { 0.0 } else { x }`. Both are prose here.
/* A block comment naming `*value + 0.0`
 * across a continuation line.
 */
fn adopt(x: f64) -> f64 {
    crate::signed_zero::plus_zero_scalar(x)
}
RS
}

gate_selftest() {
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces. Proved here rather than asserted, because before
  # `gate_grep` this exact fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  local want="outside the sanctioned home"
  gate_selftest_case "$want" plant_flush_after_a_string_with_slashes
  gate_selftest_case "$want" plant_rustfmt_branch
  gate_selftest_case "$want" plant_oneline_branch
  gate_selftest_case "$want" plant_add
  gate_selftest_case "$want" plant_deref_add
  gate_selftest_case "$want" plant_reversed_add
  gate_selftest_case "$want" plant_in_tests
  gate_selftest_case "$want" plant_sibling_the_raw_anchor_exempted
  gate_selftest_case "$want" plant_nested_path_ending_in_the_home
  gate_selftest_case "$want" plant_longer_path_beginning_with_the_home
  gate_selftest_case "no .rs files under" plant_home_symlinked_out_of_the_scan
  gate_selftest_passes "innocent literals" plant_innocent_literals
  gate_selftest_passes "a comment-only mention" plant_comment_only
  printf '%s selftest OK: 10 planted spellings fire (rustfmt-wrapped, one-line, add, deref-add, reversed, in tests/, after a string literal containing `//`, and at the three paths a home skip exempts when its escaping, its `^` or its `FILE:LINE:` boundary is dropped); clean fixture, innocent literals and comment-only mentions stay green; fires on the empty scan a home file symlinked out of SCAN_DIRS produces, which `[ -f ]` clears and `find -type f` does not; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' \
    "$(gate_name)"
}

gate_parse_args "$@"
gate_main
