#!/usr/bin/env bash
# knots-f64-interim.sh — C6's interim `f64` knots and weights: their
# readers and their holders, counted by crate, may only go down.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE (C6 in crates/geom-brep/README.md, and the INLANE
# orchestrator's rulings on its staging, 2026-10-18). A carrier's knots
# and weights are held in its scalar, and at an enclosure scalar either
# may be wider than a point. Where every one is a point, the `f64`
# vector read off them is still reachable through named interim
# accessors, which refuse loud where a value is wider than a point:
#
#   NurbsCurve{2,3}::knots_f64_interim
#   NurbsSurface::knots_u_f64_interim / knots_v_f64_interim
#   Knots::structure_f64_interim, Span::knots_f64_interim
#   NurbsCurve{2,3}::weights_f64_interim, NurbsSurface::weights_f64_interim,
#   Weights::weights_f64_interim (C6 S2)
#   spline::plan_inputs::point_knots (C6 S4/S5b: a knot-vector merge
#   and the surface's column builds take a carrier's vector through it,
#   refusing typed at a wide run; a caller of it is a reader like any
#   other. The held plan constructors beside it decide against
#   `Knots::plan_proxy`, private to the spline module: the knots' own
#   vector where every knot is a point, else a stand-in that no carrier
#   reads; the weights' own vector, read through the accessor above,
#   only where every knot and weight is a point)
#
# Every caller is a reader that has not yet moved to the held values or
# the combinatorial structure. C6's slices S2–S5 move them; S5 deletes
# the accessors with this gate. So both counts below are RATCHETS: the
# baseline file holds each crate's count, a crate above its baseline
# fails (a new reader or holder), and a crate below it fails too, asking
# for the baseline to be lowered, so the slack cannot let one back in
# unseen.
#
# READERS: every call `NAME(` of the names above in every `.rs` under
# `crates/*/` and `demos/*/` (tests included: a test that reads the f64
# vector is a reader too), less the accessors' own `fn` declarations.
# A path call (`Type::knots_f64_interim(x)`) has the same text. A call
# through a function pointer with no `(` is not counted; none exists.
#
# HOLDERS: a vector copied out at an interim read lives on in a field,
# past the call the reader count sees (`SplineCoeffs`, `TensorNet`,
# `Collocation`, `projected::breaks`, the ssi refinement's `ku`/`kv`, a
# certificate's weights, a carrier's own weight channel). So the gate
# also counts, by crate, the FIELDS of structs and enum variants, named
# or tuple, that hold `f64` knots or weights:
#   - any field whose type is `KnotVector` or a wrapper of it (`&'a`,
#     `Option`, `Cow`, `Arc`, `Rc`, `Box`, `Vec`);
#   - a named field called for knots, breaks or weights (`knots*`,
#     `breaks*`, `ku`, `kv`, `weights*`, `weight_channel`, `channel`)
#     whose type is an `f64` sequence (`Vec<f64>`, `&'a [f64]`,
#     `Option<Vec<f64>>`, `Box<[f64]>`, `[f64; N]`, `Cow<'a, [f64]>`),
#     or a field called `point` whose type is one that is not an array
#     (a value channel, not a geometric point).
# A function's parameters are not fields and are not counted. Only `src/`
# is read, and in it no `#[cfg(test)]` module: a test's fixture holding
# the `f64` numbers it builds a carrier from is an input, not a copy.
# Counted under `holders:<crate>` in the same baseline file. Not every
# holder goes in S5 (an `f64` certificate's own data may stay), so S5
# states where the count ends.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

BASELINE=scripts/gates/knots-f64-interim.counts

