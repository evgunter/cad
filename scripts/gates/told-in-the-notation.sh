#!/usr/bin/env bash
# told-in-the-notation.sh — a sentence written for a notation writes
# everything it formats in that notation. ONE home; ci.yml's `lint` job
# runs every gate in this directory.
#
# THE RULE (`crates/geom-core/README.md`, "A number told to a person":
# a refusal is said in the notation of whoever reads it). A sentence
# that quotes told quantities is written once, in a body that takes
# the reader's notation (`geom_core::Tell::tell`, or a helper it hands
# the notation and the `Formatter` to). A quantity such a body formats
# with a plain `{}` is told by its `Display`, which is metres and
# radians: true, and not what the reader asked for. So:
#
#   IN A FUNCTION THAT TAKES BOTH A `notation` PARAMETER AND A
#   `Formatter`, EVERY ARGUMENT OF A `write!`, `writeln!`, `format!` OR
#   `format_args!` IS ONE OF
#     - told in the notation: its expression holds
#       `.told_in(notation)`;
#     - a constant (an ALL_CAPS name or path), a literal, or a macro
#       call (`move_recourse!(…)`, which builds text);
#     - listed in `told-in-the-notation.allow` as
#       `path :: function :: argument`, a name or expression that is
#       not a quantity (a leg's name, a separator, a count).
#   An inline capture `{name}` is the argument `name`, unless the call
#   names it (`name = …`), and then it is that expression.
#
# An entry of the allow file that no argument matches fires too.
#
# Production code only (every `crates/<crate>/src` file but the
# test-only mounts; comments stripped).
#
# WHAT STAYS BLIND.
#  - What an allowed name is bound to. `side` is allowed in one
#    function by its name; a quantity bound to `side` there passes.
#  - A quantity turned to text before it is formatted
#    (`x.to_string()`), written with `Display::fmt(&x, f)` or
#    `f.write_str`, or formatted in a body that takes a `Formatter`
#    and no `notation` (a helper the notation was not handed to: it
#    cannot tell in it, and nothing here says it should have been
#    handed it).
#  - A sentence forwarded whole to an inner type that has no `Tell`:
#    the gate reads the argument, not the type's impls.
#  - An argument whose expression names `.told_in(notation)` on
#    something other than the value formatted.
#  - A `tell` a macro writes.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

ALLOW=scripts/gates/told-in-the-notation.allow

