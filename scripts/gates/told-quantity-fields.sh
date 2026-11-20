#!/usr/bin/env bash
# told-quantity-fields.sh — a refusal's payload holds told quantities,
# and its sentence formats no scalar itself. ONE home; ci.yml's `lint`
# job runs every gate in this directory.
#
# THE RULE (`crates/geom-core/README.md`, "A number told to a person").
# A number a sentence quotes is a `geom_core::Told`, built where the
# payload is built. So, in a converted crate:
#
#  1. NO REFUSAL TYPE DECLARES A BARE SCALAR FIELD. A refusal type is a
#     `struct` or `enum` the crate declares and gives a `Display` or an
#     `Error` impl (for the type, a reference to it or a `Box`, `Rc` or
#     `Arc` of it), and every type
#     of the crate one of those names in a field, transitively (a
#     payload's parts are payload); two types of one name are both read
#     under it. A field is bare when its type names a scalar — `f64`,
#     `f32`, `T`, `Interval`, `Dual`, `Sym`, `Probe` — anywhere but
#     inside a type the crate declares (whose own fields are read in
#     turn, a field typed by the type's own parameter counting as a
#     scalar whatever the parameter is called) or a foreign position (`Point2`, `Vec2`, `Point3`, `Vec3`
#     and the unit vectors): alone, path-qualified, or under any
#     container (`Option`, `Vec`, `Arc`, `HashMap`, `Result`, an array, a
#     tuple, a reference).
#  2. NO SENTENCE-WRITING BODY FORMATS A SCALAR ITSELF: no format whose
#     spec ends in `?`, `e` or `E` or carries a precision, whatever
#     width, sign or flag stands beside it (`{:?}`, `{x:e}`, `{:.3e}`,
#     `{:10?}`, `{:.4}`). The bodies read are every `impl … Display for
#     …` and every function taking a `Formatter`, an `impl … Debug for
#     …` body excepted. An escaped brace (`{{:?}}`) is text. A line
#     that still prints a Debug NAME is pinned in
#     `told-quantity-fields.pins` as `path :: the line`, beside its
#     reason: a line the pins do not hold fires, and so does a pin no
#     line matches. The pin holds the LINE'S TEXT; see the blind list
#     for what that does not hold.
#
# THE CRATES IT COVERS are every `crates/*` directory that
# `told-quantity-fields.uncovered` does not list. That file is the
# sweep's remaining work: converting a crate deletes its line, and a
# crate the file neither lists nor the tree holds is refused. Nothing
# stops a covered crate being listed again; that is a diff to this
# file's companion, read in review.
#
# Production code only (every `crates/<crate>/src` file but the
# test-only mounts, and in it no inline `#[cfg(test)]` module; comments
# stripped, string literals kept for rule 2 and stripped for rule 1).
#
# WHAT STAYS BLIND.
#  - A scalar inside a foreign position: `Point2<T>`, `Vec2<T>`.
#    `profile`'s refusals hold four positions as `Point2<T>`, told
#    through `Told` when rendered and read as numbers by the viewer and
#    the Python door
#    (`work/scalar/a-refusals-corner-point-is-read-as-a-number-by-the-viewer-and-python.md`).
#  - A scalar under a name this file does not list: a type alias of
#    `f64`, a second type parameter, a foreign struct that holds one.
#    The match is on the scalar names above.
#  - A refusal type with neither a `Display` nor an `Error` impl that no
#    such type names in a field: a sentence written by a free function
#    over a plain struct is not found by rule 1.
#  - A `Display` a macro writes (a derive, a `macro_rules!` body): the
#    reader sees the invocation, not the impl. The workspace derives
#    none today.
#  - A helper that returns a `String` (`fn text(&self) -> String {
#    format!("{:?}", …) }`) called from a `Display`: it takes no
#    `Formatter`, so rule 2 does not read its body.
#  - What a pinned line's arguments are bound to. A pin is the text of
#    one line, so `write!(f, "the {verb:?} verb")` stays pinned when
#    `let verb = self.at.x;` is added above it, or when the argument
#    sits on the next line and is swapped there. A pin is a reviewed
#    exception, not a guard on its own argument; the covered crates
#    hold none.
#  - A scalar turned to text before it is formatted:
#    `self.at.x.to_string()` in a `Display`, or pushed into a `String`
#    by `write!` on something that is not a `Formatter`.
#  - A scalar formatted with a bare `{}`. The formatter does not say
#    what type it prints, so a `{}` over an `f64` local in a body reads
#    as any other `{}`; rule 1 is what keeps an `f64` from being there.
#  - A `Formatter` reached through a closure (`fmt::from_fn(|f| …)`)
#    outside any body rule 2 reads.
#  - A told quantity a notation-taking body writes with a bare `{}`.
#    A sentence is written once, in `geom_core::Tell::tell` (a function
#    taking a `Formatter`, so rule 2 reads it), and each quantity in it
#    goes through `told_in(notation)`; one formatted with `{}` reads in
#    metres whatever the reader's notation, and so does a whole inner
#    sentence forwarded with `{}`. The text is true (its unit word says
#    metres) and the formatter does not say what type it prints.
#    `profile`'s rows `a_refusal_reads_in_the_readers_units` render its
#    refusals in millimetres and degrees and refuse a canonical word.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

