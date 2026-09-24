#!/usr/bin/env bash
# persist-no-backtracking.sh — nothing on editor-core's wire asks serde
# to TRY an alternative. ONE home; ci.yml's "persist wire does not
# backtrack" step and local-scripts/ci-local.sh's discipline row both
# call this file.
#
# THE CLAIM THIS GUARDS, and why a comment was not enough.
# `persist::refusal` carries a structured `DimensionError` out of a
# `Deserialize` impl through a thread-local frame, because serde hands
# an impl one error type and it is the format's. `parse_body` adopts
# the frame's value when serde_json classifies the failure as `Data`,
# and the whole thing is sound on one premise: **the first refusal
# recorded is the one that decided the parse**. Every impl between an
# expression and the file body propagates the first error it meets, so
# that holds — unless something asks serde to attempt an arm, fail, and
# fall back. Then a refusal is recorded, discarded by the retry, and
# `PersistError::Dimension` names a check that did not refuse anything.
#
# The failure mode is a confidently WRONG typed refusal: green tests, a
# plausible payload, and nothing anywhere saying which check actually
# fired. That is exactly the shape #651 says owes a mechanical guard
# rather than a paragraph, and this is it.
#
# WHAT FIRES IT, anywhere under `crates/editor-core/src`:
#
#   * `#[serde(untagged)]`      — tries each variant in declaration
#                                 order and keeps the first that parses,
#                                 so every failed attempt is a recorded
#                                 refusal that decided nothing;
#   * `#[serde(other)]`         — a catch-all variant, which turns an
#                                 unknown tag into a success after the
#                                 named arms were considered;
#   * `#[serde(flatten)]`       — buffers into a content map and replays
#                                 it into the flattened field, so a
#                                 failing deserialize can be re-attempted
#                                 against a different target;
#   * `deserialize_with = "…"`  — arbitrary code, which may retry;
#                                 allowlisted BY PATH (one entry today);
#   * `serde_json::Value`       — an intermediate the body is read into
#                                 and then re-read from, which makes the
#                                 recorded-then-discarded case reachable
#                                 without any attribute at all.
#
# The first three are matched as ATTRIBUTES rather than as bare words,
# so the module docs that name them — `refusal.rs`'s own premise
# paragraph, `eval/mod.rs`'s two prose mentions of "untagged" — stay
# green. A gate that reddened on its own explanation would be deleted
# within a week.
#
# THE ALLOWLIST is one `deserialize_with`: `persist::wire`'s
# `plane_ref`, a single `deserialize_u64` with one `visit_u64` and no
# fallback of any kind. It is allowed by its FULL PATH, the spelling
# `program.rs` uses (`crate::persist::wire::plane_ref`), so a second
# `deserialize_with` — including a bare `"plane_ref"` naming some other
# module's function — reds and wants a human, as does a `plane_ref`
# rewritten to try something else. Widening this list is a decision
# about the refusal channel's soundness, not a formatting fix.
#
# WHAT IT STILL CANNOT SEE (stated because a sweep whose blind spot is
# unstated is an unverified claim, §C15):
#
#   * a hand-written `Deserialize` impl that backtracks WITHOUT any of
#     these spellings — an `Err` matched and a second `deserialize_*`
#     called on a buffered copy. Nothing textual distinguishes that
#     from an ordinary impl; what bounds it is that `editor-core` has
#     six hand-written impls, all listed in `persist::wire`'s docs;
#   * a backtracking type reached from a DEPENDENCY's `Deserialize`
#     rather than from this crate's source. The types on this wire are
#     all local or `std`, which is the property that makes the scan's
#     scope the right one;
#   * `serde_json::Value` under an alias (`use serde_json::Value as V`)
#     or reached as `json::Value`. Not widened: the crate imports
#     nothing from `serde_json` by alias today, and a matcher for every
#     rename would fire on unrelated `Value` types — `MetaValue`,
#     `NodeValue` — which is how a gate teaches its readers to ignore
#     it;
#   * anything outside `crates/editor-core/src`. The premise is about
#     the types between an expression and the file body, and they all
#     live there; `pncad-py` re-exports refusals but deserializes
#     nothing.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SCAN_DIR=crates/editor-core/src
# The one sanctioned `deserialize_with`, by the full path it names.
ALLOWED_WITH='deserialize_with[[:space:]]*=[[:space:]]*"crate::persist::wire::plane_ref"'

