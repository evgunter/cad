#!/usr/bin/env bash
# ci-mirror-parity.sh — the two halves of CI invoke the same checks, the
# prune exception stays one job, and a mirror citation names a step that
# exists.
#
# WHAT THIS PROVES, in three claims:
#
#   1. INVOCATION PARITY. Every `scripts/**` or `demos/**` executable
#      named on a non-comment line of `.github/workflows/ci.yml` is also
#      named in `local-scripts/ci-local.sh`, and the reverse — except
#      for `scripts/gates/**`, which both halves take from the directory
#      (`gate-roster.sh` is that claim), and except for entries declared
#      in MIRROR_EXEMPT with a reason.
#
#   2. THE PRUNE EXCEPTION IS EXACTLY ONE JOB. Every ci.yml job that
#      checks the repo out deletes `local-scripts/` and `.claude/`
#      immediately, which is what makes `ci-filter.py` right to treat a
#      change under either tree as non-triggering for the build rows.
#      EVERY workflow file, not only ci.yml — `render.yml` runs four
#      checked-out jobs of its own, reached through a `uses:` job.
#      MIRROR_JOB is the single declared exception — the job whose
#      SUBJECT is the agreement between the halves, so it has to see
#      both.
#
#   3. MIRROR CITATIONS RESOLVE. A `# HOSTED MIRROR: <job> / <step>`
#      marker in the local half must name a step that ci.yml carries,
#      under that job.
#
# WHY THIS GATE EXISTS. `scripts/gates/` is a roster because it is a
# directory; a check that lives anywhere else is named BY HAND in both
# halves, and nothing detected a lost row on either side. Every
# enumeration of that population so far has been done by reading, and
# every one of them missed a member: one sweep enumerated over
# `scripts/…` paths in ci.yml and could not see the two `demos/*.py`
# checks; a later one listed five and missed
# `scripts/interval-only-selection.py`, which a different track then
# found by accident. A hand-maintained list of six would buy one
# instance and leave the class, so the population is DERIVED here and
# the only hand-maintained thing is the exemption list — which fails
# loud when it is wrong, instead of being silently incomplete.
#
# WHY THIS IS NOT A "GATE VS NOT A GATE" JUDGEMENT. Whether a check
# deserves the name was itself an unenforced judgement call, so this
# gate does not make it: the criterion is mechanical and total — an
# executable under `scripts/` or `demos/` is invoked by both halves, or
# it is declared. That admits things nobody would call gates
# (`ci-filter.py`, `k_probe_sweep.sh`), and that is the point: the
# question "is this a gate" is what let a row go unmirrored.
#
# WHAT THIS DOES NOT PROVE. Both matchers are textual. A path inside a
# string the shell builds at runtime is invisible to them, as is a
# check invoked from a script one of the halves calls rather than from
# the half itself — the parity claim is about the two ROSTERS, not
# about the transitive call graph. Claim 2 reads `.github/workflows/*.yml`
# and no other trigger of a checkout — a composite action, or a workflow
# added under a different path, is outside it. And, as everywhere in this
# directory, wiring is not execution: a step disabled by an `if:` still
# satisfies claim 3.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

GATE_SCAN_NOUN='mirrored invocation'

HOSTED_HALF=.github/workflows/ci.yml
LOCAL_HALF=local-scripts/ci-local.sh

# The one job allowed to keep `local-scripts/` after checkout, because
# the agreement between the halves is what it checks.
MIRROR_JOB=mirror

# Declared asymmetries. `<path>|<half>|<reason>`, where <half> is the
# half that DOES invoke it. An entry here is a confession, not a
# disposition: it says a check runs in one half only, on purpose.
MIRROR_EXEMPT=(
  'demos/render-uv.sh|local|the committed UV sheet drift row. Its hosted mirror was retired 2026-08-17 when render.yml`s uv lane started re-baselining itself; ci-local.sh says so at the row'
)

# A floor under claim 3, so deleting the markers is not a way to pass
# it. Raise it when a row gains a marker; lowering it is a decision.
MIRROR_MARKER_FLOOR=10

# Non-comment lines of a half: what it would actually run.
mirror_commands() { grep -vE '^[[:space:]]*#' "$1"; }

