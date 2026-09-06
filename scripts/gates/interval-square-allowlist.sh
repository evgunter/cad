#!/usr/bin/env bash
# interval-square-allowlist.sh — the interval-square `powi(2)` allowlist
# (ratified 2026-08-01). ONE home; ci.yml's "interval-square powi(2)
# allowlist (ratified 2026-08-01)" step and local-scripts/ci-local.sh's
# discipline row both call this file. Kernel comments that name that
# step name still resolve: the step is still there, and it runs this.
#
# Interval-square discipline tripwire — the rule lives HERE, and
# the kernel's comments point at this step. FOUR live bugs came
# from this one class: squaring via plain `x * x` treats the
# factors as independent, so a zero-straddling enclosure gets a
# spurious negative lower bound that poisons downstream
# sqrt/decoration. `Real::powi(2)` knows both factors are the same
# variable and returns the tight nonnegative enclosure with the
# decoration preserved. Squares of definitely-nonzero singletons
# (stored radii, bulges) may stay as `*` — for a `lo >= 0`
# enclosure the plain product's four-corner minimum ALREADY is the
# tight square, so the conversion is a no-op there and the reason
# to make it is uniformity, not width.
#
# "NEVER WIDER" IS FALSE AS WRITTEN, AND THIS TEXT USED TO SAY IT.
# It also named `inari` as the backend, which it has not been since
# M5 PR 1 — the backend is the in-repo `interval-transcendentals`
# crate. On that backend `powi(2)` is **1 ulp wider on each side
# once the square drops below `TWO_PROD_VALID_MIN = 2^-960`**, i.e.
# `|x| < 2^-480 ~ 3.2e-145: the closing `mul_hi(1.0, base)` has no
# 2Prod witness that can certify a sub-2^-960 product, so it pads a
# second time. Measured at `x = 2^-481*1.5`; **0 widening cases in
# 3,000,000 samples with `|x|` in [1e-60, 1e60]**, so it is
# unreachable in the live regime — which is a reason to state it
# once, not a reason to keep saying "never". THAT SWEEP IS AN
# UNDATED ONE-SHOT AND NOTHING RE-TAKES IT: the witness at
# `x = 2^-481*1.5` is a derivation anyone can re-run in a line, and
# the 3,000,000-sample negative is what it always is — evidence about
# a range, not a theorem about it. It is stated at the magnitude it
# was taken because the CONCLUSION is a statement about which regime
# real geometry lives in, and a re-sweep finding a case inside
# [1e-60, 1e60] would be a finding rather than a correction. A dead backend named
# in a numerics justification is S39/S112's class in the worst
# place it can land. Reviewing new geometry: this grep only sees production code
# under `crates/*/src`, so also eyeball predicate-path diffs for
# `* self`, `x * x`, and `.dot(` on possibly-zero vectors.
#
# WHAT THE MATCHER SEES, and why the shape changed. The operand is a
# FIELD PATH, not a bare identifier: `self.x * self.x` is the commonest
# spelling of this bug in vector code and the bare-identifier
# backreference could not see it at all — the live instance it walked
# past for two milestones was `linalg/vec.rs`'s `orthonormal_basis`,
# production code generic over `Real` in a file nobody had allowlisted.
# And the second operand may not be followed by `.`, `(` or `[`, because
# `a * a.norm()` is not a square and the gate used to say it was. Those
# two failures compound: a false red is a nudge toward the allowlist
# rather than the fix, and this gate is where that already happened —
# `linalg/mat.rs`'s entry was justified in writing partly by `r *
# r.transpose()` test hits that were never violations.
#
# KNOWN GAP 1: the scan is PRODUCTION CODE ONLY — a `#[cfg(test)]` item
# is dropped, and so is a module file whose `mod` declaration is one. A
# `x * x` inside a test cannot poison a shipped enclosure, and scanning
# them is what produced the false positives above. **Only a TEST-ONLY
# attribute counts**: `any(test, …)` and `not(test)` are both scanned,
# because an `any(debug_assertions, test, …)` module is every debug
# build — `topo`'s `test_support_impl` is exactly that, and an earlier
# draft skipped it. The cost is that a test-only helper later promoted
# to production arrives unscanned; the promotion is a diff a human
# reads.
#
# KNOWN GAP 2: an operand starting with an uppercase letter is invisible
# (`SOME_CONST * SOME_CONST`). Deliberate: the ALL-CAPS population in
# this tree is `usize` buffer sizing, where the rule does not apply and
# a red would be pure cry-wolf.
#
# KNOWN GAP 3: an INDEXED square (`v[i] * v[i]`) and a repeated CALL
# (`f(t) * f(t)`) are both invisible. The first is a real hole; the
# second is deliberate, since a repeated call is not obviously one
# value.
#
# THE SCAN IS STATEMENT-SCOPED, not line-scoped, because `rustfmt` wraps
# a long product: `v.long_field\n    * v.long_field` is one expression
# and two lines, and a line matcher sees neither operand beside the
# other. Same ruling as S158's, same reason it is worth stating twice —
# the wrapped form is what the formatter PRODUCES from the caught one.
#
# KNOWN GAP 4: the allowlist is FILE-granular while its reasons are
# per-seam, so a second unrelated `x * x` added to an allowlisted file
# inherits the first entry's ratification silently. That is S159/D103's
# class and it is not closed here.
#
# KNOWN GAP 5: the SCALED-square branch sees one spelling of the shape
# and four others go past it. `(k * x) * x` is matched; `(x * k) * x` is
# not, because the repeated operand must be the group's LAST factor —
# requiring only that it appear somewhere inside is what starts
# matching `(x + k) * x`. `((a + b) * x) * x` and `(f(k) * x) * x` are
# not matched either: the group may contain no nested parenthesis at
# all, which is the price of not walking past a call. `k * x * m * x`
# — the same product with a third factor wedged between the operands
# and no parentheses anywhere — is invisible to both branches, since
# one needs adjacency and the other needs the paren. And a square split
# across two statements (`let kx = k * x;` … `kx * x`) is invisible to
# everything here: the scan's unit is a statement, and that square is
# two of them.
#
# NONE OF THESE IS LIVE, AND A CLEAN RUN OF THIS GATE IS NOT WHY. These
# five spellings are exactly what this matcher cannot see, so its own
# green says nothing about them; only a differently-shaped sweep can.
# One was run: separate patterns per spelling, plus a two-statement
# `let`-binding scan over 52,471 production statements. `(x * k) * x`
# and the nested-paren forms: 0. The three-factor and two-statement
# forms: 21 + 3 candidates, every one either in an already-allowlisted
# file, a `Mat3<f64>` projector conjugation, or a Taylor term over an
# already-tight `.sqr()` value. A hole this gate cannot see is still a
# hole, so it is written down rather than closed.
#
# Allowlist rationale, per file:
#  - geom-core real.rs / ring_interval.rs — the scalar implementations
#    themselves: `x * x` is definitional (powi is BUILT from it) and
#    their tests deliberately contrast the plain product with the tight
#    square;
#  - geom-core linalg/svd.rs / lsq.rs — documented f64-only
#    selection lanes (lsq's hits are usize buffer sizing);
#  - geom-brep ssi/jet.rs / march.rs / system.rs — f64-only jet and
#    marcher numerics (finite differences, step control).
#
# `interval.rs`, `dual.rs` and `linalg/mat.rs` were on this list and are
# NOT any more. The first two lost their hits when the scan stopped
# reading test modules; `mat.rs` lost its one hit —
# `rotation_about`'s diagonal `t * x * x` — when that scaled square was
# reassociated into `t·(x²)`. Its entry had been a scheduled residue
# held open for exactly that conversion, and nothing else in the file
# is a square. An allowlist entry with nothing behind it is a
# ratification waiting to be inherited by the next line added to the
# file, which is why each of these came off rather than being left as
# harmless.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# `x * x` where x is an identifier or a field path, in TWO shapes.
#
# Branch 1 — the bare square `x * x`. The trailing look-ahead is the
# half that stops `a * a.method()` from reading as a square. This branch
# also covers the UNPARENTHESIZED scaled square `k * x * x`, because
# `(k·x)·x` written without parentheses still puts the two operands
# beside each other.
#
# Branch 2 — the PARENTHESIZED scaled square `(k * x) * x`, which is
# the spelling branch 1 structurally cannot see: the `)` sits between
# the two operands. It is the same defect — the second factor is a
# fresh, independent view of the same variable — and its fix is a
# reassociation into `k·(x²)` rather than a substitution. Two guards
# keep it off correct code: the paren must NOT be preceded by an
# identifier character, so `libm::sin(t) * t` and `foo(a * b) * b` are
# calls and not squares; and the parenthesized group must itself
# contain a `*` with the repeated operand as its LAST factor, so
# `(a + b) * b` and `(a * b) * c` are untouched.
SQUARE_PATH='(?:[a-z_][a-z0-9_]*)(?:\.[a-z_][a-z0-9_]*)*'
SQUARE_RE="(?<![\w.])($SQUARE_PATH)\s*\*\s*\1(?![\w.(\[])"
SQUARE_RE="$SQUARE_RE|(?<!\w)\(\s*[^()]*?\*\s*($SQUARE_PATH)\s*\)\s*\*\s*\2(?![\w.(\[])"

