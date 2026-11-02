#!/usr/bin/env bash
# in-space-composes-for-export-and-picture.sh — `in_space` is called by
# export and the viewer's picture and by nothing else in the façade's
# crates. ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE is D10 `[placement.world]`'s: export and the viewer's drawing
# compose a body into the coordinates of a space it reaches through one
# door that nothing else calls, and what they produce never re-enters the
# model. Every operation computes in a frame of its reads
# (`[placement.frame]`) and relates two operands through
# `Framed::relative`, so a read of a body composed into its space is a
# picture or an export, never an input.
#
# THE SCAN is the production code (`lib.sh`'s reader, `#[cfg(test)]`
# items and test-only mounts dropped) of `crates/{editor-core,pncad,
# pncad-py,viewer}/src`, in the statement view. Every call of `in_space`
# (`in_space(`, a turbofish `in_space::<T>(`, a path `editor_core::in_space(`)
# outside the homes below reds. Its definition (`fn in_space<…>(`), a
# re-export and a longer name (`in_space_of`) are not calls.
#
# THE HOMES, each a whole-file skip, each a composing reader:
#   pncad/src/export.rs        STEP export of a node and of the product,
#                              and the façade's read-outs (a value's
#                              bodies, a product's body, a pick mesh)
#                              that the bindings hand a host language;
#   viewer/src/pickindex.rs    the picture's parts, drawn and picked where
#                              their frames put them;
#   viewer/src/scene.rs        the product's picture;
#   viewer/src/splitplane.rs   the split tool's plane, drawn over the
#                              picture where its target is drawn.
#
# AN ALIAS HIDES A CALL. `use … in_space as x` (or a façade composer
# renamed so) makes every later call one this pattern cannot read, so an
# aliased import reds anywhere in the scan, a home included.
#
# THE FAÇADE'S COMPOSERS ARE DOORS TOO. pncad/src/export.rs's public
# composers — `body_in_space`, `product_body_in_space`,
# `pick_mesh_in_space` — compose through `in_space` for a reader outside
# the model, so they have callers of their own to hold: export itself,
# the viewer's picture (pickindex.rs, scene.rs) and pncad-py's read-outs
# (any pncad-py file: what it hands a host language never re-enters a
# document). A call anywhere else in the scan reds, the model-writing
# viewer/src/session.rs among them: the session keeps what the scene
# composes (`scene::product_in_world`) and composes nothing itself.
#
# THE BINDINGS NEVER COMPOSE. pncad-py reads a body as it stands in its
# space through pncad's read-out doors alone (the Python façade holds no
# code that is not a call to the Rust façade), so a pncad-py file is
# never a home: the gate refuses a home list naming one, and a call in
# pncad-py reds with its own diagnosis, naming the door to call.
#
# KNOWN GAPS. A home can grow a second call that feeds the model, and
# nothing here sees it: a skip is per file. `in_space` passed as a value
# (`.map(in_space)`) is no call this pattern reads. And a reader that
# composes a frame by hand inside `editor-core` is not this gate's shape:
# the frame's map and `Framed::point_in_space`/`vec_in_space`/
# `space_origin` are crate-private, so only editor-core can, and these
# readers there answer in the variable's space by design rather than
# feed an operation:
#   - the interrogation doors (`names/interrogate.rs`, `InItsSpace::
#     moved_by`, crate-private): a position or a pose asked of a
#     variable answers where it stands;
#   - `mate::corner_frame_in` reads a body's vertices composed into its
#     space and picks them by exact `==` against corners stated there,
#     so a copy under a frame whose composition rounds misses a corner
#     and refuses `NoFrame` (the fixtures that use it read
#     constructions, frame `Own`, where nothing composes);
#   - `mate::unobservable` reads a line stated in the space into each
#     body's frame, and measures reach from the space's origin;
#   - `Framed::heading_in_own` (public, the viewer's duplicate step)
#     reads a heading stated in the space INTO the body's frame: the
#     inverse direction, composing no body.
# Picking reads a ray into each target's frame (`resolve::pick_face`):
# a query composing through the frame, which answers on the ray as
# handed.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

