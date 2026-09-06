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
# UNDATED ONE-SHOT AND NOTHING RE-TAKES IT, and this gate is not
# where it could be re-taken: the claim is about the BACKEND's
# arithmetic, so re-running it means compiling
# `interval-transcendentals`, which a shell gate in the discipline
# job has no means to do and no business doing. What is written down
# instead is the re-run itself, so "anyone can re-run it in a line"
# is a line rather than a promise — a `#[test]` under
# `interval-transcendentals/tests/`, written in that crate's OWN public
# API because it is its own workspace and cannot depend on geom-core:
#
#     use interval_transcendentals::DInterval;
#     let x = DInterval::point(f64::from_bits(0x21E0_0000_0000_0000) * 1.5);
#     assert!(x.powi(2).lo() < (x * x).lo() && x.powi(2).hi() > (x * x).hi());
#     let y = DInterval::point(1.5);   // a magnitude above the floor
#     assert!(y.powi(2).lo() == (y * y).lo() && y.powi(2).hi() == (y * y).hi());
#
# and the FLOOR the derivation rests on has a home already —
# `TWO_PROD_VALID_MIN` at `interval-transcendentals/src/round.rs`,
# whose own value and the `mul_hi` lemma above it are what a moved
# floor would change, with the `2^-960` windows in that crate's
# `tests/certify.rs` and `tests/review_fuzz_exact.rs` sweeping it. The
# 3,000,000-sample negative is what it always is — evidence about
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
# them is what produced the false positives above. WHICH attribute is
# test-only — and which, being true under some other configuration, is
# not — is `lib.sh`'s §"THE TEST-ONLY `cfg` ATTRIBUTE", and is not
# restated here. The COST is this gate's: a test-only helper later
# promoted to production arrives unscanned, and the promotion is a diff
# a human reads.
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
# NONE OF THESE IS LIVE, AND A CLEAN RUN OF THE MATCHER ABOVE IS NOT
# WHY. Those five spellings are exactly what it cannot see, so its own
# green says nothing about them; only a differently-shaped sweep can,
# and that sweep is `census` below rather than a paragraph. It re-runs
# on every pass, its definition is `CENSUS_*_RE` and `two_statement_
# candidates` — read those, not this — and its result is checked
# against `CENSUS_REGISTER`, one entry per dispositioned site at a
# pinned count. WHAT THE DISPOSITIONS SAY IS IN THE REGISTER and is not
# summarised here: a summary is a second copy that goes stale in the
# silent direction, which is the defect this whole block is a fix for.
# A hole this gate cannot see is still a hole, so it is written down
# rather than closed — but it is now written down in a form that reds
# when it moves.
#
# THE CENSUS IS A SECOND MATCHER AND HAS ITS OWN BLIND SPOTS, which is
# why it counts CANDIDATES and not violations. Each shape reads the
# same statement view the gate reads, so everything `lib.sh`'s reader
# cannot do (macro bodies, `include!`d text, an unterminated `/*`) it
# cannot do either, and KNOWN GAPS 1-3 apply unchanged: a `#[cfg(test)]`
# item, an ALL-CAPS operand and an indexed or repeated-call operand are
# outside every shape here. Beyond those:
#
#   * THE THREE-FACTOR SHAPE IS A PRODUCT CHAIN, so the text between the
#     two operands must be `* FACTOR` and nothing else. `k * (a + b) * x
#     * x`'s outer pair is not counted — the shape KNOWN GAP 5 names is
#     the one with "no parentheses anywhere", and a middle free to run
#     through `+`, `,` or a paren counts arithmetic that is not one
#     product. UNDER-count, deliberate, and the direction to widen in if
#     a real instance ever arrives.
#   * THE NESTED-PAREN SHAPE READS ONE NESTED GROUP AT ONE LEVEL, so
#     `((a * (b + c)) * x) * x` is invisible to it: the pattern spells a
#     single `(…)` inside the outer group and cannot count depth, which
#     an ERE with a backreference cannot do at all. UNDER-count. It is
#     left because the shape is empty in this tree and the instrument
#     that would close it is a parser, not a wider pattern.
#   * THE TWO-STATEMENT SHAPE FOLLOWS ONE BINDING HOP, not two: `let
#     kx = k * x;` … `kx * x` is seen, `let kx = k * x; let m = kx * y;`
#     … `m * x` is not. UNDER-count. It also requires the binding's RHS
#     to be paren-free and to END in the repeated operand, so
#     `let kx = f(k) * x` is not a binding it tracks.
#   * A CANDIDATE IS SCOPED TO ITS FILE for the two-statement shape,
#     because the statement view carries no item boundary: two unrelated
#     functions in one file can pair. OVER-count, which is the safe
#     direction — it arrives as an unregistered candidate, never as a
#     silent pass.
#   * AN ENTRY WHOSE FILE IS GONE FROM THE SCAN goes quiet, and a tree
#     holding NONE of the register's files does not check the register's
#     counts at all — a register is a claim about the tree it describes,
#     and a fixture tree is not that tree. So a deleted FILE is a diff a
#     human reads, while a deleted or changed SITE inside a file that is
#     still there reds here.
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

