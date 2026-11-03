#!/usr/bin/env bash
# display-payload-only.sh — a refusal's `Display` carries only its
# payload; its ending exists only through `render(at)`, at the door that
# reads it. ONE home; ci.yml's `lint` job runs every gate in this
# directory.
#
# THE RULE (`geom_brep::recourse::Ended`; PCERT E1e, from Ev's ruling on
# PR 5838). An ending is read at a door (`recourse::Reading`: a build or
# at rest, or `ReadAt::File`, the import door). A `Display` cannot know
# which door reads it, so a `Display` that spelled one answered every
# other door's question with that one's answer.
#
# WHAT THIS GATE HOLDS, over production code (every `crates/*/src` file
# but the test-only mounts, and in it no inline `#[cfg(test)]` module;
# comments and string literals stripped):
#
#  1. NO `impl … Display for …` BODY SPELLS A READING OR RENDERS AT ONE:
#     no `Reading::`, no `ReadAt::`, no `render…(` call, no `.ended(`, no
#     `at_build(` / `at_rest(`. A `render(f, …)` call is a formatter
#     helper's (its first argument is the formatter, never a door) and is
#     not counted.
#  2. THE PRODUCTION SPELLING SITES ARE PINNED PER FILE AND PER READING
#     (`display-payload-only.counts`): `Reading::Build` and the
#     `recourse::at_build(` door count under `build`, `Reading::AtRest`
#     and `recourse::at_rest(` under `at_rest`, `ReadAt::File` under
#     `import`. A new site moves its file's pin, so it is added knowingly
#     and read beside its reason; a site that goes moves it down.
#
# WHAT IT CANNOT SEE. A reading reached through a variable (`render(at)`,
# a door's `at` passed down) is not a spelling and is not counted: that
# is the point of the rule, the reading travels from the door. A helper
# function that spells a reading and is called from a `Display` passes
# rule 1 (the helper is counted under rule 2, in its own file, so the
# site is pinned but its caller is not named). A constant ending string
# (`KERNEL_DEFECT_ENDING`, validate's `DEFECT`) is text, not a reading,
# and a `Display` that writes one is not caught; such a constant answers
# one door at every door, which is what
# `work/pcert/a-displays-constant-ending-answers-one-door-at-every-door.md`
# tracks.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

BASELINE=scripts/gates/display-payload-only.counts