IN_SPACE_SUBJECT='a composing reader of in_space, export or the viewer picture'
IN_SPACE_HOMES=(
  crates/pncad/src/export.rs
  crates/viewer/src/pickindex.rs
  crates/viewer/src/scene.rs
  crates/viewer/src/splitplane.rs
)
IN_SPACE_CRATES='^crates/(editor-core|pncad|pncad-py|viewer)/src/'
# The façade's composers and the files that may call them (see above).
FACADE_COMPOSERS='(^|[^A-Za-z0-9_])(body_in_space|product_body_in_space|pick_mesh_in_space)[[:space:]]*(::[[:space:]]*<[^()]*>[[:space:]]*)?\('
FACADE_CALLERS='^crates/(pncad/src/export\.rs|viewer/src/pickindex\.rs|viewer/src/scene\.rs|pncad-py/)'

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_homes "$IN_SPACE_SUBJECT" "${IN_SPACE_HOMES[@]}"
  local -a scope=()
  local f
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    if [[ "$f" =~ $IN_SPACE_CRATES ]]; then scope+=("$f"); fi
  done
  if [ "${#scope[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no production source of editor-core, pncad, pncad-py or viewer is in this tree — the gate scanned nothing, which is not a pass"
    exit 1
  fi
  for f in "${IN_SPACE_HOMES[@]}"; do
    if [[ "$f" == crates/pncad-py/* ]]; then
      gate_error "$(gate_name): $f is listed as a home, and the bindings never compose — pncad-py reads a body through pncad's read-out doors (pncad/src/export.rs)"
      exit 1
    fi
  done
  local view hits bound
  if ! view=$(gate_rust_code --skip-cfg-test --statements "${scope[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  local aliased composers
  aliased=$(printf '%s\n' "$view" \
    | gate_grep -E '(^|[^A-Za-z0-9_])(in_space|body_in_space|product_body_in_space|pick_mesh_in_space)[[:space:]]+as[[:space:]]+[A-Za-z_]')
  if [ -n "$aliased" ]; then
    echo "$aliased"
    gate_error "a composing door imported under another name — an alias hides every call of it from this gate: import in_space (and the façade's composers) by their own names"
    exit 1
  fi
  composers=$(printf '%s\n' "$view" \
    | gate_grep -E "$FACADE_COMPOSERS" \
    | gate_grep -vE '(^|[^A-Za-z0-9_])fn[[:space:]]' \
    | gate_grep -vE "$FACADE_CALLERS")
  if [ -n "$composers" ]; then
    echo "$composers"
    gate_error "a façade composer called outside export, the viewer's picture and pncad-py's read-outs — body_in_space, product_body_in_space and pick_mesh_in_space compose for a reader outside the model: compose in the picture (viewer/src/scene.rs) and keep what it answers, or relate operands through Framed::relative"
    exit 1
  fi
  hits=$(printf '%s\n' "$view" \
    | gate_grep -E '(^|[^A-Za-z0-9_])in_space[[:space:]]*(::[[:space:]]*<[^()]*>[[:space:]]*)?\(' \
    | gate_grep -vE "$(gate_record_anchor_any "${IN_SPACE_HOMES[@]}")")
  bound=$(printf '%s\n' "$hits" | gate_grep -E '^crates/pncad-py/' || true)
  if [ -n "$bound" ]; then
    echo "$bound"
    gate_error "in_space called by the bindings — pncad-py composes nothing: read a body as it stands in its space through pncad's read-out doors (pncad::export::body_in_space, product_body_in_space, pick_mesh_in_space)"
    exit 1
  fi
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "in_space called outside export and the viewer's picture — a body is composed into its space for a picture or an export alone, never as an operation's input (D10 [placement.world]): relate two operands through Framed::relative, or, for a new picture or export path, add its file to this gate's homes with its reason"
    exit 1
  fi
  gate_ok "in_space is called only by export and the viewer's picture"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  local home
  for home in "${IN_SPACE_HOMES[@]}"; do
    mkdir -p "$1/${home%/*}"
    printf 'pub fn f(b: &Framed) -> M { in_space(b) }\n' > "$1/$home"
  done
}

plant() {
  mkdir -p "$1/crates/editor-core/src"
  printf 'pub fn read(b: &Framed) -> M { crate::mate::in_space(b) }\n' \
    > "$1/crates/editor-core/src/planted.rs"
}

# The call rustfmt leaves when the argument list is long.
plant_wrapped() {
  mkdir -p "$1/crates/pncad-py/src"
  {
    printf 'pub fn read(b: &Framed) -> M {\n'
    printf '    pncad::document::in_space(\n'
    printf '        b,\n'
    printf '    )\n'
    printf '}\n'
  } > "$1/crates/pncad-py/src/planted.rs"
}

# The bindings composing a value's body themselves.
plant_bound() {
  mkdir -p "$1/crates/pncad-py/src/py"
  printf 'fn read(b: &d::Framed<f64>) -> M { pncad::document::in_space(b) }\n' \
    > "$1/crates/pncad-py/src/py/value.rs"
}

# An import that renames the door.
plant_alias() {
  mkdir -p "$1/crates/viewer/src"
  printf 'use pncad::document::in_space as frame_of;\npub fn read(b: &Framed) -> M { frame_of(b) }\n' \
    > "$1/crates/viewer/src/planted.rs"
}

# The model-writing session composing a product itself.
plant_session_composer() {
  mkdir -p "$1/crates/viewer/src"
  printf 'fn keep(p: &Product, d: &Doc) -> B { pncad::export::product_body_in_space(p, d, tol) }\n' \
    > "$1/crates/viewer/src/session.rs"
}

plant_turbofish() {
  mkdir -p "$1/crates/viewer/src"
  printf 'pub fn read(b: &Framed) -> M { in_space::<f64>(b) }\n' \
    > "$1/crates/viewer/src/planted.rs"
}

plant_not_calls() {
  mkdir -p "$1/crates/editor-core/src" "$1/crates/topo/src"
  {
    printf '//! `in_space(value)` composes a body into its space.\n'
    printf 'pub use mate::in_space;\n'
    printf "pub fn in_space<'a, T>(value: impl Into<C<'a, T>>) -> M { m() }\n"
    printf 'pub fn in_space_of(b: &Framed) -> M { m() }\n'
    printf 'pub const WHY: &str = "in_space(b)";\n'
    printf '#[cfg(test)]\n'
    printf 'mod tests {\n'
    printf '    fn t(b: &Framed) -> M { in_space(b) }\n'
    printf '}\n'
  } > "$1/crates/editor-core/src/planted.rs"
  printf 'pub fn elsewhere(b: &Framed) -> M { in_space(b) }\n' > "$1/crates/topo/src/planted.rs"
}

gate_selftest() {
  local want="in_space called outside export and the viewer's picture"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  local bound="in_space called by the bindings"
  gate_selftest_case "$bound" plant_wrapped
  gate_selftest_case "$bound" plant_bound
  gate_selftest_case "$want" plant_turbofish
  gate_selftest_case "a composing door imported under another name" plant_alias
  gate_selftest_case "a façade composer called outside export" plant_session_composer
  gate_selftest_passes "prose, a re-export, the definition, a longer name, a string, a test module and a crate outside the scope" plant_not_calls
  gate_selftest_homes --narrowed --subject "$IN_SPACE_SUBJECT" "${IN_SPACE_HOMES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying every home, and prose, a re-export, the definition, a longer name, a string, a test module and an out-of-scope crate; fires on a path call and a turbofish call outside the homes, with its own diagnosis on a call (direct or rustfmt-wrapped) in the bindings, on an aliased import, and on a façade composer called by the model-writing session; stays RED, with a diagnosis, when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
