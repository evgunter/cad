#!/usr/bin/env bash
# topology-reads-through-topo.sh — a face's loops, a face's surface and
# an edge's two ends are read through topo's doors, in every cargo root.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE. Each of these reads has one home in `topo`:
#
#  * a face's loops, in the one order every walk takes (outer, then the
#    rings): `Body::face(k)` and `FaceView::loops` (`crates/topo/src/live.rs`);
#  * a face's surface: `Body::face_surface` (`crates/topo/src/body.rs`),
#    or `FaceView::surface` beside the view's other reads;
#  * an edge's two end vertices: `Body::edge_vertices` (`body.rs`, a
#    miss answered `None`) and, inside topo, `query::edge_ends` (a miss
#    panics).
#
# Spelled by hand, each copy reads a miss its own way (a typed error, a
# skip, `?` into `None`), and the same question ends up answered several
# ways. The doors answer a stale key `None` and panic on a torn link
# (D2 row 4).
#
# THE SCAN is the statement view (`lib.sh`'s reader) of every `.rs` file
# under each cargo root's `src`, `tests`, `examples` and `benches`
# (`crates/*`, `demos/*`, `tools/*`, `benches`). Comments and string
# literals are blanked, so prose naming a shape does not red. The homes
# are skipped whole: `crates/topo/src/live.rs`, `entity.rs` and
# `body.rs`. Inside topo a face is read through `FaceView` (topo's own
# pins, `live::tests`), and these shapes are as wrong there as outside.
#
# THE SHAPES, one statement each:
#
#  1. `once(F.outer).chain(… F.rings …)`: a face's loops walked inline.
#  2. `B.get_surface(B'.get_face(K)….surface)`, and
#     `get_face(K).and_then(|f| …get_surface(f.surface))`: a face's
#     surface walked by hand.
#  3. `[E.he_plus, E.he_minus].map(…start…)`,
#     `start(E.he_plus) … start(E.he_minus)`, and
#     `get_half_edge(E.he_plus)….start … get_half_edge(E.he_minus)….start`
#     (an array, a tuple or a comparison): an edge's two ends spelled
#     inline, the last two off the twin.
#  4. Across statements, by the shape rather than the name, over each
#     file's code joined across lines: a half-edge H that is an edge's
#     `he_plus` read as `get_half_edge(H)` with `.start` within 80
#     characters after it (across lines) and as `half_edge_end(H)`. H is
#     `X.he_plus` or `X.plus.half_edge` at the read itself (its start
#     within 8 lines before or 40 after), or a name bound to one: a
#     `let H = … .he_plus;` read to its `;` across lines (rustfmt's
#     wrapped `body\n.get_edge(k)…\n.he_plus;`), or a destructured
#     `let Edge { he_plus, .. } = …` (`he_plus: H` too), each read over
#     the 40 lines after it. And an edge's twin starts,
#     `get_half_edge(E.he_plus)….start` and `get_half_edge(E.he_minus)
#     ….start`, in either order within 12 lines of each other. A half-edge's
#     own start and end are not this shape unless it is an edge's
#     `he_plus`: a loop walk's `he` reads its own ends. (Matching any
#     half-edge's start and end found 99 sites at this shape's landing,
#     most of them that walk; PR 6710.) EXEMPT (`HALF_READS`):
#     the Euler rows that assert each half-edge of a fresh edge on its
#     own, which is their subject.
#
# KNOWN GAPS. The matcher reads text, not types. It cannot see:
#  - a face record bound in one statement whose `.surface` key is
#    resolved in another (`let f = body.get_face(k)?; … get_surface(f.surface)`),
#    or bound by a `faces()` iteration (`for (k, f) in body.faces()`,
#    `.find(|(_, f)| … get_surface(f.surface))`). By text,
#    `get_surface(x.surface)` is also an edge chart's surface key
#    (`EdgeDescription::Chart`'s `c.surface`), which is not a face walk.
#    At this gate's landing the one such face read outside topo is
#    `editor-core` `mate::reach::reach_from`, held for its public
#    `MalformedBody` refusal (ENCL's
#    `a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids`);
#  - the outer loop and the rings read in separate statements;
#  - shape 4's half-edge reaching the reads other than by a `let` or a
#    destructuring of `he_plus` (a parameter, a struct field, a closure
#    argument, a `match` arm), or its reads further than 40 lines from
#    the binding; twin starts more than 12 lines apart; a `.start` more
#    than 80 characters after its `get_half_edge(..)`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOMES=(
  crates/topo/src/live.rs
  crates/topo/src/entity.rs
  crates/topo/src/body.rs
)

