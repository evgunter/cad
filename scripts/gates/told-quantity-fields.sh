#!/usr/bin/env bash
# told-quantity-fields.sh — a refusal's payload holds told quantities,
# and its sentence formats no scalar itself. ONE home; ci.yml's `lint`
# job runs every gate in this directory.
#
# THE RULE (`crates/geom-core/README.md`, "A number told to a person").
# A number a sentence quotes is a `geom_core::Told`, built where the
# payload is built. So, in a converted crate:
#
#  1. NO REFUSAL TYPE DECLARES A BARE `f64` OR `T` FIELD. A refusal type
#     is a `struct` or `enum` the crate declares and gives a `Display`
#     or an `Error` impl, and every type of the crate one of those names
#     in a field, transitively (a payload's parts are payload). A field
#     is bare when its type is `f64` or `T`, alone or as a member of an
#     `Option`, `Vec`, `Box`, array, slice, reference or tuple.
#  2. NO SENTENCE-WRITING BODY FORMATS WITH `{:?}` OR `{:e}`. The bodies
#     read are every `impl … Display for …` and every function taking a
#     `Formatter`, an `impl … Debug for …` body excepted. The Debug
#     names the crate's sentences still print are pinned per file in
#     `told-quantity-fields.pins`, each beside its reason, so a new one
#     is added knowingly.
#
# THE CRATES IT COVERS are every `crates/*` directory that
# `told-quantity-fields.uncovered` does not list. That file is the
# sweep's remaining work: converting a crate deletes its line, and a
# crate the file neither lists nor the tree holds is refused, so the
# list cannot go stale in either direction.
#
# Production code only (every `crates/<crate>/src` file but the
# test-only mounts, and in it no inline `#[cfg(test)]` module; comments
# stripped, string literals kept for rule 2 and stripped for rule 1).
#
# WHAT STAYS BLIND.
#  - A scalar inside a type another crate declares: `Point2<T>`,
#    `Vec2<T>`, an `Interval`. `profile`'s refusals hold four positions
#    as `Point2<T>`, told through `Told` when rendered and read as
#    numbers by the viewer and the Python door
#    (`work/scalar/a-refusals-corner-point-is-read-as-a-number-by-the-viewer-and-python.md`).
#  - A scalar under any other name: an `f32`, a type alias of `f64`, a
#    second type parameter. The match is on the tokens `f64` and `T`.
#  - A refusal type with neither a `Display` nor an `Error` impl that no
#    such type names in a field: a sentence written by a free function
#    over a plain struct is not found by rule 1.
#  - A scalar formatted with a bare `{}`. The formatter does not say
#    what type it prints, so a `{}` over an `f64` local in a body reads
#    as any other `{}`; rule 1 is what keeps an `f64` from being there.
#  - A `Formatter` reached through a closure (`fmt::from_fn(|f| …)`)
#    outside any body rule 2 reads.
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


def listed(path, columns):
    rows = []
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            line = line.split("#", 1)[0].split()
            if line:
                if len(line) != columns:
                    sys.exit(f"{path}: a row of {len(line)} columns, not {columns}: {line}")
                rows.append(line)
    return rows


def strip(text, keep_strings):
    # Comments out; string and char literals out unless kept. The code
    # and its newlines stay.
    out, i, n = [], 0, len(text)
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
            out.append(text[i:j].replace("{", "\x01").replace("}", "\x02") if keep_strings else '""' + "\n" * text.count("\n", i, j))
            i = j
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            j += 1
            out.append(text[i:j].replace("{", "\x01").replace("}", "\x02") if keep_strings else '""' + "\n" * text.count("\n", i, j))
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


def matching(text, at, open_="{", close="}"):
    depth = 0
    for i in range(at, len(text)):
        if text[i] == open_:
            depth += 1
        elif text[i] == close:
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
        if c in "<([{":
            depth += 1
        elif c in ">)]}":
            depth -= 1
        elif c == sep and depth == 0:
            parts.append(text[at:i])
            at = i + 1
    parts.append(text[at:])
    return [p.strip() for p in parts if p.strip()]


WRAPPER = re.compile(r"^(?:Option|Vec|Box)\s*<(.*)>$", re.S)