# A relative path with its `.` and `..` segments taken out. An exclusion
# is matched against the scan set as text, so an unnormalised
# `crates/mesh/src/../tests/common/x.rs` excludes nothing at all — a
# silent no-op wearing the shape of an exclusion.
gate_norm_path() {
  printf '%s' "$1" | awk -F/ '
    {
      n = 0
      for (i = 1; i <= NF; i++) {
        if ($i == "" || $i == ".") continue
        if ($i == ".." && n > 0 && seg[n] != "..") { n--; continue }
        seg[++n] = $i
      }
      out = ""
      for (i = 1; i <= n; i++) out = (i == 1) ? seg[i] : out "/" seg[i]
      print out
    }'
}

# WHERE A GATED DECLARATION MOUNTS, read from the CODE-ONLY view that
# `gate_rust_code` already builds — comments and string bodies are gone
# before this sees a line, so no comment rule is re-minted beside the
# shared reader's. One pass over the declaring file carries the three
# things the statement view drops:
#
#   * `{` VS `;`. An inline `mod x { … }` declares no file at all, and
#     the statement view cuts at both delimiters, so an inline module and
#     a file declaration arrive as the same record.
#   * THE ENCLOSING INLINE-MODULE CHAIN. `mod y { #[cfg(test)] mod x; }`
#     mounts at `y/x.rs`; read as top-level it excludes the unrelated
#     production `x.rs` instead, which is the unsafe direction.
#   * WHERE a `#[path]` attribute sits. Its payload is a string literal
#     and this view blanks it, so only the raw line can supply it — the
#     line, the occurrence on it and how many the code view sees there
#     are named here, so the raw read is one line long and cannot pick up
#     a mount the code view does not have.
#
# Prints `KIND|CHAIN|PATH_LINE|PATH_INDEX|PATH_COUNT` — `|` and not a
# tab, because a tab is IFS whitespace and `read` folds a run of it, so
# an empty CHAIN would shift every field after it by one.
# KIND is `attr` (a `#[path]` mount), `default` (rustc's positional
# rule), `inline` (no file) or `refuse` (an enclosing brace that is not a
# module, where rustc mounts a non-inline module only through `#[path]`).
# CHAIN is the enclosing module names, `/`-joined. NO OUTPUT means the
# code view does not place the declaration the statement view reported —
# also a refusal, and the caller says so.
declaration_shape() {
  gate_rust_code "$1" | awk -v start="$2" -v name="$3" '
    # THE ANSWER IS HELD TO `END`, NOT PRINTED AND EXITED ON. This `awk`
    # reads a pipe, and exiting at the declaration closes it while the
    # shared reader upstream is still writing: that write fails, the
    # reader dies of it, and `pipefail` reports a pipeline that did its
    # job as a pipeline that broke. Whether it lands is a race between
    # two processes, which is the worst way for a gate to be wrong —
    # green on one machine and red on the next over the same tree.
    function emit(kind,   i, c) {
      if (done) return
      done = 1
      c = ""
      for (i = 1; i <= depth; i++) {
        if (chain[i] == "") { out = "refuse||0|0|0"; return }
        c = (c == "") ? chain[i] : c "/" chain[i]
      }
      out = sprintf("%s|%s|%d|%d|%d", kind, c, pline, pidx, pcount)
    }
    done { next }
    {
      s = $0
      if (!match(s, /^[^:]*:[0-9]+:/)) next
      ln = substr(s, RSTART, RLENGTH); sub(/^[^:]*:/, "", ln); sub(/:$/, "", ln)
      s = substr(s, RSTART + RLENGTH)
      ln += 0
      if (ln >= start && pline == 0) {
        k = 0; t = s
        while (match(t, /#\[[ \t]*path[ \t]*=/)) { k++; t = substr(t, RSTART + RLENGTH) }
        if (k > 0) { pline = ln; pidx = 1; pcount = k }
      }
      i = 1; L = length(s)
      while (i <= L) {
        c = substr(s, i, 1)
        if (c == "{") {
          if (want) { emit("inline"); next }
          depth++; chain[depth] = pending; pending = ""; i++; continue
        }
        if (c == ";") {
          if (want) { emit(pline > 0 ? "attr" : "default"); next }
          pending = ""; i++; continue
        }
        if (c == "}") {
          # The declaration was found and neither delimiter followed it —
          # a shape this reader cannot place, so it says nothing and the
          # caller refuses.
          if (want) { done = 1; out = ""; next }
          if (depth > 0) { chain[depth] = ""; depth-- }
          pending = ""; i++; continue
        }
        r = substr(s, i)
        if (match(r, /^mod[ \t]+[A-Za-z_][A-Za-z0-9_]*/) &&
            (i == 1 || substr(s, i - 1, 1) !~ /[A-Za-z0-9_]/)) {
          w = substr(r, RSTART, RLENGTH); sub(/^mod[ \t]+/, "", w)
          i += RLENGTH
          pending = w
          if (ln >= start && w == name) want = 1
          continue
        }
        i++
      }
    }
    END { if (out != "") print out }'
}

# THE MOUNT'S PAYLOAD, from the one raw line the code view named. This
# one reads the file directly rather than a pipe, so stopping at that
# line closes nothing behind it. Prints
# nothing when that line does not carry the same attributes the view saw
# — a `#[path]` written inside a comment beside a live one, or a payload
# that is not a string literal on that line — because a payload the two
# views disagree about is not a decision.
path_payload() {
  awk -v ln="$2" -v idx="$3" -v want="$4" '
    NR == ln {
      k = 0; s = $0; p = ""
      while (match(s, /#\[[ \t]*path[ \t]*=[ \t]*"[^"]*"/)) {
        k++
        if (k == idx) {
          p = substr(s, RSTART, RLENGTH)
          sub(/^[^"]*"/, "", p); sub(/"$/, "", p)
        }
        s = substr(s, RSTART + RLENGTH)
      }
      if (k == want) print p
      exit
    }' "$1"
}

# A refusal, and it is loud in the one way that crosses the boundary it
# is written behind: this runs inside `production_sources`, which the
# gate reads through a process substitution, so an `exit` here cannot
# fail the caller and the list printed so far would be read as the whole
# answer. The marker is what `gate` checks the moment the list is in.
refuse_declaration() {
  gate_error "$(gate_name): $1 — that is not a pass"
  : >> "$GATE_MATCHER_FAILED"
  exit 1
}

# A module whose `mod` declaration is `#[cfg(test)]`-gated is test-only
# code that happens to live in its own file, and the reader's per-item
# skip cannot see across files. Resolved here, from the declaration
# rather than from a list of names, so a new one needs no edit.
#
# THE RESOLUTION IS RUSTC'S, and nothing else resolves both directions at
# once. `mod bar;` names the SIBLING `dir/bar.rs` (or `dir/bar/mod.rs`)
# only when the declaring file is a crate root or a `mod.rs`; declared in
# any other file `dir/foo.rs` it names `dir/foo/bar.rs` (or
# `dir/foo/bar/mod.rs`); an enclosing inline `mod y { … }` adds `y/` to
# whichever of those two the declarer chooses; and a `#[path = "P"]`
# attribute overrides the file name with `P`, relative to the declaring
# file's directory at top level and to the inline module's directory
# inside one. Reading every declaration as a sibling drops the production
# `dir/bar.rs` from the scan because an unrelated file declared a test
# module of its name, and scans the test module that was actually
# declared as production.
#
# ROOTNESS IS READ FROM THE BASENAME, which is a proxy for what Cargo
# actually says: a `[[bin]]` or `[lib]` `path =` can make any name a
# crate root. The tree's one root outside the three names —
# `crates/viewer/src/bin/viewer.rs` — declares no module at all, so the
# proxy decides nothing today; a root named otherwise that declares a
# gated module would be resolved as a non-root and mount its module one
# directory too deep.
#
# WHERE THIS READER IS BLIND, each with the direction it errs in:
#
#   * A DECLARATION SPLIT OVER LINES (`mod` and its name on separate
#     ones) is not matched by the raw narrowing below, which is
#     line-scoped, so a file whose only gated declaration takes that
#     shape never becomes a candidate and its module file is scanned as
#     production. OVER-SCAN — a false red, not a silent hole. Once the
#     file IS a candidate the same spelling is refused loudly instead.
#   * `#[cfg_attr(…, path = "…")]` is not read as a mount, so the
#     declaration falls through to the positional rule. BOTH DIRECTIONS
#     at once: the positional path is excluded though nothing mounts
#     there (under-scan if production code sits at it) and the real
#     target is scanned as production (over-scan).
#   * A DECLARATION WRITTEN BY A MACRO or pulled in by `include!` is
#     invisible to the shared reader, so its module file is scanned as
#     production. OVER-SCAN.
production_sources() {
  local decl file rest line name shape kind chain pline pidx pcount
  local base payload target f e skip keep=() narrowed=() cands=() excl=()
  # TWO STAGES, because reading every source twice costs more than this
  # gate is worth: raw `grep` narrows to the handful of files that carry
  # both the attribute and a `mod` declaration, and only those are read
  # properly. A file that fails the raw narrowing carries no
  # `#[cfg(test)]` text at all, so it cannot carry a gated declaration.
  #
  # The declaration is matched over the STATEMENT view, which is what
  # makes `#[cfg(test)] mod probes;` on ONE line and the same split over
  # two the same record — the earlier `grep -A1` form saw only the split
  # one, and cried wolf on the other.
  #
  # NO `xargs` BETWEEN THE STAGES. `xargs` reports 123 for any child that
  # exited 1-125, which folds `grep`'s "nothing matched" and its "I could
  # not search" into one status before anything here can tell them apart.
  # The narrowed list is small by construction, so the second stage takes
  # it as arguments.
  mapfile -t narrowed < <(gate_grep -lF '#[cfg(test)]' "${GATE_SOURCE_FILES[@]}")
  if [ "${#narrowed[@]}" -gt 0 ]; then
    mapfile -t cands < <(gate_grep -lE '(^|[[:space:]])mod [a-z_][a-z0-9_]*;' "${narrowed[@]}")
  fi
  if [ "${#cands[@]}" -gt 0 ]; then
    while IFS= read -r decl; do
      [ -n "$decl" ] || continue
      file=${decl%%:*}; rest=${decl#*:}; line=${rest%%:*}; name=${rest#*:}
      # A READER THAT DIED IS NOT AN ANSWER either, and captured in a
      # command substitution it would otherwise die under errexit with
      # its status thrown away and no diagnosis at all.
      if ! shape=$(declaration_shape "$file" "$line" "$name"); then
        refuse_declaration "reading $file to place its \`mod $name;\` failed, so where that module lives was not decided"
      fi
      IFS='|' read -r kind chain pline pidx pcount <<<"$shape"
      case "$kind" in
        inline) continue ;;
        refuse)
          refuse_declaration "$file:$line declares mod $name inside a brace that is not a module, where rustc mounts a non-inline module only through #[path]; where that module lives was not decided" ;;
        attr|default)
          case "${file##*/}" in
            mod.rs|lib.rs|main.rs) base=${file%/*} ;;
            *) base=${file%.rs} ;;
          esac
          if [ "$kind" = attr ]; then
            # A top-level `#[path]` is relative to the declaring file's
            # DIRECTORY whatever the file is named; inside an inline
            # module it is relative to that module's directory, which is
            # the positional base plus the chain.
            [ -n "$chain" ] || base=${file%/*}
            [ -z "$chain" ] || base=$base/$chain
            payload=$(path_payload "$file" "$pline" "$pidx" "$pcount") || payload=
            if [ -z "$payload" ]; then
              refuse_declaration "$file:$line mounts mod $name with a #[path] whose payload line $pline does not read back as the code view sees it, so where that module lives was not decided"
            fi
            target=$(gate_norm_path "$base/$payload")
          else
            [ -z "$chain" ] || base=$base/$chain
            target=$base/$name.rs
          fi ;;
        *)
          refuse_declaration "$file:$line declares mod $name in the statement view and the code view does not place it, so where that module lives was not decided" ;;
      esac
      # A declaration resolving onto its own declarer (`mod lib;` in
      # `lib.rs`) names no other file, and excluding the declarer takes
      # the whole tree with it. The directory form stays: `dir/lib/mod.rs`
      # is a different file and a real resolution of that declaration.
      [ "$target" = "$file" ] || excl+=("$target")
      excl+=("${target%.rs}/")
    done < <(gate_rust_code --statements "${cands[@]}" \
      | gate_grep -E '#\[cfg\(([^]]*[(,][[:space:]]*)?test[,)]' \
      | gate_grep -vE '#\[cfg\([^]]*(any|not)\(' \
      | gate_grep -oE '^[^:]*:[0-9]+:.*[[:space:]]mod [a-z_][a-z0-9_]*$' \
      | sed -E 's/:([0-9]+):.*[[:space:]]mod /:\1:/')
  fi
  if [ "${#excl[@]}" -eq 0 ]; then
    printf '%s\n' "${GATE_SOURCE_FILES[@]}"
    return 0
  fi
  # AN EXCLUSION IS A PATH, NOT A SUBSTRING, and that is why the filter
  # is a comparison here rather than a `grep -F` over the list: `-F`
  # matches anywhere in the line, so an excluded `foo/bar.rs` also takes
  # `foo/bar.rs_old.rs`, and an excluded `crates/p/src/foo/bar.rs` takes
  # a `crates/q/src/crates/p/src/foo/bar.rs` under another crate. A file
  # entry is the WHOLE path; a directory entry is a path PREFIX, which is
  # what its trailing `/` says. Nothing here can fail to run, so nothing
  # here needs the marker.
  for f in "${GATE_SOURCE_FILES[@]}"; do
    skip=false
    for e in "${excl[@]}"; do
      if [ "${e%/}" != "$e" ]; then
        if [ "${f#"$e"}" != "$f" ]; then skip=true; break; fi
      elif [ "$f" = "$e" ]; then
        skip=true; break
      fi
    done
    [ "$skip" = true ] || keep+=("$f")
  done
  [ "${#keep[@]}" -eq 0 ] || printf '%s\n' "${keep[@]}"
}

gate() {
  gate_require_crate_sources
  local files hits
  mapfile -t files < <(production_sources)
  # THE BOUNDARY, READ BEFORE ANY GUARD READS THE LIST. `production_sources`
  # prints its file set only once it is complete, so a refusal inside it
  # arrives here as an EMPTY list — the same shape a tree of nothing but
  # test-only sources has, and the guard below would then answer a
  # question nobody asked, with the true diagnosis scrolled off above it.
  # The marker is what crosses the process substitution; the status
  # cannot.
  if [ -e "$GATE_MATCHER_FAILED" ]; then
    rm -f "$GATE_MATCHER_FAILED"
    gate_error "$(gate_name): the scan's file set was not decided (diagnosed above), so what it does not contain is unknown — that is not a pass"
    exit 1
  fi
  GATE_SCAN_FILES=${#files[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): every source under crates/*/src in $PWD is test-only — the gate scanned no production code, which is not a pass"
    exit 1
  fi
  hits=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
    | gate_grep -P "$SQUARE_RE" \
    | gate_grep -vE '^crates/geom-core/src/(real|ring_interval)\.rs:' \
    | gate_grep -vE '^crates/geom-core/src/linalg/(svd|lsq)\.rs:' \
    | gate_grep -vE '^crates/geom-brep/src/ssi/(jet|march|system)\.rs:')
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "use powi(2): it is strictly tighter than x*x when the enclosure straddles zero, and equal elsewhere except for a square below 2^-960, where the backend pads once more (see this gate's header — NOT 'never wider'). Whether THIS enclosure can straddle zero is a global property of upstream callers that refactors change silently — four live bugs arrived exactly that way. Convert, or ratify this file into the allowlist."
    exit 1
  fi
  gate_ok "no unratified x*x outside the allowlisted files"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/lib.rs"
}

# THE SPELLING THE MATCHER USED TO WALK PAST, and the one this gate
# exists for: a field path, which is how vector code writes a square.
plant_field_path() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn sq<T: Real>(v: Vec3<T>) -> T { v.y * v.y }\n' > "$1/crates/planted/src/lib.rs"
}

plant_self_field() {
  mkdir -p "$1/crates/planted/src"
  printf 'impl<T: Real> V<T> { pub fn n(self) -> T { self.x * self.x } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_nested_field_path() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn sq<T: Real>(s: S<T>) -> T { s.dir.z * s.dir.z }\n' > "$1/crates/planted/src/lib.rs"
}

# THE PARENTHESIZED SCALED SQUARE, which is what branch 2 exists for:
# the `)` sits between the two operands, so branch 1 cannot see them
# beside each other however the line is wrapped.
plant_scaled_square() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Real>(k: T, x: T) -> T { (k * x) * x }\n' > "$1/crates/planted/src/lib.rs"
}

