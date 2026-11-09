#!/usr/bin/env bash
# span-end-gap-sites.sh — every parameter gap from a span's END metered
# into metres by hand (`Margin::metered(t − t0, rate)` and its `_sup`
# twin) in production code has a row in a site table, keyed by its
# function, with the direction argument that makes it sound. A row is
# meant only to be lowered; that is held in review (below). ONE home;
# ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE (`work/onecopy/a-parameter-gap-metered-by-hand-at-a-span-end-has-no-gate.md`).
# Where a parameter sits against a span's ends is
# `geom_brep::curve_pair::Span`'s reading (`place`, `place_param`,
# `place_param_read`): an end by the raw chord to its point, the side by
# the parameter at the span's SUP speed. A gap from an end metered at an
# INF speed under-reads arc length: its Positive ("clear of the end") is
# sound, its Zero is not, since it spans up to `(sup/inf)·z` of arc. That
# Zero read as "at the end" put a crossing 5e-7 m from a thin ellipse's
# end at the end (PR 6687's P0). Text cannot tell a site that reads the
# Zero as an end from one that only accepts the Positive, so each site
# states its direction on its row, and a new one arrives at review.
#
# EXEMPT, not in the table: the home, `crates/geom-brep/src/curve_pair/span.rs`.
#
# THE CLASSES. Each row names one; the class is the site's direction
# argument.
#   sup           metered at a SUP speed: a Zero is within the band in
#                 arc, so in chord, and reading it as the end is sound
#   exact-rate    the rate is exact (a line's length), so inf and sup are
#                 one reading
#   inside-only   an INF meter whose Positive alone is accepted: a Zero
#                 or a Negative refuses or escalates, never "at the end"
#   witness       `pair_section::within_span`'s witness reading, metered
#                 at the pointwise speed (filed on the row above: a bound
#                 over `[t, hi]` is owed)
#
# WHAT IS READ: `Margin::metered(`, `Margin::metered_sup(` and
# `Margin::metered_inf(` whose first argument is `a − b` (parenthesised
# or not) with EXACTLY ONE operand a span end's name: `t0`, `t1`, `lo`,
# `hi`, `t_lo`, `t_hi`, `start`, `end`, `span.0`, `span.1`, `u0`, `u1`,
# `v0`, `v1`, `w0`, `w1` (a `self.` before any). Both operands ends is an
# extent, the turn rule's or certification's question, not a place. In
# every `crates/*/src/**/*.rs`, comments and literals out,
# `#[cfg(test)]` modules out, test-only files out as `atan2-sites.sh`
# reads them. BLIND SPOTS: a gap spelled `Margin::of((t − t0) * speed)`
# or through a `let` first (`let gap = t − t0; Margin::metered(gap, …)`),
# an end under another name, and a meter applied inside a helper. The
# table's diff, and the review of a new end read, are where those are
# held.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

TABLE=scripts/gates/span-end-gap-sites.tsv