PAT_UNTAGGED='#\[serde\([^]]*untagged'
PAT_OTHER='#\[serde\([^]]*other[[:space:]]*[,)]'
PAT_FLATTEN='#\[serde\([^]]*flatten'
PAT_WITH='deserialize_with[[:space:]]*='
PAT_VALUE='serde_json::Value'

gate() {
  # A gate that scanned nothing is not a pass, and a renamed subject
  # would make this one green forever.
  local files hits
  mapfile -t files < <(find "$SCAN_DIR" -type f -name '*.rs' 2>/dev/null | sort)
  GATE_SCAN_FILES=${#files[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no .rs files under $SCAN_DIR in $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
  # Comments are stripped first (`gate_rust_code`), so the module docs
  # that NAME these spellings in order to argue about them do not fire
  # the gate that enforces what they argue.
  hits=$(gate_rust_code --keep-literals "${files[@]}" \
    | gate_grep -vE "$ALLOWED_WITH" \
    | gate_grep -E "$PAT_UNTAGGED|$PAT_OTHER|$PAT_FLATTEN|$PAT_WITH|$PAT_VALUE" \
    | cut -c1-140)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a backtracking or re-read spelling on editor-core's wire — \`persist::refusal\`'s \"first refusal wins\" premise is what makes \`PersistError::Dimension\` name the check that actually refused, and each of these lets a recorded refusal be discarded by a retry. Rework the declaration, or argue the new shape at \`refusal.rs\`'s premise paragraph AND in this gate's allowlist."
    exit 1
  fi
  gate_ok "nothing on editor-core's wire asks serde to try an alternative"
}

# The clean fixture carries the sanctioned `deserialize_with`, an
# externally-tagged enum (the shape the whole wire uses), and a comment
# that spells every banned attribute out — each of those was a way to
# make this gate lie, so the negative control carries all of them.
gate_plant_clean() {
  mkdir -p "$1/$SCAN_DIR/persist"
  cat > "$1/$SCAN_DIR/persist/wire.rs" <<'RS'
//! The premise this gate guards names `#[serde(untagged)]`,
//! `#[serde(other)]` and `#[serde(flatten)]` in order to refuse them,
//! and mentions a `serde_json::Value` intermediate as a thing that is
//! not here. None of that is code.
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum WireExpr {
    Literal { value: f64, unit: String },
    Add(Box<WireExpr>, Box<WireExpr>),
}

#[derive(Deserialize)]
pub(crate) struct WireProfile {
    #[serde(deserialize_with = "crate::persist::wire::plane_ref")]
    plane: u64,
}

fn plane_ref<'de, D: Deserializer<'de>>(de: D) -> Result<u64, D::Error> {
    de.deserialize_u64(PlaneRef)
}
RS
}

plant_untagged() {
  mkdir -p "$1/$SCAN_DIR/persist"
  printf '#[derive(Deserialize)]\n#[serde(untagged)]\nenum Planted { A(u8), B(String) }\n' \
    > "$1/$SCAN_DIR/persist/planted.rs"
}

plant_other() {
  mkdir -p "$1/$SCAN_DIR/persist"
  printf '#[derive(Deserialize)]\nenum Planted {\n    A,\n    #[serde(other)]\n    Rest,\n}\n' \
    > "$1/$SCAN_DIR/persist/planted.rs"
}

plant_flatten() {
  mkdir -p "$1/$SCAN_DIR/persist"
  printf '#[derive(Deserialize)]\nstruct Planted {\n    #[serde(flatten)]\n    rest: Inner,\n}\n' \
    > "$1/$SCAN_DIR/persist/planted.rs"
}

# A SECOND `deserialize_with`, which the allowlist must not cover: it is
# allowed by the function it names, not by the attribute's existence.
plant_second_deserialize_with() {
  mkdir -p "$1/$SCAN_DIR/persist"
  printf '#[derive(Deserialize)]\nstruct Planted {\n    #[serde(deserialize_with = "retry_me")]\n    x: u64,\n}\n' \
    > "$1/$SCAN_DIR/persist/planted.rs"
}

plant_value_intermediate() {
  mkdir -p "$1/$SCAN_DIR/persist"
  printf 'fn planted(text: &str) -> Result<Doc, Error> {\n    let raw: serde_json::Value = serde_json::from_str(text)?;\n    serde_json::from_value(raw)\n}\n' \
    > "$1/$SCAN_DIR/persist/planted.rs"
}

# The attribute spellings with other options beside them: a matcher
# anchored to `#[serde(` immediately followed by the word would miss
# every one of these, and rustfmt produces them routinely.
plant_untagged_beside_another_option() {
  mkdir -p "$1/$SCAN_DIR/persist"
  printf '#[derive(Deserialize)]\n#[serde(deny_unknown_fields, untagged)]\nenum Planted { A(u8) }\n' \
    > "$1/$SCAN_DIR/persist/planted.rs"
}

# `other` is matched with a following `,` or `)` so that an option
# whose name merely CONTAINS it stays green. This is the true positive
# that keeps that narrowing honest.
plant_other_beside_a_rename() {
  mkdir -p "$1/$SCAN_DIR/persist"
  printf '#[derive(Deserialize)]\nenum Planted {\n    #[serde(rename = "x", other)]\n    Rest,\n}\n' \
    > "$1/$SCAN_DIR/persist/planted.rs"
}

# The narrowing's negative control: `skip_serializing_if` and a field
# literally named `other` are not the catch-all variant.
plant_words_that_merely_contain_the_needles() {
  mkdir -p "$1/$SCAN_DIR/persist"
  printf '#[derive(Deserialize)]\nstruct Planted {\n    #[serde(rename = "otherwise")]\n    other_side: u8,\n}\n' \
    > "$1/$SCAN_DIR/persist/planted.rs"
}

# Outside the scanned tree: the premise is about editor-core's wire, and
# another crate reading JSON into a `Value` says nothing about it.
plant_value_in_another_crate() {
  mkdir -p "$1/crates/pncad-py/src"
  printf 'fn elsewhere(t: &str) -> serde_json::Value { serde_json::from_str(t).unwrap() }\n' \
    > "$1/crates/pncad-py/src/planted.rs"
}

gate_selftest() {
  gate_selftest_clean
  # A `grep` that cannot run produces no hits, and no hits is what a
  # clean tree produces — the one failure this gate cannot see for
  # itself unless it is proved here.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  local want="a backtracking or re-read spelling"
  gate_selftest_case "$want" plant_untagged
  gate_selftest_case "$want" plant_other
  gate_selftest_case "$want" plant_flatten
  gate_selftest_case "$want" plant_second_deserialize_with
  gate_selftest_case "$want" plant_value_intermediate
  gate_selftest_case "$want" plant_untagged_beside_another_option
  gate_selftest_case "$want" plant_other_beside_a_rename
  gate_selftest_passes "words that merely contain the needles" plant_words_that_merely_contain_the_needles
  gate_selftest_passes "a Value intermediate in another crate" plant_value_in_another_crate
  printf '%s selftest OK: 7 planted spellings fire (the three attributes, each also beside another option; a second `deserialize_with`, which the by-path allowlist does not cover; and a `serde_json::Value` intermediate); the sanctioned `plane_ref`, an externally-tagged enum, a doc comment naming every banned spelling, an option whose name merely contains `other`, and a `Value` in another crate all stay green; the empty scan fails rather than passing hollow; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' \
    "$(gate_name)"
}

gate_parse_args "$@"
gate_main