# --- THE FIVE-SHAPE CENSUS --------------------------------------------
#
# KNOWN GAP 5's five spellings, one pattern each, stated where their
# result is. Three read the statement view; the fourth is a binding hop
# and reads it in `two_statement_candidates` below, because a shape
# whose two halves are two records is not a regex over one.
#
# `paren-first-factor` and `nested-paren` are the two the header says
# are empty. They are here BECAUSE they are empty: a shape with no
# entries is the one whose matcher nothing exercises on a real tree, so
# the self-test plants one of each and the register is what would have
# to grow if the tree did.
#
# EVERY WEDGED FACTOR MUST DIFFER FROM THE OPERAND, and that guard is
# what keeps this census about what the LIVE MATCHER CANNOT SEE. Without
# it `h * h * h` is a three-factor candidate — and it is not: branch 1
# already reads the adjacent `h * h` inside it, so the gate is quiet
# there only because the file is allowlisted. Counting it here would
# make the OK line's claim false for that candidate, and would put a
# NEW adjacent square in an allowlisted file through the census as an
# unregistered candidate — silently replacing KNOWN GAP 4's
# file-granular ratification with a per-site one for that one spelling.
# The two shapes are kept apart instead: adjacency is branch 1's, and a
# genuinely wedged repeat is this one's.
CENSUS_PAREN_FIRST_RE="(?<!\w)\(\s*($SQUARE_PATH)\s*\*\s*[^()]*\)\s*\*\s*\1(?![\w.(\[])"
CENSUS_NESTED_PAREN_RE="(?<!\w)\(\s*[^()]*\([^()]*\)[^()]*\*\s*($SQUARE_PATH)\s*\)\s*\*\s*\1(?![\w.(\[])"
CENSUS_THREE_FACTOR_RE="(?<![\w.])($SQUARE_PATH)(?:\s*\*\s*(?!\1(?![\w.]))[A-Za-z0-9_.]+)+\s*\*\s*\1(?![\w.(\[])"

# THE ALLOWLISTED FILES, as paths and held once: the filter's exemption
# is built from this list and the clean fixture plants every entry of it,
# so a file cannot be exempt in one and absent from the other. The
# argument for each is in this file's header, above.
ALLOWLISTED_HOMES=(
  crates/geom-core/src/real.rs
  crates/geom-core/src/ring_interval.rs
  crates/geom-core/src/linalg/svd.rs
  crates/geom-core/src/linalg/lsq.rs
  crates/geom-brep/src/ssi/jet.rs
  crates/geom-brep/src/ssi/march.rs
  crates/geom-brep/src/ssi/system.rs
)

# THE EXEMPTION IS ONE PATH PER ENTRY, by construction rather than by
# coincidence: `gate_record_anchor` escapes the path and pins the
# `FILE:LINE:` shape, and which paths an anchor missing either part
# would exempt is argued once, at that function in `lib.sh`.
allowlisted_re() {
  local home
  local -a alts=()
  for home in "${ALLOWLISTED_HOMES[@]}"; do
    alts+=("$(gate_record_anchor "$home")")
  done
  local IFS='|'
  printf '%s' "${alts[*]}"
}

# THE REGISTER — `<shape>|<file>|<fragment>|<count>|<disposition>`, the
# `S49` shape: the POPULATION is re-read out of the tree on every run
# and this is only the DISPOSITION of what was read. `<fragment>` is a
# substring of the candidate statement's own text and is what keys the
# entry — never a line number, which rots under every edit above it.
# `<count>` is why this is a register and not an allowlist: without it a
# second candidate matching an existing fragment would inherit that
# entry's disposition silently, so each entry pins how many it stands
# for and a difference IN EITHER DIRECTION reds — `matched 0` is an
# entry whose site is gone.
#
# THE GATE READS THE DISPOSITION AS TEXT AND NOTHING MORE. That the
# sentence is TRUE is a reviewer's judgement; what CI holds is that the
# entry exists, that its site is still there, and that no candidate
# arrives without one.
CENSUS_REGISTER=(
  # The projector conjugation: `free1` and `p2` are `Mat3<f64>`, so
  # there is no enclosure to straddle zero and no `powi` to reach for.
  "three-factor|crates/editor-core/src/mate/coset.rs|free1 * p2 * free1|1|not an enclosure: Mat3<f64> matrix arithmetic, and the repeated factor is a matrix rather than a scalar"
  # The odd-power Taylor terms of `sin_step`. What makes them safe is
  # not that the powers differ but that every factor is NONNEGATIVE:
  # `a1 = pt(s.abs())` and `a2 = a1.sqr()`, so no product here straddles
  # zero and the plain product's four-corner minimum already IS the
  # tight bound — which is this gate's own rule for a `lo >= 0`
  # enclosure, stated at the top of this file.
  "two-statement|crates/geom-brep/src/props/quad.rs|let a3 = a1 * a2|1|no factor straddles zero: a1 is pt(s.abs()) and a2 is a1.sqr(), both nonnegative, so the plain product is already the tight bound"
  "two-statement|crates/geom-brep/src/props/quad.rs|let a5 = a3 * a2|1|no factor straddles zero: a1 is pt(s.abs()) and a2 is a1.sqr(), both nonnegative, so the plain product is already the tight bound"
  "two-statement|crates/geom-brep/src/props/quad.rs|let a7 = a5 * a2|1|no factor straddles zero: a1 is pt(s.abs()) and a2 is a1.sqr(), both nonnegative, so the plain product is already the tight bound"
)