# The same shape over a field path, and nested one level deeper — the
# spelling `orthonormal_basis` had.
plant_scaled_square_field_path() {
  mkdir -p "$1/crates/planted/src"
  printf 'impl<T: Real> V<T> { pub fn f(self, s: T, a: T) -> T { T::one() + ((s * self.x) * self.x) * a } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# WHAT RUSTFMT MAKES OF A LONG PRODUCT. One expression, two lines, and
# a line-scoped matcher sees neither operand beside the other.
plant_wrapped_product() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn sq<T: Real>(v: LongTypeName<T>) -> T {\n'
    printf '    v.some_rather_long_field_name\n'
    printf '        * v.some_rather_long_field_name\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# A test-only module declared on ONE line. The `grep -A1` form that
# preceded the statement view saw only the two-line spelling and cried
# wolf on this one.
plant_gated_module_file_one_line() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)] mod probes;\n' > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/probes.rs"
}

# `any(test, …)` is NOT test-only: an `any(debug_assertions, test, …)`
# module is every debug build, so its square is production code.
plant_any_gated_module_file() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(any(debug_assertions, test))]\nmod probes;\n' > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/probes.rs"
}

# Behind a block comment on one line — the strip has to be real.
plant_after_block_comment() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn sq<T: Real>(x: T) -> T { /* why */ x * x }\n' > "$1/crates/planted/src/lib.rs"
}

