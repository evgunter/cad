#!/usr/bin/env bash
# engine-leaves.sh — `topo`'s operation directories are leaves, pinned
# per file. ONE home; ci.yml's `lint` job runs every gate in this
# directory.
#
# THE RULE is `[topo.homes]` in crates/topo/README.md: `boolean/` and
# `splitting/` are leaves — nothing outside an engine names it, neither
# names the other, and an engine declares nothing crate-wide that is not
# a `pub` door.
#
# So the sites that break it today are PINNED per (kind, file) in
# `engine-leaves.counts`, and the pin only falls: a new site reds, and so
# does a pinned one that is gone, so the change that clears a site lowers
# its line in the same diff. The file is the move list. A pin above the
# merge base's reds too (`gate_base_file`, `GATE_BASE`), unless it is a
# reader's `names-*` pin whose line cites the `work/` row that moves the
# site away; `pin-cites-an-open-row.sh` holds that row open. An
# `ambiguous NAME` line is held the same way: one absent from the base
# reds unless its line cites the row that retires the namesake, since
# listing a name takes every bare use of it out of the count.
#
# WHAT IT COUNTS, in every file of crates/topo/src, test code included
# (comments and literals stripped, `gate_rust_code`): a path through an
# engine's module, and every occurrence of an item the engine DECLARES,
# by name, however it reached scope — so a hub's re-export, `use
# super::X`, a glob and the root's own `pub use` are all the same site.
# By name is sound only while every item is spelled by its own name, so a
# second name for one (`use … as Q`, `use crate as C`, `type Q = …`, a
# macro spelling the engine) is refused outright. A name also declared
# outside the engines cannot be told apart by name: it is listed as
# `ambiguous` in the counts file. The counter, and what it can and cannot
# see, is `engine_leaves.py`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
CENSUS="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/engine_leaves.py"

BASELINE=scripts/gates/engine-leaves.counts
MANIFEST=crates/topo/Cargo.toml
RULE='`[topo.homes]` (crates/topo/README.md): a red here means that what you need belongs at the root or below. The item named is a reading the engine hosts, so move it out of the engine to the crate root (to geom-brep if it takes no body); do not move the reader into the engine'