# Every scripts/** or demos/** executable a half names, one per line.
# The leading boundary keeps `local-scripts/ci-local.sh` from being read
# as an invocation of `scripts/ci-local.sh`, which is how a naive
# matcher invents a hosted-only row that does not exist.
mirror_invocations() {
  mirror_commands "$1" |
    grep -oE '(^|[^A-Za-z0-9_/.-])(scripts|demos)/[A-Za-z0-9_/.-]+\.(sh|py)' |
    sed -E 's#^[^A-Za-z0-9_/.-]##' |
    grep -v '^scripts/gates/' |
    sort -u
}

# `<job><TAB><has-checkout><TAB><has-prune>` for every job in ci.yml.
mirror_jobs() {
  awk '
    /^  [a-z0-9_-]+:$/ { if (job != "") print job "\t" co "\t" pr;
                         job = substr($0, 3, length($0) - 3); co = 0; pr = 0; next }
    /actions\/checkout/ { co = 1 }
    /rm -rf local-scripts/ { pr = 1 }
    END { if (job != "") print job "\t" co "\t" pr }
  ' "$1"
}

# `<job><TAB><step name>` for every named step in ci.yml.
mirror_steps() {
  awk '
    /^  [a-z0-9_-]+:$/ { job = substr($0, 3, length($0) - 3); next }
    /^      - name: / { print job "\t" substr($0, 15) }
  ' "$1"
}