# Set by `--register FILE`, which replaces the array above. It exists so
# the self-test can plant a MALFORMED entry — a red no fixture tree can
# reach, because the register it guards is baked into this file.
GATE_CENSUS_REGISTER_FILE=

# THE BINDING HOP, over the statement view on stdin. A `let N = … * X`
# whose RHS is paren-free and ends in `X` is remembered; the first later
# statement in the SAME FILE holding `N * X` or `X * N` completes the
# square, and the candidate is reported AT THE BINDING, which is the
# statement a fix would rewrite. One completion per binding — the
# binding is the site, and a value used twice is still one place where
# the square was split.
two_statement_candidates() {
  awk '
    # A REMEMBERED NAME IS DATA AND IT IS SPLICED INTO A REGEX, so it is
    # escaped first. Both names come from the code, and the only
    # metacharacter their character classes admit is a DOT: the operand
    # may be a FIELD PATH (v.x), and unescaped that dot is a wildcard
    # matching vax, a candidate the source does not contain. The escape
    # is a one-member bracket expression and not a backslash, which is
    # the ruling in loop-boundary-discards.sh and holds under every awk.
    # Completeness is checkable rather than hoped for: the two classes
    # are [a-z_][A-Za-z0-9_]* and [A-Za-z_][A-Za-z0-9_.]*, so the dot is
    # the whole list, and widening either class means widening this.
    # (No apostrophe appears in this program, which is itself
    # single-quoted.)
    function esc(s,   t) { t = s; gsub(/[.]/, "[.]", t); return t }
    {
      i = index($0, ":"); file = substr($0, 1, i - 1); rest = substr($0, i + 1)
      j = index(rest, ":"); line = substr(rest, 1, j - 1) + 0
      txt = substr(rest, j + 1); sub(/^ /, "", txt)
      if (file != cf) { cf = file; nb = 0 }
      for (k = 1; k <= nb; k++) {
        if (done[k]) continue
        if (txt ~ ("(^|[^A-Za-z0-9_.])" en[k] " [*] " ex[k] "([^A-Za-z0-9_.([]|$)") ||
            txt ~ ("(^|[^A-Za-z0-9_.])" ex[k] " [*] " en[k] "([^A-Za-z0-9_.([]|$)")) {
          done[k] = 1
          print bf[k] ":" bl[k] ": " bt[k]
        }
      }
      if (match(txt, /^let [a-z_][A-Za-z0-9_]* = /)) {
        nm = substr(txt, 5); sub(/ =.*/, "", nm)
        rhs = substr(txt, RLENGTH + 1)
        if (rhs !~ /[()]/ && match(rhs, /[*] [A-Za-z_][A-Za-z0-9_.]*$/)) {
          nb++
          en[nb] = esc(nm); ex[nb] = esc(substr(rhs, RSTART + 2))
          bf[nb] = file; bl[nb] = line; bt[nb] = txt; done[nb] = 0
        }
      }
    }
  '
}

