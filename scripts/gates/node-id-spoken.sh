#!/usr/bin/env bash
# node-id-spoken.sh — a sentence never spells a node or step id by its
# bits. ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE (DESIGN.md Band 1, "Node labels"; `editor_core::spoken`). A
# person reads a node as the document speaks it — `Extrude
# 000000000003`, its kind noun and tag (`Doc::spoken`) — or, where no
# document is at hand, by its tag alone (`RecipeNodeId`'s and `StepId`'s
# `Display`). A machine channel prints every bit through `.full()`.
# What none of them does is hand the raw integer to a format macro:
# `format!("node {}", id.0)` printed `node 3` for a counter id and will
# print a 20-digit number for a minted one, and it is the spelling every
# one of those sites drifted to because `.0` is the only field there is.
#
# WHAT THIS MATCHES. In the production sources of the crates that can
# name a `RecipeNodeId` — `editor-core` and the three that build on it
# here, `pncad`, `pncad-py` and `viewer` (`SCOPE` below) — an argument
# of a format-family macro (`format!`, `format_args!`, `write!`,
# `writeln!`, `print!`, `println!`, `eprint!`, `eprintln!`, `panic!`,
# `unreachable!`, `todo!`, `unimplemented!`) that IS a tuple-field read
# `<expr>.0`: the whole argument, at the macro's own nesting depth.
# `doc.spoken(node.0)` and `node.0.full()` are not bare and pass.
#
# THE TYPE IS NOT VISIBLE TO A READER, so the rule is the shape and not
# the type: every bare `.0` argument in scope is refused, whatever the
# tuple holds. That costs a non-id newtype or pair one binding (`let (lo,
# hi) = at;`, a destructuring pattern, or a `LowerHex` on the key) and
# buys a rule with no allowlist. ONE exemption, `self.0`: a newtype
# formatting its own field inside its own impl is where `.0` is the
# spelling — the id types' own `Display` is `editor_core::spoken`'s.
#
# WHAT IT CANNOT SEE, stated:
#  - an id laundered through a binding first (`let n = id.0;` then
#    `{n}`), or read inside a larger expression (`id.0 + 1`,
#    `id.0.to_string()`);
#  - a wrapper around an id formatting its own `self.0` (the exemption
#    above): `pncad-py`'s `NodeId` and `StepId` are the two today, and
#    both print `.full()`;
#  - the assert family, whose leading arguments are conditions;
#  - any crate outside `SCOPE`, a test module, and a test-only mount.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SCOPE=(editor-core pncad pncad-py viewer)