# A test-only module in its own file, registered WITHOUT the cfg gate:
# ordinary production code, and it must fire.
plant_ungated_module_file() {
  mkdir -p "$1/crates/planted/src"
  printf 'mod probes;\n' > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/probes.rs"
}

# THE NEAR MISSES, bundled: in the must-NOT-fire direction any one line
# firing fails the case, so a bundle is strictly stronger than separate
# fixtures. Every line here is correct code, or prose, or test-only.
#
# EACH LINE MUST BE KILLABLE BY ONE DEFECT, or it is decoration. The two
# branch-2 guards are independent, so a near-miss excluded by BOTH tests
# neither: `libm::sin(t) * t` sat here and its group `(t)` has no `*`,
# so removing either guard alone left it green. The rows below are
# split so each dies to exactly one: `libm::sin(a * b) * b` and
# `foo(a * b) * b` fire if the `(?<!\w)` call lookbehind goes (the
# first in its `::`-qualified spelling), and `(a + b) * b` fires if the
# group-must-contain-`*` rule goes.
plant_not_squares() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '// the rule forbids x * x here\n'
    printf '/// and `self.y * self.y` in a doc comment\n'
    printf '/*\n * and v.z * v.z inside a block comment\n */\n'
    printf 'pub const S: &str = "x * x";\n'
    printf 'pub fn a<T: Real>(x: T) -> T { x * x.recip() } // and a * a.m()\n'
    printf 'pub fn b<T: Real>(v: V<T>) -> T { v.x * v.y }\n'
    printf 'pub fn c<T: Real>(v: V<T>) -> T { v.x * v.x.abs() }\n'
    printf 'pub fn d<T: Real>(v: V<T>) -> T { v.norm() * v.norm() }\n'
    printf 'pub fn e<T: Real>(x: T) -> T { x.powi(2) }\n'
    printf 'pub fn f<T: Real>(a: T, b: T) -> T { libm::sin(a * b) * b }\n'
    printf 'pub fn g<T: Real>(a: T, b: T) -> T { foo(a * b) * b }\n'
    printf 'pub fn h<T: Real>(a: T, b: T) -> T { (a + b) * b }\n'
    printf 'pub fn i<T: Real>(a: T, b: T, c: T) -> T { (a * b) * c }\n'
    printf '#[cfg(test)]\nmod tests {\n    fn t() { let q = 2.0; let _ = q * q; }\n}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# EVERY SOURCE EXCLUDED, WITH NO CYCLE IN THE TREE ITSELF. A crate root
