#!/usr/bin/env bash
# argmin-folds.sh — a minimum over decided comparisons is read through
# its one door, `geom_core::ladder::argmin_decided`; every hand-rolled
# fold that keeps an incumbent across a decided reading has a row here,
# keyed by its function, with the reason it is not the door's question.
# A row is meant only to be lowered; ONE home; ci.yml's `lint` job runs
# every gate in this directory.
#
# THE RULE (`work/onecopy/an-argmin-by-decided-comparison-is-spelled-three-ways.md`).
# An argmin whose comparison is a decided reading (a margin through
# `decide`, `prove`, `sign_within` or a band) asks what a tie and an
# undecided pair leave of the minimum. The door answers once: the set
# rule (a candidate drops out only where another is decided nearer than
# it), the ending by what the caller is free of (`FreeOf`), and the
# recording. A fold that keeps an incumbent answers it again, its own
# way, and the answer depends on the candidates' order. This gate is a
# backstop for the door: a new fold raises its function above its row
# and fails, and one that went away lowers it and fails until the row is
# lowered.
#
# THE CLASSES. Each row names one; the class says why the site is not
# the door's question.
#   nearest-unique  a nearest that must be decided apart from EVERY other
#                   candidate, a tie with the winner a refusal of its own
#                   kind (`ray_walk::Crossings::closest`'s graze), where
#                   the set rule would let a tie between losers pass
#   cluster         the least cluster of spans, where a tie joins the
#                   cluster rather than contesting the minimum
#                   (`ray_walk::Crossings::first_count`)
#   refusal-only    a best kept only to say which refusal a search
#                   reports, every definite answer certified elsewhere
#                   (the row's site cites the step)
#   lex-total       a minimum under a strict total order on what the fold
#                   reads (no two candidates tie, every undecided read
#                   escalates), so its n − 1 reads find the door's answer;
#                   the door's strict-total-order mode is its home once a
#                   second site has that premise (the argmin row)
#   pair            two candidates read by one decided comparison, its
#                   tie and its undecided reading each the site's own
#                   refusal: an argmin of two is that comparison
#
# WHAT IS A FOLD (a heuristic; its blind spots below). In a production
# function, a binding declared `let mut NAME` that is reassigned
# (`NAME = …`, or `NAME.0 = …`, not `==`) within six lines after a
# decided call (`decide(`, `decide_reported(`, `decide_magnitude(`,
# `prove(`, `sign_within(`, `nearest_sign(`, `along_sign(`, and the
# named deciding helper `lex_less(`) whose
# argument text names NAME: the fold compares a candidate against its
# incumbent. ALSO, by name alone: a `let mut` binding named best, least,
# nearest, closest, incumbent, winner or min, in a function holding a
# loop and a decided call. Each (file, function) counts its matching
# bindings once.
#
# BLIND SPOTS, stated as this gate's own: a fold whose comparison is
# a helper's that the list above does not name, and whose binding has
# none of the names above; a fold over `.min_by`/`.fold`
# with a decided closure; an incumbent reassigned more than six lines
# after its comparison; a call through a path or a macro. A raised
# count, a new row, or a class changed within a row passes here; the
# table's diff is read in review, and that is where it is held.
#
# WHAT IS READ: every `crates/*/src/**/*.rs`, comments and literals out,
# `#[cfg(test)]` modules out, files declared test-only by a
# `#[cfg(test)] mod NAME;` (with their directory) out, and `_rows.rs`,
# `_fuzz.rs` and `test_support*` files out. The door's own file
# (`crates/geom-core/src/ladder.rs`) is exempt.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

TABLE=scripts/gates/argmin-folds.tsv

