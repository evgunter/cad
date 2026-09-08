#!/usr/bin/env bash
# panic-free-macro-bodies.sh — the stanza's macro-blind lints reach
# INSIDE `macro_rules!` bodies. ci.yml's "panic family inside macro
# bodies" step and local-scripts/ci-local.sh's discipline row both call
# this file.
#
# WHY IT EXISTS. `[workspace.lints.clippy]` sets `unwrap_used`,
# `expect_used`, `panic`, `todo`, `unimplemented` and `dbg_macro` to
# `warn`, and CI runs `-D warnings`, so they are hard errors everywhere
# clippy looks — and clippy does not lint inside a `macro_rules!` body.
# The same `.expect(…)` is a build failure in a `fn` and silent one line
# away inside the macro that generates that `fn`: three
# `.expect("locate_spans returns a nonempty first span")` calls lived
# inside `nurbs_curve!` through a fully green CI, and the curve
# evaluation API — one of the kernel's hottest surfaces — is entirely
# macro-generated. D9 makes "the kernel never panics on any input" a
# ratified property and this family is how it is enforced rather than
# asserted, so a reviewer reading green clippy over a macro-generated
# API is reading a guarantee that stops at the macro. This gate is the
# part of it clippy cannot give.
#
# WHAT FIRES IT, inside a `macro_rules!` body under `crates/*/src` — the
# stanza's six macro-blind lints, and no more:
#
#   * `.unwrap` / `.expect`, matched by NAME and followed by a
#     non-identifier character, so `.unwrap()`, `.expect(` and a call
#     wrapped after the method name fire while `.unwrap_or(`,
#     `.unwrap_or_else(` and `.expect_err(` do not. A user-defined
#     method called `unwrap` fires too: the matcher has no types, and a
#     method with that name is the shape the rule is about.
#   * `panic!`, `todo!`, `unimplemented!`, `dbg!`, the name preceded by
#     a non-identifier character, so `assert_panic!` does not fire and
#     `::panic!` does; any invocation delimiter, since `!` is the
#     anchor. `dbg_macro` is not panic-family, but it is denied by the
#     same stanza and blind in the same place, and "no macro body
#     carries a `dbg!`" is otherwise a reading nothing re-derives.
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
# HOW THE BODY IS FOUND. From the first `{`, `(` or `[` after the macro
# name, nesting is counted over all three delimiter kinds until it
# returns to zero: a macro body is a token tree, in which every kind
# balances, and the definition itself may use any of them
# (`macro_rules! m ( … );` is planted, as is the `macro_rules ! m`
# spacing the compiler accepts). Only text INSIDE the body is matched —
# the same call in a `fn` beside the macro is clippy's, not this
# gate's to report twice.
#
# A `{` INSIDE A COMMENT OR A LITERAL IS NOT A BRACE the tracker counts:
# the shared view lexes away strings, raw strings and char literals (all
# three planted below) and block comments to their BALANCING `*/`,
# nesting included. A nested `/* /* */ */` was on the list below while
# `lib.sh`'s lexer closed at the first `*/`; it nests now, so it is not.
#
# WHAT THE TRACKER CANNOT SEE:
#
#   * A CALL SPLIT ACROSS LINES between the receiver and the method:
#     `value\n.expect(…)` fires (the method and its name are on one
#     record), `value.\nexpect(…)` does not.
#   * THE UFCS FORM, `Option::unwrap(v)` / `Option::expect(v, …)`. This
#     one is PARITY, not a hole this gate opens: clippy's `unwrap_used`
#     and `expect_used` are method-call lints and do not fire on it
#     either (measured on clippy 0.1.94 with the stanza set to `warn`:
#     the UFCS pair produced no diagnostic while `.expect(…)` one line
#     up produced one). The macro body is exactly as covered as the `fn`
#     beside it, which is all this gate claims.
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
#   * a whole file whose `mod` line is `#[cfg(test)] mod x;`, in either
#     spelling — the attribute alone on its line, or the whole
#     declaration on one. `crates/topo/src/review_m1_pr5_internal.rs`
#     is the live resident: its `leak_probe!` body carries a
#     `.unwrap()` inside a `#[test] fn`, and the file carries no `cfg`
#     of its own because its `mod` line in `lib.rs` carries it.
#
# WHERE that file lives is not decided here: `gate_test_only_mounts`
# places it and `gate_filter_test_only_paths` takes it out of the scan,
# under `lib.sh`'s §"WHERE A TEST-ONLY MODULE LIVES".
#
# Panicking IS a test's failure mechanism, which is why the stanza lets
# test code allow the family per-module; a gate that fired there would
# teach the next author that it is noise.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# The six tokens, minus `unreachable` — see the header. `.unwrap` and
# `.expect` by name; the four macros by name and `!`. Every token needs
# a non-identifier character beside it, which the emitted record
# guarantees at both ends: the body is preceded by its `:` separator and
# `macro_bodies` appends a trailing space.
PANIC_TOKENS='\.(unwrap|expect)[^A-Za-z0-9_]|[^A-Za-z0-9_](panic|todo|unimplemented|dbg)[[:space:]]*!'
# THE MATCH IS FENCED TO THE BODY FIELD. A record is
# `FILE:LINE:MACRO:BODY`; the LINE and MACRO fields hold no colon, so
# the fence below consumes them exactly and a macro name that happens to
# spell a token is not scanned as if it were code.
#
# THE FENCE OPENS AT THE FIRST `:LINE:`, `lib.sh`'s one reading of where
# the FILE column ends (§"THE COLUMNS OF A RECORD"), and NOT at
# `GATE_RECORD_PREFIX_RE` — that expression demands a FILE column with
# no colon of its own, so a record from `a:b.rs` matched it nowhere and
# every panic token in a macro body in such a file went UNSEEN. The
# residue is the ambiguous path `bounds-allowlist.sh` registers as its
# KNOWN GAP 8 and `lib.sh` states in general: a path spelling `:digits:`
# itself opens the fence early, and a token in the tail of that path
# would be read as body text.
#
# IT IS NO LONGER `^`-ANCHORED, AND THAT IS A PROPERTY OF LEFTMOST
# MATCHING RATHER THAN OF THE PATTERN. `^[^:]*` was the anchor and it is
# the expression that cannot survive a colon in the path, so what is
# left starts at a `:LINE:` wherever the record has one. `grep -E` is a
# FILTER here — the question is whether the record matches anywhere, not
# where — so "the fence opens at the FIRST `:LINE:`" holds because the
# earliest `:digits:` of a well-formed record IS its line column, not
# because the pattern says so. A match starting at a LATER one is not a
# false hit: everything after a colon inside the BODY is still body, so
# the token it finds is in the field this fence exists to pin. The one
# reading that changes is the ambiguous path above, which is the same
# residue the reading itself carries.
PANIC_RE="$GATE_RECORD_LINE_RE[^:]*:.*($PANIC_TOKENS)"

