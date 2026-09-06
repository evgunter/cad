#!/usr/bin/env bash
# panic-free-macro-bodies.sh — the panic-family lints reach INSIDE
# `macro_rules!` bodies. ci.yml's "panic family inside macro bodies"
# step and local-scripts/ci-local.sh's discipline row both call this
# file.
#
# WHY IT EXISTS. `[workspace.lints.clippy]` sets `unwrap_used`,
# `expect_used`, `panic`, `todo` and `unimplemented` to `warn`, and CI
# runs `-D warnings`, so the family is a hard error everywhere clippy
# looks — and clippy does not lint inside a `macro_rules!` body. The
# same `.expect(…)` is a build failure in a `fn` and silent one line
# away inside a macro that generates that `fn`. That is not a
# hypothetical hole: three `.expect("locate_spans returns a nonempty
# first span")` calls lived inside `nurbs_curve!` through a fully green
# CI, and the curve evaluation API — one of the kernel's hottest
# surfaces — is entirely macro-generated. D9 makes "the kernel never
# panics on any input" a ratified property, and this lint family is how
# it is enforced rather than asserted; a reviewer reading green clippy
# over a macro-generated API is reading a guarantee that stops at the
# macro. This gate is the part of that guarantee clippy cannot give.
#
# WHAT FIRES IT, inside a `macro_rules!` body under `crates/*/src`:
#
#   * `.unwrap` / `.expect` — matched by NAME, followed by anything
#     that is not an identifier character, so `.unwrap()`, `.expect(`
#     and a call wrapped after the method name all fire while
#     `.unwrap_or(`, `.unwrap_or_else(` and `.expect_err(` do not. A
#     user-defined method called `unwrap` fires too: the matcher reads
#     text, has no types, and a method with that name is the shape the
#     rule is about.
#   * `panic!`, `todo!`, `unimplemented!` — the name preceded by a
#     non-identifier character, so `assert_panic!` does not fire and
#     `::panic!` does. Any invocation delimiter, since `!` is what the
#     matcher anchors on.
#
# `unreachable!` IS DELIBERATELY NOT MATCHED, and this is the same
# decision the workspace stanza already made rather than a softening of
# it: `unreachable` is absent from `[workspace.lints.clippy]` by D9's
# D2 addendum (ratified 2026-08-19) — it is the sanctioned mechanism
# for a kernel bug the code can OBSERVE in a branch, is by construction
# not input-reachable, and the thing it replaces is the silent `if let
# Some` discard. Matching it here would make a macro body STRICTER than
# the `fn` beside it, which inverts the hole this gate closes. The gate
# is the stanza, reaching one place clippy cannot.
#
# `dbg_macro` is in that stanza and is equally invisible inside a macro
# body; it is not matched here because it is not panic-family and this
# gate's subject is D9. No macro body in the tree carries a `dbg!`.
#
# HOW THE BODY IS FOUND. Each source file is read through `lib.sh`'s
# code-only view, and from the first `{`, `(` or `[` after a
# `macro_rules! <name>` the scan counts nesting over all three
# delimiter kinds until it returns to zero. All three are counted
# because a macro body is a token tree, in which every kind balances,
# and because the body itself may be delimited by any of them —
# `macro_rules! m ( … );` is a definition a brace-only tracker walks
# straight past. Only text INSIDE the body is matched: the same call in
# a `fn` beside the macro belongs to clippy and is not this gate's to
# report twice.
#
# WHAT THE TRACKER CANNOT SEE:
#
#   * A NESTED BLOCK COMMENT. The shared view already strips the three
#     things that would otherwise desynchronise the count — a `{` in a
#     string, a raw string or a char literal are all lexed away before
#     the tracker runs, and both are planted below. What it does not
#     know is that Rust nests `/* /* */ */`: the first `*/` closes, so
#     the tail of a nested comment is read as code and its delimiters
#     are counted. That is `lib.sh`'s stated limit, inherited here.
#   * A CALL SPLIT ACROSS LINES between the receiver and the method:
#     `value\n.expect(…)` fires (the method and its name are on one
#     record), `value.\nexpect(…)` does not.
#   * A PANIC REACHED INDIRECTLY: a body that invokes a macro fragment
#     (`$mac!(…)`), calls a helper `fn` that panics, or indexes a slice.
#     The first is invisible to any text matcher; the second is clippy's
#     at the helper; the third is not in the family (`indexing_slicing`
#     is absent from the stanza for the same reason `unreachable` is).
#   * A PROC MACRO, which is not a `macro_rules!` body at all.
#   * ANYTHING OUTSIDE `crates/*/src` — no `tests/`, no `demos/`, no
#     `tools/`. Test code is where the family is sanctioned, and the two
#     excluded trees are not workspace members that inherit the stanza.
#
# THE ALLOW IS `#[cfg(test)]` AND NOTHING ELSE — no path list, because a
# path list would grandfather the population the gate reds. A macro body
# is skipped when it is test-only in one of the two ways this tree
# spells that:
#
#   * an inline `#[cfg(test)]` item or module around it, dropped by the
#     shared view (`--skip-cfg-test`);
#   * a whole file whose `mod` line is `#[cfg(test)] mod x;`, resolved
#     from the declaring file's directory. `crates/topo/src/
#     review_m1_pr5_internal.rs` is the live resident: its `leak_probe!`
#     body carries a `.unwrap()` inside a `#[test] fn`, and the file
#     carries no `cfg` of its own because its `mod` line in `lib.rs`
#     carries it.
#
# THAT SECOND RESOLUTION IS TEXTUAL AND IS NOT RUSTC'S. `mod x;` in a
# file that is not a `mod.rs`, `lib.rs` or `main.rs` resolves to
# `<declarer>/x.rs`, not to the sibling `x.rs` this derivation names, so
# a declaration outside a crate root can exclude the wrong file in
# either direction. It is written the way the two gates that already do
# this write it, so the population stays uniform for the row that owns
# the repair (`work/gates/gate-mod-path-resolved-textually.md`); the
# only resident it decides today is declared in a `lib.rs`, where both
# resolutions name the same file.
#
# Panicking IS a test's failure mechanism, which is why the stanza lets
# test code allow the family per-module; a gate that fired there would
# teach the next author that it is noise.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# The family, minus `unreachable` — see the header. `.unwrap`/`.expect`
# by name; the three macros by name and `!`.
PANIC_RE='\.(unwrap|expect)([^A-Za-z0-9_]|$)|[^A-Za-z0-9_](panic|todo|unimplemented)[[:space:]]*!'

