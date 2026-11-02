#!/usr/bin/env bash
# bare-rounding-allowlist.sh — a bare `f64::EPSILON` or `UNIT_ROUNDOFF`,
# or a call on geom-core's rounding charge, outside `geom-core` is a
# finding, and the list of today's sites only shrinks. ONE home; ci.yml's `lint` job runs every gate in this
# directory.
#
# THE RULE (DESIGN `[simple.conditioned]`; `crates/geom-core/README.md`,
# "Float rounding"). Every computation is posed well-conditioned where
# its result is decided, and at `f64` is then treated as exact. The one
# statement about rounding is the run's admission rule
# (`geom_core::tolerance::admissible`), and no site charges, pads or
# estimates rounding. A site that names the unit roundoff is therefore
# either a test's oracle or a formula or stored form owed a re-posing.
# This gate is the backstop: it cannot tell a charge from an oracle, so
# it makes every site a listed decision, and refuses a new one.
#
# THE SCAN is every `.rs` file under `crates/*/` (but `crates/geom-core/`),
# `demos/*/`, `tools/*/`, `benches/` and `interval-transcendentals/`,
# comments and string literals stripped. A site is `f64::EPSILON` (any
# path ending in it) or `UNIT_ROUNDOFF` as a word, or a use of the charge
# geom-core offers: a `rounding_charge(` call or a `Rounded::…` path or
# `Rounded { … }` literal (`geom_core::running`). A charge read through
# geom-core is the same decision as a bare one, so it is counted under
# the same ratchet. Each file's sites are
# counted in two scopes: `test`, for a file under a `tests/`, `benches/`
# or `examples/` directory, under `demos/`, `tools/` or `benches/`, a
# file a `#[cfg(test)] mod` mounts (`lib.sh`'s `gate_test_only_mounts`),
# or a site inside a `#[cfg(test)] mod { … }` block; and `prod`
# otherwise.
#
# THE LIST is `bare-rounding-sites.tsv` beside this file, one row per
# (file, scope): file, scope, count, reason. The reason is one word, the
# row's own: a file whose sites are of two kinds carries the one that
# schedules its production sites. Every `prod` row other than
# `exact-arith` and `fixture` is scheduled by
# `work/onehome/site-rounding-charges-owed-a-re-posing.md`, which lists
# its file; `oracle` rows are a test's own reading of the unit roundoff,
# not a charge, and are not expected to shrink.
#
#   oracle      a test's expectation, a fixture's pose or a probe's
#               scale (`test` scope only);
#   re-pose     a production formula or stored form that charges its own
#               rounding, owed a re-posing (a distance form, a pose
#               decided first) so that the charge can go;
#   pose        a production pose decision reading rounding as its zero
#               (a sign of an exact value), owed a decision at the band;
#   far-centre  a production charge for a conic stored far from the arc
#               it serves, which waits on
#               `work/onehome/a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex.md`;
#   exact-arith an error-free transformation (a compensated sum or
#               product), which reads the unit roundoff to compute
#               exactly, not to charge;
#   fixture     test-support code compiled into the library (`prod`
#               scope only).
#
# WHAT REDS:
#
#   UNLISTED  a (file, scope) holding more sites than its row says, or
#             holding sites with no row;
#   STALE     a row whose (file, scope) holds fewer sites than it says
#             (a site went: lower or drop the row in the same change, so
#             the ratchet holds);
#   REASON    a row whose reason is not one of the six, `oracle` on a
#             `prod` row, or `fixture` on a `test` row.
#
# KNOWN GAPS. A token scan sees spellings, not values: a literal
# `2.220446049250313e-16`, `2f64.powi(-52)`, `f64::EPSILON` re-exported
# under another name, a crate-local constant or function derived from
# one (its definition is a site; its uses are not), or a `Rounded` value
# carried through arithmetic without a fresh path (its operators charge;
# only the paths that make one are counted) are not counted.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