UNCOVERED=scripts/gates/told-quantity-fields.uncovered
PINS=scripts/gates/told-quantity-fields.pins

gate() {
  gate_require_file "$UNCOVERED"
  gate_require_file "$PINS"
  gate_require_crate_sources
  gate_production_sources
  local report
  if ! report=$(python3 - "$UNCOVERED" "$PINS" "${GATE_PRODUCTION_FILES[@]}" <<'PY'
import os, re, sys

uncovered_path, pins_path, files = sys.argv[1], sys.argv[2], sys.argv[3:]

# A scalar by its type name, and the foreign geometry types a scalar
# inside is not read through (the header's blind list names them).
SCALARS = {"f64", "f32", "T", "Interval", "Dual", "Dual64", "DualInterval", "Sym", "Probe"}
POSITIONS = {"Point2", "Point3", "Vec2", "Vec3", "UnitVec2", "UnitVec3", "PhantomData"}


def rows(path):
    out = []
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            line = line.rstrip("\n")
            if line.strip() and not line.lstrip().startswith("#"):
                out.append(line)
    return out


def strip(text, keep_strings):
    # Comments out; string and char literals out unless kept (their
    # braces are then marked, so no bracket count reads them). The code
    # and its newlines stay.
    out, i, n = [], 0, len(text)

    def literal(j):
        body = text[i:j]
        if keep_strings:
            # An escaped brace is text, not a format.
            body = body.replace("{{", "  ").replace("}}", "  ")
            return body.replace("{", "\x01").replace("}", "\x02")
        return '""' + "\n" * body.count("\n")

    while i < n:
        c = text[i]
        two = text[i:i + 2]
        if two == "//":
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif two == "/*":
            depth, j = 1, i + 2
            while j < n and depth:
                if text[j:j + 2] == "/*":
                    depth, j = depth + 1, j + 2
                elif text[j:j + 2] == "*/":
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            out.append("\n" * text.count("\n", i, j))
            i = j
        elif c == "r" and re.match(r'r#*"', text[i:i + 8]) and not (i and (text[i - 1].isalnum() or text[i - 1] == "_")):
            hashes = len(re.match(r"r(#*)", text[i:]).group(1))
            close = '"' + "#" * hashes
            j = text.find(close, i + 2 + hashes)
            j = n if j < 0 else j + len(close)
            out.append(literal(j))
            i = j
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            j += 1
            out.append(literal(j))
            i = j
        elif c == "'":
            m = re.match(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^'\\\n])'", text[i:])
            if m:
                out.append("' '")
                i += m.end()
            else:
                out.append(c)
                i += 1
        else:
            out.append(c)
            i += 1
    return "".join(out)


def angle(text, i):
    # Whether the `<` or `>` at `i` is a generic bracket: not the head
    # of an arrow, and not in a run of them with a space on each side (a
    # shift or a comparison, as the formatter writes one).
    c = text[i]
    if c == ">" and i and text[i - 1] in "-=":
        return False
    lo = hi = i
    while lo and text[lo - 1] == c:
        lo -= 1
    while hi + 1 < len(text) and text[hi + 1] == c:
        hi += 1
    before = text[lo - 1] if lo else " "
    after = text[hi + 1] if hi + 1 < len(text) else " "
    return not (before == " " and after in " =")


def matching(text, at, open_="{", close="}"):
    depth = 0
    for i in range(at, len(text)):
        c = text[i]
        if c in "<>" and open_ == "<" and not angle(text, i):
            continue
        if c == open_:
            depth += 1
        elif c == close:
            depth -= 1
            if depth == 0:
                return i
    return len(text) - 1


def without_test_mods(text):
    out, at = [], 0
    for m in re.finditer(r"#\[cfg\(test\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{", text):
        if m.start() < at:
            continue
        out.append(text[at:m.start()])
        end = matching(text, m.end() - 1) + 1
        out.append("\n" * text.count("\n", m.start(), end))
        at = end
    out.append(text[at:])
    return "".join(out)


def split_top(text, sep=","):
    parts, depth, at = [], 0, 0
    for i, c in enumerate(text):
        if c in "<>" and not angle(text, i):
            continue
        if c in "<([{":
            depth += 1
        elif c in ">)]}":
            depth -= 1
        elif c == sep and depth == 0:
            parts.append(text[at:i])
            at = i + 1
    parts.append(text[at:])
    return [p.strip() for p in parts if p.strip()]


def bare_scalars(ty, declared, scalars):
    # The scalar names `ty` holds that a sentence could reach: every one
    # not inside a type this crate declares (its own fields are read in
    # turn) or a foreign position.
    found, stack, last = [], [], ""
    for m in re.finditer(r"'?[A-Za-z_]\w*|[<>]", ty):
        tok = m.group(0)
        if tok == "<":
            if angle(ty, m.start()):
                stack.append(last)
            continue
        if tok == ">":
            if angle(ty, m.start()) and stack:
                stack.pop()
            continue
        if tok.startswith("'"):
            continue
        last = tok
        if tok in scalars and not any(s in declared or s in POSITIONS for s in stack):
            found.append(tok)
    return found


ATTR = re.compile(r"#\[[^\]]*\]")
VIS = re.compile(r"^pub(?:\([^)]*\))?\s+")


def field_types(body, tuple_like):
    # (name, type) of each field in a brace body or a tuple body.
    out = []
    for part in split_top(ATTR.sub("", body)):
        part = VIS.sub("", part)
        if tuple_like:
            out.append(("a tuple field", part))
        elif ":" in part:
            name, ty = part.split(":", 1)
            out.append((name.strip(), ty.strip()))
    return out


DECL = re.compile(r"\b(struct|enum)\s+(\w+)\s*")
IMPL = re.compile(
    r"\bimpl\b[^{;]*?\b(Display|Error)\s+for\s+(?:&\s*(?:'\w+\s+)?(?:mut\s+)?)?"
    r"(?:(?:[\w:]*::)?(?:Box|Rc|Arc)\s*<\s*)*(?:[\w:]*::)?(\w+)[^{;]*\{"
)
DISPLAY = re.compile(r"\bimpl\b[^{;]*?\bDisplay\s+for\s+[^{;]*\{")
DEBUG = re.compile(r"\bimpl\b[^{;]*?\bDebug\s+for\s+[^{;]*\{")
WRITER = re.compile(r"\bfn\s+\w+\s*(?:<[^>]*>)?\s*\([^)]*\bFormatter\b[^)]*\)[^{;]*\{")
# A format whose spec asks for Debug, an exponent or a precision.
FORMATS = re.compile("\x01[^\x01\x02:]*:([^\x01\x02]*)\x02")


def scalar_format(spec):
    return spec.endswith(("?", "e", "E")) or re.search(r"\.(\d+|\*|\w+\$)", spec) is not None


def declarations(code):
    # (kind, name, fields, line) of every struct and enum, its generic
    # list and where clause skipped by bracket, whatever they hold.
    for m in DECL.finditer(code):
        kind, name, at = m.group(1), m.group(2), m.end()
        params = []
        if at < len(code) and code[at] == "<":
            close = matching(code, at, "<", ">")
            params = [
                re.match(r"\w+", part).group(0)
                for part in split_top(code[at + 1:close])
                if re.match(r"[A-Za-z_]\w*", part) and not part.startswith("const ")
            ]
            at = close + 1
        # A tuple body comes before any `where`; a brace body after it,
        # past whatever brackets the clause holds.
        head = re.compile(r"\s*(?:(\()|where\b[^{;]*([{;])|([{;]))").match(code, at)
        if not head:
            continue
        opener, at = next(g for g in head.groups() if g), head.end()
        fields = []
        if opener == "{":
            body = code[at:matching(code, at - 1)]
            if kind == "struct":
                fields = field_types(body, False)
            else:
                for variant in split_top(ATTR.sub("", body)):
                    vm = re.match(r"(\w+)\s*([{(])", variant)
                    if not vm:
                        continue
                    open_ = vm.group(2)
                    end = matching(variant, vm.end() - 1, open_, "}" if open_ == "{" else ")")
                    inner = variant[vm.end():end]
                    fields += [(f"{vm.group(1)}::{n}", t) for n, t in field_types(inner, open_ == "(")]
        elif opener == "(":
            fields = field_types(code[at:matching(code, at - 1, "(", ")")], True)
        yield kind, name, params, fields, code.count("\n", 0, m.start()) + 1


uncovered = set()
for row in rows(uncovered_path):
    if len(row.split()) != 1:
        sys.exit(f"{uncovered_path}: not one crate name: {row}")
    uncovered.add(row.strip())
crates = sorted({p.split("/")[1] for p in files})
for name in sorted(uncovered - set(crates)):
    if not os.path.isdir(os.path.join("crates", name)):
        print(f"STALE {name}")
covered = [c for c in crates if c not in uncovered]

offences, sites = [], set()
for crate in covered:
    # Every declaration of a name: two types of one name in two modules
    # are both read under it.
    declared, refusal = {}, set()
    for path in (p for p in files if p.split("/")[1] == crate):
        with open(path, encoding="utf-8") as fh:
            raw = fh.read()
        code = without_test_mods(strip(raw, False))
        for kind, name, params, fields, line in declarations(code):
            declared.setdefault(name, []).append((path, line, params, fields))
        for m in IMPL.finditer(code):
            refusal.add(m.group(2))
        # Rule 2, over the same file with its strings kept.
        text = without_test_mods(strip(raw, True))
        lines = raw.split("\n")
        skip = [(m.end() - 1, matching(text, m.end() - 1)) for m in DEBUG.finditer(text)]
        for pat in (DISPLAY, WRITER):
            for m in pat.finditer(text):
                start, end = m.end() - 1, matching(text, m.end() - 1)
                if any(a <= start <= b for a, b in skip):
                    continue
                for s in FORMATS.finditer(text, start, end):
                    if scalar_format(s.group(1)):
                        line = lines[text.count("\n", 0, s.start())]
                        sites.add(f"{path} :: {' '.join(line.split())}")
    # The payload closure: every declared type a refusal type names.
    reach, frontier = set(), [n for n in refusal if n in declared]
    while frontier:
        name = frontier.pop()
        if name in reach:
            continue
        reach.add(name)
        for _, _, _, fields in declared[name]:
            for _, ty in fields:
                frontier += [w for w in re.findall(r"\b[A-Z]\w*\b", ty) if w in declared and w not in reach]
    for name in sorted(reach):
        for path, line, params, fields in declared[name]:
            for field, ty in fields:
                # A type's own parameter, held bare, is a scalar wherever
                # the type is held at one (`Wrap<S> { inner: S }` as
                # `Wrap<f64>`).
                held = bare_scalars(ty, declared, SCALARS | set(params))
                if held:
                    offences.append(f"{path}:{line}: {name} declares `{field}: {' '.join(ty.split())}`")

pins = set(rows(pins_path))
for o in offences:
    print("FIELD " + o)
for site in sorted(sites - pins):
    print("NEW " + site)
for site in sorted(pins - sites):
    print("GONE " + site)
print("COVERED " + " ".join(covered))
print(f"UNCOVERED {len(uncovered)}")
print(f"PINNED {len(pins)}")
PY
  ); then
    gate_error "$(gate_name): the reader failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if grep -q '^STALE ' <<<"$report"; then
    grep '^STALE ' <<<"$report"
    gate_error "$UNCOVERED lists a crate the tree does not hold: delete its line, so the list says which crates the sweep has left"
    exit 1
  fi
  if grep -q '^FIELD ' <<<"$report"; then
    grep '^FIELD ' <<<"$report"
    gate_error "a refusal type declares a bare scalar field: a number a sentence quotes is a \`geom_core::Told\`, built where the payload is built with the role it is told in (given, computed, offered); see crates/geom-core/README.md, \"A number told to a person\""
    exit 1
  fi
  if grep -qE '^(NEW|GONE) ' <<<"$report"; then
    grep -E '^(NEW|GONE) ' <<<"$report"
    gate_error "the Debug, exponent and precision formats in sentence-writing bodies moved — a scalar is told through \`geom_core::Told\`, never through \`{:?}\`, \`{:e}\` or \`{:.N}\`; a NEW line that prints a name and no scalar is added to $PINS as \`path :: the line\` beside its reason, and a GONE line is deleted from it"
    exit 1
  fi
  gate_ok "no refusal type declares a bare scalar field and no sentence formats one past its pinned lines in: $(sed -n 's/^COVERED //p' <<<"$report") ($(sed -n 's/^UNCOVERED //p' <<<"$report") crate(s) not yet covered, $(sed -n 's/^PINNED //p' <<<"$report") pinned line(s))"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# crate\n' > "$1/$UNCOVERED"
  printf '# path :: line\n' > "$1/$PINS"
}

plant() { cat >> "$1/crates/clean/src/lib.rs"; }
# A Display that prints nothing of its own, for a plant about a field.
displayed() {
  printf 'impl%s core::fmt::Display for %s {\n    fn fmt(&self, f: &mut F) -> R { f.write_str("refused") }\n}\n' "$2" "$3" \
    >> "$1/crates/clean/src/lib.rs"
}

plant_bare_field() {
  plant "$1" <<'RS'
pub enum Refusal<T> { TooShort { length: T } }
RS
  displayed "$1" "<T>" "Refusal<T>"
}
plant_bare_f64_in_an_option() {
  plant "$1" <<'RS'
pub struct Refusal { bound: Option<f64> }
impl std::error::Error for Refusal {}
RS
}
plant_bare_field_in_a_part() {
  plant "$1" <<'RS'
pub struct Leg<T> { pub setback: [T; 2] }
pub enum Refusal<T> { Overrun(Leg<T>) }
RS
  displayed "$1" "<T>" "Refusal<T>"
}
plant_tuple_field() {
  plant "$1" <<'RS'
pub enum Refusal { Miss(f64, usize) }
RS
  displayed "$1" "" "Refusal"
}
# A field after a function pointer's arrow, after a shift in an array
# length, and in a variant after one.
plant_field_after_an_arrow() {
  plant "$1" <<'RS'
pub struct Refusal { told: fn(&u8) -> Told, margin: f64 }
RS
  displayed "$1" "" "Refusal"
}
plant_field_after_a_shift() {
  plant "$1" <<'RS'
pub struct Refusal { a: [u8; 1 << 3], margin: f64 }
RS
  displayed "$1" "" "Refusal"
}
plant_variant_after_an_arrow() {
  plant "$1" <<'RS'
pub enum Refusal { A { cb: fn() -> u8 }, B { margin: f64 } }
RS
  displayed "$1" "" "Refusal"
}
# plant_container TYPE ROOT: a scalar under a container the reader does not
# know by name, or path-qualified, or a scalar type other than f64.
plant_container() {
  printf 'pub struct Refusal<T> { held: %s, kept: core::marker::PhantomData<T> }\n' "$1" \
    >> "$2/crates/clean/src/lib.rs"
  displayed "$2" "<T>" "Refusal<T>"
}
plant_two_types_of_one_name() {
  plant "$1" <<'RS'
pub mod a { pub struct Leg { pub setback: f64 } }
pub mod b { pub struct Leg { pub count: usize } }
pub struct Refusal { leg: a::Leg }
RS
  displayed "$1" "" "Refusal"
}
plant_display_for_a_reference() {
  plant "$1" <<'RS'
pub struct Refusal { margin: f64 }
impl core::fmt::Display for &Refusal {
    fn fmt(&self, f: &mut F) -> R { f.write_str("refused") }
}
RS
}
# plant_where CLAUSE BODY ROOT: a declaration whose `where` clause holds
# brackets of its own.
plant_where() {
  printf 'pub %s\n' "$1" >> "$2/crates/clean/src/lib.rs"
  displayed "$2" "<U>" "Refusal<U>"
}
plant_display_for_a_box() {
  plant "$1" <<'RS'
pub struct Refusal { margin: f64 }
impl core::fmt::Display for Box<Refusal> {
    fn fmt(&self, f: &mut F) -> R { f.write_str("refused") }
}
RS
}
# A type of the crate whose parameter is not called `T`, held bare.
plant_own_parameter_by_another_name() {
  plant "$1" <<'RS'
pub struct Wrap<S> { inner: S }
pub struct Refusal { held: Wrap<f64> }
RS
  displayed "$1" "" "Refusal"
}
plant_generic_bound_with_parentheses() {
  plant "$1" <<'RS'
pub struct Refusal<F: Fn(f64) -> f64> { margin: f64, spell: F }
RS
  displayed "$1" "<F: Fn(f64) -> f64>" "Refusal<F>"
}
# plant_format SPEC ROOT: a Display that formats a told field with SPEC.
plant_format() {
  printf 'pub struct Refusal { at: Told }\nimpl core::fmt::Display for Refusal {\n    fn fmt(&self, f: &mut F) -> R { write!(f, "at {%s}", self.at) }\n}\n' "$1" \
    >> "$2/crates/clean/src/lib.rs"
}
plant_exp_format_in_a_writer() {
  plant "$1" <<'RS'
fn write_bound(f: &mut core::fmt::Formatter<'_>, bound: &Told) -> R {
    write!(f, "below {bound:e}")
}
RS
}
plant_pin_gone() {
  printf 'crates/clean/src/lib.rs :: write!(f, "at {:?}", self.verb)\n' >> "$1/$PINS"
}
# A pinned line rewritten to print something else: the pin no longer
# matches, and the new line is not pinned.
plant_pinned_line_rewritten() {
  printf 'crates/clean/src/lib.rs :: fn fmt(&self, f: &mut F) -> R { write!(f, "the {:?} verb", self.verb) }\n' >> "$1/$PINS"
  plant "$1" <<'RS'
pub struct Refusal { verb: Verb, margin: Told }
impl core::fmt::Display for Refusal {
    fn fmt(&self, f: &mut F) -> R { write!(f, "the {:?} verb", self.margin) }
}
RS
}
plant_stale_uncovered() {
  printf 'gone\n' >> "$1/$UNCOVERED"
}
# A told payload, a position another crate declares, a scalar inside a
# type of the crate with no scalar field, a function pointer over
# scalars' neighbours, a scalar field of a type no sentence reaches, a
# Debug impl, a width on a name, a pinned Debug name, a test module, and
# a crate the list leaves for a later sweep.
plant_not_offences() {
  plant "$1" <<'RS'
pub struct Leg { pub setback: Told, pub at: Point2<f64> }
pub struct Tagged<T> { tag: usize, kept: core::marker::PhantomData<T> }
pub enum Refusal<T> {
    Overrun { leg: Leg, corner: Option<Point2<T>>, count: usize, tagged: Vec<Tagged<T>> },
    Shifted { bits: [u8; 1 << 3], spell: fn(&u8) -> Told },
}
impl<T> core::fmt::Display for Refusal<T> {
    fn fmt(&self, f: &mut F) -> R { write!(f, "overrun by {} at {:?} in {:>8}, as {{:?}} would write", self.leg.setback, Verb::Line, "here") }
}
pub struct Carrier<T> { pub radius: T }
impl<T> core::fmt::Debug for Carrier<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> R { write!(f, "{:?}", 1.0) }
}
#[cfg(test)]
mod tests {
    pub struct Probe { x: f64 }
    impl core::fmt::Display for Probe {
        fn fmt(&self, f: &mut F) -> R { write!(f, "{:e}", self.x) }
    }
}
RS
  printf 'crates/clean/src/lib.rs :: fn fmt(&self, f: &mut F) -> R { write!(f, "overrun by {} at {:?} in {:>8}, as {{:?}} would write", self.leg.setback, Verb::Line, "here") }\n' >> "$1/$PINS"
  mkdir -p "$1/crates/later/src"
  printf 'pub struct Refusal { margin: f64 }\nimpl core::fmt::Display for Refusal {\n    fn fmt(&self, f: &mut F) -> R { write!(f, "{:e}", self.margin) }\n}\n' \
    > "$1/crates/later/src/lib.rs"
  printf 'later\n' >> "$1/$UNCOVERED"
}