# Files whose whole content is test code because their `mod` line is
# `#[cfg(test)]`. Resolved from the DECLARING file's directory, both
# spellings (`x.rs` and `x/mod.rs`), so a renamed module cannot quietly
# re-enter the scan as shipped code. A `#[cfg(test)]` may be followed by
# further attributes before the `mod` line, and `#[path = "…"]` renames
# the file outright; both appear in this tree.
cfg_test_module_files() {
  awk '
    FNR == 1 { armed = 0; path = "" }
    /^[[:space:]]*#\[cfg\(test\)\][[:space:]]*$/ { armed = 1; path = ""; next }
    armed && /^[[:space:]]*#\[path[[:space:]]*=/ {
      if (match($0, /"[^"]+"/)) path = substr($0, RSTART + 1, RLENGTH - 2)
      next
    }
    armed && /^[[:space:]]*#\[/ { next }
    armed && /^[[:space:]]*(pub(\([a-z]+\))?[[:space:]]+)?mod[[:space:]]+[a-z_0-9]+[[:space:]]*;/ {
      name = $0
      sub(/^[[:space:]]*(pub(\([a-z]+\))?[[:space:]]+)?mod[[:space:]]+/, "", name)
      sub(/[[:space:]]*;.*$/, "", name)
      dir = FILENAME
      sub(/\/[^\/]*$/, "", dir)
      if (path != "") print dir "/" path
      else { print dir "/" name ".rs"; print dir "/" name "/" }
      armed = 0; next
    }
    { armed = 0 }
  ' "${GATE_SOURCE_FILES[@]}"
}