gate() {
  gate_require_file "$TABLE"
  gate_require_crate_sources
  local report
  if ! report=$(python3 - "$TABLE" <<'PY'
import glob, os, re, sys

table_path = sys.argv[1]
EXEMPT = {"crates/geom-core/src/ladder.rs"}
CLASSES = {"nearest-unique", "cluster", "refusal-only", "pair", "lex-total"}
TEST_NAME = re.compile(r"(?:_rows|_fuzz)\.rs$|^test_support")
# The decided calls, and the named helpers that decide (a helper's own
# decide is in another function, out of this gate's sight).
DECIDED = re.compile(r"\b(?:decide|decide_reported|decide_magnitude|prove|sign_within|nearest_sign|along_sign|lex_less)\s*\(")
NAMES = {"best", "least", "nearest", "closest", "incumbent", "winner", "min"}
LOOP = re.compile(r"\b(?:for|while|loop)\b")


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


FN = re.compile(r"\bfn\s+(\w+)[^{;]*\{")


def functions(text):
    """(name, body) of every fn with a body, innermost bodies counted in
    their own fn and not again in the outer."""
    spans = []
    for m in FN.finditer(text):
        open_at = m.end() - 1
        spans.append((m.group(1), open_at, matching(text, open_at)))
    out = []
    for name, a, b in spans:
        inner = [(x, y) for _, x, y in spans if a < x and y <= b]
        body, at = [], a
        for x, y in sorted(inner):
            if x < at:
                continue
            body.append(text[at:x])
            at = y + 1
        body.append(text[at:b + 1])
        out.append((name, "".join(body)))
    return out


def folds(body):
    lines = body.split("\n")
    found = set()
    for m in re.finditer(r"\blet\s+mut\s+(\w+)", body):
        name = m.group(1)
        if name == "_":
            continue
        decl_line = body.count("\n", 0, m.start())
        assign = re.compile(r"(?<![\w.])" + re.escape(name) + r"(?:\.\d+)?\s*=(?!=)")
        uses = re.compile(r"\b" + re.escape(name) + r"\b")
        hit = False
        for i in range(decl_line + 1, len(lines)):
            if not assign.search(lines[i]):
                continue
            window = "\n".join(lines[max(decl_line + 1, i - 6):i + 1])
            for d in DECIDED.finditer(window):
                close = matching_paren(window, d.end() - 1)
                if uses.search(window[d.end():close]):
                    hit = True
                    break
            if hit:
                break
        if not hit and name in NAMES and LOOP.search(body) and DECIDED.search(body):
            hit = True
        if hit:
            found.add(name)
    return found


def matching_paren(text, at):
    depth = 0
    for i in range(at, len(text)):
        if text[i] == "(":
            depth += 1
        elif text[i] == ")":
            depth -= 1
            if depth == 0:
                return i
    return len(text)


files = sorted(glob.glob("crates/*/src/**/*.rs", recursive=True))
code = {p: strip(open(p, encoding="utf-8").read()) for p in files}
test_only, test_dirs = set(), set()
for path, text in code.items():
    d, stem = os.path.dirname(path), os.path.basename(path)[:-3]
    base = d if stem in ("lib", "mod", "main") else os.path.join(d, stem)
    for m in re.finditer(CFG_TEST + ";", text):
        test_only.add(os.path.join(base, m.group(1) + ".rs"))
        test_dirs.add(os.path.join(base, m.group(1)) + os.sep)

counts = {}
for path in files:
    if (path in EXEMPT or path in test_only or TEST_NAME.search(os.path.basename(path))
            or any(path.startswith(d) for d in test_dirs)):
        continue
    for fn, body in functions(without_test_mods(code[path])):
        n = len(folds(body))
        if n:
            key = (path, fn)
            counts[key] = counts.get(key, 0) + n

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
    gate_error "a fold keeping an incumbent across a decided reading arrived — read the minimum through geom_core::ladder::argmin_decided, declaring what it is free of (FreeOf) with its proof, not a new table row: a row is meant only to be lowered"
    exit 1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "folds went away — lower (or delete) their rows in $TABLE, so the ratchet holds them there"
    exit 1
  fi
  gate_ok "folds over decided comparisons outside geom_core::ladder at their table ($(sed -n 's/^TOTAL //p' <<<"$report") in all)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# path function count class\n' > "$1/$TABLE"
}

# A fold that keeps its incumbent unless a candidate decides nearer.
plant_fold() {
  printf 'pub fn pick(xs: &[f64]) -> usize {\n    let mut best = 0;\n    for i in 1..xs.len() {\n        if decide("row", Margin::of(xs[i] - xs[best]), band) == Ok(Sign::Negative) {\n            best = i;\n        }\n    }\n    best\n}\n' >> "$1/crates/clean/src/lib.rs"
}

# The same fold under a name the name list does not hold: caught by its
# shape (the incumbent named in the decided call, then reassigned).
plant_renamed_fold() {
  printf 'pub fn pick(xs: &[f64]) -> usize {\n    let mut keep = 0;\n    for i in 1..xs.len() {\n        if decide("row", Margin::of(xs[i] - xs[keep]), band) == Ok(Sign::Negative) {\n            keep = i;\n        }\n    }\n    keep\n}\n' >> "$1/crates/clean/src/lib.rs"
}

plant_at_table() {
  printf 'crates/clean/src/lib.rs pick 1 nearest-unique\n' >> "$1/$TABLE"
  plant_fold "$1"
}

plant_stale_row() {
  printf 'crates/clean/src/lib.rs pick 1 cluster\n' >> "$1/$TABLE"
}

plant_bad_class() {
  printf 'crates/clean/src/lib.rs pick 1 incumbent\n' >> "$1/$TABLE"
  plant_fold "$1"
}

# A comment, a string, a test module, a rows file, the door's own file,
# a plain `f64` minimum and a decided filter that keeps no incumbent are
# not folds.
plant_not_folds() {
  printf '// let mut best = 0; best = decide(best);\npub const S: &str = "let mut best = decide(best)";\n#[cfg(test)]\nmod tests {\n    fn f(xs: &[f64]) -> usize { let mut best = 0; for i in 0..xs.len() { if decide("r", Margin::of(xs[i] - xs[best]), b) == Ok(Sign::Negative) { best = i; } } best }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
  printf 'pub fn lo(xs: &[f64]) -> usize {\n    let mut best = 0;\n    for i in 1..xs.len() {\n        if xs[i] < xs[best] {\n            best = i;\n        }\n    }\n    best\n}\n' >> "$1/crates/clean/src/lib.rs"
  printf 'pub fn kept(xs: &[f64]) -> Vec<f64> {\n    let mut out = Vec::new();\n    for &x in xs {\n        if decide("row", Margin::of(x), band) == Ok(Sign::Positive) {\n            out.push(x);\n        }\n    }\n    out\n}\n' >> "$1/crates/clean/src/lib.rs"
  printf 'fn f(xs: &[f64]) -> usize { let mut best = 0; for i in 0..xs.len() { if decide("r", Margin::of(xs[i] - xs[best]), b) == Ok(Sign::Negative) { best = i; } } best }\n' > "$1/crates/clean/src/pick_rows.rs"
  mkdir -p "$1/crates/geom-core/src"
  printf 'fn f(xs: &[f64]) -> usize { let mut best = 0; for i in 0..xs.len() { if decide("r", Margin::of(xs[i] - xs[best]), b) == Ok(Sign::Negative) { best = i; } } best }\n' > "$1/crates/geom-core/src/ladder.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "fold keeping an incumbent" plant_fold
  gate_selftest_case "fold keeping an incumbent" plant_renamed_fold
  gate_selftest_case "went away" plant_stale_row
  gate_selftest_case "is not a class" plant_bad_class
  gate_selftest_passes "a fold at its row" plant_at_table
  gate_selftest_passes "comments, strings, test modules, rows files, the door's file, a plain minimum and a decided filter" plant_not_folds
  printf '%s selftest OK: passes a clean fixture, a fold at its row, and non-folds (comments, strings, test modules, rows files, the door, a plain minimum, a decided filter); fires on a new fold, a renamed one, a stale row and an unknown class\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