# resolves its declarations as siblings, so two roots in one directory
# each declaring a test module of the OTHER's name exclude each other:
# `production_sources` returns empty and the guard below fires. Neither
# declaration names its own file, which is the case the guard beside it
# covers.
plant_every_source_excluded_by_a_sibling() {
  printf '#[cfg(test)]\nmod main;\n' > "$1/crates/clean/src/lib.rs"
  printf '#[cfg(test)]\nmod lib;\n' > "$1/crates/clean/src/main.rs"
}

# A DECLARATION THAT RESOLVES ONTO ITS OWN DECLARER excludes nothing:
# `mod lib;` in `lib.rs` names the file it is written in, and dropping
# that file would take a whole crate out of the scan on the strength of
# one line. The square here is production code and has to be read.
plant_self_naming_declaration() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)]\nmod lib;\npub fn sq<T: Real>(x: T) -> T { x * x }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# THE UNDER-SCAN DIRECTION, and it is the silent one: `bar.rs` is
# production code that no declaration gates, while the module `foo.rs`
# declares lives at `foo/bar.rs`. Resolving the declaration to the
# SIBLING drops this file from the scan entirely and its square is never
# read — the file at the resolved path is deliberately clean, so only
# the sibling can decide this case.
plant_production_sibling_of_gated_module() {
  mkdir -p "$1/crates/planted/src/foo"
  printf 'mod bar;\nmod foo;\n' > "$1/crates/planted/src/lib.rs"
  printf '#[cfg(test)]\nmod bar;\n' > "$1/crates/planted/src/foo.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/bar.rs"
  printf 'pub fn t<T: Real>(x: T) -> T { x.powi(2) }\n' \
    > "$1/crates/planted/src/foo/bar.rs"
}