# census STATEMENT_VIEW — the candidates of all four shapes, checked
# against the register. The summary the gate's OK line carries comes
# back in `CENSUS_SUMMARY` and NOT on stdout, because this function also
# PRINTS its findings: captured in a command substitution the offending
# lines would go into the caller's variable and its `gate_error` into
# nothing, which is S157 and is the defect `lib.sh` exists to close.
CENSUS_SUMMARY=
census() {
  local view=$1 entries cands report bad files
  if [ -n "$GATE_CENSUS_REGISTER_FILE" ]; then
    # THE OS's REASON IS KEPT. `2>/dev/null` here would report "cannot
    # read" for a missing file, an unreadable one and a directory alike,
    # and the one thing a reader needs is which.
    local why
    why=$(cat "$GATE_CENSUS_REGISTER_FILE" 2>&1 >/dev/null) || true
    if ! entries=$(cat "$GATE_CENSUS_REGISTER_FILE" 2>/dev/null); then
      gate_error "$(gate_name): cannot read the census register at $GATE_CENSUS_REGISTER_FILE, so the five shapes were counted against nothing — ${why:-no reason reported}"
      exit 1
    fi
  else
    entries=$(printf '%s\n' "${CENSUS_REGISTER[@]}")
  fi
  # The shape name is prefixed here rather than carried through the
  # matchers, so each pattern stays exactly the text the header points
  # at. `sed` cannot fail to match; `gate_grep` reports a matcher that
  # could not run, and its marker crosses this substitution.
  if ! cands=$(
    printf '%s\n' "$view" | gate_grep -P "$CENSUS_PAREN_FIRST_RE" | sed 's/^/paren-first-factor|/'
    printf '%s\n' "$view" | gate_grep -P "$CENSUS_NESTED_PAREN_RE" | sed 's/^/nested-paren|/'
    printf '%s\n' "$view" | gate_grep -P "$CENSUS_THREE_FACTOR_RE" | sed 's/^/three-factor|/'
    printf '%s\n' "$view" | two_statement_candidates | sed 's/^/two-statement|/'
  ); then
    gate_error "$(gate_name): a census matcher could not run over the statement view, so the shapes this gate cannot see were not counted — that is not a pass"
    exit 1
  fi
  # WHICH REGISTER FILES THIS TREE HOLDS, as whole paths. The count
  # checks run only over the entries whose file is in the scan, and only
  # when the tree holds at least one of them: a fixture tree holds none,
  # and a register is a claim about the tree it describes.
  files=$(printf '%s\n' "${GATE_PRODUCTION_FILES[@]}")
  if ! report=$(printf '%s\n===\n%s\n===\n%s\n' "$entries" "$files" "$cands" | awk '
    /^===$/ { phase++; next }
    phase == 0 {
      if ($0 == "") next
      ne++
      n = split($0, f, "\\|")
      if (n != 5 || f[1] !~ /^(paren-first-factor|nested-paren|three-factor|two-statement)$/ ||
          f[2] == "" || f[3] == "" || f[4] !~ /^[1-9][0-9]*$/ || f[5] == "") {
        print "MALFORMED|" $0
      }
      es[ne] = f[1]; ef[ne] = f[2]; eg[ne] = f[3]; ec[ne] = f[4] + 0
      next
    }
    phase == 1 { if ($0 != "") have[$0] = 1; next }
    {
      if ($0 == "") next
      i = index($0, "|"); shape = substr($0, 1, i - 1); r = substr($0, i + 1)
      i = index(r, ":"); file = substr(r, 1, i - 1); r = substr(r, i + 1)
      i = index(r, ":"); line = substr(r, 1, i - 1); txt = substr(r, i + 1)
      sub(/^ /, "", txt)
      seen[shape]++
      hit = 0; best = -1
      for (k = 1; k <= ne; k++) {
        if (es[k] != shape || ef[k] != file) continue
        if (index(txt, eg[k]) == 0) continue
        if (length(eg[k]) > best) { best = length(eg[k]); hit = k }
      }
      # THE WHOLE RECORD, NOT A TRIMMED ONE. `gate_grep` trims its
      # diagnosis because it echoes a few hundred file operands; a
      # statement record is one statement, so there is nothing to trim
      # and a width literal here would be an undisclosed second copy of
      # that one. The record IS the evidence a reader needs to
      # disposition the candidate.
      if (hit == 0) print "UNREG|" shape "|" file "|" line "|" txt
      else matched[hit]++
      next
    }
    END {
      live = 0
      for (k = 1; k <= ne; k++) if (ef[k] in have) live = 1
      for (k = 1; k <= ne; k++) {
        if (!live) continue
        if (!(ef[k] in have)) { print "ABSENT|" es[k] "|" ef[k] "|" eg[k]; continue }
        got = (k in matched) ? matched[k] : 0
        if (got != ec[k]) print "MISCOUNT|" es[k] "|" ef[k] "|" eg[k] "|pinned " ec[k] "|matched " got
      }
      printf "COUNT|%d|%d|%d|%d|%d|%d\n", seen["paren-first-factor"] + 0,
        seen["nested-paren"] + 0, seen["three-factor"] + 0,
        seen["two-statement"] + 0, ne + 0, live
    }
  '); then
    gate_error "$(gate_name): the five-shape census could not run, so what this matcher cannot see was not counted — that is not a pass"
    exit 1
  fi
  bad=$(printf '%s\n' "$report" | sed -n '/^COUNT|/!p')
  if [ -n "$bad" ]; then
    printf '%s\n' "$bad"
    gate_error "$(gate_name): the five-shape census does not match CENSUS_REGISTER — a MALFORMED line is an entry that is not <shape>|<file>|<fragment>|<count>|<disposition> with a known shape and a positive count; an UNREG line is a candidate of a shape THIS GATE'S MATCHER CANNOT SEE and no entry names, so disposition it in the register (or fix it, if the square is real); a MISCOUNT line is an entry standing for a different number of candidates than it pins, and both directions matter — 'matched 0' is an entry whose site is gone and whose disposition now covers nothing; an ABSENT line is an entry whose file has left the scan while the rest of the register is still live"
    exit 1
  fi
  local counts p1 p2 p3 p4 nent live
  counts=$(printf '%s\n' "$report" | sed -n 's/^COUNT|//p')
  IFS='|' read -r p1 p2 p3 p4 nent live <<<"$counts"
  CENSUS_SUMMARY="the five spellings this matcher cannot see re-derived over the same statement view: $p1 + $p2 + $p3 + $p4 candidates (parenthesized-first, nested-paren, three-factor, two-statement)"
  if [ "$live" = 1 ]; then
    CENSUS_SUMMARY="$CENSUS_SUMMARY, each dispositioned by one of $nent register entries at its pinned count"
  else
    CENSUS_SUMMARY="$CENSUS_SUMMARY, none unregistered — this tree holds no file the $nent register entries describe, so their counts are not its to answer for"
  fi
}

gate() {
  gate_require_crate_sources
  local hits view
  # WHERE A TEST-ONLY MODULE LIVES is `lib.sh`'s, resolved rustc's way,
  # and so are the two refusals it can end in: a declaration nothing can
  # place, and a tree whose every source is test-only.
  gate_production_sources
  # ONE READ, TWO MATCHERS. The census below asks a different question
  # of the SAME view, and re-reading the tree for it would let the two
  # answers be about different trees — as well as costing a second pass
  # over crates/*/src. A reader that died is caught here rather than
  # inherited: captured in a substitution its status would otherwise be
  # thrown away, which is the defect `lib.sh` exists to close.
  if ! view=$(gate_rust_code --skip-cfg-test --statements "${GATE_PRODUCTION_FILES[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  if [ -z "$view" ]; then
    gate_error "$(gate_name): the shared Rust reader returned NOTHING over $GATE_SCAN_FILES production source file(s) — the scan decided nothing, which is not a pass"
    exit 1
  fi
  hits=$(printf '%s\n' "$view" \
    | gate_grep -P "$SQUARE_RE" \
    | gate_grep -vE "$(allowlisted_re)")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "use powi(2): it is strictly tighter than x*x when the enclosure straddles zero, and equal elsewhere except for a square below 2^-960, where the backend pads once more (see this gate's header — NOT 'never wider'). Whether THIS enclosure can straddle zero is a global property of upstream callers that refactors change silently — four live bugs arrived exactly that way. Convert, or ratify this file into the allowlist."
    exit 1
  fi
  census "$view"
  gate_ok "no unratified x*x outside the allowlisted files; and $CENSUS_SUMMARY"
}

# THE ALLOWLIST IS IN THE CLEAN FIXTURE, which is `lib.sh`'s exact-skip
# contract read for a whole-file skip: a skip no fixture exercises is
# dead in every case, and an anchor that over-narrows is then noticed by
# nobody. Each home carries the adjacent square it is ratified FOR — a
# spelling the LIVE matcher reads, so the census never sees it and the
# clean case reds the moment one of the seven stops being covered.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  local home
  for home in "${ALLOWLISTED_HOMES[@]}"; do
    mkdir -p "$1/${home%/*}"
    printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/$home"
  done
}

# THE SHARED "no production source left" CASE, over a clean fixture that
# plants seven files more than `lib.sh` assumes. That planter excludes
# exactly the two sources `gate_plant_clean_sources` writes; the homes
# this gate's clean fixture adds are production, so the tree the case is
# about does not exist and the refusal it wants cannot fire. They are
# taken back out here, which is what "every source is test-only" means
# for this gate's tree. THE FIX BELONGS IN `lib.sh` — the shared planter
# should clear whatever the gate's own clean fixture planted, rather than
# every such gate carrying this override.
gate_plant_home_every_source_excluded() {
  local home
  for home in "${ALLOWLISTED_HOMES[@]}"; do
    rm -f "$1/$home"
  done
  printf '#[cfg(test)]\nmod main;\n' > "$1/crates/clean/src/lib.rs"
  printf '#[cfg(test)]\nmod lib;\n' > "$1/crates/clean/src/main.rs"
}

# THE `FILE:LINE:` SHAPE, which a skip ending at `:` does not pin: a
# file whose own path carries a colon after the home reads as the home
# plus a line number and rides the exemption. The path is legal on this
# filesystem and in git, and the anchor's `[0-9]+` is what refuses it —
# `gate_record_anchor` in `lib.sh` argues the reachable set once. One
# entry stands for the seven: the anchor is built the same way for each.
plant_colon_after_the_home_that_is_not_a_line_number() {
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' \
    > "$1/${ALLOWLISTED_HOMES[0]}:x.rs"
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

# Behind a block comment on one line — the strip has to be real.
plant_after_block_comment() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn sq<T: Real>(x: T) -> T { /* why */ x * x }\n' > "$1/crates/planted/src/lib.rs"
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

# THE BREACH `lib.sh`'s test-module cases plant, and the only thing this
# gate supplies to them: a square, appended to a file whose directory
# they have already made.
plant_square_at() {
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' >> "$1"
}

# --- THE CENSUS'S OWN FIXTURES ----------------------------------------
#
# ONE UNDISPOSITIONED CANDIDATE PER SHAPE, and each must be invisible to
# the LIVE matcher or the case proves nothing about the census: the gate
# would red on the square itself and the register would never be
# consulted. That is why none of the four below puts the two operands
# beside each other or makes the repeated operand a group's last factor.
plant_undispositioned_paren_first() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Real>(k: T, x: T) -> T { (x * k) * x }\n' > "$1/crates/planted/src/lib.rs"
}

plant_undispositioned_nested_paren() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Real>(a: T, b: T, x: T) -> T { ((a + b) * x) * x }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_undispositioned_three_factor() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Real>(k: T, m: T, x: T) -> T { k * x * m * x }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_undispositioned_two_statement() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Real>(k: T, x: T) -> T { let kx = k * x; let y = kx * x; y }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# THE REGISTER'S OWN TREE, written FROM the register — the only way a
# gate with these reds can have a fixture that reaches them at all.
# Every entry must find exactly the number of candidates it pins, so the
# fixture plants that many, and it is a check on the register's own
# shape: an entry whose fragment the matchers cannot reproduce fails
# here rather than on a real tree. The three statement-scoped shapes
# plant their fragment as a statement; the binding hop plants the
# binding AND a use, because one statement of it is not the shape.
# census_plant_entry ROOT ENTRY — one entry's site(s), appended to the
# file it names. Both directions of the register are written from THIS
# function, so a fixture cannot pass by planting something the register
# does not describe.
census_plant_entry() {
  local root=$1 shape file frag count name x i
  IFS='|' read -r shape file frag count _ <<<"$2"
  mkdir -p "$root/${file%/*}"
  i=0
  while [ "$i" -lt "$count" ]; do
    case "$shape" in
      paren-first-factor|nested-paren|three-factor)
        printf 'pub fn c() { %s; }\n' "$frag" >> "$root/$file" ;;
      two-statement)
        name=$(printf '%s\n' "$frag" | awk '{ print $2 }')
        x=$(printf '%s\n' "$frag" | awk '{ print $NF }')
        printf 'pub fn c() { %s; let use_%s = %s * %s; }\n' \
          "$frag" "$name" "$name" "$x" >> "$root/$file" ;;
      *)
        printf 'SELFTEST FAILED: no planter for census shape %s\n' "$shape" >&2
        exit 1 ;;
    esac
    i=$((i + 1))
  done
}