gate() {
  gate_require_file "$BASELINE"
  gate_require_crate_sources
  local report
  if ! report=$(python3 - "$BASELINE" <<'PY'
import glob, re, sys

baseline_path = sys.argv[1]
NAMES = r"(?:knots_f64_interim|knots_u_f64_interim|knots_v_f64_interim|structure_f64_interim|weights_f64_interim|point_knots)"
CALL = re.compile(r"\b" + NAMES + r"\s*\(")
DECL = re.compile(r"\bfn\s+" + NAMES + r"\s*\(")
KV = re.compile(r"^(?:&'[a-z_]+)?(?:(?:Option|Cow|Arc|Rc|Box|Vec)<(?:'[a-z_]+,)?)?(?:&'[a-z_]+)?KnotVector>?$")
F64 = re.compile(r"^(?:Vec<f64>|&'[a-z_]+\[f64\]|Option<Vec<f64>>|Box<\[f64\]>|\[f64;[^\]]+\]|Cow<'[a-z_]+,\[f64\]>)$")
HOLD_NAME = re.compile(r"^(?:knots\w*|breaks\w*|ku|kv|weights\w*|weight_channel|channel)$")
# `point` is a value channel's name only as a sequence, never a fixed
# array (a geometric point).
POINT_F64 = re.compile(r"^(?:Vec<f64>|&'[a-z_]+\[f64\]|Option<Vec<f64>>|Box<\[f64\]>)$")


def strip(text):
    # Comments, string and char literals out (a small Rust lexer), the
    # code and its newlines kept.
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


def matching(text, at, open_ch, close_ch):
    depth = 0
    for i in range(at, len(text)):
        c = text[i]
        if c == open_ch:
            depth += 1
        elif c == close_ch:
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
        at = matching(text, m.end() - 1, "{", "}") + 1
    out.append(text[at:])
    return "".join(out)


def split_top(body):
    parts, depth, cur = [], 0, []
    for c in body:
        if c in "<([{":
            depth += 1
        elif c in ">)]}":
            depth -= 1
        if c == "," and depth == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(c)
    parts.append("".join(cur))
    return [p for p in (x.strip() for x in parts) if p]


def field_holds(piece, named):
    piece = re.sub(r"#\[[^\]]*\]", "", piece).strip()
    piece = re.sub(r"^pub(?:\([^)]*\))?\s+", "", piece)
    if named:
        m = re.match(r"^([A-Za-z_]\w*)\s*:\s*(.+)$", piece, re.S)
        if not m:
            return False
        name, ty = m.group(1), re.sub(r"\s+", "", m.group(2))
        return (
            bool(KV.match(ty))
            or (bool(HOLD_NAME.match(name)) and bool(F64.match(ty)))
            or (name == "point" and bool(POINT_F64.match(ty)))
        )
    return bool(KV.match(re.sub(r"\s+", "", piece)))


def bodies(text):
    # (body, named) for every struct, and every enum variant with fields.
    for m in re.finditer(r"\bstruct\s+[\w$]+\s*(?:<)?", text):
        i = m.end()
        if text[i - 1] == "<":
            i = matching(text, i - 1, "<", ">") + 1
        rest = text[i:]
        k = re.match(r"\s*(?:where[^{;]*)?([({;])", rest)
        if not k:
            continue
        at = i + k.end() - 1
        if text[at] == "{":
            yield text[at + 1:matching(text, at, "{", "}")], True
        elif text[at] == "(":
            yield text[at + 1:matching(text, at, "(", ")")], False
    for m in re.finditer(r"\benum\s+[\w$]+[^{;]*\{", text):
        at = m.end() - 1
        body = text[at + 1:matching(text, at, "{", "}")]
        for v in re.finditer(r"\b[A-Z]\w*\s*([({])", body):
            j = v.end() - 1
            if body[j] == "{":
                yield body[j + 1:matching(body, j, "{", "}")], True
            else:
                yield body[j + 1:matching(body, j, "(", ")")], False


def holders(text):
    text = without_test_mods(strip(text))
    return sum(
        1 for body, named in bodies(text) for piece in split_top(body) if field_holds(piece, named)
    )

counts = {}
for path in sorted(glob.glob("crates/*/**/*.rs", recursive=True)
                   + glob.glob("demos/*/**/*.rs", recursive=True)):
    crate = "/".join(path.split("/")[:2])
    with open(path, encoding="utf-8") as fh:
        text = fh.read()
    n = len(CALL.findall(text)) - len(DECL.findall(text))
    if n:
        counts[crate] = counts.get(crate, 0) + n
    h = holders(text) if "/src/" in path else 0
    if h:
        counts["holders:" + crate] = counts.get("holders:" + crate, 0) + h

base = {}
with open(baseline_path, encoding="utf-8") as fh:
    for line in fh:
        line = line.split("#", 1)[0].strip()
        if not line:
            continue
        crate, n = line.split()
        base[crate] = int(n)

rose, fell = [], []
for crate in sorted(set(counts) | set(base)):
    now, was = counts.get(crate, 0), base.get(crate, 0)
    if now > was:
        rose.append(f"{crate}: {was} -> {now}")
    elif now < was:
        fell.append(f"{crate}: {was} -> {now}")
total = sum(v for k, v in counts.items() if not k.startswith("holders:"))
held = sum(v for k, v in counts.items() if k.startswith("holders:"))
for r in rose:
    print("ROSE " + r)
for f in fell:
    print("FELL " + f)
print(f"TOTAL {total}")
print(f"HELD {held}")
PY
  ); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if grep -q '^ROSE holders:' <<<"$report"; then
    grep '^ROSE holders:' <<<"$report"
    gate_error "C6 interim f64 knot or weight holders rose — a new field keeps a copy of a carrier's f64 knots or weights past the interim read. Hold the knots (\`Knots<T>\`) or the structure, not an f64 copy"
    exit 1
  fi
  if grep -q '^ROSE ' <<<"$report"; then
    grep '^ROSE ' <<<"$report"
    gate_error "C6 interim f64 knot or weight readers rose — a new reader takes a carrier's f64 knots or weights past the funnel. Read the held values (\`knot_values\`, \`weights()\`) or the combinatorial structure (\`structure()\`), not the interim accessor"
    exit 1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "C6 interim f64 knot or weight readers or holders fell — lower $BASELINE to the new counts, so the ratchet holds them there"
    exit 1
  fi
  gate_ok "C6 interim f64 knot and weight readers at their baseline ($(sed -n 's/^TOTAL //p' <<<"$report") in all; $(sed -n 's/^HELD //p' <<<"$report") f64 knot or weight holders)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# crate count\n' > "$1/$BASELINE"
}

