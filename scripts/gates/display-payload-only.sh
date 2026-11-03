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
#  1. NO BODY THAT WRITES A PAYLOAD REACHES AN ENDING. Two kinds of
#     body are read:
#     - every `impl … Display for …` body. It may not spell a reading
#       (`Reading::`, `ReadAt::`), call `render…(` (a `render(f, …)`
#       formatter helper excepted), `.ended(`, `Ended::ended(`,
#       `defect_ending(`, `unreadable_margin_note(`,
#       `recourse::at_build(` / `at_rest(`, `read_at(_, Some(…))`, its
#       own `write(f, Some(…))`, or `.ending(…)` with anything but a
#       constant lever (an all-caps constant, or a `.recourse` field);
#     - every function that takes an optional reading,
#       `…: Option<ReadAt>`: the `write(f, at)` that
#       `geom_brep::ended_by_write!` wraps, `EulerOpError::text`, a
#       decision's `written(diag, at)`. Its `None` is the payload, so it
#       may reach an ending only through the `at` a `Some` binds. It may
#       not spell a reading (`ReadAt::is_build`, the one door test,
#       excepted), call `Ended::ended(`, `read_at(_, Some(…))` or
#       `write(f, Some(…))`. `geom_brep::recourse`, the mechanism's
#       home, is not read.
#  2. THE PRODUCTION SPELLING SITES ARE PINNED PER FILE AND PER READING
#     (`display-payload-only.counts`): `Reading::Build` and
#     `recourse::at_build(` count under `build`, `Reading::AtRest` and
#     `recourse::at_rest(` under `at_rest`, `ReadAt::File` under
#     `import`. Only the path-qualified `recourse::at_…(` is a reading:
#     a bare `at_rest(` is another function. A new site moves its file's
#     pin, so it is added knowingly and read beside its reason; a site
#     that goes moves it down.
#
# WHAT STAYS BLIND.
#  - A reading reached through a variable (`render(at)`, a door's `at`
#    passed down) is not a spelling: that is the point, the reading
#    travels from the door. Inside an optional body the gate does not
#    tell a `Some` arm from a `None` arm. An unwrapped `at` exists only
#    in a `Some` arm, so a call taking it cannot sit in a `None` arm.
#    But a `.ending(…)` or `defect_ending(…)` taking a reading built some
#    other way there is not seen. Review holds that, and the rows that
#    pin a type's payload against its door renders (`TrimOffFace`'s,
#    the split's) hold it where they exist.
#  - A helper that spells a reading inside it, called from a body, passes
#    rule 1. The helper is counted under rule 2 in its own file, so the
#    site is pinned but its caller is not named.
#  - A constant ending string (`KERNEL_DEFECT_ENDING`, validate's
#    `DEFECT`) is text, not a reading, and a body that writes one is not
#    caught. Such a constant answers one door at every door; the
#    remaining ones, and the fixed-door helpers, are listed by
#    `file:line` in
#    `work/pcert/a-displays-constant-ending-answers-one-door-at-every-door.md`.
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
# A function that takes an optional reading (`at: Option<ReadAt>`): a
# refusal's `write(f, at)` (what `ended_by_write!` wraps), `text(at)`,
# a decision's `written(diag, at)`. Its body writes the payload when
# `at` is `None` and may end it only through that `at`.
OPTIONAL = re.compile(
    r"\bfn\s+\w+\s*(?:<[^>]*>)?\s*\([^)]*?:\s*Option\s*<\s*(?:[\w:]*::)?ReadAt\s*>[^)]*\)[^{;]*\{"
)
# What reaches an ending, in any of the bodies.
BOTH = [
    r"\bReading::",
    r"\bReadAt::(?!is_build\b)",
    r"\bEnded::ended\s*\(",
    r"\bread_at\s*\([^;]*?,\s*Some\s*\(",
    r"\bwrite\s*\(\s*f\s*,\s*Some\s*\(",
]
# What reaches an ending only in a `Display`: a `write(f, at)` reaches
# these through the `at` a `Some` arm binds, which a `None` arm does
# not have.
DISPLAY_ONLY = [
    r"\brender\w*\s*\((?!\s*f\s*[,)])",
    r"\.ended\s*\(",
    r"\bdefect_ending\s*\(",
    r"\bunreadable_margin_note\s*\(",
    r"\brecourse::at_(?:build|rest)\s*\(",
    # An ending taken with anything but a constant lever.
    r"\.ending\s*\((?!\s*(?:[\w:]*::)?[A-Z][A-Z0-9_]*\s*\)|\s*[\w.]*\.recourse\s*\))",
]
SPELLS = re.compile("|".join(BOTH + DISPLAY_ONLY))
SPELLS_OPTIONAL = re.compile("|".join(BOTH))
# Only the mechanism's own home spells a reading inside an optional
# body (`at_build`, `at_rest`, `read_at`).
HOME = "crates/geom-brep/src/recourse.rs"
KEYS = [
    ("build", re.compile(r"\bReading::Build\b|\brecourse::at_build\s*\(")),
    ("at_rest", re.compile(r"\bReading::AtRest\b|\brecourse::at_rest\s*\(")),
    ("import", re.compile(r"\bReadAt::File\b")),
]