plant_register_tree() {
  local e
  for e in "${CENSUS_REGISTER[@]}"; do census_plant_entry "$1" "$e"; done
}

# census_field INDEX FIELD — one field of one register entry, read from
# the array rather than written out, so a register that is re-ordered or
# re-worded does not silently stop being tested. EVERY MUTATION BELOW
# TAKES AN INDEX: keyed on entry 0 alone they exercised the three-factor
# shape only, and the binding hop's own MISCOUNT and ABSENT paths were
# reached by nothing.
census_field() { printf '%s' "${CENSUS_REGISTER[$1]}" | cut -d'|' -f"$2"; }

# plant_register_tree_less INDEX ROOT — every entry but one. The file of
# the omitted entry is created anyway if nothing else plants it, so the
# case is "the SITE is gone" and not "the FILE is gone" — two different
# reds, and a fixture that cannot tell them apart proves neither.
plant_register_tree_less() {
  local idx=$1 root=$2 i file
  for i in "${!CENSUS_REGISTER[@]}"; do
    [ "$i" = "$idx" ] || census_plant_entry "$root" "${CENSUS_REGISTER[$i]}"
  done
  file=$(census_field "$idx" 2)
  mkdir -p "$root/${file%/*}"
  [ -f "$root/$file" ] || printf 'pub fn benign(x: f64) -> f64 { x }\n' > "$root/$file"
}