# AN INLINE `mod x { ... }` DECLARES NO FILE. The statement view cuts at
# `{` and `;` alike, so an inline test module reaches the matcher as the
# same record a file declaration does — and read as one it excludes a
# production file that has nothing to do with it.
plant_inline_test_module_beside_named_file() {
  mkdir -p "$1/crates/planted/src/foo"
  printf 'mod foo;\n' > "$1/crates/planted/src/lib.rs"
  printf 'mod bar;\n#[cfg(test)]\nmod probes {\n    fn t() {}\n}\n' \
    > "$1/crates/planted/src/foo.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' \
    > "$1/crates/planted/src/foo/probes.rs"
}

# THE OVER-SCAN DIRECTION: the file the declaration actually names.
# `#[cfg(test)] mod bar;` inside `foo.rs` is `foo/bar.rs`, so this square
# is test-only and reading it as production is crying wolf.
plant_gated_module_in_declarer_directory() {
  mkdir -p "$1/crates/planted/src/foo"
  printf 'mod foo;\n' > "$1/crates/planted/src/lib.rs"
  printf '#[cfg(test)]\nmod bar;\n' > "$1/crates/planted/src/foo.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' \
    > "$1/crates/planted/src/foo/bar.rs"
}

# A GATED DECLARATION INSIDE AN INLINE MODULE mounts under that module:
# `mod y { #[cfg(test)] mod x; }` in a crate root is `y/x.rs`, so the
# top-level `x.rs` beside it is production code and has to be read. This
# is the direction that loses a production file, and the resolved path
# carries no square, so only the sibling can decide the case.
plant_nested_gated_declaration_sibling() {
  mkdir -p "$1/crates/planted/src/y"
  printf 'mod x;\npub mod y {\n    #[cfg(test)]\n    mod x;\n}\n' \
    > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/x.rs"
  printf 'pub fn t<T: Real>(x: T) -> T { x.powi(2) }\n' > "$1/crates/planted/src/y/x.rs"
}