gate_selftest() {
  gate_selftest_clean
  local field="declares a bare scalar field" format="formats in sentence-writing bodies moved"
  gate_selftest_case "$field" plant_bare_field
  gate_selftest_case "$field" plant_bare_f64_in_an_option
  gate_selftest_case "$field" plant_bare_field_in_a_part
  gate_selftest_case "$field" plant_tuple_field
  gate_selftest_case "$field" plant_field_after_an_arrow
  gate_selftest_case "$field" plant_field_after_a_shift
  gate_selftest_case "$field" plant_variant_after_an_arrow
  local ty
  for ty in 'std::vec::Vec<f64>' 'core::option::Option<f64>' 'Arc<f64>' 'Rc<T>' \
      'HashMap<u8, f64>' 'Result<f64, u8>' "Cow<'static, [f64]>" 'core::primitive::f64' \
      'Interval' 'Dual<f64>' 'Sym<T>' 'f32'; do
    gate_selftest_case "$field" plant_container "$ty"
  done
  gate_selftest_case "$field" plant_two_types_of_one_name
  gate_selftest_case "$field" plant_display_for_a_reference
  gate_selftest_case "$field" plant_generic_bound_with_parentheses
  local decl
  for decl in 'struct Refusal<U> where U: Fn(u8) -> u8 { count: usize, margin: f64 }' \
      'enum Refusal<U> where U: Fn(u8) { A { margin: f64 }, B(usize) }' \
      'struct Refusal<U> where (U, u8): Clone { margin: f64 }' \
      'struct Refusal<U>(f64, usize) where U: Fn(u8);'; do
    gate_selftest_case "$field" plant_where "$decl"
  done
  gate_selftest_case "$field" plant_display_for_a_box
  gate_selftest_case "$field" plant_own_parameter_by_another_name
  local spec
  for spec in ':?' ':#?' ':.3e' ':.4e' ':+e' ':10?' ':.3' 'x:.17?'; do
    gate_selftest_case "$format" plant_format "$spec"
  done
  gate_selftest_case "$format" plant_exp_format_in_a_writer
  gate_selftest_case "$format" plant_pin_gone
  gate_selftest_case "$format" plant_pinned_line_rewritten
  gate_selftest_case "lists a crate the tree does not hold" plant_stale_uncovered
  gate_selftest_passes "told payloads, foreign positions, the crate's own generics, arrows and shifts, Debug impls, a width, a pinned line and an uncovered crate" plant_not_offences
  printf '%s selftest OK: passes a clean fixture, a told payload, a position another crate declares, a scalar parameter of one of the crate own types, a field after an arrow or a shift, a scalar no sentence reaches, a Debug impl, a test module, a width on a name, an escaped brace, a pinned line and a crate the list leaves uncovered; fires on a bare T or f64 field, in an Option, in an array of a payload part, in a tuple variant, after an arrow or a shift, under twelve containers and scalar names, in one of two types of one name, under a Display for a reference or a Box, behind a parenthesised bound or where clause, and through one of the crate own types whose parameter has another name; on eight format specs in a Display and an exponent in a Formatter function, on a pin no line matches, on a pinned line rewritten, and on a listed crate the tree does not hold\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