gate() {
  gate_require_file "$ALLOW"
  gate_require_crate_sources
  gate_production_sources
  local report
  if ! report=$(python3 - "$ALLOW" "${GATE_PRODUCTION_FILES[@]}" <<'PY'
import re, sys

allow_path, files = sys.argv[1], sys.argv[2:]
allowed = {}
for n, line in enumerate(open(allow_path, encoding="utf-8"), 1):
    line = line.split("#", 1)[0].strip()
    if line:
        parts = [p.strip() for p in line.split(" :: ")]
        if len(parts) != 3:
            print(f"{allow_path}:{n}: not `path :: function :: argument`")
            sys.exit(3)
        allowed[tuple(parts)] = False

def strip_comments(text):
    out, i, n = [], 0, len(text)
    while i < n:
        c = text[i]
        if c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append(text[i:j + 1]); i = j + 1
        elif text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif text.startswith("/*", i):
            j = text.find("*/", i)
            i = n if j < 0 else j + 2
        elif c == "'" and i + 2 < n and (text[i + 2] == "'" or (text[i + 1] == "\\" and text.find("'", i + 2) - i <= 4)):
            j = text.find("'", i + 2)
            out.append(text[i:j + 1]); i = j + 1
        else:
            out.append(c); i += 1
    return "".join(out)

def matching(text, i, open_, close):
    """Index one past the bracket closing the one at text[i]."""
    depth, n = 0, len(text)
    while i < n:
        c = text[i]
        if c == '"':
            i += 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
        elif c == open_:
            depth += 1
        elif c == close:
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return n

def split_args(inner):
    args, depth, start, i, n = [], 0, 0, 0, len(inner)
    while i < n:
        c = inner[i]
        if c == '"':
            i += 1
            while i < n and inner[i] != '"':
                i += 2 if inner[i] == "\\" else 1
        elif c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == "," and depth == 0:
            args.append(inner[start:i]); start = i + 1
        i += 1
    args.append(inner[start:])
    return [a.strip() for a in args if a.strip()]

def passes(expr):
    if ".told_in(notation)" in expr:
        return True
    if re.fullmatch(r'(?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Z][A-Z0-9_]*', expr):
        return True
    if re.fullmatch(r'"(?:[^"\\]|\\.)*"|-?[0-9][0-9A-Za-z_.]*', expr, re.S):
        return True
    return bool(re.match(r'(?:[A-Za-z_][A-Za-z0-9_]*::)*[a-z_][a-z0-9_]*!\s*[(\[]', expr))

bad = []
for path in files:
    text = strip_comments(open(path, encoding="utf-8").read())
    for m in re.finditer(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^({]*>)?\s*\(', text):
        end_params = matching(text, m.end() - 1, "(", ")")
        params = text[m.end():end_params]
        if not (re.search(r'\bnotation\s*:', params) and "Formatter" in params):
            continue
        brace = text.find("{", end_params)
        semi = text.find(";", end_params)
        if brace < 0 or (0 <= semi < brace):
            continue
        body = text[brace:matching(text, brace, "{", "}")]
        for call in re.finditer(r'\b(write|writeln|format|format_args)!\s*\(', body):
            close = matching(body, call.end() - 1, "(", ")")
            args = split_args(body[call.end():close - 1])
            if call.group(1) in ("write", "writeln"):
                args = args[1:]
            if not args or not args[0].startswith('"'):
                continue
            literal = re.sub(r'\\\s*\n\s*', "", args[0]).replace("{{", "").replace("}}", "")
            named, positional = {}, []
            for a in args[1:]:
                eq = re.match(r'([A-Za-z_][A-Za-z0-9_]*)\s*=(?!=)\s*(.*)', a, re.S)
                if eq:
                    named[eq.group(1)] = eq.group(2).strip()
                else:
                    positional.append(a)
            exprs = list(positional)
            for hole in re.findall(r'\{([A-Za-z_][A-Za-z0-9_]*)?(?::[^}]*)?\}', literal):
                if hole:
                    exprs.append(named.get(hole, hole))
            for expr in dict.fromkeys(exprs):
                expr = " ".join(expr.split())
                key = (path, m.group(1), expr)
                if key in allowed:
                    allowed[key] = True
                elif not passes(expr):
                    bad.append(f"{path}: fn {m.group(1)}: `{expr}` is formatted in a body that takes a notation and is not told in it")
for (path, function, expr), used in allowed.items():
    if not used and path in files:
        bad.append(f"{allow_path}: `{path} :: {function} :: {expr}` matches no argument")
    elif not used:
        bad.append(f"{allow_path}: `{path}` is not a production source of the tree")
print("\n".join(bad))
sys.exit(1 if bad else 0)
PY
  ); then
    printf '%s\n' "$report"
    gate_error "a body that takes a notation formats something outside it (above): tell it with .told_in(notation), or, if it is not a quantity, list it in $ALLOW as path :: function :: argument (crates/geom-core/README.md, A number told to a person)"
    exit 1
  fi
  gate_ok "every argument a notation-taking sentence formats is told in the notation, a constant, or listed"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# path :: function :: argument\n' > "$1/$ALLOW"
}

plant() { cat >> "$1/crates/clean/src/lib.rs"; }

plant_inline_capture() {
  plant "$1" <<'RS'
impl Tell for R {
    fn tell(&self, notation: Notation, f: &mut Formatter<'_>) -> Result {
        let Self { radius } = self;
        write!(f, "a radius-{radius} fillet")
    }
}
RS
}
plant_positional() {
  plant "$1" <<'RS'
fn write_lever(f: &mut core::fmt::Formatter<'_>, notation: Notation, bound: &Told) -> R {
    write!(f, "use a radius of {} or less", bound)
}
RS
}
plant_named_untold() {
  plant "$1" <<'RS'
impl Tell for R {
    fn tell(&self, notation: Notation, f: &mut Formatter<'_>) -> Result {
        write!(f, "setback {setback}, turn {turn}", setback = fit.first(), turn = self.turn.told_in(notation))
    }
}
RS
}
plant_inner_sentence() {
  plant "$1" <<'RS'
impl Tell for R {
    fn tell(&self, notation: Notation, f: &mut Formatter<'_>) -> Result {
        let what = format!("at corner {}", self.corner);
        f.write_str(&what)
    }
}
RS
}
plant_stale_allow() {
  printf 'crates/clean/src/lib.rs :: tell :: side\n' >> "$1/$ALLOW"
}
plant_not_offences() {
  plant "$1" <<'RS'
impl Tell for R {
    fn tell(&self, notation: Notation, f: &mut Formatter<'_>) -> Result {
        let Self { radius, side } = self;
        write!(
            f,
            "the {side} takes a radius-{radius} fillet of {} {{braces}}. {LEVER} {note}",
            self.turn.told_in(notation),
            radius = radius.told_in(notation),
            note = geom_core::move_recourse!("so it fits"),
        )
    }
}
// No notation to tell in: the other gate's ground.
impl core::fmt::Display for Q {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}", self.x) }
}
fn label(notation: Notation, x: &Told) -> String { format!("{x}") }
RS
  printf 'crates/clean/src/lib.rs :: tell :: side\n' >> "$1/$ALLOW"
}

gate_selftest() {
  local want="formats something outside it"
  gate_selftest_clean
  gate_selftest_case "$want" plant_inline_capture
  gate_selftest_case "$want" plant_positional
  gate_selftest_case "$want" plant_named_untold
  gate_selftest_case "$want" plant_inner_sentence
  gate_selftest_case "$want" plant_stale_allow
  gate_selftest_passes "a told argument, a listed name, a constant, a text macro, an escaped brace, a Display and a String helper" plant_not_offences
  printf '%s selftest OK: passes a clean fixture, an argument told in the notation by position and by name, a listed name, a constant, a text-building macro, an escaped brace, a Display with no notation and a helper that takes no Formatter; fires on an inline capture, a positional argument and a named argument that are not told in the notation, on an inner sentence formatted with a plain hole, and on a listed entry no argument matches\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
