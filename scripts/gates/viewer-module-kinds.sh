#!/usr/bin/env bash
# viewer-module-kinds.sh — the viewer crate's vocabulary/driver rule,
# checked. ONE home; ci.yml's "viewer module kinds" step in the `mirror`
# job runs it and local-scripts/ci-local.sh runs it in `tier_blind_rows`
# (and again in the directory loop on a building change set).
#
# `crates/viewer/README.md`'s `## Module boundaries` ratifies the rule:
# **every module in that crate is a VOCABULARY or a DRIVER, and its
# `use` block says which** — a driver may name any vocabulary, and no
# vocabulary may name a driver or the toolkit. It sells the rule on
# being "mechanically checkable — read the `use` block — which is the
# property that makes it survive contact with the next unit", and for
# its first life NOTHING read one. Not a gate here, not `doc-gate.sh`
# (rustdoc links, not imports), not clippy (it has no lint for "this
# module names a type from that module"). The failure that bought
# nothing red: a `use crate::session::DocSession;` added to a
# vocabulary module compiles, passes clippy, passes the doc gate, and
# silently makes the README's vocabulary tables false.
#
# WHY THIS GATE IS SITED IN THE `mirror` JOB. *A gate must be sited
# where it can fire on its own inputs* (Ev, 2026-08-20, on S61;
# `.github/workflows/ci.yml` states it above that job). Check 5's
# subject is `crates/viewer/README.md` and checks 6-7's are that file
# and `crates/viewer/Cargo.toml`. A change set of only the README
# classifies TIER=docs, `RUN_BUILD=false`, and every `if: run_build`
# job — `discipline` included — is skipped. Sited there, a docs-only PR
# renaming a table heading, adding a ghost row or deleting a table
# would merge with every arm of check 5 unrun. `scripts/check-ci-mirror
# -parity.py`'s TIER_BLIND names this gate, so the siting is enforced
# rather than remembered.
#
# WHERE A MODULE'S KIND IS DECLARED, and why it is not the README.
# Each module says its own kind, once, in its own doc header:
#
#     //! Module kind: **vocabulary** — …
#     //! Module kind: **driver** …
#
# The subject of the rule is a module's `use` block, so the
# declaration sits in the same file as the thing it classifies: an
# author changing a module's role meets the contradiction on screen
# rather than in a document they may not open, and a new module cannot
# be added without answering the question.
#
# WHAT IS DERIVED AND WHAT IS WRITTEN DOWN. Only two things in this
# file are hand-kept, and both are checked against the tree on every
# pass. Everything else is READ from the documents it enforces:
#
#   * the DRIVER ROSTER is the README's `### The drivers` table;
#   * the VOCABULARY roster is the README's two vocabulary tables;
#   * the FORBIDDEN CRATES are `crates/viewer/Cargo.toml`'s `app`
#     feature — every `dep:` in it, and nothing else. That is the right
#     population rather than a curated "toolkit" list, and the manifest
#     already says why (`Cargo.toml`, the app-graph comment): every
#     entry there is optional and reached only through `app`, so a
#     vocabulary — which is compiled in a DEFAULT-feature build — naming
#     one is naming something that is not there. A hand-kept list of
#     this got it wrong in both directions on its first day: it carried
#     `egui_dock`, which this crate does not depend on, and omitted
#     `rfd`, which opens a native file dialog — the literal thing "can
#     be read and tested without a window existing" forbids;
#   * the DRIVER MODULE PATHS are the driver table's top-level module
#     names MINUS any that host a vocabulary in the vocabulary tables.
#     `session` is a driver AND the parent of six vocabularies, so
#     `crate::session::SessionOp` must stay green while
#     `crate::app::…` reds — and which modules those are is read off
#     the same tables rather than carved out by hand.
#
# The two hand-kept things: FORBIDDEN_TYPES (two names, cross-checked
# against the README's own rule text by check 6) and VOCAB_EXCEPTIONS
# (below).
#
# THE EXCEPTIONS ARE SITE-GRANULAR, AND THAT IS D103's CLASS. `pick.rs`
# and `parts.rs` take a `&DocSession` as a read-only argument, so the
# rule as ratified is already false of the tree at five sites across
# two files. An entry here is `FILE|NEEDLE|COUNT`, and all three parts
# are load-bearing:
#
#   * NEEDLE: the exemption covers the reason it was granted and
#     nothing else, so an exempted file that gains `use eframe::egui;`
#     or `use crate::app::…` still reds;
#   * COUNT: the exemption covers the sites that were argued for and
#     no more, so a SIXTH `&DocSession` reds. A file-granular entry
#     would ratify every line later added to the file — which is
#     exactly `work/code-quality/D103.md` (unruled, track K, and its
#     fence is this directory): *"the allowlist is file-granular while
#     its justifications are per-seam, so later bounds inherit
#     ratification"*. D103 lists "a count pinned per file" as one of the
#     three shapes a taker should weigh; this is that shape, applied in
#     the fence D103 names, and it is offered as evidence for the
#     ruling rather than as a substitute for one.
#   * The count also RETIRES the entry: fix a site and the count no
#     longer matches, so the exemption cannot outlive its reason the
#     way `interval-square-allowlist.sh:125-133` records its own
#     entries doing (*"an allowlist entry with nothing behind it is a
#     ratification waiting to be inherited by the next line added to
#     the file"*).
#
# The exempted files say so in their own headers, and check 2 requires
# it: a module whose header denies naming a driver type nine lines
# above naming one is the defect this whole unit is about, published
# through rustdoc.
#
# WHAT IT CANNOT CATCH (stated because a sweep whose blind spot is
# unstated is an unverified claim):
#
#   * ROLE. This decides what a module NAMES, never what it IS. A
#     module that owns mutable state and dispatches but imports nothing
#     forbidden passes as a vocabulary; the semantic half of the
#     README's definition is not mechanised and cannot be by a grep.
#   * A DRIVER TYPE REACHED WITHOUT NAMING IT: through a re-export
#     under another name, a generic parameter, a trait object, or a
#     macro that expands to the import. The scan reads source text,
#     never expansions.
#   * A USE TREE SPREAD OVER MORE THAN 12 LINES. The nested form
#     (`use crate::{app::x, camera::Camera};`) is read from a 12-line
#     window; rustfmt does not produce wider ones in this crate, but a
#     hand-written one would evade it.
#   * MODULES OUTSIDE `crates/viewer/src`. `tests/` is not scanned: a
#     suite is allowed to name the driver it drives.
#   * A CRATE REACHED THROUGH A RE-EXPORT OF A NON-`app` DEPENDENCY.
#     `pollster` is the live near-miss and it is correctly ABSENT from
#     the derived set: `Cargo.toml:244` makes it an unconditional
#     dependency, present in the default build, so a vocabulary naming
#     it compiles and breaks no rule this file enforces.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SRC=crates/viewer/src
README=crates/viewer/README.md
MANIFEST=crates/viewer/Cargo.toml
GATE_SCAN_NOUN='viewer module'