# One record per line of `macro_rules!` body, as
# `FILE:LINE:MACRO:BODY-TEXT` — the `grep -rn` shape the filters below
# expect, with the macro name carried so a hit names what it is in. Only
# the in-body span of a line is emitted, so a call OUTSIDE the body
# never reaches the matcher.
macro_bodies() {
  awk '
    function scan(f, ln, t,   pos, seg, i, n, j, c, endpos, body) {
      pos = 1
      while (pos <= length(t)) {
        seg = substr(t, pos)
        if (state == 0) {                      # outside any body
          i = index(seg, "macro_rules!")
          if (i == 0) return
          pos = pos + i + 11
          state = 1
          mname = "?"
          if (match(substr(t, pos), /[A-Za-z_][A-Za-z0-9_]*/)) {
            mname = substr(substr(t, pos), RSTART, RLENGTH)
          }
          continue
        }
        if (state == 1) {                      # after the name, before
          i = match(seg, /[{([]/)               # the opening delimiter
          if (i == 0) return
          pos = pos + i
          state = 2; depth = 1
          continue
        }
        # Inside the body: find where nesting returns to zero.
        n = length(seg); j = 1; endpos = 0
        while (j <= n) {
          c = substr(seg, j, 1)
          if (c ~ /[{([]/) depth++
          else if (c ~ /[]})]/) {
            depth--
            if (depth == 0) { endpos = j; break }
          }
          j++
        }
        body = (endpos > 0) ? substr(seg, 1, endpos - 1) : seg
        if (body != "") print f ":" ln ":" mname ":" body
        if (endpos > 0) { state = 0; pos = pos + endpos } else return
      }
    }
    {
      # The view emits FILE:LINE:TEXT; a body cannot span two files, so
      # the state resets with the filename.
      p1 = index($0, ":"); f = substr($0, 1, p1 - 1); rest = substr($0, p1 + 1)
      p2 = index(rest, ":"); ln = substr(rest, 1, p2 - 1); t = substr(rest, p2 + 1)
      if (f != curf) { curf = f; state = 0; depth = 0 }
      scan(f, ln, t)
    }
  '
}

gate() {
  gate_require_crate_sources
  local excluded hits
  excluded=$(cfg_test_module_files | sort -u)
  hits=$(gate_rust_code --skip-cfg-test "${GATE_SOURCE_FILES[@]}" \
    | macro_bodies \
    | { if [ -n "$excluded" ]; then gate_grep -vF -f <(printf '%s\n' "$excluded" | sed 's#\.rs$#.rs:#'); else cat; fi } \
    | gate_grep -E "$PANIC_RE" \
    | cut -c1-160)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a panic-family call inside a macro_rules! body, where clippy's unwrap_used/expect_used/panic/todo/unimplemented lints do not look. D9 makes \"no panic on any input\" a property of the kernel, not of the code clippy happens to see: return a typed error from the generated function, or move the fallible step into a plain fn the lints reach. If the body is test-only, gate it \`#[cfg(test)]\` — that is the only allow this gate has."
    exit 1
  fi
  gate_ok "no panic-family call inside any macro_rules! body"
}

# The clean fixture carries the negative control in it, so every planted
# case re-proves it: a panic-free macro, and the SAME `.expect` in a
# plain `fn` beside it, which is clippy's to report and not this gate's.
gate_plant_clean() {
  mkdir -p "$1/crates/clean/src"
  cat > "$1/crates/clean/src/lib.rs" <<'RS'
macro_rules! forward {
    ($n:ident) => {
        pub fn $n(v: Option<u32>) -> Option<u32> {
            v
        }
    };
}
forward!(passthrough);
pub fn direct(v: Option<u32>) -> u32 {
    v.expect("clippy sees this one and denies it")
}
RS
}

# The row's own repro, verbatim: clippy warns on `written_directly` and
# says nothing about the generated body.
plant_row_repro() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
macro_rules! make { ($n:ident) => { pub fn $n(v: Option<u32>) -> u32 { v.expect("in macro") } }; }
make!(from_macro);
pub fn written_directly(v: Option<u32>) -> u32 { v.expect("direct") }
RS
}

plant_unwrap() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
macro_rules! take {
    ($n:ident) => {
        pub fn $n(v: Option<u32>) -> u32 {
            v.unwrap()
        }
    };
}
RS
}

plant_panic_macro() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
macro_rules! refuse {
    ($n:ident) => {
        pub fn $n(v: u32) -> u32 {
            if v == 0 {
                panic!("zero");
            }
            v
        }
    };
}
RS
}