def bare(ty):
    # Whether `ty` is `f64` or `T`, alone or inside the transparent
    # wrappers.
    ty = re.sub(r"^&\s*(?:'\w+\s+)?(?:mut\s+)?", "", ty.strip())
    if ty in ("f64", "T"):
        return True
    m = WRAPPER.match(ty)
    if m:
        return any(bare(p) for p in split_top(m.group(1)))
    if ty.startswith("[") and ty.endswith("]"):
        return bare(split_top(ty[1:-1], ";")[0])
    if ty.startswith("(") and ty.endswith(")"):
        return any(bare(p) for p in split_top(ty[1:-1]))
    return False


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


DECL = re.compile(r"\b(struct|enum)\s+(\w+)\s*(<[^{;(]*?>)?\s*(?:where[^{;(]*)?([{(;])")
IMPL = re.compile(r"\bimpl\b[^{;]*?\b(Display|Error)\s+for\s+(?:[\w:]*::)?(\w+)[^{;]*\{")
DISPLAY = re.compile(r"\bimpl\b[^{;]*?\bDisplay\s+for\s+[^{;]*\{")
DEBUG = re.compile(r"\bimpl\b[^{;]*?\bDebug\s+for\s+[^{;]*\{")
WRITER = re.compile(r"\bfn\s+\w+\s*(?:<[^>]*>)?\s*\([^)]*\bFormatter\b[^)]*\)[^{;]*\{")
FORMATS = re.compile("\x01[^\x01\x02]*:#?(?:\\?|e|E)\x02")

uncovered = {row[0] for row in listed(uncovered_path, 1)}
crates = sorted({p.split("/")[1] for p in files})
for name in sorted(uncovered - set(crates)):
    if not os.path.isdir(os.path.join("crates", name)):
        print(f"STALE {name}")
covered = [c for c in crates if c not in uncovered]

offences, formats = [], {}
for crate in covered:
    declared, refusal = {}, set()
    for path in (p for p in files if p.split("/")[1] == crate):
        with open(path, encoding="utf-8") as fh:
            raw = fh.read()
        code = without_test_mods(strip(raw, False))
        for m in DECL.finditer(code):
            kind, name, opener = m.group(1), m.group(2), m.group(4)
            fields = []
            if opener == "{":
                body = code[m.end():matching(code, m.end() - 1)]
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
                body = code[m.end():matching(code, m.end() - 1, "(", ")")]
                fields = field_types(body, True)
            line = code.count("\n", 0, m.start()) + 1
            declared[name] = (path, line, fields)
        for m in IMPL.finditer(code):
            refusal.add(m.group(2))
        # Rule 2, over the same file with its strings kept.
        text = without_test_mods(strip(raw, True))
        skip = [(m.end() - 1, matching(text, m.end() - 1)) for m in DEBUG.finditer(text)]
        seen = set()
        for pat in (DISPLAY, WRITER):
            for m in pat.finditer(text):
                start, end = m.end() - 1, matching(text, m.end() - 1)
                if any(a <= start <= b for a, b in skip):
                    continue
                for s in FORMATS.finditer(text, start, end):
                    seen.add(s.start())
        if seen:
            formats[path] = len(seen)
    # The payload closure: every declared type a refusal type names.
    reach, frontier = set(), [n for n in refusal if n in declared]
    while frontier:
        name = frontier.pop()
        if name in reach:
            continue
        reach.add(name)
        for _, ty in declared[name][2]:
            frontier += [w for w in re.findall(r"\b[A-Z]\w*\b", ty) if w in declared and w not in reach]
    for name in sorted(reach):
        path, line, fields = declared[name]
        for field, ty in fields:
            if bare(ty):
                offences.append(f"{path}:{line}: {name} declares `{field}: {ty}`")

pins = {row[0]: int(row[1]) for row in listed(pins_path, 2)}
for o in offences:
    print("FIELD " + o)
for path in sorted(set(formats) | set(pins)):
    now, was = formats.get(path, 0), pins.get(path, 0)
    if now > was:
        print(f"ROSE {path}: {was} -> {now}")
    elif now < was:
        print(f"FELL {path}: {was} -> {now}")