LIST=scripts/gates/bare-rounding-sites.tsv

gate() {
  gate_require_file "$LIST"
  gate_require_crate_sources
  local mounts report
  mounts=$(mktemp)
  gate_test_only_mounts "${GATE_SOURCE_FILES[@]}" > "$mounts"
  if [ -e "$GATE_MATCHER_FAILED" ]; then
    rm -f "$GATE_MATCHER_FAILED" "$mounts"
    gate_error "$(gate_name): the test-only mounts were not decided (diagnosed above) — that is not a pass"
    exit 1
  fi
  if ! report=$(python3 - "$LIST" "$mounts" <<'PY'
import glob, os, re, sys

list_path, mounts_path = sys.argv[1], sys.argv[2]
SITE = re.compile(r"\bf64::EPSILON\b|\bUNIT_ROUNDOFF\b|\brounding_charge\s*\(|\bRounded\s*(?:::|\{)")
REASONS = {"oracle", "re-pose", "pose", "far-centre", "exact-arith", "fixture"}


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
            close = '"' + "#" * hashes
            j = text.find(close, i + 2 + hashes)
            j = n if j < 0 else j + len(close)
            out.append('""')
            i = j
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append('""')
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


def split_test_mods(text):
    prod, test, at = [], [], 0
    for m in re.finditer(r"#\[cfg\(test\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{", text):
        if m.start() < at:
            continue
        end = matching(text, m.end() - 1) + 1
        prod.append(text[at:m.start()])
        test.append(text[m.start():end])
        at = end
    prod.append(text[at:])
    return "".join(prod), "".join(test)


mounts = [l.strip() for l in open(mounts_path, encoding="utf-8") if l.strip()]


def test_only(path):
    parts = path.split("/")
    if parts[0] in ("demos", "tools", "benches") or any(p in ("tests", "benches", "examples") for p in parts[1:]):
        return True
    return any(path.startswith(m) if m.endswith("/") else path == m for m in mounts)


paths = set()
for pattern in ("crates/*/**/*.rs", "demos/*/**/*.rs", "tools/*/**/*.rs", "benches/**/*.rs",
                "interval-transcendentals/**/*.rs"):
    paths.update(glob.glob(pattern, recursive=True))
counts = {}
for path in sorted(paths):
    if path.startswith("crates/geom-core/") or "/target/" in path:
        continue
    with open(path, encoding="utf-8") as fh:
        text = strip(fh.read())
    if not SITE.search(text):
        continue
    if test_only(path):
        scoped = {"test": text}
    else:
        prod, test = split_test_mods(text)
        scoped = {"prod": prod, "test": test}
    for scope, body in scoped.items():
        n = len(SITE.findall(body))
        if n:
            counts[(path, scope)] = n

rows, bad = {}, []
for k, line in enumerate(open(list_path, encoding="utf-8"), 1):
    if not line.strip() or line.startswith("#"):
        continue
    cols = line.rstrip("\n").split("\t")
    if len(cols) != 4 or not cols[2].isdigit():
        bad.append(f"REASON {list_path}:{k}: not `file<TAB>scope<TAB>count<TAB>reason`: {line.strip()}")
        continue
    path, scope, n, reason = cols
    if reason not in REASONS or scope not in ("prod", "test") or (reason == "oracle" and scope == "prod") or (reason == "fixture" and scope == "test"):
        bad.append(f"REASON {path} ({scope}): `{reason}` is not a reason this scope may carry")
    rows[(path, scope)] = int(n)

for key in sorted(set(counts) | set(rows)):
    now, was = counts.get(key, 0), rows.get(key, 0)
    if now > was:
        bad.append(f"UNLISTED {key[0]} ({key[1]}): {now} rounding sites, listed {was}")
    elif now < was:
        bad.append(f"STALE {key[0]} ({key[1]}): {now} rounding sites, listed {was}")
for b in bad:
    print(b)
print(f"TOTAL {sum(counts.values())} {sum(v for (p, s), v in counts.items() if s == 'prod')}")
PY
  ); then
    rm -f "$mounts"
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  rm -f "$mounts"
  if grep -q '^UNLISTED \|^REASON ' <<<"$report"; then
    grep '^UNLISTED \|^REASON ' <<<"$report"
    gate_error "a bare f64::EPSILON / UNIT_ROUNDOFF or a rounding charge outside geom-core: no site charges rounding ([simple.conditioned]). Re-pose the formula (a distance form, a pose decided first), or, for a test's oracle, list it in $LIST with its one-word reason"
    exit 1
  fi
  if grep -q '^STALE ' <<<"$report"; then
    grep '^STALE ' <<<"$report"
    gate_error "bare rounding sites fell — lower or drop the row in $LIST, so the list only shrinks"
    exit 1
  fi
  local total prod
  read -r total prod < <(sed -n 's/^TOTAL //p' <<<"$report")
  gate_ok "bare f64::EPSILON / UNIT_ROUNDOFF and rounding charges outside geom-core at their listed counts ($total sites, $prod in production code)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# file\tscope\tcount\treason\n' > "$1/$LIST"
}