# THE SAME CHAIN, from the other side: the file that declaration really
# mounts is test-only and must stay out of the scan.
plant_nested_gated_declaration_target() {
  mkdir -p "$1/crates/planted/src/y"
  printf 'pub mod y {\n    #[cfg(test)]\n    mod x;\n}\n' \
    > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/y/x.rs"
}

# AN INLINE MODULE IN A CRATE ROOT, which the non-root rule never
# reaches: here the inline check is the only thing standing between
# `mod probes { … }` and the production `probes.rs` beside it.
plant_inline_test_module_in_a_root() {
  mkdir -p "$1/crates/planted/src"
  printf 'mod other;\n#[cfg(test)]\nmod probes {\n    fn t() {}\n}\n' \
    > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' \
    > "$1/crates/planted/src/probes.rs"
}

# AN EXCLUSION IS A PATH, NOT A SUBSTRING. `foo/bar.rs.rs` is a different
# file from the excluded `foo/bar.rs` and it is production code; a
# substring filter drops it with the module it merely starts with.
plant_production_file_extending_an_exclusion() {
  mkdir -p "$1/crates/planted/src/foo"
  printf 'mod foo;\n' > "$1/crates/planted/src/lib.rs"
  printf '#[cfg(test)]\nmod bar;\n' > "$1/crates/planted/src/foo.rs"
  printf 'pub fn t<T: Real>(x: T) -> T { x.powi(2) }\n' \
    > "$1/crates/planted/src/foo/bar.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' \
    > "$1/crates/planted/src/foo/bar.rs.rs"
}

# A DECLARATION NEAR THE TOP OF A LONG FILE, which is the shape that
# catches a reader that stops reading once it has its answer: the shared
# reader upstream is still writing, its write fails on the closed pipe,
# and `pipefail` turns a declaration this gate DID place into a refusal.
# The file is longer than a pipe buffer on purpose — that is the whole
# difference between the two outcomes, and it is why the same tree could
# pass here and fail on a runner.
plant_early_declaration_in_a_long_file() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '#[cfg(test)]\nmod probes;\n'
    seq 4000 | awk '{ printf "pub fn f%s(x: f64) -> f64 { x + %s.0 }\n", $1, $1 }'
  } > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/probes.rs"
}

# THE DIRECTORY FORM of the same resolution: `mod bar;` names
# `foo/bar.rs` OR `foo/bar/mod.rs`, and a module big enough to be a
# directory is exactly the test module a gate would otherwise scan
# whole.
plant_gated_module_directory_form() {
  mkdir -p "$1/crates/planted/src/foo/bar"
  printf 'mod foo;\n' > "$1/crates/planted/src/lib.rs"
  printf '#[cfg(test)]\nmod bar;\n' > "$1/crates/planted/src/foo.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' \
    > "$1/crates/planted/src/foo/bar/mod.rs"
}