gate() {
  gate_require_file "$TABLE"
  gate_require_crate_sources
  local report
  if ! report=$(python3 - "$TABLE" <<'PY'
import glob, os, re, sys

table_path = sys.argv[1]
CALL = re.compile(r"Margin::metered(?:_sup|_inf)?\(\s*\(?\s*([\w.]+)\s*-\s*([\w.]+)\s*\)?\s*,")
END = re.compile(r"^(?:self\.)?(?:t0|t1|lo|hi|t_lo|t_hi|start|end|span\.0|span\.1|u0|u1|v0|v1|w0|w1)$")
EXEMPT = {"crates/geom-brep/src/curve_pair/span.rs"}
CLASSES = {"sup", "exact-rate", "inside-only", "witness"}
TEST_NAME = re.compile(r"(?:_rows|_fuzz)\.rs$|^test_support")


def strip(text):
    out, i, n = [], 0, len(text)
    while i < n:
        c, two = text[i], text[i:i + 2]
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
            j = text.find('"' + "#" * hashes, i + 2 + hashes)
            j = n if j < 0 else j + 1 + hashes
            out.append('""' + "\n" * text.count("\n", i, j))
            i = j
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append('""' + "\n" * text.count("\n", i, j))
            i = j + 1
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


def matching(text, at):
    depth = 0
    for i in range(at, len(text)):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return i
    return len(text) - 1


CFG_TEST = r"#\[cfg\(test\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*"


def without_test_mods(text):
    out, at = [], 0
    for m in re.finditer(CFG_TEST + r"\{", text):
        if m.start() < at:
            continue
        out.append(text[at:m.start()])
        at = matching(text, m.end() - 1) + 1
    out.append(text[at:])
    return "".join(out)


files = sorted(glob.glob("crates/*/src/**/*.rs", recursive=True))
code = {p: strip(open(p, encoding="utf-8").read()) for p in files}
test_only = set()
test_dirs = set()
for path, text in code.items():
    d, stem = os.path.dirname(path), os.path.basename(path)[:-3]
    base = d if stem in ("lib", "mod", "main") else os.path.join(d, stem)
    for m in re.finditer(CFG_TEST + ";", text):
        test_only.add(os.path.join(base, m.group(1) + ".rs"))
        test_dirs.add(os.path.join(base, m.group(1)) + os.sep)

FN = re.compile(r"\bfn\s+(\w+)")
counts = {}
for path in files:
    if (path in EXEMPT or path in test_only or TEST_NAME.search(os.path.basename(path))
            or any(path.startswith(d) for d in test_dirs)):
        continue
    text = without_test_mods(code[path])
    for m in CALL.finditer(text):
        if bool(END.match(m.group(1))) == bool(END.match(m.group(2))):
            continue
        fns = list(FN.finditer(text, 0, m.start()))
        key = (path, fns[-1].group(1) if fns else "-")
        counts[key] = counts.get(key, 0) + 1

table, bad = {}, []
with open(table_path, encoding="utf-8") as fh:
    for number, line in enumerate(fh, 1):
        line = line.split("#", 1)[0].strip()
        if not line:
            continue
        fields = line.split()
        if len(fields) != 4 or not fields[2].isdigit():
            bad.append(f"{table_path}:{number}: a row is `path function count class`")
            continue
        path, fn, n, cls = fields
        if cls not in CLASSES:
            bad.append(f"{table_path}:{number}: {path}: `{cls}` is not a class (the header lists them)")
        if (path, fn) in table:
            bad.append(f"{table_path}:{number}: {path} {fn}: one row per (file, function)")
        table[(path, fn)] = int(n)

for b in bad:
    print("BAD " + b)
for key in sorted(set(counts) | set(table)):
    now, was = counts.get(key, 0), table.get(key, 0)
    if now > was:
        print(f"ROSE {key[0]} {key[1]}: {was} -> {now}")
    elif now < was:
        print(f"FELL {key[0]} {key[1]}: {was} -> {now}")
print(f"TOTAL {sum(counts.values())}")
PY
  ); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if grep -q '^BAD ' <<<"$report"; then
    sed -n 's/^BAD //p' <<<"$report"
    gate_error "$TABLE has a row this gate refuses (above)"
    exit 1
  fi
  if grep -q '^ROSE ' <<<"$report"; then
    grep '^ROSE ' <<<"$report"
    gate_error "a parameter gap from a span's end metered by hand arrived — read the parameter's place through geom_brep::curve_pair::Span (place, place_param, place_param_read), or, where only a Positive is accepted, add its row with its direction class and argue it in review"
    exit 1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "hand-metered end gaps went away — lower (or delete) their rows in $TABLE, so the ratchet holds them there"
    exit 1
  fi
  gate_ok "hand-metered parameter gaps from a span's end at their table ($(sed -n 's/^TOTAL //p' <<<"$report") in all)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# path function count class\n' > "$1/$TABLE"
}

plant_new_site() {
  printf 'pub fn gap(t: f64, t0: f64) -> f64 { Margin::metered(t - t0, s) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_fell() {
  printf 'crates/clean/src/lib.rs gap 1 inside-only\n' >> "$1/$TABLE"
}

plant_bad_class() {
  printf 'crates/clean/src/lib.rs gap 1 at-end\n' >> "$1/$TABLE"
  plant_new_site "$1"
}

plant_at_table() {
  printf 'crates/clean/src/lib.rs gap 1 inside-only\n' >> "$1/$TABLE"
  plant_new_site "$1"
}

# An extent (both operands ends), a gap between two free parameters, a
# comment, a test module and the home are not sites.
plant_not_sites() {
  printf 'pub fn len(t0: f64, t1: f64) -> f64 { Margin::metered(t1 - t0, s) }\npub fn ord(a: f64, b: f64) -> f64 { Margin::metered(b - a, s) }\n// Margin::metered(t - t0, s)\n#[cfg(test)]\nmod tests {\n    fn f(t: f64, t0: f64) -> f64 { Margin::metered(t - t0, s) }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
  mkdir -p "$1/crates/geom-brep/src/curve_pair"
  printf 'fn f(t: f64, t0: f64) -> f64 { Margin::metered(t - t0, s) }\n' > "$1/crates/geom-brep/src/curve_pair/span.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "metered by hand arrived" plant_new_site
  gate_selftest_case "went away" plant_fell
  gate_selftest_case "is not a class" plant_bad_class
  gate_selftest_passes "a site at its row" plant_at_table
  gate_selftest_passes "an extent, a free gap, a comment, a test module and the home" plant_not_sites
  printf '%s selftest OK: passes a clean fixture, a site at its row, and extents, free gaps, comments, test modules and the home; fires on a new site, an unrecorded fall and an unknown class\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