plant_todo() {
  printf 'macro_rules! later { ($n:ident) => { pub fn $n() -> u32 { todo!() } }; }\n' \
    > "$1/crates/clean/src/repro.rs"
}

plant_unimplemented() {
  printf 'macro_rules! never { ($n:ident) => { pub fn $n() -> u32 { unimplemented!() } }; }\n' \
    > "$1/crates/clean/src/repro.rs"
}

# THE DELIMITER EVASION. A `macro_rules!` definition may be delimited by
# parentheses or brackets, and a brace-only tracker never enters the
# body at all — it reports green having read nothing.
plant_paren_delimited() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
macro_rules! take (
    ($n:ident) => {
        pub fn $n(v: Option<u32>) -> u32 {
            v.unwrap()
        }
    };
);
RS
}

# A BRACE INSIDE A STRING LITERAL is the common case, not an edge one:
# every `format!("{}", x)` in a macro body has one. If it reached the
# tracker the count would end the body early and everything after it
# would be invisible.
plant_after_a_braced_format_string() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
macro_rules! describe {
    ($n:ident) => {
        pub fn $n(v: Option<u32>) -> String {
            let label = format!("{} {{", "v");
            format!("{}{}", label, v.unwrap())
        }
    };
}
RS
}

# The same question for a char literal and a raw string, which are the
# other two ways to write a brace that is not code.
plant_after_a_brace_char_literal() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
macro_rules! describe {
    ($n:ident) => {
        pub fn $n(v: Option<u32>) -> u32 {
            let open = '{';
            let raw = r#"} { "#;
            let _ = (open, raw);
            v.unwrap()
        }
    };
}
RS
}

# THE TRACKER MUST LET GO. A body that closes and a second macro after
# it: if the first body never ends, the second is read as part of it and
# a fix that broke the exit would still look green here — so the hit has
# to name the SECOND macro.
plant_second_macro_after_a_clean_one() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
macro_rules! first {
    ($n:ident) => {
        pub fn $n(v: u32) -> u32 {
            v
        }
    };
}
macro_rules! second {
    ($n:ident) => {
        pub fn $n(v: Option<u32>) -> u32 {
            v.expect("second")
        }
    };
}
RS
}

# GREEN cases. A gate that fires on these is a gate people route around.

# The whole point of the fence: clippy owns the `fn` beside the macro,
# and a gate that reported it too would double every diagnostic in the
# tree.
plant_plain_fn_only() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
pub fn direct(v: Option<u32>) -> u32 {
    v.expect("clippy denies this one")
}
pub fn also_direct(v: Option<u32>) -> u32 {
    v.unwrap()
}
RS
}

# THE ALLOW, both spellings: the attribute on the item, and the item
# inside a `#[cfg(test)]` module.
plant_cfg_test_macro() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
#[cfg(test)]
macro_rules! probe {
    ($n:ident) => {
        fn $n(v: Option<u32>) -> u32 {
            v.expect("a test panics on purpose")
        }
    };
}
#[cfg(test)]
mod tests {
    macro_rules! assert_some {
        ($v:expr) => {
            assert_eq!($v.unwrap(), 1);
        };
    }
    #[test]
    fn t() {
        assert_some!(Some(1u32));
    }
}
RS
}