# One record per line of `macro_rules!` body, as
# `FILE:LINE:MACRO:BODY-TEXT` — the `grep -rn` shape the filters below
# expect, with the macro name carried so a hit names what it is in. Only
# the in-body span of a line is emitted, so a call OUTSIDE the body
# never reaches the matcher.
macro_bodies() {
  gate_record_awk '
    function scan(f, ln, t,   pos, seg, i, n, j, c, endpos, body) {
      pos = 1
      while (pos <= length(t)) {
        seg = substr(t, pos)
        if (state == 0) {                      # outside any body
          # `macro_rules ! m { … }` is valid Rust, so the bang is
          # matched with the space rustfmt would remove.
          if (match(seg, /macro_rules[[:space:]]*!/) == 0) return
          pos = pos + RSTART + RLENGTH - 1
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
        # The trailing space is the matcher s right-hand fence: a token
        # ending the body has a non-identifier character after it.
        if (body != "") print f ":" ln ":" mname ":" body " "
        if (endpos > 0) { state = 0; pos = pos + endpos } else return
      }
    }
    {
      # The view emits FILE:LINE:TEXT; a body cannot span two files, so
      # the state resets with the filename. WHICH filename comes from
      # `gate_record_split`, the one reading in this directory (no
      # apostrophe may appear in this program, which is itself
      # single-quoted, so possessives are written around): read to the
      # first colon instead, a path carrying one of its own gave a
      # truncated key that two files could share, splicing one body into
      # the next, and handed the line number to the body text as if it
      # were code.
      if (!gate_record_split($0)) next
      f = GR_FILE; ln = GR_LINE; t = GR_TEXT
      if (f != curf) { curf = f; state = 0; depth = 0 }
      scan(f, ln, t)
    }
  '
}

gate() {
  gate_require_crate_sources
  local hits
  # A FILE the scan never reads, rather than a record filtered after it:
  # the test-only modules leave the file set, so the count this gate
  # prints names what it actually read.
  gate_production_sources
  hits=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}" \
    | macro_bodies \
    | gate_grep -E "$PANIC_RE" \
    | cut -c1-160)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "one of the stanza's macro-blind lints inside a macro_rules! body, where clippy's unwrap_used/expect_used/panic/todo/unimplemented/dbg_macro do not look. D9 makes \"no panic on any input\" a property of the kernel, not of the code clippy happens to see: return a typed error from the generated function, or move the fallible step into a plain fn the lints reach. If the body is test-only, gate it \`#[cfg(test)]\` — that is the only allow this gate has."
    exit 1
  fi
  gate_ok "no macro_rules! body spells .unwrap, .expect, panic!, todo!, unimplemented! or dbg! — the stanza's six macro-blind lints, in bodies delimited by {}, () or [], less the blind spots this file's header lists"
}