# The declaration, anchored at column zero so only a module-level doc
# comment can carry it. The trailing prose is free; the kind is not.
KIND_LINE='^//! Module kind: \*\*(vocabulary|driver)\*\*'
# What an exempted module's header must say, and what every other
# vocabulary's must not.
EXCEPTION_MARK='^//!.*recorded exception'

# The README tables this gate reads. The kind each asserts is the
# table's own; a row is a module in its first column.
DRIVER_TABLE='### The drivers'
VOCAB_TABLES=(
  "### The session's vocabularies"
  "### The app's vocabularies"
)

# The driver types, by name. The only needle set not derived, and check
# 6 holds it against the README's rule text so deleting one there
# cannot silently narrow this.
FORBIDDEN_TYPE_NAMES=(DocSession ViewerApp)

# FILE|NEEDLE|COUNT. See the header: the needle is what the exemption
# is FOR and the count is how many sites it covers.
VOCAB_EXCEPTIONS=(
  "pick.rs|DocSession|2"
  "parts.rs|DocSession|3"
)

# LIST MEMBERSHIP IN BASH, not `printf … | grep -qxF`. `grep -q` as a
# predicate is sanctioned by `lib.sh`, but only where exit 1 IS the
# answer — and here a grep that could not run (exit 2) would read as
# "not in the list", which is a FALSE RED on the roster checks and a
# silently DROPPED exemption on the exception check. The lists are a
# handful of short strings in memory; a search that cannot fail cannot
# fail wrong.
contains() {
  local needle=$1 item
  shift
  for item in "$@"; do
    [ "$item" = "$needle" ] && return 0
  done
  return 1
}

join_alt() {
  local out="" item
  for item in "$@"; do
    out="${out:+$out|}$item"
  done
  printf '%s' "$out"
}

kind_count() { gate_grep -cE "$KIND_LINE" "$1"; }
kind_of() {
  gate_grep -m1 -oE "$KIND_LINE" "$1" | sed -E 's/.*\*\*(vocabulary|driver)\*\*/\1/'
}

# A README table's first column: the rows between its heading and the
# next heading, `session::select` style.
readme_table_modules() {
  awk -v want="$1" '
    $0 == want { inside = 1; next }
    inside && /^#/ { inside = 0 }
    inside && /^\|/ { print }
  ' "$README" |
    sed -nE 's/^\|[[:space:]]*`([A-Za-z0-9_:]+)`[[:space:]]*\|.*/\1/p'
}

# `session::select` is `session/select.rs`; `forms` is `forms.rs`.
module_path() { printf '%s.rs\n' "${1//:://}"; }
# `session::select` is hosted by `session`.
module_head() { printf '%s\n' "${1%%::*}"; }

# THE `app` FEATURE'S `dep:` ENTRIES, in the spelling Rust source uses
# (cargo's `-` is the crate's `_`). Read from the manifest rather than
# restated: see the header.
app_only_crates() {
  awk '
    /^app[[:space:]]*=[[:space:]]*\[/ { inside = 1; next }
    inside && /^\]/ { inside = 0 }
    inside { print }
  ' "$MANIFEST" |
    sed -nE 's/.*"dep:([A-Za-z0-9_-]+)".*/\1/p' |
    tr '-' '_' | sort -u
}