# A SECOND CANDIDATE UNDER AN ENTRY THAT PINS ONE — what a register
# without a count would absorb, taking that entry's disposition for a
# square nobody has looked at.
plant_register_extra() {
  plant_register_tree "$2"
  census_plant_entry "$2" "${CENSUS_REGISTER[$1]}"
}

# AN ENTRY WHOSE SITE IS GONE while its file is still read: the
# disposition now covers nothing, and an entry standing for nothing is a
# ratification waiting to be inherited.
plant_register_site_gone() { plant_register_tree_less "$1" "$2"; }

# AN ENTRY WHOSE FILE HAS LEFT THE SCAN while the rest of the register
# is still live — the other direction of the same question, and the one
# the count check alone cannot tell from a site that moved. Every entry
# sharing that file goes ABSENT with it.
plant_register_file_gone() {
  plant_register_tree "$2"
  rm -f "$2/$(census_field "$1" 2)"
}

# --- THE (file, shape) KEY, ONE HALF EACH -----------------------------
#
# Both cases hand the gate a WRITTEN register rather than mutating a
# tree, because what has to vary is the ENTRY and not the tree. A
# fixture that can tell a missing check from a present one needs the
# candidate the entry would WRONGLY claim to be the ONLY candidate there
# is: with the right one also in the tree, dropping the check turns a
# pass into a MISCOUNT and the case reds either way, proving nothing.
census_written_register_case() {
  local what=$1 want=$2 entry=$3 file=$4 body=$5
  local tmp out reg
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  mkdir -p "$tmp/${file%/*}"
  printf '%s\n' "$body" > "$tmp/$file"
  reg=$tmp/planted-register.txt
  printf '%s\n' "$entry" > "$reg"
  if out=$("$0" --root "$tmp" --register "$reg" 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate PASSED on %s\n%s\n' "$what" "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "$what" "$out"
  case "$out" in
    *"$want"*) ;;
    *) printf 'SELFTEST FAILED (%s): the gate fired with an unexpected message — wanted %s, got:\n%s\n' \
         "$what" "$want" "$out" >&2
       exit 1 ;;
  esac
}

