#!/usr/bin/env bash
# atan2-sites.sh — every `.atan2(` in production code outside the angle
# home has a row in a site table, keyed by its function, with its class.
# A row is meant only to be lowered; that is held in review (below).
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE (`work/onecopy/an-angle-read-clear-of-its-cut-is-spelled-five-ways-and-angle-of-by-hand.md`,
# "Decided"). An angle read off the geometry has its doors in `geom`:
# `Azimuth::principal`/`principal_in` (a window's read), `Azimuth::near`
# (a stated reference's copy), `geom::separation` (the distance round
# the turn), `Conic::azimuth` and `Curve3::azimuth` (a point's direction
# on a carrier), and a held angle renamed by whole turns goes through
# `geom_core::period::Period::near`. This gate is a backstop for those
# doors, not the canonical reading: a new `.atan2(` call anywhere else
# raises its function above its row and fails, and a call that went
# away lowers it and fails until the row is lowered, so the slack cannot
# let one back in unseen.
#
# EXEMPT, not in the table: the angle home `crates/geom/src/azimuth.rs`,
# and the scalar implementations of `Real::atan2`
# (`crates/geom-core/src/{dual,interval,sym}.rs`).
#
# THE CLASSES. Each row names one; the class says why the site is not a
# door's question. There is NO in-frame class: the angle of a point or
# direction in a carrier's `(u_ref, v_ref)` frame is `Conic::azimuth`'s,
# `Curve3::azimuth`'s or `Azimuth::of(..)`'s question, and a row naming
# `in-frame` is refused.
#   turn          the angle between two directions, atan2(cross, dot)
#   harmonic      the phase of a harmonic's coefficients, `a cos + b sin`
#   harmonic-cut  a harmonic phase read clear of `atan2`'s cut by a
#                 hand-written two-formula choice (filed:
#                 work/onecopy/harmonic-phases-pick-their-cut-side-by-hand.md)
#   chart         a chart's own closed form (a latitude, a projected
#                 chart, a pcurve's closed form, a torus window's bound)
#   upper-half    a read whose `y` is never negative (a square root, an
#                 absolute value), so it lies clear of `atan2`'s cut and
#                 asks no decision, as `geom::separation` does; at `Real`
#                 there is no door to ask (`geom::surfaces::sheet_apex`)
#   user          an angle the user states or reads (the expression
#                 language's atan2, a mate's angles, a measure, a sketch)
#   awaits-6443   `topo::boolean::rim_wedge`, rewritten by ONEDOOR's
#                 PR 6443 (stage 3 of the placement decision)
#
# THE KEY is (file, enclosing function, class): a row per function, so a
# site that moves between functions, or a function whose class changes,
# changes a line. WHAT THIS GATE DOES NOT ENFORCE, stated as
# `engine-leaves.counts` states its own: a raised count, a new row, or a
# swap within one function of a site of one class for one of another
# (an in-frame read for a turn) passes; the table's diff is read in
# review, and that is where it is held.
#
# ITS SECOND BLIND SPOT is the in-frame read spelled through the door's
# own constructor: `geom::Azimuth::of(w.dot(v_ref), w.dot(u_ref))` by
# hand is the read this gate exists to send to `Conic::azimuth` /
# `Curve3::azimuth` (and `Frame::azimuth`, filed), and no `.atan2(`
# shows it. The sites are listed in
# `work/onecopy/frame-azimuth-waits-on-the-frame-module.md`.
#
# WHAT IS READ: `.atan2(` (whitespace allowed before the paren) in every
# `crates/*/src/**/*.rs`, comments and literals out, `#[cfg(test)]`
# modules out. A file is test-only, and not read, where a
# `#[cfg(test)] mod NAME;` declares it, or where its name ends `_rows.rs`
# or `_fuzz.rs` or starts `test_support`. BLIND SPOTS: a call through a
# path (`f64::atan2(y, x)`, `Real::atan2(y, x)`), through a function
# pointer, or inside a macro's expansion; a test-only module declared
# with a `#[path]` attribute or a `cfg` other than `test`. The second
# pass shaped at the first: `grep -rn 'atan2(' crates/*/src | grep -v
# '\.atan2'` lists the path calls by hand.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

TABLE=scripts/gates/atan2-sites.tsv

gate() {
  gate_require_file "$TABLE"
  gate_require_crate_sources
  local report
  if ! report=$(python3 - "$TABLE" <<'PY'
import glob, os, re, sys

table_path = sys.argv[1]
CALL = re.compile(r"\.atan2\s*\(")
EXEMPT = {
    "crates/geom/src/azimuth.rs",
    "crates/geom-core/src/dual.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/sym.rs",
}
CLASSES = {"turn", "harmonic", "harmonic-cut", "chart", "upper-half", "user", "awaits-6443"}
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
for path, text in code.items():
    d, stem = os.path.dirname(path), os.path.basename(path)[:-3]
    base = d if stem in ("lib", "mod", "main") else os.path.join(d, stem)
    for m in re.finditer(CFG_TEST + ";", text):
        test_only.add(os.path.join(base, m.group(1) + ".rs"))
        test_only.add(os.path.join(base, m.group(1), "mod.rs"))

FN = re.compile(r"\bfn\s+(\w+)")
counts = {}
for path in files:
    if path in EXEMPT or path in test_only or TEST_NAME.search(os.path.basename(path)):
        continue
    text = without_test_mods(code[path])
    for m in CALL.finditer(text):
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
        if cls == "in-frame":
            bad.append(f"{table_path}:{number}: {path}: an in-frame angle is Conic::azimuth's, Curve3::azimuth's or geom::Azimuth's to read, never a table row")
        elif cls not in CLASSES:
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
    gate_error "a production .atan2( call arrived outside geom::azimuth — read the angle through its door (Conic::azimuth / Curve3::azimuth / geom::Azimuth::near or ::principal_in, geom::separation, Period::near for a held angle), not a new table row: a row is meant only to be lowered"
    exit 1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "production .atan2( calls went away — lower (or delete) their rows in $TABLE, so the ratchet holds them there"
    exit 1
  fi
  gate_ok "production .atan2( calls outside geom::azimuth at their table ($(sed -n 's/^TOTAL //p' <<<"$report") in all)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# path function count class\n' > "$1/$TABLE"
}

plant_new_site() {
  printf 'pub fn angle(y: f64, x: f64) -> f64 { y.atan2(x) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_fell() {
  printf 'crates/clean/src/lib.rs angle 1 turn\n' >> "$1/$TABLE"
}

plant_in_frame_row() {
  printf 'crates/clean/src/lib.rs angle 1 in-frame\n' >> "$1/$TABLE"
  plant_new_site "$1"
}

plant_at_table() {
  printf 'crates/clean/src/lib.rs angle 1 turn\n' >> "$1/$TABLE"
  plant_new_site "$1"
}

# A comment, a string, a test module, a `_rows.rs` file and the exempt
# home are not production calls.
plant_not_sites() {
  printf '// y.atan2(x)\npub const S: &str = "y.atan2(x)";\n#[cfg(test)]\nmod tests {\n    fn f(y: f64) -> f64 { y.atan2(1.0) }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
  printf 'fn f(y: f64) -> f64 { y.atan2(1.0) }\n' > "$1/crates/clean/src/angle_rows.rs"
  mkdir -p "$1/crates/geom/src"
  printf 'fn f(y: f64) -> f64 { y.atan2(1.0) }\n' > "$1/crates/geom/src/azimuth.rs"
}

# The row's site moved to another function: the same file's count, a
# different key.
plant_moved_function() {
  printf 'crates/clean/src/lib.rs angle 1 turn\n' >> "$1/$TABLE"
  printf 'pub fn other(y: f64, x: f64) -> f64 { y.atan2(x) }\n' >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "arrived outside geom::azimuth" plant_new_site
  gate_selftest_case "went away" plant_fell
  gate_selftest_case "in-frame angle" plant_in_frame_row
  gate_selftest_case "arrived outside geom::azimuth" plant_moved_function
  gate_selftest_passes "a site at its row" plant_at_table
  gate_selftest_passes "a comment, a string, a test module, a rows file and the home" plant_not_sites
  printf '%s selftest OK: passes a clean fixture, a site at its row, and calls in comments, strings, test modules, rows files and the home; fires on a new site, an unrecorded fall, an in-frame row and a site moved to another function\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