plant_new_caller() {
  printf 'pub fn read(c: &C) -> usize { c.knots_f64_interim().knots().len() }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_lowered_unrecorded() {
  printf 'crates/clean 2\n' >> "$1/$BASELINE"
  printf 'pub fn read(c: &C) -> usize { c.knots_u_f64_interim().len() }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_at_baseline() {
  printf 'crates/clean 1\n' >> "$1/$BASELINE"
  printf 'pub fn read(k: &K) -> usize { k.structure_f64_interim().len() }\n' \
    >> "$1/crates/clean/src/lib.rs"
  printf 'pub fn knots_f64_interim(&self) -> &V { &self.v }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_new_holder() {
  printf 'pub struct Copy(Option<KnotVector>);\n' >> "$1/crates/clean/src/lib.rs"
}

plant_new_weight_holder() {
  printf 'pub struct Held<T> {\n    pub(crate) weight_channel: Vec<f64>,\n    values: Vec<T>,\n}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

# Parameters are not fields, a test module's fixture is not a holder,
# and a geometric point is not a value channel: none is counted.
plant_not_holders() {
  printf 'pub fn fit(knots: KnotVector, weights: Vec<f64>) {}\n' >> "$1/crates/clean/src/lib.rs"
  printf 'pub struct P { point: [f64; 3], label: &'"'"'static str }\n' >> "$1/crates/clean/src/lib.rs"
  printf '#[cfg(test)]\nmod tests {\n    struct F { knots: Vec<f64> }\n}\n' >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "knot or weight holders rose" plant_new_holder
  gate_selftest_case "weight holders rose" plant_new_weight_holder
  gate_selftest_passes "parameters, a test module's fixture and a geometric point" plant_not_holders
  gate_selftest_case "readers rose" plant_new_caller
  gate_selftest_case "readers or holders fell" plant_lowered_unrecorded
  gate_selftest_passes "a caller at its baseline beside the accessor's declaration" plant_at_baseline
  printf '%s selftest OK: passes a clean fixture and one at its baseline, passes parameters, a test fixture and a geometric point, fires on a rise of readers, of knot holders or of weight holders and on an unrecorded fall\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