LOOPS='once\(\s*&?\s*\(?\s*(?!self\b)([A-Za-z_][\w.()]*)\.outer\b.*\.chain\(.*\b\1\.rings\b'
SURFACE_NESTED='get_surface\(\s*[\w.()&*]*\.get_face\('
SURFACE_AND_THEN='get_face\(.*\)\s*\.and_then\(\s*\|\s*(\w+)\s*\|.*get_surface\(\s*\1\.surface\b'
ENDS_ARRAY='\[\s*(\w+(?:\.\w+)*)\.he_plus\s*,\s*\1\.he_minus\s*\]\s*\.map\(.*(\.start\b|half_edge_start_point)'
ENDS_STARTS='start\(\s*(\w+(?:\.\w+)*)\.he_plus\s*\).*start\(\s*\1\.he_minus\s*\)'
ENDS_HALVES='get_half_edge\(\s*(\w+(?:\.\w+)*)\.he_plus\s*\)[^;]*\.start\b[^;]*get_half_edge\(\s*\1\.he_minus\s*\)[^;]*\.start\b'

# Shape 4 (header), over the line view on stdin (`FILE:LINE:CODE`); prints
# one `FILE:LINE: …` record per site.
EDGE_ENDS_SHAPE='
import re, sys, collections, bisect
HALF_READS = {
    ("crates/topo/src/euler.rs", "lone_mev_grows_a_segment"),
    ("crates/topo/src/review_m1_pr2/degenerates_and_sequences.rs", "fan_mev_across_a_circular_edge"),
}
WINDOW = 40
lines = collections.defaultdict(list)
for l in sys.stdin:
    m = re.match(r"^(.*?\.rs):(\d+):(.*)$", l.rstrip("\n"))
    if m:
        lines[m.group(1)].append((int(m.group(2)), m.group(3)))
# A half-edge expression read for its start: `get_half_edge(H)` and,
# within the next 80 characters (across lines), `.start`.
def starts(text, h):
    for m in re.finditer(r"get_half_edge\(\s*&?" + re.escape(h) + r"\s*\)", text):
        if re.search(r"\.start\b", text[m.end():m.end() + 80]):
            yield m.start()
def ends(text, h):
    for m in re.finditer(r"half_edge_end\(\s*&?" + re.escape(h) + r"\s*\)", text):
        yield m.start()