plant_new_site() {
  printf 'pub fn pad(x: f64) -> f64 { x * (1.0 + 4.0 * f64::EPSILON) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_new_roundoff() {
  printf 'pub fn pad(x: f64) -> f64 { x * geom_core::UNIT_ROUNDOFF }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_listed() {
  printf 'crates/clean/src/lib.rs\tprod\t1\tre-pose\n' >> "$1/$LIST"
  plant_new_site "$1"
}

plant_stale() {
  printf 'crates/clean/src/lib.rs\tprod\t2\tre-pose\n' >> "$1/$LIST"
  plant_new_site "$1"
}

plant_oracle_in_prod() {
  printf 'crates/clean/src/lib.rs\tprod\t1\toracle\n' >> "$1/$LIST"
  plant_new_site "$1"
}

plant_test_module_as_prod() {
  printf 'crates/clean/src/lib.rs\tprod\t1\tre-pose\n' >> "$1/$LIST"
  printf '#[cfg(test)]\nmod tests {\n    const U: f64 = f64::EPSILON;\n}\n' >> "$1/crates/clean/src/lib.rs"
}

# A comment, a string and geom-core itself are not sites.
plant_new_charge() {
  printf 'pub fn pad(x: f64) -> f64 { x + geom_core::rounding_charge(x) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_new_running() {
  printf 'pub fn pad(x: f64) -> f64 { geom_core::running::Rounded::exact(x).value }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_not_sites() {
  printf '// f64::EPSILON in a comment\npub const S: &str = "UNIT_ROUNDOFF";\n' >> "$1/crates/clean/src/lib.rs"
  mkdir -p "$1/crates/geom-core/src"
  printf 'pub const UNIT_ROUNDOFF: f64 = f64::EPSILON * 0.5;\n' > "$1/crates/geom-core/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "UNLISTED" plant_new_site
  gate_selftest_case "UNLISTED" plant_new_roundoff
  gate_selftest_case "UNLISTED" plant_new_charge
  gate_selftest_case "UNLISTED" plant_new_running
  gate_selftest_case "STALE" plant_stale
  gate_selftest_case "REASON" plant_oracle_in_prod
  gate_selftest_case "UNLISTED crates/clean/src/lib.rs (test)" plant_test_module_as_prod
  gate_selftest_passes "a listed site" plant_listed
  gate_selftest_passes "a comment, a string and geom-core's own" plant_not_sites
  printf '%s selftest OK: passes a clean fixture, a listed site, and comments, strings and geom-core; fires on an unlisted site of each spelling (bare, a rounding_charge call, a Rounded path), a stale row, an oracle in production and a test module counted as production\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