# THE MALFORMED RED, which no fixture tree can reach: the register it
# guards is baked into this file, so the case hands the gate a written
# one instead.
# AN ADJACENT TRIPLE IN AN ALLOWLISTED FILE, and it is the fixture the
# three-factor shape's wedge guard exists for. `h * h * h` contains the
# adjacent `h * h` branch 1 already reads, so the gate is quiet here on
# the allowlist's ratification and on nothing else. Counted as a census
# candidate it would arrive UNREGISTERED — turning KNOWN GAP 4's
# file-granular ratification into a per-site one for this one spelling,
# through a pass whose whole subject is meant to be what the matcher
# CANNOT see.
plant_adjacent_triple_in_an_allowlisted_file() {
  mkdir -p "$1/crates/geom-brep/src/ssi"
  printf 'pub fn c<T: Real>(h: T) -> T { h * h * h }\n' \
    > "$1/crates/geom-brep/src/ssi/march.rs"
}

# A BINDING HOP WHOSE OPERAND IS A FIELD PATH, beside the name its dot
# would match as a wildcard. `v.x` unescaped is the regex `v.x`, which
# matches `vax`, so the reader would report a two-statement candidate
# the source does not contain — and this file has no entry for it, so it
# would arrive as an unregistered candidate and red a correct tree.
plant_field_path_binding_near_miss() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Real>(k: T, v: V<T>, vax: T) -> T { let kx = k * v.x; kx * vax }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# `--register` WITH NO ARGUMENT. Left to `shift 2` this ends the script
# under errexit with nothing printed, which is the one failure a gate
# must never have: it decided nothing and said so to nobody.
gate_selftest_census_register_without_argument() {
  local tmp out
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  if out=$("$0" --root "$tmp" --register 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate PASSED with --register given no argument\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "--register with no argument" "$out"
  case "$out" in
    *"--register takes a FILE"*) ;;
    *) printf 'SELFTEST FAILED (--register with no argument): the gate failed for some OTHER reason:\n%s\n' "$out" >&2
       exit 1 ;;
  esac
}