gate() {
  gate_require_file "$BASELINE"
  gate_require_crate_sources
  gate_production_sources
  local report
  if ! report=$(python3 - "$BASELINE" "${GATE_PRODUCTION_FILES[@]}" <<'PY'
import re, sys

baseline_path, files = sys.argv[1], sys.argv[2:]


def strip(text):
    # Comments, string and char literals out, the code and its newlines
    # kept.
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


DISPLAY = re.compile(r"\bimpl\b[^{;]*?\bDisplay\s+for\s+[^{;]*\{")
SPELLS = re.compile(
    r"\bReading::|\bReadAt::|\brender\w*\s*\((?!\s*f\s*[,)])|\.ended\s*\(|\bat_build\s*\(|\bat_rest\s*\("
)
KEYS = [
    ("build", re.compile(r"\bReading::Build\b|(?<!fn )\bat_build\s*\(")),
    ("at_rest", re.compile(r"\bReading::AtRest\b|(?<!fn )\bat_rest\s*\(")),
    ("import", re.compile(r"\bReadAt::File\b")),
]

offences, counts = [], {}
for path in files:
    with open(path, encoding="utf-8") as fh:
        code = without_test_mods(strip(fh.read()))
    for m in DISPLAY.finditer(code):
        body = code[m.end() - 1:matching(code, m.end() - 1) + 1]
        for s in SPELLS.finditer(body):
            line = code.count("\n", 0, m.end() - 1 + s.start()) + 1
            offences.append(f"{path}:{line}: {s.group(0)}")
    for key, pat in KEYS:
        n = len(pat.findall(code))
        if n:
            counts[f"{path} {key}"] = n

base = {}
with open(baseline_path, encoding="utf-8") as fh:
    for line in fh:
        line = line.split("#", 1)[0].strip()
        if not line:
            continue
        path, key, n = line.split()
        base[f"{path} {key}"] = int(n)

for o in offences:
    print("DISPLAY " + o)
for k in sorted(set(counts) | set(base)):
    now, was = counts.get(k, 0), base.get(k, 0)
    if now > was:
        print(f"ROSE {k}: {was} -> {now}")
    elif now < was:
        print(f"FELL {k}: {was} -> {now}")
for key, _ in KEYS:
    print(f"TOTAL {key} {sum(v for k, v in counts.items() if k.endswith(' ' + key))}")
PY
  ); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if grep -q '^DISPLAY ' <<<"$report"; then
    grep '^DISPLAY ' <<<"$report"
    gate_error "a Display impl spells a reading or renders at one: a refusal's Display is its payload alone. Write the type once as \`fn write(&self, f, at: Option<ReadAt>)\` (\`geom_brep::ended_by_write!\`), nest refusals through \`recourse::read_at(e, at)\`, and let the door that reports it call \`render(at)\` with its own reading"
    exit 1
  fi
  if grep -qE '^(ROSE|FELL) ' <<<"$report"; then
    grep -E '^(ROSE|FELL) ' <<<"$report"
    gate_error "the production sites that spell a reading moved — set $BASELINE to the new per-file counts in the change that moves them, so each new site is added knowingly: a door renders at its own reading, and a nested refusal ended at another door than its reader's is a finding to file"
    exit 1
  fi
  gate_ok "no Display spells a reading; the production spelling sites at their per-file pins (build $(sed -n 's/^TOTAL build //p' <<<"$report"), at rest $(sed -n 's/^TOTAL at_rest //p' <<<"$report"), import $(sed -n 's/^TOTAL import //p' <<<"$report"))"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# path reading count\n' > "$1/$BASELINE"
}

plant_display_renders() {
  printf 'impl core::fmt::Display for E {\n    fn fmt(&self, f: &mut F) -> R {\n        f.write_str(&self.render(Reading::Build))\n    }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_display_spells_through_a_variable() {
  printf 'impl core::fmt::Display for E {\n    fn fmt(&self, f: &mut F) -> R {\n        write!(f, "{}", self.inner.render(at))\n    }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_new_site() {
  printf 'pub fn door(e: &E) -> String { e.render(Reading::AtRest) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_pinned_site() {
  printf 'crates/clean/src/lib.rs build 1\ncrates/clean/src/lib.rs import 1\n' >> "$1/$BASELINE"
  printf 'pub fn door(e: &E, file: C) -> (String, String) {\n    (e.render(Reading::Build), e.render(ReadAt::File(file)))\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_site_gone() {
  printf 'crates/clean/src/lib.rs at_rest 2\n' >> "$1/$BASELINE"
  printf 'pub fn door(e: &E) -> String { e.render(Reading::AtRest) }\n' >> "$1/crates/clean/src/lib.rs"
}

# A formatter helper named `render`, a payload Display, a test module's
# spelling and a comment's are none of them a door's spelling.
plant_not_sites() {
  printf 'impl core::fmt::Display for M {\n    fn fmt(&self, f: &mut F) -> R {\n        self.render(f, |x, f| x.fmt(f))\n    }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
  printf '// e.render(Reading::Build)\n#[cfg(test)]\nmod tests {\n    fn t(e: &E) -> String { e.render(Reading::Build) }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "a Display impl spells a reading" plant_display_renders
  gate_selftest_case "a Display impl spells a reading" plant_display_spells_through_a_variable
  gate_selftest_case "spell a reading moved" plant_new_site
  gate_selftest_case "spell a reading moved" plant_site_gone
  gate_selftest_passes "two sites at their pins" plant_pinned_site
  gate_selftest_passes "a formatter helper, a test module and a comment" plant_not_sites
  printf '%s selftest OK: passes a clean fixture, sites at their pins, a formatter helper, a test module and a comment; fires on a Display that renders at a reading or through a variable, on a new spelling site and on one gone\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