gate() {
  local rc=0 path half reason entry hosted local_ found in_h in_l job co pr marker n steps
  gate_require_file "$HOSTED_HALF"
  gate_require_file "$LOCAL_HALF"

  hosted=$(mirror_invocations "$HOSTED_HALF")
  local_=$(mirror_invocations "$LOCAL_HALF")
  if [ -z "$hosted" ] || [ -z "$local_" ]; then
    gate_error "$(gate_name): one half names no scripts/ or demos/ invocation at all under $PWD — the matcher scanned nothing, which is not a pass"
    exit 1
  fi

  # CLAIM 1, both directions.
  for path in $(printf '%s\n%s\n' "$hosted" "$local_" | sort -u); do
    in_h=false; in_l=false
    if grep -qxF "$path" <<<"$hosted"; then in_h=true; fi
    if grep -qxF "$path" <<<"$local_"; then in_l=true; fi
    if [ "$in_h" = true ] && [ "$in_l" = true ]; then continue; fi
    if [ "$in_h" = true ]; then found=hosted; else found=local; fi
    half=; reason=
    for entry in "${MIRROR_EXEMPT[@]}"; do
      [ "${entry%%|*}" = "$path" ] || continue
      half=${entry#*|}; reason=${half#*|}; half=${half%%|*}
    done
    if [ -n "$half" ]; then
      if [ "$half" != "$found" ]; then
        gate_error "$(gate_name): $path is declared as $half-only in MIRROR_EXEMPT but is invoked by the $found half. The exemption is now describing the opposite of the tree — re-read the reason (\"$reason\") and fix whichever side moved"
        rc=1
      fi
      continue
    fi
    if [ "$found" = hosted ]; then
      gate_error "$(gate_name): $HOSTED_HALF invokes $path and $LOCAL_HALF does not. Every check outside scripts/gates/ is named by hand in both halves, so a row added to one side is invisible on the other until someone reads for it — mirror it, or declare it in MIRROR_EXEMPT with the reason it is hosted-only"
    else
      gate_error "$(gate_name): $LOCAL_HALF invokes $path and $HOSTED_HALF does not. A check that runs only on a developer's machine gates nothing on merge — mirror it, or declare it in MIRROR_EXEMPT with the reason it is local-only"
    fi
    rc=1
  done

  # CLAIM 2, over every workflow file: `render.yml`'s four lanes check
  # the repo out too, and a hole there is the same hole.
  local wf
  found=
  for wf in .github/workflows/*.yml; do
  while IFS=$'\t' read -r job co pr; do
    [ -n "$job" ] || continue
    [ "$co" = 1 ] || continue
    if [ "$wf" = "$HOSTED_HALF" ] && [ "$job" = "$MIRROR_JOB" ]; then
      found=yes
      if [ "$pr" = 1 ]; then
        gate_error "$(gate_name): job \`$MIRROR_JOB\` prunes local-scripts/, but it is the one job whose subject is the agreement between the halves — with the tree deleted it can only check the hosted side, and the gates sited there would pass for the wrong reason"
        rc=1
      fi
      continue
    fi
    if [ "$pr" != 1 ]; then
      gate_error "$(gate_name): $wf job \`$job\` checks the repo out and does not delete local-scripts/ and .claude/. That deletion is what makes \`scripts/ci-filter.py\` right to classify a change under either tree as non-triggering; a job that keeps them can couple the hosted gate to a developer's machine without anything saying so"
      rc=1
    fi
  done < <(mirror_jobs "$wf")
  done
  if [ -z "$found" ]; then
    gate_error "$(gate_name): ci.yml has no job \`$MIRROR_JOB\` that checks the repo out. That job is where the gates whose inputs are docs, prose and local-scripts/ are sited — without it they are back in a job that skips on exactly the change class they are about"
    rc=1
  fi

  # CLAIM 3.
  steps=$(mirror_steps "$HOSTED_HALF")
  n=0
  while IFS= read -r marker; do
    [ -n "$marker" ] || continue
    n=$((n + 1))
    job=${marker%% / *}; entry=${marker#* / }
    if [ "$job" = "$marker" ]; then
      gate_error "$(gate_name): $LOCAL_HALF has a HOSTED MIRROR marker \`$marker\` that is not \`<job> / <step name>\`. The job half is the part that drifted last time — the prose named the wrong job for a step that existed"
      rc=1
      continue
    fi
    grep -qxF "$job	$entry" <<<"$steps" ||
      { gate_error "$(gate_name): $LOCAL_HALF cites a hosted mirror \`$job / $entry\`, and ci.yml has no step named \`$entry\` in job \`$job\`. Renaming a step or moving it between jobs leaves every sentence citing it quietly false"; rc=1; }
  done < <(grep -oE '# HOSTED MIRROR: .*$' "$LOCAL_HALF" | sed -E 's/^# HOSTED MIRROR: //')
  if [ "$n" -lt "$MIRROR_MARKER_FLOOR" ]; then
    gate_error "$(gate_name): $LOCAL_HALF carries $n HOSTED MIRROR marker(s), below the $MIRROR_MARKER_FLOOR it had when this floor was set. Deleting a marker is how a citation check becomes vacuous; if a row genuinely lost its hosted mirror, lower the floor deliberately"
    rc=1
  fi

  [ "$rc" -eq 0 ] || exit 1
  GATE_SCAN_FILES=$(printf '%s\n%s\n' "$hosted" "$local_" | sort -u | wc -l | tr -d ' ')
  gate_ok "both halves invoke the same checks outside scripts/gates/, every checked-out job in .github/workflows/ but \`$MIRROR_JOB\` prunes local-scripts/, and all $n hosted-mirror citations resolve to a step ci.yml carries"
}

# The subject is two CI halves, so the fixture is a miniature repo.
gate_plant_clean() {
  local t=$1 i
  mkdir -p "$t/.github/workflows" "$t/local-scripts" "$t/scripts/gates" "$t/demos"
  {
    printf 'jobs:\n'
    printf '  %s:\n    steps:\n      - uses: actions/checkout@v4\n' "$MIRROR_JOB"
    printf '      - name: gate roster parity\n        run: scripts/gates/gate-roster.sh\n'
    printf '  discipline:\n    steps:\n'
    printf '      - uses: actions/checkout@v4\n'
    printf '      - name: prune local-only tooling\n        run: rm -rf local-scripts .claude\n'
    for ((i = 0; i < MIRROR_MARKER_FLOOR; i++)); do
      printf '      - name: mirrored step %s\n        run: scripts/check-%s.sh\n' "$i" "$i"
    done
    printf '      - name: local-only lane\n        run: echo none\n'
  } > "$t/.github/workflows/ci.yml"
  {
    printf 'discipline() {\n'
    for ((i = 0; i < MIRROR_MARKER_FLOOR; i++)); do
      printf '  # HOSTED MIRROR: discipline / mirrored step %s\n' "$i"
      printf '  scripts/check-%s.sh\n' "$i"
    done
    printf '  demos/render-uv.sh\n'
    printf '}\n'
  } > "$t/local-scripts/ci-local.sh"
}

# A hosted row with no local mirror: the S13 shape, hosted side.
plant_hosted_only() {
  printf '      - name: new hosted row\n        run: scripts/check-new.sh\n' \
    >> "$1/.github/workflows/ci.yml"
}
# The same, local side — a check that gates nothing on merge.
plant_local_only() {
  printf '  scripts/check-local.sh\n' >> "$1/local-scripts/ci-local.sh"
}
# A new job that checks out and keeps local-scripts/.
plant_unpruned_job() {
  printf '  newjob:\n    steps:\n      - uses: actions/checkout@v4\n      - run: echo hi\n' \
    >> "$1/.github/workflows/ci.yml"
}
# A SECOND workflow file grows a checked-out job that keeps the tree.
# The reusable render workflow is exactly this shape, so the claim is
# over the directory and not over one file.
plant_unpruned_second_workflow() {
  printf 'jobs:\n  tour:\n    steps:\n      - uses: actions/checkout@v4\n      - run: echo hi\n' \
    > "$1/.github/workflows/render.yml"
}
# And a job called `mirror` in that other file gets no exemption: the
# one declared exception is ci.yml's.
plant_mirror_named_job_elsewhere() {
  printf 'jobs:\n  %s:\n    steps:\n      - uses: actions/checkout@v4\n      - run: echo hi\n' \
    "$MIRROR_JOB" > "$1/.github/workflows/render.yml"
}
# The mirror job starts pruning, so it can no longer see the half it exists to check.
plant_mirror_job_prunes() {
  sed -i "s#      - name: gate roster parity#      - run: rm -rf local-scripts\\n      - name: gate roster parity#" \
    "$1/.github/workflows/ci.yml"
}
# THE DRIFT THIS GATE WAS WRITTEN FOR: the step exists, the job named is
# the wrong one.
plant_marker_wrong_job() {
  sed -i '0,/# HOSTED MIRROR: discipline \//s//# HOSTED MIRROR: k-lint \//' \
    "$1/local-scripts/ci-local.sh"
}
# The step itself renamed under a marker that still cites the old name.
plant_marker_step_renamed() {
  sed -i '0,/- name: mirrored step 0/s//- name: mirrored step zero/' \
    "$1/.github/workflows/ci.yml"
}
# Markers deleted: the citation check made vacuous.
plant_markers_deleted() {
  sed -i '/# HOSTED MIRROR: /d' "$1/local-scripts/ci-local.sh"
}
# The declared local-only row acquires a hosted call, so the exemption
# now describes the opposite of the tree.
plant_exemption_inverted() {
  sed -i '/demos\/render-uv.sh/d' "$1/local-scripts/ci-local.sh"
  printf '      - name: uv sheet\n        run: demos/render-uv.sh\n' \
    >> "$1/.github/workflows/ci.yml"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case 'and local-scripts/ci-local.sh does not' plant_hosted_only
  gate_selftest_case 'and .github/workflows/ci.yml does not' plant_local_only
  gate_selftest_case 'checks the repo out and does not delete' plant_unpruned_job
  gate_selftest_case 'render.yml job `tour` checks the repo out' plant_unpruned_second_workflow
  gate_selftest_case 'render.yml job `mirror` checks the repo out' plant_mirror_named_job_elsewhere
  gate_selftest_case 'prunes local-scripts/, but it is the one job' plant_mirror_job_prunes
  gate_selftest_case 'has no step named' plant_marker_wrong_job
  gate_selftest_case 'has no step named' plant_marker_step_renamed
  gate_selftest_case 'below the' plant_markers_deleted
  gate_selftest_case 'declared as local-only in MIRROR_EXEMPT' plant_exemption_inverted
  printf '%s selftest OK: passes a clean fixture; fires on a hosted-only row, a local-only row, a checked-out job that keeps local-scripts/, a second workflow file growing one, a job merely NAMED like the exempt one in another file, the mirror job pruning it, a marker naming the wrong job, a marker whose step was renamed, the markers deleted wholesale, and an exemption that now describes the opposite of the tree\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