# Bindings of an edge`s he_plus, read to the statement`s `;` across lines:
# `let H = … .he_plus;` (or `.plus.half_edge`), and a destructured
# `let Edge { he_plus, .. } = …` (or `he_plus: H`).
BIND = re.compile(r"\blet\s+(?:mut\s+)?(\w+)\s*(?::[^=;]*)?=\s*[^;]*?(?:\.he_plus|\bplus\.half_edge)\s*;")
DESTRUCT = re.compile(r"\blet\s+(?:[\w:]+\s*)?\{[^}]*?\bhe_plus\b(?:\s*:\s*(\w+))?[^}]*\}\s*=")
DIRECT = re.compile(r"half_edge_end\(\s*&?([A-Za-z_][\w.]*\.(?:he_plus|plus\.half_edge))\s*\)")
TWIN = re.compile(r"get_half_edge\(\s*&?(\w+(?:\.\w+)*)\.he_(plus|minus)\s*\)")
out = set()
for f, ls in lines.items():
    text = "\n".join(t for _, t in ls)
    offs, o = [], 0
    for _, t in ls:
        offs.append(o)
        o += len(t) + 1
    line_of = lambda pos: ls[bisect.bisect_right(offs, pos) - 1][0]
    idx_of = lambda pos: bisect.bisect_right(offs, pos) - 1
    names, name = [], "-"
    for _, t in ls:
        m = re.search(r"\bfn\s+(\w+)", t)
        if m:
            name = m.group(1)
        names.append(name)
    exempt = lambda pos: (f, names[idx_of(pos)]) in HALF_READS
    def window(pos):
        i = idx_of(pos)
        lo = offs[max(0, i - 8)]
        hi = offs[min(len(offs) - 1, i + WINDOW)] + len(ls[min(len(offs) - 1, i + WINDOW)][1])
        return lo, hi
    def record(pos, what):
        if not exempt(pos):
            out.add(f"{f}:{line_of(pos)}: {what}: {ls[idx_of(pos)][1].strip()}")
    # A binding, then its start and its end within the window after it.
    for m in BIND.finditer(text):
        h, lo, hi = m.group(1), m.end(), window(m.start())[1]
        seg = text[lo:hi]
        e = next(ends(seg, h), None)
        if e is not None and next(starts(seg, h), None) is not None:
            record(lo + e, "an edge end read through a local bound to its he_plus")
    for m in DESTRUCT.finditer(text):
        h = m.group(1) or "he_plus"
        lo, hi = m.end(), window(m.start())[1]
        seg = text[lo:hi]
        e = next(ends(seg, h), None)
        if e is not None and next(starts(seg, h), None) is not None:
            record(lo + e, "an edge end read through a destructured he_plus")
    # The edge`s he_plus spelled at the read itself.
    for m in DIRECT.finditer(text):
        lo, hi = window(m.start())
        if next(starts(text[lo:hi], m.group(1)), None) is not None:
            record(m.start(), "an edge end read through its he_plus beside its start")
    # Twin starts in either order, within 12 lines of each other.
    reads = collections.defaultdict(list)
    for m in TWIN.finditer(text):
        if re.search(r"\.start\b", text[m.end():m.end() + 80]):
            reads[m.group(1)].append((idx_of(m.start()), m.group(2), m.start()))
    for e, rs in reads.items():
        for i, side, pos in rs:
            if side == "plus" and any(s2 == "minus" and abs(j - i) <= 12 for j, s2, _ in rs):
                record(pos, "an edge end read off the twin")
for o in sorted(out):
    print(o)
'