print("COVERED " + " ".join(covered))
print(f"UNCOVERED {len(uncovered)}")
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
  if grep -qE '^(ROSE|FELL) ' <<<"$report"; then
    grep -E '^(ROSE|FELL) ' <<<"$report"
    gate_error "the \`{:?}\` and \`{:e}\` formats in sentence-writing bodies moved — a scalar is told through \`geom_core::Told\`, never through Debug or LowerExp; set $PINS to the new per-file counts in the change that moves them, each beside what it prints"
    exit 1
  fi
  gate_ok "no refusal type declares a bare f64 or T field and no sentence formats past its pins in: $(sed -n 's/^COVERED //p' <<<"$report") ($(sed -n 's/^UNCOVERED //p' <<<"$report") crate(s) not yet covered)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# crate\n' > "$1/$UNCOVERED"
  printf '# path count\n' > "$1/$PINS"
}

plant() { cat >> "$1/crates/clean/src/lib.rs"; }

plant_bare_field() {
  plant "$1" <<'RS'
pub enum Refusal<T> { TooShort { length: T } }
impl<T> core::fmt::Display for Refusal<T> {
    fn fmt(&self, f: &mut F) -> R { f.write_str("too short") }
}
RS
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
impl<T> core::fmt::Display for Refusal<T> {
    fn fmt(&self, f: &mut F) -> R { f.write_str("overrun") }
}
RS
}
plant_tuple_field() {
  plant "$1" <<'RS'
pub enum Refusal { Miss(f64, usize) }
impl core::fmt::Display for Refusal {
    fn fmt(&self, f: &mut F) -> R { f.write_str("miss") }
}
RS
}
plant_debug_format() {
  plant "$1" <<'RS'
pub struct Refusal { at: Told }
impl core::fmt::Display for Refusal {
    fn fmt(&self, f: &mut F) -> R { write!(f, "at {:?}", self.at) }
}
RS
}
plant_exp_format_in_a_writer() {
  plant "$1" <<'RS'
fn write_bound(f: &mut core::fmt::Formatter<'_>, bound: &Told) -> R {
    write!(f, "below {bound:e}")
}
RS
}
plant_pin_gone() {
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$PINS"
}
plant_stale_uncovered() {
  printf 'gone\n' >> "$1/$UNCOVERED"
}
# A told payload, a position another crate declares, a scalar field of a
# type no sentence reaches, a Debug impl, a pinned Debug name, and a
# crate the list leaves for a later sweep.
plant_not_offences() {
  plant "$1" <<'RS'
pub struct Leg { pub setback: Told, pub at: Point2<f64> }
pub enum Refusal<T> { Overrun { leg: Leg, corner: Point2<T>, count: usize } }
impl<T> core::fmt::Display for Refusal<T> {
    fn fmt(&self, f: &mut F) -> R { write!(f, "overrun by {} at {:?}", self.leg.setback, Verb::Line) }
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
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$PINS"
  mkdir -p "$1/crates/later/src"
  printf 'pub struct Refusal { margin: f64 }\nimpl core::fmt::Display for Refusal {\n    fn fmt(&self, f: &mut F) -> R { write!(f, "{:e}", self.margin) }\n}\n' \
    > "$1/crates/later/src/lib.rs"
  printf 'later\n' >> "$1/$UNCOVERED"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "declares a bare scalar field" plant_bare_field
  gate_selftest_case "declares a bare scalar field" plant_bare_f64_in_an_option
  gate_selftest_case "declares a bare scalar field" plant_bare_field_in_a_part
  gate_selftest_case "declares a bare scalar field" plant_tuple_field
  gate_selftest_case "formats in sentence-writing bodies moved" plant_debug_format
  gate_selftest_case "formats in sentence-writing bodies moved" plant_exp_format_in_a_writer
  gate_selftest_case "formats in sentence-writing bodies moved" plant_pin_gone
  gate_selftest_case "lists a crate the tree does not hold" plant_stale_uncovered
  gate_selftest_passes "told payloads, foreign positions, Debug impls, a pinned name and an uncovered crate" plant_not_offences
  printf '%s selftest OK: passes a clean fixture, a told payload, a position another crate declares, a scalar no sentence reaches, a Debug impl, a test module, a format at its pin and a crate the list leaves uncovered; fires on a bare T or f64 field, in an Option, in an array of a payload part and in a tuple variant, on a Debug format in a Display and a LowerExp one in a Formatter function, on a pin with no format behind it and on a listed crate the tree does not hold\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