offences, counts = [], {}
for path in files:
    with open(path, encoding="utf-8") as fh:
        code = without_test_mods(strip(fh.read()))
    for pat, spells in ((DISPLAY, SPELLS), (OPTIONAL, SPELLS_OPTIONAL)):
        if pat is OPTIONAL and path == HOME:
            continue
        for m in pat.finditer(code):
            body = code[m.end() - 1:matching(code, m.end() - 1) + 1]
            for s in spells.finditer(body):
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
    gate_error "a Display impl, or a write(f, at) body, reaches an ending: a refusal's Display is its payload alone. Write the type once as \`fn write(&self, f, at: Option<ReadAt>)\` (\`geom_brep::ended_by_write!\`), nest refusals through \`recourse::read_at(e, at)\`, and let the door that reports it call \`render(at)\` with its own reading"
    exit 1
  fi
  if grep -qE '^(ROSE|FELL) ' <<<"$report"; then
    grep -E '^(ROSE|FELL) ' <<<"$report"
    gate_error "the production sites that spell a reading moved — set $BASELINE to the new per-file counts in the change that moves them, so each new site is added knowingly: a door renders at its own reading, and a nested refusal ended at another door than its reader's is a finding to file"
    exit 1
  fi
  gate_ok "no Display or write(f, at) body reaches an ending; the production spelling sites at their per-file pins (build $(sed -n 's/^TOTAL build //p' <<<"$report"), at rest $(sed -n 's/^TOTAL at_rest //p' <<<"$report"), import $(sed -n 's/^TOTAL import //p' <<<"$report"))"
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

# The shapes review 6430 r1 showed passing the first version of rule 1,
# one case each: a Display through `Ended::ended`, `read_at(_, Some)`,
# `defect_ending`, its own `write(f, Some)` and `.ending(reading)`, and a
# `write(f, at)` body that renders at a build whatever `at` is.
plant_display_ufcs() {
  printf 'impl core::fmt::Display for E {\n    fn fmt(&self, f: &mut F) -> R {\n        f.write_str(&geom_brep::recourse::Ended::ended(&self.0, self.1))\n    }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}
plant_display_read_at_some() {
  printf 'impl core::fmt::Display for E {\n    fn fmt(&self, f: &mut F) -> R {\n        write!(f, "{}", read_at(&self.0, Some(self.1)))\n    }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}
plant_display_defect_ending() {
  printf 'impl core::fmt::Display for E {\n    fn fmt(&self, f: &mut F) -> R {\n        f.write_str(geom_brep::recourse::defect_ending(self.1))\n    }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}
plant_display_write_some() {
  printf 'impl core::fmt::Display for E {\n    fn fmt(&self, f: &mut F) -> R {\n        self.write(f, Some(self.1))\n    }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}
plant_display_ending_of_a_reading() {
  printf 'impl core::fmt::Display for E {\n    fn fmt(&self, f: &mut F) -> R {\n        f.write_str(&self.0.ending(self.1))\n    }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}
plant_write_body_renders_at_a_build() {
  printf 'impl E {\n    fn write(&self, f: &mut F, at: Option<geom_brep::recourse::ReadAt>) -> R {\n        f.write_str(&self.0.render(Reading::Build))\n    }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}
# A write(f, at) body that ends only through its `at`, and a Display of
# a constant lever, are no door's spelling.
plant_write_body_through_at() {
  printf 'crates/clean/src/lib.rs build 1\n' >> "$1/$BASELINE"
  printf 'impl E {\n    fn write(&self, f: &mut F, at: Option<geom_brep::recourse::ReadAt>) -> R {\n        match at {\n            Some(at) if at.is_build() => f.write_str(&self.0.render(at)),\n            _ => write!(f, "{} {}", read_at(&self.1, at), read_at(&self.2, geom_brep::recourse::at_build(at))),\n        }\n    }\n}\nimpl core::fmt::Display for L {\n    fn fmt(&self, f: &mut F) -> R {\n        f.write_str(&self.0.ending(COINCIDENCE_RECOURSE))\n    }\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "reaches an ending" plant_display_renders
  gate_selftest_case "reaches an ending" plant_display_spells_through_a_variable
  gate_selftest_case "reaches an ending" plant_display_ufcs
  gate_selftest_case "reaches an ending" plant_display_read_at_some
  gate_selftest_case "reaches an ending" plant_display_defect_ending
  gate_selftest_case "reaches an ending" plant_display_write_some
  gate_selftest_case "reaches an ending" plant_display_ending_of_a_reading
  gate_selftest_case "reaches an ending" plant_write_body_renders_at_a_build
  gate_selftest_passes "a write body that ends only through its at, and a constant lever" plant_write_body_through_at
  gate_selftest_case "spell a reading moved" plant_new_site
  gate_selftest_case "spell a reading moved" plant_site_gone
  gate_selftest_passes "two sites at their pins" plant_pinned_site
  gate_selftest_passes "a formatter helper, a test module and a comment" plant_not_sites
  printf '%s selftest OK: passes a clean fixture, sites at their pins, a formatter helper, a test module, a comment, a write body ending only through its at and a constant lever; fires on a Display that renders at a reading or through a variable, through Ended::ended, read_at(_, Some), defect_ending, write(f, Some) or .ending(reading), on a write body that renders at a build, and on a new spelling site and one gone\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