gate() {
  local rc=0 f rel n kind
  gate_require_file "$README"
  gate_require_file "$MANIFEST"
  if [ ! -d "$SRC" ]; then
    gate_error "$(gate_name): $SRC does not exist under $PWD — the gate's subject is gone, so it scanned nothing, which is not a pass"
    exit 1
  fi

  # THE MODULE ROSTER IS THE TREE. `lib.rs` declares the modules rather
  # than being one, and `bin/` is a binary rather than a module of the
  # library; everything else answers the question.
  local -a modules=()
  mapfile -t modules < <(find "$SRC" -type f -name '*.rs' \
    ! -name lib.rs ! -path "$SRC/bin/*" | sed "s#^$SRC/##" | sort)
  GATE_SCAN_FILES=${#modules[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no modules under $SRC in $PWD besides lib.rs and bin/ — the gate scanned nothing, which is not a pass"
    exit 1
  fi

  # --- what the README says ------------------------------------------
  local -a driver_rows=() vocab_rows=() table
  mapfile -t driver_rows < <(readme_table_modules "$DRIVER_TABLE")
  if [ "${#driver_rows[@]}" -eq 0 ]; then
    gate_error "$(gate_name): $README's \"$DRIVER_TABLE\" section yielded no module rows, so the driver roster this gate enforces came from nowhere. Either the heading was renamed or the table was reshaped — a roster that scans nothing is not a pass"
    exit 1
  fi
  for table in "${VOCAB_TABLES[@]}"; do
    local -a rows=()
    mapfile -t rows < <(readme_table_modules "$table")
    if [ "${#rows[@]}" -eq 0 ]; then
      gate_error "$(gate_name): $README's \"$table\" section yielded no module rows, so the cross-check over it decided nothing. Either the heading was renamed or the table was reshaped — a cross-check that scans nothing is not a pass"
      rc=1
      continue
    fi
    vocab_rows+=("${rows[@]}")
  done
  [ "$rc" -eq 0 ] || exit 1

  local -a driver_roster=() row
  for row in "${driver_rows[@]}"; do
    driver_roster+=("$(module_path "$row")")
  done

  # --- 1. EVERY MODULE DECLARES EXACTLY ONE KIND ----------------------
  local -a vocab=() driver=()
  for rel in "${modules[@]}"; do
    f=$SRC/$rel
    n=$(kind_count "$f")
    if [ "$n" -eq 0 ]; then
      gate_error "$SRC/$rel declares no module kind — every module in this crate is a VOCABULARY or a DRIVER ($README, Module boundaries), and the declaration is one line in the module's own doc header: \`//! Module kind: **vocabulary**\` or \`//! Module kind: **driver**\`"
      rc=1
      continue
    fi
    if [ "$n" -gt 1 ]; then
      gate_error "$SRC/$rel declares $n module kinds — a module is one or the other, and two declarations means the gate would read whichever came first"
      rc=1
      continue
    fi
    kind=$(kind_of "$f")
    if [ "$kind" = driver ]; then driver+=("$rel"); else vocab+=("$rel"); fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # --- 2. THE EXEMPTED MODULES SAY SO IN THEIR OWN HEADERS ------------
  # A header that denies naming a driver type above a line that names
  # one is the defect this unit is about, and rustdoc publishes it.
  local ex spec exfile exneedle excount
  local -a exception_files=()
  for spec in "${VOCAB_EXCEPTIONS[@]}"; do
    exception_files+=("${spec%%|*}")
  done
  for rel in "${modules[@]}"; do
    f=$SRC/$rel
    n=$(gate_grep -cE "$EXCEPTION_MARK" "$f")
    if contains "$rel" "${exception_files[@]}"; then
      if [ "$n" -eq 0 ]; then
        gate_error "$SRC/$rel is on this gate's exception list and its doc header does not say so. A module header that reads \"names no driver type\" nine lines above naming one is false, and rustdoc publishes it — state the exception in the \`Module kind:\` declaration and point at $README's \"Two vocabularies that read the session\""
        rc=1
      fi
    elif [ "$n" -gt 0 ]; then
      gate_error "$SRC/$rel claims a \"recorded exception\" in its doc header and this gate grants it none — an exemption written in prose is not one, and a reader of that header would believe it. Either add the entry here with its needle and site count, or drop the claim"
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # --- 3. THE DRIVER ROSTER, BOTH WAYS -------------------------------
  local d
  for rel in ${driver[@]+"${driver[@]}"}; do
    if ! contains "$rel" "${driver_roster[@]}"; then
      gate_error "$SRC/$rel declares itself a DRIVER and $README's \"$DRIVER_TABLE\" table does not list it. The README says there are exactly two drivers and that table is the roster — a third one is a README amendment, not a header edit, because a module that declares \`driver\` is exempt from every import check below"
      rc=1
    fi
  done
  for d in "${driver_roster[@]}"; do
    if [ ! -f "$SRC/$d" ]; then
      gate_error "$README's \"$DRIVER_TABLE\" lists a module whose file $SRC/$d does not exist — the table outran the code"
      rc=1
    elif ! contains "$d" ${driver[@]+"${driver[@]}"}; then
      gate_error "$README's \"$DRIVER_TABLE\" lists $d but $SRC/$d does not declare \`//! Module kind: **driver**\` — a module demoted in its own header while the README still calls it a driver is exempt from every import check and classified as a vocabulary by every reader"
      rc=1
    fi
  done
  # --- 4. THE VOCABULARY TABLES, AGAINST THE MODULES THEY NAME -------
  local path
  for row in "${vocab_rows[@]}"; do
    path=$(module_path "$row")
    if [ ! -f "$SRC/$path" ]; then
      gate_error "$README's vocabulary tables list \`$row\`, which is not a module in the tree ($SRC/$path does not exist) — the table outran the code"
      rc=1
    elif ! contains "$path" ${vocab[@]+"${vocab[@]}"}; then
      gate_error "$README's vocabulary tables list \`$row\` as a vocabulary, but $SRC/$path declares itself a DRIVER — the README and the module disagree about what the module is, which is the drift a per-module declaration buys and this check pays for"
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # --- 5. THE NEEDLES, DERIVED ---------------------------------------
  local -a crates_list=() heads=() vocab_heads=() path_mods=()
  mapfile -t crates_list < <(app_only_crates)
  if [ "${#crates_list[@]}" -eq 0 ]; then
    gate_error "$(gate_name): $MANIFEST's \`app\` feature yielded no \`dep:\` entries, so the forbidden-crate set came from nowhere. Either the feature was renamed or its shape changed — a needle set derived from nothing matches nothing, which is not a pass"
    exit 1
  fi
  for row in "${vocab_rows[@]}"; do vocab_heads+=("$(module_head "$row")"); done
  for row in "${driver_rows[@]}"; do
    d=$(module_head "$row")
    contains "$d" ${path_mods[@]+"${path_mods[@]}"} && continue
    # A driver that HOSTS a vocabulary cannot be a forbidden path:
    # `crate::session::SessionOp` is the rule working. Read off the
    # vocabulary tables, never carved out by hand.
    contains "$d" ${vocab_heads[@]+"${vocab_heads[@]}"} && continue
    path_mods+=("$d")
  done
  if [ "${#path_mods[@]}" -eq 0 ]; then
    gate_error "$(gate_name): every driver in $README's table also hosts a vocabulary, so no driver module path is forbidden and check 6's path arm matches nothing — that is not a pass"
    exit 1
  fi

  local types_alt crates_alt mods_alt
  types_alt=$(join_alt "${FORBIDDEN_TYPE_NAMES[@]}")
  crates_alt=$(join_alt "${crates_list[@]}")
  mods_alt=$(join_alt "${path_mods[@]}")
  local pat_types="\\b($types_alt)\\b"
  local pat_crates="\\b($crates_alt)\\b"
  # THREE PATH SPELLINGS, and the third is the one the README's own
  # slogan names. `crate::app` is the bare import; `app::` is the
  # segment-with-children form, which also catches a nested
  # `crate::{app::x, …}`; and the use-tree arm below catches
  # `use crate::{app, camera::Camera};`, where the driver appears as a
  # bare leaf inside braces and neither of the first two sees it. That
  # idiom is written in 22 places elsewhere in this workspace.
  local pat_paths="crate::($mods_alt)\\b|\\b($mods_alt)::"
  local pat_tree="use[[:space:]]+(crate|self|super)::\\{[^;]*\\b($mods_alt)\\b"
  local pat_all="$pat_types|$pat_crates|$pat_paths"

  # --- 6. THE README'S RULE STILL NAMES THE TYPES --------------------
  local t
  for t in "${FORBIDDEN_TYPE_NAMES[@]}"; do
    # PLAIN `grep -q`, NOT `gate_grep`: lib.sh folds exit 1 to 0 so a
    # scan that matched nothing reads as a clean scan, which is right
    # for a matcher and WRONG for a predicate — folded, this arm could
    # never fire. lib.sh says so at gate_grep; this is the case it means.
    if ! grep -qF "\`$t\`" "$README"; then
      gate_error "$(gate_name): $README no longer names \`$t\` anywhere, and this gate forbids it in every vocabulary on that document's authority. A needle whose ratification has left the README is a rule this gate invented — restore the rule text or drop the name here"
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # --- 7. NO VOCABULARY NAMES A DRIVER, A DRIVER MODULE, OR A CRATE
  # THAT ONLY EXISTS BEHIND `app` ------------------------------------
  # Read over the shared CODE-ONLY view, so a header that spells
  # `DocSession` out to say it does not name one — `forms.rs` and
  # `drafts.rs` both do — stays green, and a string literal cannot fire
  # it either.
  local -a scanned=()
  for rel in ${vocab[@]+"${vocab[@]}"}; do scanned+=("$SRC/$rel"); done
  if [ "${#scanned[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no vocabulary modules under $SRC — every module declares itself a driver, which is not a pass"
    exit 1
  fi
  local lines_hits tree_hits hits
  lines_hits=$(gate_rust_code "${scanned[@]}" | gate_grep -E "$pat_all")
  tree_hits=$(gate_rust_code --window 12 "${scanned[@]}" | gate_grep -E "$pat_tree")
  # ONE RECORD PER SITE. The window arm restates a line the line arm
  # may already have matched, and both spell the site `FILE:LINE:`, so
  # the union is deduplicated on that key — otherwise an exception's
  # site count would depend on which arms fired.
  hits=$(printf '%s\n%s\n' "$lines_hits" "$tree_hits" | grep -v '^[[:space:]]*$' |
    awk -F: '{ k = $1 ":" $2 } !(k in seen) { seen[k] = 1; print }' | sort)

  # --- 8. THE EXCEPTIONS, SITE BY SITE -------------------------------
  local found kept
  for spec in "${VOCAB_EXCEPTIONS[@]}"; do
    exfile=${spec%%|*}
    excount=${spec##*|}
    exneedle=${spec#*|}; exneedle=${exneedle%|*}
    if [ ! -f "$SRC/$exfile" ]; then
      gate_error "$(gate_name): the exception list names $SRC/$exfile, which is not a file under $PWD — drop the entry"
      rc=1
      continue
    fi
    if ! contains "$exfile" ${vocab[@]+"${vocab[@]}"}; then
      gate_error "$(gate_name): the exception list names $exfile, which no longer declares \`//! Module kind: **vocabulary**\` — an exception to the vocabulary rule on a module that is not a vocabulary exempts nothing and hides the module from every check here"
      rc=1
      continue
    fi
    found=$(printf '%s\n' "$hits" | grep -c -E "^$SRC/$exfile:[0-9]+:.*$exneedle" || true)
    if [ "$found" -gt "$excount" ]; then
      printf '%s\n' "$hits" | grep -E "^$SRC/$exfile:[0-9]+:.*$exneedle" | cut -c1-160
      gate_error "$SRC/$exfile names $exneedle at $found sites and its recorded exception covers $excount. The exception is SITE-granular on purpose ($README, Two vocabularies that read the session): the sites that were argued for are exempt and a new one is not, so a later unit cannot inherit the ratification by adding a line to an allowlisted file (work/code-quality/D103.md's class). Take the driver's answer as a value, or argue the new site and raise the count with it"
      rc=1
      continue
    fi
    if [ "$found" -lt "$excount" ]; then
      gate_error "$SRC/$exfile names $exneedle at $found sites and its recorded exception covers $excount — the exception has outlived part of its reason. An entry with nothing behind it is a ratification waiting to be inherited by the next line added to the file (interval-square-allowlist.sh says the same of its own entries), so lower the count or delete the entry"
      rc=1
      continue
    fi
    hits=$(printf '%s\n' "$hits" | grep -v -E "^$SRC/$exfile:[0-9]+:.*$exneedle" || true)
  done
  [ "$rc" -eq 0 ] || exit 1

  kept=$(printf '%s\n' "$hits" | grep -v '^[[:space:]]*$' || true)
  if [ -n "$kept" ]; then
    printf '%s\n' "$kept" | cut -c1-160
    gate_error "a vocabulary module names a driver type, a driver module, or a crate that only exists behind the \`app\` feature. $README's rule: a vocabulary holds values, their wording and pure functions over them, and can be read and tested without a session or a window existing. Move the code to the driver, or take the driver's answer as a value — do not reclassify the module to silence this"
    exit 1
  fi

  gate_ok "every module under $SRC declares a kind, ${#driver[@]} drivers match $README's own table, ${#scanned[@]} vocabularies name none of ${#FORBIDDEN_TYPE_NAMES[@]} driver types, ${#path_mods[@]} driver module paths or ${#crates_list[@]} \`app\`-only crates read from $MANIFEST (${#VOCAB_EXCEPTIONS[@]} recorded exceptions, each still live at exactly its recorded site count), and the README's tables agree with the modules they name"
}

# This gate's subject is one crate's src tree plus its README and
# manifest, not `crates/*/src`, so the fixture is a miniature viewer
# crate. THE MANIFEST IS THE REAL ONE, copied: the derived needle set in
# the fixture is then the derived needle set in the tree, so the
# coverage cases below cannot fall behind a dependency someone adds to
# the `app` feature. The rosters and the exception list are planted from
# themselves for the same reason.
gate_plant_clean() {
  local root=$1 d ex spec exfile exneedle excount i
  mkdir -p "$root/$SRC/session" "$root/$SRC/pane" "$root/$SRC/bin"
  cp "$GATE_REPO_ROOT/$MANIFEST" "$root/$MANIFEST"

  # lib.rs and bin/ carry NO declaration, and stay green: they are the
  # two exclusions, asserted by the clean fixture rather than assumed.
  printf 'pub mod camera;\n' > "$root/$SRC/lib.rs"
  printf 'fn main() {}\n' > "$root/$SRC/bin/viewer.rs"

  # A vocabulary with both near misses baked in: a COMMENT naming the
  # driver type and the toolkit, and a real import of `session::op`,
  # which is a vocabulary living under a driver's module path.
  cat > "$root/$SRC/camera.rs" <<'RS'
//! The camera value.
//!
//! Names no `DocSession` and no `egui`; both words are prose here.
//!
//! Module kind: **vocabulary** — it names no driver type.
use crate::session::SessionOp;
pub fn apply(op: SessionOp) -> SessionOp { op }
RS
  for d in forms drafts; do
    {
      printf '//! An app vocabulary.\n//!\n'
      printf '//! Module kind: **vocabulary** — it names no driver type.\n'
      printf 'pub struct Thing%s;\n' "$d"
    } > "$root/$SRC/$d.rs"
  done
  {
    printf '//! What is selected.\n//!\n'
    printf '//! Module kind: **vocabulary** — it names no driver type.\n'
    printf 'pub struct Selection;\n'
  } > "$root/$SRC/session/select.rs"

  for d in "${FIXTURE_DRIVERS[@]}"; do
    mkdir -p "$root/$SRC/$(dirname "$d")"
    {
      printf '//! A driver.\n//!\n'
      printf '//! Module kind: **driver** (README, Module boundaries).\n'
      printf 'use eframe::egui;\n'
      printf 'pub struct DocSession;\n'
    } > "$root/$SRC/$d"
  done

  # The exceptions, planted from their own entries: the file names its
  # needle exactly COUNT times and its header states the exception.
  for spec in "${VOCAB_EXCEPTIONS[@]}"; do
    exfile=${spec%%|*}
    excount=${spec##*|}
    exneedle=${spec#*|}; exneedle=${exneedle%|*}
    mkdir -p "$root/$SRC/$(dirname "$exfile")"
    {
      printf '//! A ratified exception.\n//!\n'
      printf '//! Module kind: **vocabulary**, with a recorded exception.\n'
      i=0
      while [ "$i" -lt "$excount" ]; do
        printf 'pub fn read_%s(_s: &%s) {}\n' "$i" "$exneedle"
        i=$((i + 1))
      done
    } > "$root/$SRC/$exfile"
  done

  fixture_readme > "$root/$README"
}

# The fixture's driver roster. Named here rather than read from the real
# README so the fixture is a miniature crate rather than a copy of this
# one, but it carries the shapes that matter: a driver that hosts
# vocabularies (`session`), a driver split into a parent and children
# (`pane`), and plain ones.
FIXTURE_DRIVERS=(app.rs gpu.rs pane.rs pane/create.rs session.rs widgets.rs)

fixture_readme() {
  local d
  printf '## Module boundaries\n\n'
  printf 'Every module is a vocabulary or a driver. A vocabulary names\n'
  printf 'no `DocSession`, no `ViewerApp` and no `egui`.\n\n'
  printf '%s\n\n' "$DRIVER_TABLE"
  printf '| Module | Is |\n|---|---|\n'
  for d in "${FIXTURE_DRIVERS[@]}"; do
    d=${d%.rs}
    printf '| `%s` | a driver |\n' "${d////::}"
  done
  printf '\n'
  printf "%s\n\n" "${VOCAB_TABLES[0]}"
  printf '| Module | Holds |\n|---|---|\n'
  printf '| `session::select` | what is selected |\n\n'
  printf "%s\n\n" "${VOCAB_TABLES[1]}"
  printf '| Module | Holds |\n|---|---|\n'
  printf '| `forms` | what the panels offer |\n'
  printf '| `drafts` | in-flight form state |\n\n'
  printf '### Two vocabularies that read the session\n\nProse.\n'
}

# --- planters -------------------------------------------------------

plant_undeclared_module() {
  printf '//! A new module with no kind.\npub struct Thing;\n' > "$1/$SRC/thing.rs"
}

plant_two_kinds() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! Two answers.
//!
//! Module kind: **vocabulary** — it names no driver type.
//! Module kind: **driver** (README, Module boundaries).
pub struct Thing;
RS
}

# ONE PLANTER FOR EVERY NEEDLE, driven by the derived sets rather than
# by a hand-written case list. `lib.sh` requires a fixture saying where
# a matcher stops; this is the other half — a fixture per alternate
# saying that it starts. Nine of thirteen alternates had no case behind
# them when this gate first landed, and deleting them from the pattern
# left `--selftest` green.
plant_named() {
  local text=$1 root=$2
  {
    printf '//! A vocabulary that names something it may not.\n//!\n'
    printf '//! Module kind: **vocabulary** — it names no driver type.\n'
    printf '%s\n' "$text"
  } > "$root/$SRC/thing.rs"
}

plant_type_use() { plant_named "use crate::session::$1; pub fn peek(_s: &$1) {}" "$2"; }
# NOT AN IMPORT: a fully-qualified name in a signature evades any check
# that reads only `use` lines, which is what the README's slogan sells
# the rule on.
plant_type_inline() { plant_named "pub fn peek(_s: &crate::session::$1) {}" "$2"; }
plant_crate_use() { plant_named "use $1::Thing; pub fn go(_t: Thing) {}" "$2"; }

# THE PATH ARMS, ONE ISOLATING FIXTURE EACH. A fixture that trips two
# arms proves neither: the first version of this gate had four path
# plants and every one of them was caught by a second arm, so deleting
# either the segment arm or the whole use-tree scan left `--selftest`
# green. Each planter below names its driver in exactly ONE of the
# three spellings, so deleting that arm turns this file red.
#
#   bare      `use crate::app as chrome;`  — `crate::app`, and the
#             alias means no `app::` anywhere else in the file.
#   segment   `self::app::run()`           — `app::`, with no `crate::`
#             prefix and no brace group.
#   use tree  `use crate::{app, …};`       — the driver as a bare leaf
#             inside braces, which neither of the other two sees. The
#             import stands alone: naming it in a body would trip the
#             segment arm and hide a broken tree scan.
plant_path_bare_aliased() {
  plant_named "use crate::$1 as chrome; pub fn go() { chrome::run() }" "$2"
}
plant_path_segment_via_self() {
  plant_named "pub fn go() { self::$1::run() }" "$2"
}
# rustfmt wraps a long tree, so the wrapped form is what the window
# scan actually has to read.
plant_path_use_tree() {
  plant_named "use crate::{
    $1,
    camera::Camera,
};
pub fn go(_c: Camera) {}" "$2"
}
plant_path_use_tree_oneline() {
  plant_named "use crate::{$1, camera::Camera}; pub fn go(_c: Camera) {}" "$2"
}
# The realistic spelling, kept because it is what a real edit looks
# like even though it trips two arms at once.
plant_path_child() { plant_named "use crate::$1::helper; pub fn go() { helper() }" "$2"; }

# A TEST MODULE IS IN SCOPE. The README's definition is that a
# vocabulary can be read AND TESTED without a session existing.
plant_test_module_names_driver() {
  plant_named "pub fn go() {}
#[cfg(test)]
mod tests {
    use crate::session::DocSession;
    #[test]
    fn t() { let _ = DocSession; }
}" "$1"
}

plant_self_promoted_driver() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! A module that promoted itself out of the rule.
//!
//! Module kind: **driver** (README, Module boundaries).
use eframe::egui;
pub fn draw(_ui: &mut egui::Ui) {}
RS
}

plant_driver_row_is_a_ghost() {
  sed -i 's#^| `app` | a driver |$#&\n| `retired` | a driver |#' "$1/$README"
}

plant_driver_demoted() {
  cat > "$1/$SRC/app.rs" <<'RS'
//! Demoted in its own header while the README still calls it a driver.
//!
//! Module kind: **vocabulary** — it names no driver type.
pub fn go() {}
RS
}

plant_driver_table_renamed() {
  sed -i "s|^### The drivers\$|### Drivers|" "$1/$README"
}

plant_vocab_table_renamed() {
  sed -i "s|^### The session's vocabularies\$|### Session vocabularies|" "$1/$README"
}

plant_vocab_row_is_a_ghost() {
  sed -i 's#^| `session::select` | what is selected |$#&\n| `session::retired` | gone |#' \
    "$1/$README"
}

plant_readme_calls_a_driver_a_vocabulary() {
  sed -i 's#^| `forms` | what the panels offer |$#| `widgets` | drawn helpers |#' "$1/$README"
}

# THE EXCEPTION'S THREE FAILURES, one per part of the entry.
plant_exception_gains_a_site() {
  local spec=${VOCAB_EXCEPTIONS[0]}
  local exfile=${spec%%|*} exneedle=${spec#*|}
  exneedle=${exneedle%|*}
  printf 'pub fn brand_new_session_read(_s: &%s) {}\n' "$exneedle" >> "$1/$SRC/$exfile"
}

plant_exception_loses_a_site() {
  local spec=${VOCAB_EXCEPTIONS[0]}
  local exfile=${spec%%|*} exneedle=${spec#*|}
  exneedle=${exneedle%|*}
  grep -v "$exneedle" "$1/$SRC/$exfile" > "$1/$SRC/$exfile.new"
  mv "$1/$SRC/$exfile.new" "$1/$SRC/$exfile"
}

# THE HOLE THE FILE-GRANULAR VERSION LEFT: an exempted file loses no
# `DocSession` and gains an entirely different forbidden name. Under a
# union-matched, file-granular exemption this was green.
plant_exception_file_gains_another_needle() {
  local spec=${VOCAB_EXCEPTIONS[0]}
  local exfile=${spec%%|*}
  printf 'use eframe::egui;\nuse crate::app::ViewerApp;\n' >> "$1/$SRC/$exfile"
}

plant_exception_header_denies_it() {
  local spec=${VOCAB_EXCEPTIONS[0]}
  local exfile=${spec%%|*}
  sed -i 's|^//! Module kind: \*\*vocabulary\*\*, with a recorded exception.$|//! Module kind: **vocabulary** — it names no driver type.|' \
    "$1/$SRC/$exfile"
}

plant_unexempted_module_claims_an_exception() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! A module writing itself a permission.
//!
//! Module kind: **vocabulary**, with a recorded exception for the
//! session read below.
pub fn go() {}
RS
}

plant_readme_drops_a_type_name() {
  sed -i 's|no `ViewerApp` and ||' "$1/$README"
}

plant_manifest_app_feature_renamed() {
  sed -i 's|^app = \[|application = [|' "$1/$MANIFEST"
}

# GREEN cases, asserted rather than assumed.
plant_prose_and_literals() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! Names `DocSession`, `ViewerApp` and `egui` in prose to say it does
//! not name them — which `forms.rs` and `drafts.rs` both do today.
//!
//! Module kind: **vocabulary** — it names no driver type.
/* A block comment about egui::Ui and crate::app. */
pub fn note() -> &'static str {
    // A trailing comment naming DocSession.
    "egui and DocSession as a string literal"
}
RS
}

plant_imports_a_session_vocabulary() {
  plant_named "use crate::session::{OpOutcome, SessionOp};
pub fn go(_o: OpOutcome, _s: SessionOp) {}" "$1"
}

# A NESTED USE TREE THAT NAMES NO DRIVER. The tree arm must stop here,
# or every wrapped `use crate::{…}` in the crate reds.
plant_innocent_use_tree() {
  plant_named "use crate::{
    camera::Camera,
    session::SessionOp,
};
pub fn go(_c: Camera, _s: SessionOp) {}" "$1"
}

# `pollster` is a DEFAULT-feature dependency (Cargo.toml), not an
# `app`-only one, so a vocabulary may name it. The derived set says so;
# a curated "toolkit" list would have had to remember.
plant_names_a_default_feature_dependency() {
  plant_named "use pollster::block_on; pub fn go() { let _ = block_on; }" "$1"
}

gate_selftest() {
  local want t c m spec exfile
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"

  gate_selftest_case "declares no module kind" plant_undeclared_module
  gate_selftest_case "declares 2 module kinds" plant_two_kinds

  # NEEDLE COVERAGE, derived from the same sets the matcher is built
  # from, so a name added to the `app` feature or to the driver table
  # gets a fixture without anyone remembering to write one.
  want="names a driver type, a driver module, or a crate that only exists behind"
  for t in "${FORBIDDEN_TYPE_NAMES[@]}"; do
    gate_selftest_case "$want" plant_type_use "$t"
    gate_selftest_case "$want" plant_type_inline "$t"
  done
  for c in $(app_only_crates); do
    gate_selftest_case "$want" plant_crate_use "$c"
  done
  for m in app gpu pane widgets; do
    gate_selftest_case "$want" plant_path_bare_aliased "$m"
    gate_selftest_case "$want" plant_path_segment_via_self "$m"
    gate_selftest_case "$want" plant_path_use_tree "$m"
    gate_selftest_case "$want" plant_path_use_tree_oneline "$m"
    gate_selftest_case "$want" plant_path_child "$m"
  done
  gate_selftest_case "$want" plant_test_module_names_driver

  gate_selftest_case "declares itself a DRIVER and" plant_self_promoted_driver
  gate_selftest_case "does not exist — the table outran the code" plant_driver_row_is_a_ghost
  gate_selftest_case "does not declare" plant_driver_demoted
  gate_selftest_case "yielded no module rows" plant_driver_table_renamed
  gate_selftest_case "yielded no module rows" plant_vocab_table_renamed
  gate_selftest_case "is not a module in the tree" plant_vocab_row_is_a_ghost
  gate_selftest_case "declares itself a DRIVER — the README and the module disagree" \
    plant_readme_calls_a_driver_a_vocabulary

  spec=${VOCAB_EXCEPTIONS[0]}; exfile=${spec%%|*}
  gate_selftest_case "and its recorded exception covers" plant_exception_gains_a_site
  gate_selftest_case "has outlived part of its reason" plant_exception_loses_a_site
  gate_selftest_case "$want" plant_exception_file_gains_another_needle
  gate_selftest_case "its doc header does not say so" plant_exception_header_denies_it
  gate_selftest_case "this gate grants it none" plant_unexempted_module_claims_an_exception

  gate_selftest_case "no longer names" plant_readme_drops_a_type_name
  gate_selftest_case "yielded no \`dep:\` entries" plant_manifest_app_feature_renamed

  gate_selftest_passes "prose and string-literal mentions" plant_prose_and_literals
  gate_selftest_passes "an import of a vocabulary under session::" \
    plant_imports_a_session_vocabulary
  gate_selftest_passes "a nested use tree naming no driver" plant_innocent_use_tree
  gate_selftest_passes "a default-feature dependency (pollster)" \
    plant_names_a_default_feature_dependency

  printf '%s selftest OK: every forbidden name has its own fixture, and the fixture LIST is derived from the same two documents the matcher is — one case per driver type, one per `dep:` in %s'"'"'s `app` feature, and five per driver module path — an ISOLATING fixture for each of the three spellings the matcher has (aliased bare import, `self::`-qualified segment, wrapped use tree, one-line use tree) plus the realistic child path that trips two arms at once, so deleting any one arm turns this self-test red. The clean fixture proves lib.rs and bin/ are excluded on purpose and that a site-granular exception suppresses exactly its recorded sites; the exception fires on a SIXTH site, on a lost site, on a different forbidden name in the same file, on a header that denies the exception, and on a module writing itself one. The README arms fire on a ghost driver row, a demoted driver, either table heading renamed, a ghost vocabulary row, a driver listed as a vocabulary, and the rule text losing a type name; the manifest arm fires when the `app` feature can no longer be read. Prose, string literals, an import under `session::`, an innocent nested use tree and a default-feature dependency stay green; and the gate stays RED, with a diagnosis, when `grep` itself cannot run\n' \
    "$(gate_name)" "$MANIFEST"
}

gate_parse_args "$@"
gate_main