gate_files() {
  local roots=()
  local d
  for d in crates/*/src crates/*/tests crates/*/examples crates/*/benches \
           demos/*/src demos/*/tests demos/*/examples tools/*/src tools/*/tests benches; do
    [ -d "$d" ] && roots+=("$d")
  done
  [ ${#roots[@]} -gt 0 ] || return 0
  find "${roots[@]}" -type f -name '*.rs' | sort
}

gate() {
  gate_require_crate_sources
  local files=() f
  while IFS= read -r f; do
    local skip=0 h
    for h in "${HOMES[@]}"; do [ "$f" = "$h" ] && skip=1; done
    [ $skip = 1 ] || files+=("$f")
  done < <(gate_files)
  GATE_SCAN_FILES=${#files[@]}
  local view hits
  if ! view=$(gate_rust_code --statements "${files[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  local needle all=''
  for needle in "$LOOPS" "$SURFACE_NESTED" "$SURFACE_AND_THEN" "$ENDS_ARRAY" "$ENDS_STARTS" "$ENDS_HALVES"; do
    hits=$(printf '%s\n' "$view" | gate_grep -P "$needle" || true)
    all=$(printf '%s\n%s\n' "$all" "$hits")
  done
  local lines_view shaped
  if ! lines_view=$(gate_rust_code "${files[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the line view — that is not a pass"
    exit 1
  fi
  if ! shaped=$(printf '%s\n' "$lines_view" | python3 -c "$EDGE_ENDS_SHAPE"); then
    gate_error "$(gate_name): the edge-ends shape reader failed — that is not a pass"
    exit 1
  fi
  all=$(printf '%s\n%s\n' "$all" "$shaped")
  hits=$(printf '%s\n' "$all" | sed '/^$/d' | sort -u)
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "a topology read spelled beside topo's door: walk a face's loops through Body::face(k) and FaceView::loops, read its surface through Body::face_surface, and an edge's two ends through Body::edge_vertices (query::edge_ends inside topo)"
    exit 1
  fi
  gate_ok "a face's loops, a face's surface and an edge's two ends are read through topo's doors in every root"
}

plant_loops() {
  mkdir -p "$1/crates/planted/tests"
  printf 'fn w(b: &B, f: &Face) { for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) { b.get_loop(lk); } }\n' \
    > "$1/crates/planted/tests/a.rs"
}

plant_loops_wrapped() {
  mkdir -p "$1/demos/planted/src"
  {
    printf 'fn w(b: &B, face: &Face) -> Vec<L> {\n'
    printf '    core::iter::once(face.outer)\n'
    printf '        .chain(face.rings.iter().copied())\n'
    printf '        .collect()\n'
    printf '}\n'
  } > "$1/demos/planted/src/lib.rs"
}

plant_surface_nested() {
  mkdir -p "$1/crates/planted/src"
  printf 'fn s(body: &B, k: K) -> Option<&S> { body.get_surface(body.get_face(k)?.surface) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_surface_and_then() {
  mkdir -p "$1/tools/planted/src"
  {
    printf 'fn s(body: &B, k: K) -> Option<&S> {\n'
    printf '    body.get_face(k)\n'
    printf '        .and_then(|f| body.get_surface(f.surface))\n'
    printf '}\n'
  } > "$1/tools/planted/src/lib.rs"
}

plant_ends_array() {
  mkdir -p "$1/crates/planted/tests"
  printf 'fn e(body: &B, e: &Edge) -> [V; 2] { [e.he_plus, e.he_minus].map(|h| body.get_half_edge(h).unwrap().start) }\n' \
    > "$1/crates/planted/tests/a.rs"
}

plant_ends_halves() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'fn e(body: &B, d: &Edge) -> Option<[V; 2]> {\n'
    printf '    Some([\n'
    printf '        body.get_half_edge(d.he_plus)?.start,\n'
    printf '        body.get_half_edge(d.he_minus)?.start,\n'
    printf '    ])\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

plant_ends_in_topo() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn e(edge: &Edge) -> (V, V) { (start(edge.he_plus), start(edge.he_minus)) }\n' \
    > "$1/crates/topo/src/lineage.rs"
}

plant_ends_local() {
  mkdir -p "$1/crates/planted/tests"
  {
    printf 'fn e(body: &B, k: K) -> [V; 2] {\n'
    printf '    let he = body.get_edge(k).unwrap().he_plus;\n'
    printf '    [\n'
    printf '        body.get_half_edge(he).unwrap().start,\n'
    printf '        body.half_edge_end(he).unwrap(),\n'
    printf '    ]\n'
    printf '}\n'
  } > "$1/crates/planted/tests/a.rs"
}

plant_ends_split_twin() {
  mkdir -p "$1/crates/planted/tests"
  {
    printf 'fn e(body: &B, e: &Edge) -> bool {\n'
    printf '    let s1 = body.get_half_edge(e.he_plus).unwrap().start;\n'
    printf '    let s2 = body.get_half_edge(e.he_minus).unwrap().start;\n'
    printf '    s1 == s2\n'
    printf '}\n'
  } > "$1/crates/planted/tests/a.rs"
}

plant_ends_local_wrapped() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'fn e(body: &B, k: K) -> Option<[V; 2]> {\n'
    printf '    let he = body\n'
    printf '        .get_edge(k)\n'
    printf '        .unwrap()\n'
    printf '        .he_plus;\n'
    printf '    body.get_half_edge(he)\n'
    printf '        .map(|h| h.start)\n'
    printf '        .zip(body.half_edge_end(he))\n'
    printf '        .map(|(s, t)| [s, t])\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

plant_ends_destructured() {
  mkdir -p "$1/crates/planted/tests"
  {
    printf 'fn e(body: &B, k: K) -> (V, V) {\n'
    printf '    let Edge { he_plus, .. } = *body.get_edge(k).unwrap();\n'
    printf '    (body.get_half_edge(he_plus).unwrap().start, body.half_edge_end(he_plus).unwrap())\n'
    printf '}\n'
  } > "$1/crates/planted/tests/a.rs"
}

plant_ends_twins_reversed_and_apart() {
  mkdir -p "$1/crates/planted/tests"
  {
    printf 'fn e(body: &B, e: &Edge) -> bool {\n'
    printf '    let s2 = body.get_half_edge(e.he_minus).unwrap().start;\n'
    printf '    let a = 1;\n'
    printf '    let b = 2;\n'
    printf '    let c = 3;\n'
    printf '    let d = 4;\n'
    printf '    let s1 = body.get_half_edge(e.he_plus).unwrap().start;\n'
    printf '    s1 == s2 && a + b + c + d > 0\n'
    printf '}\n'
  } > "$1/crates/planted/tests/a.rs"
}

# A loop walk reading a half-edge's own two ends is not an edge's ends.
plant_half_edge_own_ends() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'fn w(body: &B, he: H) -> (V, V) {\n'
    printf '    let a = body.get_half_edge(he).unwrap().start;\n'
    printf '    let b = body.half_edge_end(he).unwrap();\n'
    printf '    (a, b)\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# THE NEAR MISSES: the doors themselves, prose and string literals naming
# the shapes, a surface key read off an edge's chart (not a face's), the
# halves read for their faces, and a home spelling its own walk.
plant_near_misses() {
  mkdir -p "$1/crates/planted/src" "$1/crates/topo/src"
  {
    printf '//! Never once(f.outer).chain(f.rings) here, nor body.get_surface(body.get_face(k)?.surface).\n'
    printf 'pub const WHY: &str = "[e.he_plus, e.he_minus].map(|h| h.start)";\n'
    printf 'fn a(body: &B, k: K) { for (lk, _, _) in body.face(k).unwrap().loops() {} }\n'
    printf 'fn b(body: &B, k: K) -> Option<&S> { body.face_surface(k) }\n'
    printf 'fn c(body: &B, c: &Chart) -> Option<&S> { body.get_surface(c.surface) }\n'
    printf 'fn d(body: &B, k: E) -> Option<(V, V)> { body.edge_vertices(k) }\n'
    printf 'fn e(body: &B, e: &Edge) -> Vec<F> { [e.he_plus, e.he_minus].into_iter().filter_map(|h| body.face_of_half_edge(h)).collect() }\n'
  } > "$1/crates/planted/src/lib.rs"
  printf 'fn w(f: &Face) { for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {} }\n' \
    > "$1/crates/topo/src/live.rs"
  printf 'fn e(e: &Edge) -> Option<(V, V)> { Some((start(e.he_plus)?, start(e.he_minus)?)) }\n' \
    > "$1/crates/topo/src/body.rs"
}

gate_selftest() {
  local want="a topology read spelled beside topo's door"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_loops
  gate_selftest_case "$want" plant_loops_wrapped
  gate_selftest_case "$want" plant_surface_nested
  gate_selftest_case "$want" plant_surface_and_then
  gate_selftest_case "$want" plant_ends_array
  gate_selftest_case "$want" plant_ends_in_topo
  gate_selftest_case "$want" plant_ends_halves
  gate_selftest_case "$want" plant_ends_local
  gate_selftest_case "$want" plant_ends_split_twin
  gate_selftest_case "$want" plant_ends_local_wrapped
  gate_selftest_case "$want" plant_ends_destructured
  gate_selftest_case "$want" plant_ends_twins_reversed_and_apart
  gate_selftest_passes "a half-edge's own two ends in a loop walk" plant_half_edge_own_ends
  gate_selftest_passes "the doors, prose, string literals, a chart's surface key, the halves read for their faces, and the homes' own walks" plant_near_misses
  printf '%s selftest OK: passes a clean fixture and the near misses; fires on an inline loop walk (one line, and rustfmt-wrapped in a demo root), a nested and an and_then face-to-surface walk, an inline edge-ends array in a test root, a closure spelling inside topo and a rustfmt-wrapped twin-starts array; stays RED, with a diagnosis, when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