gate() {
  gate_require_file "$BASELINE"
  gate_require_file "$MANIFEST"
  local files report base_note="no base given (GATE_BASE unset), so no pin was compared with one"
  local base_args=()
  if gate_base_file "$BASELINE"; then
    base_args=(--base "$GATE_BASE_FILE")
    base_note="none above the base $GATE_BASE's"
  fi
  mapfile -t files < <(find crates/topo/src -type f -name '*.rs' 2>/dev/null | sort)
  GATE_SCAN_FILES=${#files[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no .rs file under crates/topo/src in $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
  if ! report=$(gate_rust_code "${files[@]}" | GATE_ROW_CITATION_ERE="$GATE_ROW_CITATION_ERE" python3 "$CENSUS" "$BASELINE" "$MANIFEST" ${LINES_FOR:+--lines "$LINES_FOR"} ${base_args[@]+"${base_args[@]}"}); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if ! grep -q '^TOTAL [0-9]' <<<"$report"; then
    gate_error "$(gate_name): the counter exited without its TOTAL line — it did not finish its census, which is not a pass"
    exit 1
  fi
  grep '^LINES ' <<<"$report" || true
  local failed=0
  if grep -q '^MALFORMED ' <<<"$report"; then
    grep '^MALFORMED ' <<<"$report"
    gate_error "$BASELINE has a line it cannot read (above)"
    failed=1
  fi
  if grep -qE '^ROSE (names|boolean-names|splitting-names)' <<<"$report"; then
    grep -E '^ROSE (names|boolean-names|splitting-names)' <<<"$report"
    gate_error "an engine is named past its pin (above) — $RULE. Move the code; do not raise the count"
    failed=1
  fi
  if grep -q '^ROSE refused-binding' <<<"$report"; then
    grep '^ROSE refused-binding' <<<"$report"
    gate_error "a second name is bound to an engine item or module, or to this crate (above: \`use … as Q\`, \`use crate as C\` or \`use topo as C\`, \`extern crate self\`, \`::topo::\`, \`type Q = …\` or a macro spelling the engine at any depth, \`#[macro_use]\` on or inside an engine, or a lib.rs re-export of an engine module). This gate counts an engine item by its own name, so a binding that renames it is refused outright: spell the item by its name"
    failed=1
  fi
  if grep -q '^ROSE crate-visible' <<<"$report"; then
    grep '^ROSE crate-visible' <<<"$report"
    gate_error "an engine declares an item crate-wide (above). An engine's \`pub\` items are its doors and it declares nothing \`pub(crate)\` (\`[topo.homes]\`): make it a \`pub\` door, or private to the engine — \`pub(in crate::<engine>)\` or narrower (in an engine's mod.rs, \`pub(super)\` is crate-wide)"
    failed=1
  fi
  if grep -q '^RAISED ' <<<"$report"; then
    grep '^RAISED ' <<<"$report"
    gate_error "a pin is above the merge base's (above), and a pin only falls. $RULE. A reader's \`names-*\` pin rises only for a site a move row retires, and an \`ambiguous NAME\` line is new only for a namesake a row retires: cite that row on the line (\`# work/<program>/<id>\`). Never pay for one with a new \`pub(crate)\` in the engine, and prefer renaming the namesake to listing it"
    failed=1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "a pinned site is gone (above): drop or lower its line in $BASELINE"
    failed=1
  fi
  if grep -qE '^AMBIGUOUS-(NEW|GONE) ' <<<"$report"; then
    grep -E '^AMBIGUOUS-(NEW|GONE) ' <<<"$report"
    gate_error "the names an engine shares with code outside it moved (above): add or drop their \`ambiguous NAME\` lines in $BASELINE (a new one cites the row that retires the namesake, or reds against the base) — an ambiguous name is counted only where a path or an import through the engine spells it"
    failed=1
  fi
  [ "$failed" = 0 ] || exit 1
  if grep -q '^CITED ' <<<"$report"; then
    grep '^CITED ' <<<"$report"
    base_note="no uncited pin above the base $GATE_BASE's; $(grep -c '^CITED ' <<<"$report") cited rise(s), listed above"
  fi
  gate_ok "the engines' doors at their pinned counts ($(sed -n 's/^TOTAL //p' <<<"$report") sites in all); pins: $base_note"
}

# THE FIXTURE: a root naming one engine item through a full path, pinned,
# beside near misses every case below must keep quiet.
gate_plant_clean() {
  local t="$1/crates/topo"
  mkdir -p "$t/src/boolean" "$t/src/splitting" "$1/scripts/gates"
  printf '[package]\nname = "topo"\n[dependencies]\ngeom-brep = { path = "." }\n' > "$t/Cargo.toml"
  printf 'pub mod boolean;\npub mod splitting;\nmod reader;\nmod other;\npub use boolean::{Op, union, Shared};\n' > "$t/src/lib.rs"
  {
    printf 'pub enum Op { A }\npub fn union() {}\npub(super) fn wide() {}\nmod zip;\npub mod gear;\n'
    printf 'pub struct Shared;\nmacro_rules! private_rule { () => {} }\n#[macro_export]\nmacro_rules! mk { () => {} }\n'
  } > "$t/src/boolean/mod.rs"
  printf 'pub(super) fn run() {}\npub(in crate::boolean) struct Inner;\n' > "$t/src/boolean/zip.rs"
  printf 'pub fn turn() {}\n' > "$t/src/boolean/gear.rs"
  printf 'pub struct Plane;\npub fn cut() {}\n' > "$t/src/splitting/mod.rs"
  {
    printf 'pub fn read() { let _ = crate::boolean::Op::A; }\npub struct Side;\n'
    printf 'fn local(union: u8) -> u8 { union }\n'
    printf 'fn near(x: &S) { x.union(); geom_brep::union(); private_rule!(); }\n'
    printf 'fn generic<T: Tr>() { T::union(); Self::union(); }\n'
    printf 'use geom_brep::Shared as GShared; fn ext() -> GShared { todo!() }\n'
  } > "$t/src/reader.rs"
  printf 'pub struct Shared;\nuse geom_brep::Op;\nfn bare() -> Op { todo!() }\n' > "$t/src/other.rs"
  printf '# kind count file\ncrate-visible 1 boolean/mod.rs\nnames-boolean 1 reader.rs\nambiguous Shared\n' > "$1/$BASELINE"
}

append() { printf '%s\n' "$2" >> "$1/crates/topo/src/$3"; }
pin() { printf '%s\n' "$2" >> "$1/$BASELINE"; }

# Each mechanism the count relies on, pinned at the count it gives: a
# broken mechanism moves the count, and the case reds.
plant_glob_through_engine() { append "$1" 'mod g { use crate::boolean::*; fn a() { union(); let _: Shared; } }' other.rs; pin "$1" 'names-boolean 3 other.rs'; }
plant_super_glob() {
  append "$1" 'mod p { use crate::boolean::Shared; mod c { use super::*; fn a(_: Shared) {} } }' other.rs
  pin "$1" 'names-boolean 2 other.rs'
}
plant_module_through() {
  append "$1" 'mod t { use crate::boolean::gear; use gear::Shared; fn a() { gear::turn(); let _: Shared; } }' other.rs
  pin "$1" 'names-boolean 4 other.rs'
}
plant_root_reexport_ambiguous() { append "$1" 'fn a(_: crate::Shared) {}' other.rs; pin "$1" 'names-boolean 1 other.rs'; }
plant_fn_triggers() { append "$1" 'fn a() { union::<u8>(); mk!(); crate::union; }' other.rs; pin "$1" 'names-boolean 3 other.rs'; }
plant_capital_head() { append "$1" 'fn a() { Unknown::union(); }' other.rs; pin "$1" 'names-boolean 1 other.rs'; }

plant_root_path_long()   { append "$1" 'use crate::Shared::{A, B}; fn a() { let _ = crate::Shared::A; }' other.rs; pin "$1" 'names-boolean 3 other.rs'; }
plant_super_to_root()    {
  append "$1" 'use super::Shared::X; fn a(_: super::Shared) {} mod m { fn b(_: super::super::Shared) {} }' other.rs
  pin "$1" 'names-boolean 3 other.rs'
}
plant_topo_path()        { append "$1" 'fn a(_: topo::Shared, _: topo::Op) {}' other.rs; pin "$1" 'names-boolean 2 other.rs'; }
plant_local_const()      { append "$1" 'fn a() { const X: crate::Op = crate::Op::A; }' other.rs; pin "$1" 'names-boolean 2 other.rs'; }
plant_assoc_type()       { append "$1" 'impl Tr for S { type Out = crate::Op; }' other.rs; pin "$1" 'names-boolean 1 other.rs'; }
plant_alias_of_crate()   { append "$1" 'fn a() { crate::boolean::union(); }' reader.rs; }
plant_nested_hub()       { append "$1" 'mod hub { pub use crate::boolean::Op; }' lib.rs; }
plant_super_import()     { append "$1" 'use super::Op;' reader.rs; }
plant_root_glob()        { append "$1" 'use crate::*; fn a() { union(); }' reader.rs; }
plant_root_reexport()    { append "$1" 'fn a() -> crate::Op { crate::Op::A }' reader.rs; }
plant_cross_engine()     { append "$1" 'use crate::boolean::Op;' splitting/mod.rs; }
plant_cross_by_name()    { append "$1" 'fn a() { crate::cut(); }' boolean/zip.rs; }
plant_crate_visible()    { append "$1" 'pub(crate) fn leak() {}' boolean/zip.rs; }
plant_super_from_mod()   { append "$1" 'pub(super) fn leak() {}' splitting/mod.rs; }
plant_site_gone()        { sed -i 's/crate::boolean::Op::A/0/' "$1/crates/topo/src/reader.rs"; }
plant_zero_line()        { pin "$1" 'names-splitting 0 reader.rs'; }
plant_unknown_kind()     { pin "$1" 'names-everything 1 reader.rs'; }
plant_duplicate_line()   { pin "$1" 'names-boolean 1 reader.rs'; }
plant_ambiguous()        { append "$1" 'pub struct Plane;' reader.rs; }
plant_ambiguous_gone()   { pin "$1" 'ambiguous Op'; }
plant_mod_boolean()      { append "$1" 'mod boolean {}' reader.rs; }

plant_use_crate_as()     { append "$1" 'use crate as c;' other.rs; }
plant_use_crate_self_as() { append "$1" 'use crate::{self as r};' other.rs; }
plant_super_self_as()    { append "$1" 'use super::{self as s};' other.rs; }
plant_use_topo_as()      { append "$1" 'use topo as t;' other.rs; }
plant_macro_use_engine() { sed -i 's/^pub mod boolean;/#[macro_use]\npub mod boolean;/' "$1/crates/topo/src/lib.rs"; }
plant_macro_use_inside() { append "$1" '#[macro_use] mod inner {}' boolean/zip.rs; }
plant_local_type_alias() { append "$1" 'fn a() { type Q = crate::Op; }' other.rs; pin "$1" 'names-boolean 1 other.rs'; }
plant_local_macro()      { append "$1" 'fn a() { macro_rules! m { () => { crate::Op::A } } }' other.rs; pin "$1" 'names-boolean 1 other.rs'; }
plant_lib_module_reexport() { append "$1" 'pub(crate) use boolean::gear;' lib.rs; }
plant_extern_self()      { append "$1" 'extern crate self as topo;' other.rs; }
plant_leading_colons()   { append "$1" 'fn a() { ::topo::boolean::union(); }' other.rs; pin "$1" 'names-boolean 1 other.rs'; }
plant_rename_item()      { append "$1" 'use crate::boolean::Op as Q;' other.rs; pin "$1" 'names-boolean 1 other.rs'; }
plant_rename_module()    { append "$1" 'use crate::boolean as b;' other.rs; pin "$1" 'names-boolean 1 other.rs'; }
plant_rename_root()      { append "$1" 'use crate::Shared as S;' other.rs; pin "$1" 'names-boolean 1 other.rs'; }
plant_rename_in_lib()    { sed -i 's/pub use boolean::{Op, union, Shared};/pub use boolean::{Op as Operation, union, Shared};/' "$1/crates/topo/src/lib.rs"; }
plant_type_alias()       { append "$1" 'type Q = crate::boolean::Op;' other.rs; pin "$1" 'names-boolean 1 other.rs'; }
plant_macro_path()       { append "$1" 'macro_rules! op { () => { $crate::boolean::Op::A } }' other.rs; pin "$1" 'names-boolean 1 other.rs'; }

# THE BASE: the fixture as it stands, recorded as a tree at
# `refs/gate-base` in a repository planted over it, which the cases below
# name as `GATE_BASE`.
plant_base() {
  git -C "$1" init -q
  git -C "$1" add -A
  git -C "$1" update-ref refs/gate-base "$(git -C "$1" write-tree)"
}
add_site() { append "$1" 'fn more() { crate::boolean::union(); }' reader.rs; }
set_pin() { sed -i "s/^names-boolean [0-9]* reader.rs.*/names-boolean $2 reader.rs${3:-}/" "$1/$BASELINE"; }
plant_raised()       { plant_base "$1"; add_site "$1"; set_pin "$1" 2; }
plant_raised_cited() { plant_base "$1"; add_site "$1"; set_pin "$1" 2 '  # work\/onecopy\/move-row'; }
plant_raised_other() {
  plant_base "$1"
  append "$1" 'pub(super) fn wider() {}' boolean/mod.rs
  sed -i 's/^crate-visible 1 boolean\/mod.rs/crate-visible 2 boolean\/mod.rs  # work\/onecopy\/move-row/' "$1/$BASELINE"
}
plant_lowered()      { add_site "$1"; set_pin "$1" 2; plant_base "$1"; sed -i '/fn more()/d' "$1/crates/topo/src/reader.rs"; set_pin "$1" 1; }
plant_ambiguous_new()   { plant_base "$1"; append "$1" 'pub struct Plane;' reader.rs; pin "$1" 'ambiguous Plane'; }
plant_ambiguous_cited() { plant_base "$1"; append "$1" 'pub struct Plane;' reader.rs; pin "$1" 'ambiguous Plane  # work/onecopy/rename-row'; }
plant_new_line()     { plant_base "$1"; append "$1" 'fn a() { crate::boolean::union(); }' other.rs; pin "$1" 'names-boolean 1 other.rs'; }
plant_repo_without_base() { git -C "$1" init -q; }
plant_base_without_counts() { plant_base "$1"; git -C "$1" rm -q --cached "$BASELINE"; git -C "$1" update-ref refs/gate-base "$(git -C "$1" write-tree)"; }

gate_selftest() {
  unset GATE_BASE
  gate_selftest_clean
  gate_selftest_without_tool python3 "the counter failed"
  gate_selftest_with_broken_tool python3 "without its TOTAL line" '"$GATE_REAL_TOOL" "$@" | sed "/^TOTAL/d"'
  local p
  for p in plant_glob_through_engine plant_super_glob plant_module_through plant_root_reexport_ambiguous \
      plant_fn_triggers plant_capital_head plant_root_path_long plant_super_to_root plant_topo_path \
      plant_local_const plant_assoc_type; do
    gate_selftest_passes "$p at its pin" "$p"
  done
  for p in plant_alias_of_crate plant_nested_hub plant_super_import plant_root_glob plant_root_reexport; do
    gate_selftest_case --also "ROSE names-boolean" "Move the code" "$p"
  done
  gate_selftest_case --also "ROSE splitting-names-boolean" "Move the code" plant_cross_engine
  gate_selftest_case --also "ROSE boolean-names-splitting" "Move the code" plant_cross_by_name
  gate_selftest_case --also "ROSE crate-visible boolean/zip.rs" "private to the engine" plant_crate_visible
  gate_selftest_case --also "ROSE crate-visible splitting/mod.rs" "private to the engine" plant_super_from_mod
  gate_selftest_case --also "FELL names-boolean reader.rs: 1 -> 0" "drop or lower" plant_site_gone
  gate_selftest_case "cannot read" plant_zero_line
  gate_selftest_case "cannot read" plant_unknown_kind
  gate_selftest_case "cannot read" plant_duplicate_line
  gate_selftest_case --also "AMBIGUOUS-NEW Plane" "ambiguous NAME" plant_ambiguous
  gate_selftest_case --also "AMBIGUOUS-GONE Op" "ambiguous NAME" plant_ambiguous_gone
  gate_selftest_case "declares a module named boolean" plant_mod_boolean
  GATE_BASE=refs/gate-base gate_selftest_passes "the fixture against itself as the base" plant_base
  GATE_BASE=refs/gate-base gate_selftest_passes "a pin lowered against the base" plant_lowered
  GATE_BASE=refs/gate-base gate_selftest_passes "a reader's pin raised on a line citing its move row" plant_raised_cited
  GATE_BASE=refs/gate-base gate_selftest_case --also "RAISED names-boolean reader.rs: 1 -> 2" "only falls" plant_raised
  # A cited rise passes, and its green log names it.
  local tmp out
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  plant_raised_cited "$tmp"
  out=$(GATE_BASE=refs/gate-base "$0" --root "$tmp" 2>&1) || { printf 'SELFTEST FAILED: a cited rise reds:\n%s\n' "$out" >&2; rm -rf "$tmp"; exit 1; }
  rm -rf "$tmp"
  case "$out" in
    *"CITED names-boolean reader.rs: 1 -> 2 (work/onecopy/move-row)"*"1 cited rise(s), listed above"*) ;;
    *) printf 'SELFTEST FAILED: a cited rise passed without naming itself in the green log:\n%s\n' "$out" >&2; exit 1 ;;
  esac
  GATE_BASE=refs/gate-base gate_selftest_case --also "RAISED names-boolean other.rs: 0 -> 1" "cite that row" plant_new_line
  GATE_BASE=refs/gate-base gate_selftest_case --also "RAISED ambiguous Plane" "cite that row" plant_ambiguous_new
  # A cited ambiguous line passes, and its green log names it.
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  plant_ambiguous_cited "$tmp"
  out=$(GATE_BASE=refs/gate-base "$0" --root "$tmp" 2>&1) || { printf 'SELFTEST FAILED: a cited ambiguous line reds:\n%s\n' "$out" >&2; rm -rf "$tmp"; exit 1; }
  rm -rf "$tmp"
  case "$out" in
    *"CITED ambiguous Plane (work/onecopy/rename-row)"*"1 cited rise(s), listed above"*) ;;
    *) printf 'SELFTEST FAILED: a cited ambiguous line passed without naming itself in the green log:\n%s\n' "$out" >&2; exit 1 ;;
  esac
  GATE_BASE=refs/gate-base gate_selftest_case --also "RAISED crate-visible boolean/mod.rs: 1 -> 2" "only falls" plant_raised_other
  GATE_BASE=refs/gate-base gate_selftest_case "is not a commit" plant_repo_without_base
  GATE_BASE=refs/gate-base gate_selftest_case "does not exist at the base" plant_base_without_counts
  for p in plant_use_crate_as plant_use_crate_self_as plant_super_self_as plant_extern_self plant_leading_colons \
      plant_rename_item plant_rename_module plant_rename_root plant_rename_in_lib plant_type_alias \
      plant_macro_path plant_use_topo_as plant_macro_use_engine plant_macro_use_inside plant_local_type_alias \
      plant_local_macro plant_lib_module_reexport; do
    gate_selftest_case --also "ROSE refused-binding" "refused outright" "$p"
  done
  printf '%s selftest OK: passes a pinned fixture with near misses (a field, a local, a method, a type parameter, Self, another crate'"'"'s item bare and renamed, a private macro) and each counting mechanism at its pin (a glob through the engine, use super::*, a module bound through the engine, a root re-export of an ambiguous name, a crate::Name::… path of any length, super:: and super::super:: climbing to the root, topo:: as crate::, a fn-local const, an associated type in an impl, f::<, mk! and a path-qualified fn, an unknown capitalised head); fires on an engine named through a full path, a hub nested in lib.rs, use super::X, a root glob and a root re-export, across engines by path and by name, on crate-wide visibility (pub(super) in an engine mod.rs included), on a pinned site gone, on a zero, unknown or duplicate baseline line, on a pin above the base pin (an uncited reader pin, a new line, any other kind even cited) while passing one lowered or a cited reader pin (named in the green log), on a base that is not a commit or has no counts file, on an ambiguous name new or gone, on an ambiguous line new against the base while passing one that cites its row (named in the green log), on a module named boolean, and on every refused binding (use crate as, use topo as, #[macro_use] on an engine or inside one, a lib.rs re-export of an engine module, a type alias or macro inside a fn body, use crate::{self as}, use super::{self as} at the root, extern crate self, ::topo::, an item, module or root re-export renamed, a lib.rs re-export renamed, a type alias, a macro spelling the engine), and diagnoses a counter that cannot run or that ends without its TOTAL\n' "$(gate_name)"
}

# `--lines FILE` (relative to crates/topo/src) lists that file's counted
# sites by kind; the gate's own arguments are the rest.
LINES_FOR=
GATE_ARGV=()
while [ $# -gt 0 ]; do
  if [ "$1" = --lines ] && [ $# -ge 2 ]; then LINES_FOR=$2; shift 2; else GATE_ARGV+=("$1"); shift; fi
done
gate_parse_args ${GATE_ARGV[@]+"${GATE_ARGV[@]}"}
gate_main
