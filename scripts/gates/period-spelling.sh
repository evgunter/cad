#!/usr/bin/env bash
# period-spelling.sh — every spelling of a whole turn in production code
# outside its homes has a row in a site table, keyed by its function, with
# its class. A row is meant only to be lowered; that is held in review.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE (`work/onecopy/a-turn-is-still-spelled-outside-its-carriers-period.md`).
# A turn is read from where it is stated:
# - a carrier's period (`geom::Curve3::period`, `Circle::period`,
#   `Ellipse::period`), or a chart's (`geom::Surface::angle_periods`);
# - a conic kind's own statement where only a `Conic` is held
#   (`geom::conic_period`);
# - `geom_core::period::Period::angle()` for an angle by construction.
# The spellings this gate counts are `from_f64(TAU)` (any `consts::`
# path), `2.0 * PI` and a bare `TAU`. `T::tau()` is the scalar's own τ,
# which `Period::angle` reads, and is not counted. A new spelling raises its
# function above its row and fails. One that went away lowers it and
# fails until the row is lowered, so the slack cannot let one back in
# unseen. A hand census missed the sites PR 6788's review found; this
# gate is what finds the next one.
#
# EXEMPT, not in the table: the period home `crates/geom-core/src/period.rs`,
# and the scalar implementations of τ (`crates/geom-core/src/{real,dual,
# interval,sym}.rs`).
#
# THE CLASSES. Each row names one:
#   angle     an angle by construction (a rotation, a sweep's or a view's
#             angle, phases round a full turn) at `f64`, where
#             `Period::angle()` would read the same τ
#   constant  a 2π that is no period (a normal density's `√(2π)`)
#   home      an enclosure or bracket of τ itself, the number, not a turn
#   fixture   a test oracle or fixture compiled for test support
#   owed      a site that should read its carrier's or chart's period,
#             filed (`work/onecopy/a-turn-read-off-a-value-or-a-literal-remains-at-f64.md`)

set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

TABLE=scripts/gates/period-spelling.tsv

gate() {
  gate_require_file "$TABLE"
  gate_require_crate_sources
  local report
  if ! report=$(python3 - "$TABLE" <<'PY'
import glob, os, re, sys

table_path = sys.argv[1]
CALL = re.compile(r"from_f64\(\s*(?:(?:core|std)::f64::consts::|f64::consts::)?TAU\s*\)|\b2\.0\s*\*\s*(?:(?:core|std)::f64::consts::)?PI\b|(?<![\w:])TAU\b|(?<=consts::)TAU\b")
EXEMPT = {
    "crates/geom-core/src/period.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/dual.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/sym.rs",
}
CLASSES = {"angle", "constant", "home", "fixture", "owed"}
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
        start = text.rfind("\n", 0, m.start()) + 1
        if text[start:m.start()].lstrip().startswith(("use ", "pub use ")):
            continue
        # The site's function: the last `fn` declared before it.
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
    gate_error "a whole turn is spelled in production code outside its homes — read it from where it is stated (the carrier's period: Curve3::period, Circle::period, Ellipse::period; a chart's: Surface::angle_periods; a conic's: geom::conic_period; an angle by construction: geom_core::period::Period::angle()), not a new table row: a row is meant only to be lowered"
    exit 1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "spellings of a turn went away — lower (or delete) their rows in $TABLE, so the ratchet holds them there"
    exit 1
  fi
  gate_ok "production spellings of a turn outside its homes at their table ($(sed -n 's/^TOTAL //p' <<<"$report") in all)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# path function count class\n' > "$1/$TABLE"
}

plant_new_site() {
  printf 'pub fn turn() -> f64 { 2.0 * core::f64::consts::PI }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_from_f64() {
  printf 'pub fn turn<T: Real>() -> T { T::from_f64(core::f64::consts::TAU) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_bare() {
  printf 'use core::f64::consts::TAU;\npub fn turn() -> f64 { TAU }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_fell() {
  printf 'crates/clean/src/lib.rs turn 1 angle\n' >> "$1/$TABLE"
}

plant_bad_class() {
  printf 'crates/clean/src/lib.rs turn 1 period\n' >> "$1/$TABLE"
  plant_new_site "$1"
}

plant_at_table() {
  printf 'crates/clean/src/lib.rs turn 1 angle\n' >> "$1/$TABLE"
  plant_new_site "$1"
}

# A comment, a string, a `use`, a test module, a `_rows.rs` file, a file
# under a test-mounted directory and the period home are not sites.
plant_not_sites() {
  printf '// 2.0 * PI\npub const S: &str = "TAU";\nuse core::f64::consts::TAU;\n#[cfg(test)]\nmod tests {\n    fn f() -> f64 { 2.0 * core::f64::consts::PI }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
  printf 'fn f() -> f64 { core::f64::consts::TAU }\n' > "$1/crates/clean/src/turn_rows.rs"
  printf '#[cfg(test)]\nmod probes;\n' >> "$1/crates/clean/src/lib.rs"
  mkdir -p "$1/crates/clean/src/probes"
  printf 'fn f() -> f64 { core::f64::consts::TAU }\n' > "$1/crates/clean/src/probes/deep.rs"
  mkdir -p "$1/crates/geom-core/src"
  printf 'fn f() -> f64 { core::f64::consts::TAU }\n' > "$1/crates/geom-core/src/period.rs"
}

# The row's site moved to another function: the same file's count, a
# different key.
plant_moved_function() {
  printf 'crates/clean/src/lib.rs turn 1 angle\n' >> "$1/$TABLE"
  printf 'pub fn other() -> f64 { 2.0 * core::f64::consts::PI }\n' >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "spelled in production code outside its homes" plant_new_site
  gate_selftest_case "spelled in production code outside its homes" plant_from_f64
  gate_selftest_case "spelled in production code outside its homes" plant_bare
  gate_selftest_case "went away" plant_fell
  gate_selftest_case "is not a class" plant_bad_class
  gate_selftest_case "spelled in production code outside its homes" plant_moved_function
  gate_selftest_passes "a site at its row" plant_at_table
  gate_selftest_passes "a comment, a string, a use, a test module and a file under its directory, a rows file and the home" plant_not_sites
  printf '%s selftest OK: passes a clean fixture, a site at its row, and spellings in comments, strings, uses, test modules, rows files and the home; fires on a new 2.0 * PI, from_f64(TAU) and bare TAU, an unrecorded fall, a row of no class and a site moved to another function\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