gate_selftest_census_malformed_register() {
  local tmp out reg
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  reg=$tmp/planted-register.txt
  printf '%s\n' "${CENSUS_REGISTER[@]}" | sed '1s/|1|/|0|/' > "$reg"
  if out=$("$0" --root "$tmp" --register "$reg" 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate PASSED on a census register entry pinning a count of zero\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "a malformed census register entry" "$out"
  case "$out" in
    *MALFORMED*) ;;
    *) printf 'SELFTEST FAILED (a malformed census register entry): the gate fired for some OTHER reason:\n%s\n' "$out" >&2
       exit 1 ;;
  esac
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
  gate_selftest_case "$want" plant_wrapped_product
  gate_selftest_case "$want" plant_scaled_square
  gate_selftest_case "$want" plant_scaled_square_field_path
  gate_selftest_case "$want" plant_colon_after_the_home_that_is_not_a_line_number
  gate_selftest_passes "prose, string literals, mixed products, a * a.method(), a call whose result multiplies its own argument, a parenthesized product whose last factor is not the repeated one, and a cfg(test) module" plant_not_squares
  gate_selftest_test_module_homes "$want" plant_square_at
  # THE CENSUS, in both directions. `census` is the half whose subject
  # is what the matcher above CANNOT see, so every one of its cases has
  # to be invisible to that matcher: a fixture the live matcher reds is
  # a fixture that proves nothing here.
  local cwant="the five-shape census does not match CENSUS_REGISTER"
  gate_selftest_case "$cwant" plant_undispositioned_paren_first
  gate_selftest_case "$cwant" plant_undispositioned_nested_paren
  gate_selftest_case "$cwant" plant_undispositioned_three_factor
  gate_selftest_case "$cwant" plant_undispositioned_two_statement
  # EVERY `want` BELOW IS A FRAGMENT OF THE OFFENDING LINE, never a word
  # from the umbrella diagnosis. `lib.sh` warns that `$want` alone can
  # be satisfied by text the gate prints for some other reason, and this
  # gate is where that happened: `matched 0` and `ABSENT` both appear in
  # the message that explains what those lines MEAN, so either case was
  # satisfied by any census red at all — including one with the check it
  # is about deleted. The register supplies the fragments, so a
  # re-worded register does not silently stop testing.
  local i e
  for i in 0 1; do
    e="$(census_field "$i" 1)|$(census_field "$i" 2)|$(census_field "$i" 3)"
    gate_selftest_case "MISCOUNT|$e|pinned 1|matched 2" plant_register_extra "$i"
    gate_selftest_case "MISCOUNT|$e|pinned 1|matched 0" plant_register_site_gone "$i"
    gate_selftest_case "ABSENT|$e" plant_register_file_gone "$i"
  done
  # THE (file, shape) KEY. Each half is a written register whose single
  # entry would wrongly claim the tree's single candidate if that half
  # of the key were dropped — so the case reds ONLY because the check is
  # there, which is what the two cases above cannot say for it.
  census_written_register_case \
    "a candidate in one file and an entry naming another" \
    "UNREG|three-factor|crates/planted/src/here.rs" \
    "three-factor|crates/planted/src/there.rs|free1 * p2 * free1|1|planted" \
    "crates/planted/src/here.rs" \
    "pub fn c() { free1 * p2 * free1; }"
  census_written_register_case \
    "a three-factor candidate and an entry of another shape naming its file and text" \
    "UNREG|three-factor|crates/planted/src/one.rs" \
    "two-statement|crates/planted/src/one.rs|let a3 = a1 * a2|1|planted" \
    "crates/planted/src/one.rs" \
    "pub fn c() { let a3 = a1 * a2 * b * a2; }"
  gate_selftest_census_malformed_register
  gate_selftest_census_register_without_argument
  gate_selftest_passes "the tree the census register describes, every entry finding exactly the candidates it pins" plant_register_tree
  gate_selftest_passes "an adjacent triple in an allowlisted file, which branch 1 already reads and the census must not claim" plant_adjacent_triple_in_an_allowlisted_file
  gate_selftest_passes "a binding hop whose operand is a FIELD PATH, beside a name the unescaped dot in it would match" plant_field_path_binding_near_miss
  printf '%s selftest OK: passes a clean fixture carrying all seven allowlisted files, and prose, string literals and near-miss products; fires on each square spelling the matcher claims — bare identifier, field path, `self.` field, nested path, rustfmt-wrapped product, behind a block comment, and the parenthesized scaled square in both forms — and at the colon-carrying path a home skip that ends at `:` exempts; re-derives the five spellings it CANNOT see and passes the register'"'"'s own tree while firing, on a fragment of the offending line rather than on the umbrella text, on an undispositioned candidate of each of the four shapes and — for a three-factor entry and a binding-hop one alike — on a second candidate under a one-site entry, an entry whose site is gone and an entry whose file has left the scan; holds BOTH halves of the (file, shape) key against a written register; refuses a --register with no argument and a malformed entry; keeps a field-path binding from matching a name its unescaped dot would, and an adjacent triple in an allowlisted file out of the census entirely; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

# `--register FILE` is this gate's own flag, so it is taken out of argv
# before lib.sh's parser — which rejects what it does not know — sees
# the rest. It is NOT in that parser's usage line, and that line is
# lib-wide: a per-gate flag added to it would be printed by every gate
# that does not have it. So it is written here, which is where a reader
# of this gate looks.
#
# THE MISSING ARGUMENT IS DIAGNOSED, not left to `shift 2`: under
# errexit a short shift ends the script with nothing printed at all,
# which is a gate that decided nothing and said so to nobody.
GATE_ARGV=()
while [ $# -gt 0 ]; do
  case "$1" in
    --register)
      if [ $# -lt 2 ]; then
        gate_error "$(gate_name): --register takes a FILE holding the census register, one entry per line, and none was given"
        exit 2
      fi
      GATE_CENSUS_REGISTER_FILE=$2; shift 2 ;;
    *) GATE_ARGV+=("$1"); shift ;;
  esac
done
gate_parse_args ${GATE_ARGV[@]+"${GATE_ARGV[@]}"}
gate_main