# WHERE THE RAW READER STOPS, and it stops LOUDLY. `mod` and its name on
# separate lines are one statement to the code view and no `mod NAME;`
# line to the raw one, so the mount point is not decided — and a gate
# that cannot decide where a module lives has not cleared the tree.
# (The file needs a `mod x;` of its own to reach this stage at all: the
# raw narrowing above is line-scoped, so a lone split declaration is
# invisible to the gate and its file is scanned as production.)
plant_declaration_split_across_lines() {
  mkdir -p "$1/crates/planted/src"
  printf 'mod other;\n#[cfg(test)]\nmod\nbar;\n' > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/bar.rs"
}

# A `#[path]` MOUNT overrides both positional rules, relative to the
# declaring file's own directory — and the tree has live ones, so a
# resolver without it re-mints the over-scan it just fixed. The dead
# mount above the live one is the reason the attribute is LOCATED in the
# code-only view and only its payload read from the raw line: a reader
# that went to the raw text for both would follow `decoy.rs`, exclude
# nothing that exists and scan this square as production.
plant_gated_module_behind_path_attribute() {
  mkdir -p "$1/crates/planted/src"
  printf 'mod foo;\n' > "$1/crates/planted/src/lib.rs"
  printf '#[cfg(test)]\n// #[path = "decoy.rs"]\n#[path = "bar_impl.rs"]\nmod bar;\n' \
    > "$1/crates/planted/src/foo.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' \
    > "$1/crates/planted/src/bar_impl.rs"
}

# The same square, in a module file whose declaration is cfg(test)-gated:
# test-only code, and it must NOT fire. This case and the one above are
# the two directions of the same rule, and neither is evidence without
# the other.
plant_gated_module_file() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)]\nmod probes;\n' > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/probes.rs"
}

gate_selftest() {
  local want="use powi(2)"
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces. Proved here rather than asserted, because before
  # `gate_grep` this exact fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_field_path
  gate_selftest_case "$want" plant_self_field
  gate_selftest_case "$want" plant_nested_field_path
  gate_selftest_case "$want" plant_after_block_comment
  gate_selftest_case "$want" plant_ungated_module_file
  gate_selftest_case "$want" plant_any_gated_module_file
  gate_selftest_case "$want" plant_wrapped_product
  gate_selftest_case "$want" plant_scaled_square
  gate_selftest_case "$want" plant_scaled_square_field_path
  gate_selftest_case "is test-only — the gate scanned no production code" \
    plant_every_source_excluded_by_a_sibling
  gate_selftest_case "$want" plant_self_naming_declaration
  gate_selftest_case "$want" plant_production_sibling_of_gated_module
  gate_selftest_case "$want" plant_inline_test_module_beside_named_file
  gate_selftest_case "where that module lives was not decided" \
    plant_declaration_split_across_lines
  # THE SAME FIXTURE, ASSERTED ON THE OTHER HALF of the refusal: without
  # the marker read in `gate` the list arrives empty and the every-source
  # guard answers instead, with a diagnosis that is false about the tree.
  gate_selftest_case "the scan's file set was not decided" \
    plant_declaration_split_across_lines
  gate_selftest_case "$want" plant_nested_gated_declaration_sibling
  gate_selftest_case "$want" plant_inline_test_module_in_a_root
  gate_selftest_case "$want" plant_production_file_extending_an_exclusion
  gate_selftest_passes "prose, string literals, mixed products, a * a.method(), a call whose result multiplies its own argument, a parenthesized product whose last factor is not the repeated one, and a cfg(test) module" plant_not_squares
  gate_selftest_passes "a square in a module file whose declaration is cfg(test)-gated" plant_gated_module_file
  gate_selftest_passes "the same, declared on one line" plant_gated_module_file_one_line
  gate_selftest_passes "a square in the module file a non-root declarer actually names" \
    plant_gated_module_in_declarer_directory
  gate_selftest_passes "a square in the module file a #[path] attribute mounts" \
    plant_gated_module_behind_path_attribute
  gate_selftest_passes "a square in the mod.rs of a resolved module directory" \
    plant_gated_module_directory_form
  gate_selftest_passes "a square in the file an inline module's own gated declaration mounts" \
    plant_nested_gated_declaration_target
  gate_selftest_passes "a gated declaration near the top of a file longer than a pipe buffer" \
    plant_early_declaration_in_a_long_file
  printf '%s selftest OK: passes a clean fixture, and prose, string literals, near-miss products and every shape of test-only module; fires on each square spelling the matcher claims — bare identifier, field path, `self.` field, nested path, rustfmt-wrapped product, behind a block comment, and the parenthesized scaled square in both forms; places a cfg(test) declaration where rustc mounts it, so a production sibling, a production file under an inline module and a production path that merely extends an exclusion all stay in the scan and red, while the file the declaration names — positional, #[path]-mounted, directory-form or nested in an inline module — does not; and it REFUSES, with its own diagnosis and never a second false one, a declaration it cannot place — while a declaration it CAN place stays placed however long the file under it runs, a tree whose sources exclude each other, and a `grep` that cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