# THE ALLOW's third spelling and the live one: a whole file that is test
# code because its `mod` line says so, carrying no `cfg` of its own.
plant_cfg_test_module_file() {
  cat > "$1/crates/clean/src/lib.rs" <<'RS'
#[cfg(test)]
mod probes;
pub fn identity(x: f64) -> f64 {
    x
}
RS
  cat > "$1/crates/clean/src/probes.rs" <<'RS'
macro_rules! leak_probe {
    ($arena:ident) => {{
        let k = $arena.keys().next().unwrap();
        k
    }};
}
#[test]
fn t() {
    let _ = 1;
}
RS
}

# `unreachable!` is absent from the workspace stanza by ratified
# decision, so it must stay green here: a macro body stricter than the
# `fn` beside it is the inverse of the hole this gate closes. The live
# resident is `nurbs_fit!` in crates/geom/src/curves/fit.rs.
plant_unreachable() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
macro_rules! decide {
    ($n:ident) => {
        pub fn $n(v: u32) -> u32 {
            match v {
                0..=9 => v,
                _ => unreachable!("callers clamp"),
            }
        }
    };
}
RS
}

# The near misses the family does NOT contain, inside a body so the only
# thing that could clear them is the matcher itself.
plant_near_miss_names() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
macro_rules! careful {
    ($n:ident) => {
        pub fn $n(v: Option<u32>) -> u32 {
            let a = v.unwrap_or(0);
            let b = v.unwrap_or_else(|| 0);
            let c: Result<u32, u32> = Err(1);
            let d = c.expect_err("carries no panic in the family");
            debug_assert!(a == b);
            assert!(d > 0);
            a
        }
    };
}
RS
}

# The tokens as PROSE and as string literals inside a body — this gate's
# own header spells every one of them, and a gate reading literals as
# code reds on its own documentation.
plant_prose_only() {
  cat > "$1/crates/clean/src/repro.rs" <<'RS'
macro_rules! documented {
    ($n:ident) => {
        // Never write v.expect("x") or panic!() in here.
        /* Nor .unwrap() nor todo!() across
         * a continuation line. */
        pub fn $n(v: Option<u32>) -> u32 {
            let why = "v.unwrap() and unimplemented!() are prose here";
            let _ = why;
            v.unwrap_or(0)
        }
    };
}
RS
}

gate_selftest() {
  local want="a panic-family call inside a macro_rules! body"
  gate_selftest_clean
  # A `grep` that cannot run produces no hits, and no hits is what a
  # clean tree produces.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_row_repro
  gate_selftest_case "$want" plant_unwrap
  gate_selftest_case "$want" plant_panic_macro
  gate_selftest_case "$want" plant_todo
  gate_selftest_case "$want" plant_unimplemented
  gate_selftest_case "$want" plant_paren_delimited
  gate_selftest_case "$want" plant_after_a_braced_format_string
  gate_selftest_case "$want" plant_after_a_brace_char_literal
  gate_selftest_case "$want" plant_second_macro_after_a_clean_one
  gate_selftest_passes "the same call in a plain fn beside the macro" plant_plain_fn_only
  gate_selftest_passes "a #[cfg(test)] macro item and one in a #[cfg(test)] module" plant_cfg_test_macro
  gate_selftest_passes "a macro in a file declared #[cfg(test)] mod x;" plant_cfg_test_module_file
  gate_selftest_passes "unreachable!, which the workspace stanza omits" plant_unreachable
  gate_selftest_passes "unwrap_or/unwrap_or_else/expect_err/assert!/debug_assert!" plant_near_miss_names
  gate_selftest_passes "the tokens in comments and string literals" plant_prose_only
  printf '%s selftest OK: 9 planted spellings fire (the row repro, .unwrap, panic!, todo!, unimplemented!, a paren-delimited definition, a body after a braced format string, a body after a brace char literal and raw string, and a second macro after a clean one); the same call in a plain fn, both #[cfg(test)] spellings, a file declared `#[cfg(test)] mod x;`, unreachable!, the unwrap_or/expect_err/assert! near misses and comment/string-literal mentions stay green; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' \
    "$(gate_name)"
}

gate_parse_args "$@"
gate_main