# The clean fixture carries the negative control in it — a panic-free
# macro, and the SAME `.expect` in a plain `fn` beside it, which is
# clippy's to report and not this gate's. Every planted case re-proves
# it, because each writes a second file and leaves this one standing.
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

# The stanza's sixth macro-blind lint. Not panic-family, denied by the
# same table, and invisible in the same place.
plant_dbg() {
  printf 'macro_rules! trace { ($n:ident) => { pub fn $n(v: u32) -> u32 { dbg!(v) } }; }\n' \
    > "$1/crates/clean/src/repro.rs"
}

# `macro_rules ! m { … }` is what rustfmt normalises away and the
# compiler accepts either way, so a matcher spelling the bang without
# the space is a definition the gate never enters.
plant_space_before_bang() {
  printf 'macro_rules ! take { ($n:ident) => { pub fn $n(v: Option<u32>) -> u32 { v.unwrap() } }; }\n' \
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

# THE BREACH `lib.sh`'s test-module cases plant, and the only thing this
# gate supplies to them: a macro body spelling one of the six tokens,
# appended to a file whose directory they have already made. It is the
# live resident's own shape — a `.unwrap()` in a body, in a file whose
# `mod` line carries the only `cfg` there is.
plant_macro_panic_at() {
  printf 'macro_rules! leak_probe { ($a:ident) => { $a.keys().next().unwrap() }; }\n' >> "$1"
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

# THE RECORD'S OWN PREFIX. A matcher run over the whole
# `FILE:LINE:MACRO:BODY` record reads the path as if it were code, and a
# path may hold a token: this plants the file name that spells one.
# A MACRO BODY IN A FILE WHOSE PATH CARRIES A COLON, which is legal here
# and in git. Both halves of this gate read the record's FILE column —
# the reader, which keys its per-file state on it and cuts the body text
# after it, and the matcher, whose fence opens at the line number — and
# read to the first colon both were wrong at once: the reader handed the
# body the line number as if it were code and gave two files one key,
# and the matcher then found no `LINE` field where it expected one and
# matched NOTHING. Blind, which is why this is a must-FIRE case and its
# green twin is `plant_token_in_the_path` above.
plant_colon_path_macro_body() {
  cat > "$1/crates/clean/src/a:b.rs" <<'RS'
macro_rules! take {
    ($n:ident) => {
        pub fn $n(v: Option<u32>) -> u32 {
            v.unwrap()
        }
    };
}
RS
}

plant_token_in_the_path() {
  cat > "$1/crates/clean/src/todo!.rs" <<'RS'
macro_rules! fine {
    ($n:ident) => {
        pub fn $n(v: Option<u32>) -> Option<u32> {
            v
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
  local want="inside a macro_rules! body"
  gate_selftest_clean
  # A `grep` that cannot run produces no hits, and no hits is what a
  # clean tree produces.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_row_repro
  gate_selftest_case "$want" plant_unwrap
  gate_selftest_case "$want" plant_panic_macro
  gate_selftest_case "$want" plant_todo
  gate_selftest_case "$want" plant_unimplemented
  gate_selftest_case "$want" plant_dbg
  gate_selftest_case "$want" plant_space_before_bang
  gate_selftest_case "$want" plant_paren_delimited
  gate_selftest_case "$want" plant_after_a_braced_format_string
  gate_selftest_case "$want" plant_after_a_brace_char_literal
  gate_selftest_case "$want" plant_second_macro_after_a_clean_one
  gate_selftest_case "$want" plant_colon_path_macro_body
  gate_selftest_passes "the same call in a plain fn beside the macro" plant_plain_fn_only
  gate_selftest_passes "a #[cfg(test)] macro item and one in a #[cfg(test)] module" plant_cfg_test_macro
  gate_selftest_passes "unreachable!, which the workspace stanza omits" plant_unreachable
  gate_selftest_passes "unwrap_or/unwrap_or_else/expect_err/assert!/debug_assert!" plant_near_miss_names
  gate_selftest_passes "a token in the FILE PATH rather than the body" plant_token_in_the_path
  gate_selftest_passes "the tokens in comments and string literals" plant_prose_only
  gate_selftest_test_module_homes "$want" plant_macro_panic_at
  printf '%s selftest OK: 12 planted spellings fire (the row repro, .unwrap, panic!, todo!, unimplemented!, dbg!, a space before the bang, a paren-delimited definition, a body after a braced format string, a body after a brace char literal and raw string, a second macro after a clean one, and a body in a file whose PATH carries a colon, which both the reader and the matcher read the FILE column out of); the same call in a plain fn, both inline #[cfg(test)] spellings, unreachable!, the unwrap_or/expect_err/assert! near misses, a token spelled by the FILE PATH and comment/string-literal mentions stay green; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' \
    "$(gate_name)"
}

gate_parse_args "$@"
gate_main