# Records on stdin; prints the ones carrying a bare `.0` macro argument.
bare_field_args() {
  gate_record_awk '
    function bare(arg,   t) {
      t = arg
      sub(/^[ \t]+/, "", t); sub(/[ \t]+$/, "", t)
      if (t !~ /([A-Za-z_][A-Za-z0-9_]*|[\)\]])[ \t]*\.0$/) return 0
      if (t ~ /^self\.0$/) return 0
      return 1
    }
    {
      if (!gate_record_split($0)) next
      s = GR_TEXT; hit = 0
      while (!hit && match(s, /(^|[^A-Za-z0-9_])(format|format_args|write|writeln|print|println|eprint|eprintln|panic|unreachable|todo|unimplemented)![ \t]*[\(\[\{]/)) {
        start = RSTART + RLENGTH       # just past the opening bracket
        depth = 1; arg = ""
        for (i = start; i <= length(s) && depth > 0; i++) {
          c = substr(s, i, 1)
          if (c ~ /[\(\[\{]/) { depth++; if (depth > 1) arg = arg c; continue }
          if (c ~ /[\)\]\}]/) {
            depth--
            if (depth == 0) { if (bare(arg)) hit = 1; break }
            arg = arg c; continue
          }
          if (c == "," && depth == 1) { if (bare(arg)) { hit = 1; break } arg = ""; continue }
          arg = arg c
        }
        s = substr(s, start)
      }
      if (hit) print $0
    }'
}

gate() {
  gate_require_crate_sources
  gate_production_sources
  local -a scan=()
  local f c
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    for c in "${SCOPE[@]}"; do
      case "$f" in crates/"$c"/src/*) scan+=("$f"); break ;; esac
    done
  done
  if [ "${#scan[@]}" -eq 0 ]; then
    gate_error "$(gate_name): none of ${SCOPE[*]} has a production source under $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
  local hits
  hits=$(gate_rust_code --skip-cfg-test --statements "${scan[@]}" | bare_field_args)
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "a format macro is handed a bare tuple field \`.0\` — for a RecipeNodeId or StepId that is the raw integer a person should never read. Speak the node where the document is at hand (\`doc.spoken(id)\`), else format the id itself (its tag), or \`.full()\` on a machine channel; for any other tuple, bind or destructure it first (this file's header says why the rule is the shape)."
    exit 1
  fi
  gate_ok "no format macro in ${SCOPE[*]} hands a bare \`.0\` to a sentence (${#scan[@]} production file(s))"
}

gate_plant_clean() {
  local c
  for c in "${SCOPE[@]}"; do
    mkdir -p "$1/crates/$c/src"
    printf 'pub fn identity(x: f64) -> f64 { x }\n' > "$1/crates/$c/src/lib.rs"
  done
}

plant_bare_node() {
  printf 'fn f(node: RecipeNodeId) -> String { format!("node {}", node.0) }\n' \
    >> "$1/crates/editor-core/src/lib.rs"
}

plant_bare_in_write_over_lines() {
  cat >> "$1/crates/viewer/src/lib.rs" <<'RS'
impl core::fmt::Display for E {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Gone { node } => write!(
                f,
                "feature {} is gone, and so is {}",
                node.0,
                other(node)
            ),
        }
    }
}
RS
}

plant_bare_in_nested_format_args() {
  printf 'fn f(at: RecipeNodeId) -> String { format!("{}", Recourse(format_args!("repair node {}", at.0))) }\n' \
    >> "$1/crates/pncad/src/lib.rs"
}

plant_bare_path_field() {
  printf 'fn f(s: &S) -> String { format!("mate={}", self_like.0.mate.0) }\n' \
    >> "$1/crates/pncad-py/src/lib.rs"
}

# The spellings that are not a bare field, every one of them live in
# this tree: a spoken node, the full id, a field inside a call, a float
# literal, a newtype formatting its own field, a field in prose, a
# string and a comment, an assert's condition, and a test module.
plant_near_misses() {
  cat >> "$1/crates/editor-core/src/lib.rs" <<'RS'
fn a(doc: &Doc, node: NodeId) -> String { format!("{} is not a profile", doc.spoken(node.0)) }
fn b(id: RecipeNodeId) -> String { format!("NodeId({})", id.full()) }
fn c(x: f64) -> String { format!("{} {}", x * 2.0, 1.0) }
impl core::fmt::Display for Recourse { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "Recourse: {}", self.0) } }
fn d(node: RecipeNodeId) -> String { format!("node {node} (not {{node.0}})") }
// format!("node {}", node.0) in a comment
pub const WHY: &str = "format!(\"node {}\", node.0)";
fn e(a: (u64, u64)) { assert!(a.0 < a.1, "{}", a.1); }
#[cfg(test)]
mod tests {
    fn t(node: RecipeNodeId) -> String { format!("node {}", node.0) }
}
RS
}

# A crate outside the scope is not read.
plant_outside_scope() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(p: (f64, f64)) -> String { format!("{}", p.0) }\n' > "$1/crates/topo/src/lib.rs"
}

gate_selftest() {
  local want="a format macro is handed a bare tuple field"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_bare_node
  gate_selftest_case "$want" plant_bare_in_write_over_lines
  gate_selftest_case "$want" plant_bare_in_nested_format_args
  gate_selftest_case "$want" plant_bare_path_field
  gate_selftest_passes "a spoken node, the full id, a field inside a call, float literals, a newtype's own field, prose, a string, a comment, an assert and a test module" plant_near_misses
  gate_selftest_passes "a crate outside the scope" plant_outside_scope
  printf '%s selftest OK: passes a clean fixture, the near misses (a spoken node, the full id, a field inside a call, float literals, a newtype formatting its own field, prose, a string, a comment, an assert, a test module) and a crate outside its scope; fires on a bare field in format!, in a multi-line write!, inside a nested format_args!, and at the end of a field path; and stays RED, with a diagnosis, when grep itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
