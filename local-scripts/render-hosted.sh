#!/usr/bin/env bash
# ONE COMMAND FOR A RENDER: install the one CI already made — or, on
# request, dispatch -> poll -> install.
#
#   local-scripts/render-hosted.sh                      # take your branch's
#                                                 # newest CI render
#   local-scripts/render-hosted.sh --lane uv            # one lane of it
#   local-scripts/render-hosted.sh --on-demand          # render fresh instead
#   local-scripts/render-hosted.sh --run 12345678       # pull a specific run
#
# FIRST, THE SHORT ANSWER: YOU PROBABLY WANT `git pull`, NOT THIS
# SCRIPT (2026-08-17). CI now RE-BASELINES all four lanes itself. A PR
# run whose render differs REPORTS it with a neutral check ("!", not
# "x") naming the cells; main's own run then COMMITS them. So the frames
# arrive by merging and pulling, not by installing. So the ordinary flow is: push, wait for CI, `git pull`,
# look at the frames. Nothing to download, nothing to install.
#
# ALL FOUR LANES RE-BASELINE, uv included — there is no lane left that
# needs a manual install after an ordinary CI run. What this script is
# still genuinely good for on a PR: LOOKING at the new cells before you
# merge, since the PR run reports them rather than committing them.
#
# WHAT THIS SCRIPT IS STILL FOR:
#   * A DISPATCH AIMED AT A BARE SHA, which has no branch to commit to;
#     those runs report the drift and name this command, as before.
#   * `--on-demand` renders CI has not covered: an unpushed branch, no
#     CI run yet, or a deliberate re-render at a different scene budget.
#   * `--verify`, the byte-exactness round trip (see below).
#
# TAKING IS THE DEFAULT; RENDERING IS THE FLAG. Every CI run on a pushed
# branch renders all four lanes (ci.yml's `renders` job calls
# render.yml), so once your branch has a CI run the frames already
# exist — a dispatch would render the same tree a second time, for ~5
# more runner-minutes and no new information.
#
# It takes the CI run WHATEVER ITS CONCLUSION: a run can still fail for
# a wedged pass or the matplotlib-fallback assertion, and lanes upload
# before any of that is decided, so the artifact is there either way.
#
# Renders are hosted (`.github/workflows/render.yml`); the local entry
# points refuse without an explicit override (demos/hosted-render-guard.sh).
# This is the front end that makes that refusal reasonable: it triggers
# the workflow on your branch, prints per-job progress until the run
# settles, and — on success — downloads each lane's artifact and
# INSTALLS it back into the working tree at the committed path, so the
# frames land exactly where a local pass would have put them and are
# reviewed and committed the ordinary way.
#
# IT RENDERS THE PUSHED TREE, SO IT REFUSES AN UNPUSHED HEAD. A runner
# checks out a ref from the remote; it cannot see your working tree and
# it cannot see unpushed commits. Rendering "your branch" while the
# remote is three commits behind is the failure mode this exists to make
# impossible — the check is a hard refusal, not a warning, because the
# result of getting it wrong is a plausible-looking set of frames drawn
# from the wrong scenes. Uncommitted changes are a warning by the same
# logic one step down: they were definitely not rendered, but you can
# see them in `git status` next to the frames.
#
# BYTE-EXACTNESS IS THE CONTRACT. The whole design rests on an artifact
# being the file, not a rendering of the file: the committed PNGs carry
# provenance `tEXt` chunks that demos/check_render_provenance.py gates
# on, and a pipeline that re-encoded or re-stamped anything would launder
# them. actions/upload-artifact zips and `gh run download` unzips, both
# lossless — and `--verify` proves it end to end rather than asserting
# it, by round-tripping the wild lane (matplotlib Agg, pinned, no GL
# anywhere, so byte-identity is a real expectation) and diffing what
# came back against what is committed.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

WORKFLOW=render.yml
CI_WORKFLOW=ci.yml
LANE=all
RUN_ID=""
ON_DEMAND=0
# The lane jobs, by the name each ends with. A dispatched render.yml run
# names them exactly; called from ci.yml they arrive prefixed ("render
# lanes / freecad montages (kernel + freecad)"), so the match is on the
# SUFFIX and one list serves both. This is what lets the poll wait for the
# render rather than for a whole CI run — the lanes settle in ~3 minutes,
# CI in ~12.
#
# TWO JOBS, FIVE LANES (2026-08-22). render.yml merged its five lane jobs
# into two — the three renderer-free ones into `scene inputs + uv sheet +
# wild montage`, the two FreeCAD ones into `freecad montages (kernel +
# freecad)` — to stop paying five runner setups, two 821 MB FreeCAD cache
# restores and two apt installs for work that shares all of it. Every lane
# still uploads its own artifact under its own name, which is what the
# download path below actually keys on; this regex only decides which jobs
# the PROGRESS DISPLAY waits for and reports.
RENDER_JOBS_RE='(scene inputs \+ uv sheet \+ wild montage|freecad montages \(kernel \+ freecad\))$'
REF=""
SCENE_TIMEOUT=""
INSTALL=1
VERIFY=0
# Outer stop. The merged FreeCAD job is capped at 150 min by the workflow
# (two lanes on one runner) and the scene-inputs job at 75; queueing on top
# of that is the slack.
POLL_BUDGET_MIN=200
POLL_INTERVAL=20

usage() {
    cat <<'EOF'
usage: local-scripts/render-hosted.sh [options]

  (default)                             install the render your branch's
                                        newest CI run already made
  --on-demand                           render fresh instead of taking CI's
  --lane <kernel|freecad|uv|wild|all>   which lane(s) (default: all)
  --ref <branch|tag|sha>                which branch (default: current)
  --run <id>                            take a specific run; no new render
  --scene-timeout <seconds>             FreeCAD per-scene budget (default: 300)
  --no-install                          download to a temp dir, do not touch the tree
  --verify                              round-trip proof: assert the pulled bytes
                                        equal the committed ones (wild/uv lanes)
  --budget-min <n>                      give up polling after n minutes (default: 150)
  -h, --help

Artifacts land at their committed paths:
  kernel  -> demos/renders/        freecad -> demos/renders-freecad/
  uv      -> demos/renders-uv/     wild    -> demos/renders-wild/
EOF
}

die() { echo "render-hosted: $*" >&2; exit 1; }
say() { echo "==> $*"; }

while [ $# -gt 0 ]; do
    case "$1" in
        --lane) LANE="${2:?--lane needs a value}"; shift 2 ;;
        --ref) REF="${2:?--ref needs a value}"; shift 2 ;;
        --run) RUN_ID="${2:?--run needs a value}"; shift 2 ;;
        --on-demand) ON_DEMAND=1; shift ;;
        --scene-timeout) SCENE_TIMEOUT="${2:?--scene-timeout needs a value}"; shift 2 ;;
        --budget-min) POLL_BUDGET_MIN="${2:?--budget-min needs a value}"; shift 2 ;;
        --no-install) INSTALL=0; shift ;;
        --verify) VERIFY=1; shift ;;
        -h|--help) usage; exit 0 ;;
        *) usage >&2; die "unknown argument: $1" ;;
    esac
done

case "$LANE" in
    kernel|freecad|uv|wild|all) ;;
    *) die "--lane must be one of kernel, freecad, uv, wild, all (got '$LANE')" ;;
esac
command -v gh >/dev/null || die "gh is not installed (https://cli.github.com)"
gh auth status >/dev/null 2>&1 || die "gh is not authenticated — run: gh auth login"

# Lane -> (artifact name, committed directory). One table, used by the
# download, the install and the verify, so a new lane is one line.
artifact_for() {
    case "$1" in
        kernel) echo "renders-kernel" ;;
        freecad) echo "renders-freecad" ;;
        uv) echo "renders-uv" ;;
        wild) echo "renders-wild" ;;
    esac
}
dir_for() {
    case "$1" in
        kernel) echo "demos/renders" ;;
        freecad) echo "demos/renders-freecad" ;;
        uv) echo "demos/renders-uv" ;;
        wild) echo "demos/renders-wild" ;;
    esac
}
lanes_of() {
    if [ "$1" = all ]; then echo "kernel freecad uv wild"; else echo "$1"; fi
}

# ------------------------------------------------------- take CI's render

# THE DEFAULT IS TO TAKE, NOT TO RENDER. ci.yml's `renders` job calls
# render.yml on every push that builds anything, so a pushed branch's
# newest CI run already holds all four lanes' artifacts — the same
# bytes a dispatch would produce, from the same pipeline, at no extra
# runner cost. Rendering again would render the same tree twice, so
# that is the flag (`--on-demand`) and this is the default.
#
# NOTE THAT TAKING IS USUALLY UNNECESSARY NOW (2026-08-17): a lane that
# drifted has already been re-baselined and COMMITTED by that same run,
# so `git pull` gets you the frames and this script gets you a copy of
# what you already have. What is left for it: a bare-SHA dispatch (no
# branch to commit to), `--verify`, `--no-install`, and pulling a
# specific run's bytes for comparison.
#
# The run is taken WHATEVER ITS CONCLUSION, which is the point rather
# than a leniency: a run can still fail on a wedged pass or the
# matplotlib-fallback assertion, and the artifacts are uploaded before
# any of that is decided, so a failed run still has them.
if [ "$ON_DEMAND" = 0 ] && [ -z "$RUN_ID" ]; then
    if [ -z "$REF" ]; then
        REF="$(git rev-parse --abbrev-ref HEAD)"
        [ "$REF" != HEAD ] || die "detached HEAD — pass --ref explicitly"
    fi
    say "looking for the newest $CI_WORKFLOW run on $REF"
    RUN_ID="$(gh run list --workflow "$CI_WORKFLOW" --branch "$REF" --limit 1 \
        --json databaseId --jq '.[0].databaseId // empty')"
    # No fallback to dispatching. Rendering costs ~5 runner-minutes and
    # the caller asked for the cheap path; silently taking the expensive
    # one is the kind of helpfulness that surprises.
    [ -n "$RUN_ID" ] || die "no $CI_WORKFLOW run on '$REF' yet — CI renders every lane \
on every push, so this usually means the branch is unpushed. Push it, or render on \
demand: local-scripts/render-hosted.sh --on-demand --lane $LANE"
    say "taking $CI_WORKFLOW run $RUN_ID (no new render)"
fi

# ---------------------------------------------------------------- dispatch

# The ref a run renders is the ref it was dispatched on; `gh run list
# --branch` is how we find it again, so both must agree.
if [ -z "$RUN_ID" ]; then
    if [ -z "$REF" ]; then
        REF="$(git rev-parse --abbrev-ref HEAD)"
        [ "$REF" != HEAD ] || die "detached HEAD — pass --ref explicitly"
    fi

    # THE PUSH CHECK. Fetch first so "already pushed" is a fact about the
    # remote right now, not about a stale remote-tracking ref.
    say "checking $REF is pushed"
    git fetch -q origin "$REF" 2>/dev/null \
        || die "origin has no ref '$REF' — push it first: git push -u origin $REF"
    local_head="$(git rev-parse HEAD)"
    remote_head="$(git rev-parse FETCH_HEAD)"
    if [ "$REF" = "$(git rev-parse --abbrev-ref HEAD)" ] && [ "$local_head" != "$remote_head" ]; then
        {
            echo
            echo "REFUSING: your HEAD is not what origin/$REF points at."
            echo
            echo "    local  HEAD       $local_head"
            echo "    origin/$REF   $remote_head"
            echo
            echo "The runner checks out the PUSHED tree — it cannot see local"
            echo "commits, so this render would draw scenes you are not looking at."
            echo
            echo "    git push origin $REF"
            echo
        } >&2
        exit 1
    fi
    if ! git diff --quiet HEAD -- || [ -n "$(git ls-files --others --exclude-standard)" ]; then
        echo "render-hosted: WARNING — the working tree is dirty." >&2
        echo "render-hosted:   Uncommitted changes are NOT in the render; the run" >&2
        echo "render-hosted:   draws $remote_head." >&2
    fi

    # Remember the newest existing run so the one we are about to create
    # is identifiable without racing on timestamps.
    before="$(gh run list --workflow "$WORKFLOW" --limit 1 --json databaseId \
        --jq '.[0].databaseId // 0')"

    say "dispatching $WORKFLOW (lanes=$LANE) on $REF"
    dispatch=(gh workflow run "$WORKFLOW" --ref "$REF" -f "lanes=$LANE")
    [ -z "$SCENE_TIMEOUT" ] || dispatch+=(-f "scene_timeout=$SCENE_TIMEOUT")
    "${dispatch[@]}"

    # The dispatch API is asynchronous: the run exists a moment later.
    for _ in $(seq 1 30); do
        RUN_ID="$(gh run list --workflow "$WORKFLOW" --branch "$REF" --limit 1 \
            --json databaseId --jq '.[0].databaseId // 0')"
        [ "$RUN_ID" = "$before" ] || [ "$RUN_ID" = 0 ] || break
        sleep 2
    done
    [ -n "$RUN_ID" ] && [ "$RUN_ID" != 0 ] && [ "$RUN_ID" != "$before" ] \
        || die "dispatched, but no new run appeared within 60s — check: gh run list --workflow $WORKFLOW"
fi

say "run $RUN_ID  ($(gh run view "$RUN_ID" --json url --jq .url))"

# -------------------------------------------------------------------- poll

# Per-job status lines, printed on CHANGE. Silence-aware: a run that is
# genuinely working prints nothing for many minutes (a FreeCAD leg is
# ~20 scenes at up to 300 s each), so an unchanged poll still emits a
# heartbeat on a slower cadence — the difference between "wedged" and
# "working" is visible without drowning the log.
deadline=$(( $(date +%s) + POLL_BUDGET_MIN * 60 ))
last_signature=""
last_print=0
while :; do
    now=$(date +%s)
    [ "$now" -lt "$deadline" ] \
        || die "gave up after ${POLL_BUDGET_MIN}m — the run is still going: gh run view $RUN_ID"

    view="$(gh run view "$RUN_ID" --json status,conclusion,jobs)"
    status="$(jq -r .status <<<"$view")"
    # Only the render lanes are reported and waited on. On a dispatched
    # run that is every job; on a CI run it is the handful that matter,
    # and the twenty test shards around them are neither this script's
    # business nor worth eleven minutes of its patience.
    lanes_view="$(jq --arg re "$RENDER_JOBS_RE" \
        '{jobs: [.jobs[] | select(.name | test($re))]}' <<<"$view")"
    signature="$(jq -r '[.jobs[] | "\(.name)=\(.status)/\(.conclusion // "-")"] | join(",")' <<<"$lanes_view")"

    if [ "$signature" != "$last_signature" ] || [ $(( now - last_print )) -ge 300 ]; then
        printf '    [%s]\n' "$(date +%H:%M:%S)"
        # `gh` reports an unfinished job's conclusion as "", not null,
        # and jq's `//` only falls through on null/false — so the state
        # has to be chosen explicitly or every in-flight lane prints
        # blank where its status belongs.
        jq -r '.jobs[] | "      \(if (.conclusion // "") == "" then .status else .conclusion end | ascii_upcase)  \(.name)"' <<<"$lanes_view"
        last_signature="$signature"
        last_print="$now"
    fi

    # Done when the LANES are done — or when the run ends without them
    # ever appearing (a CI run whose change filter skipped the render
    # job, say), which the artifact check below then reports.
    lanes_pending="$(jq '[.jobs[] | select(.status != "completed")] | length' <<<"$lanes_view")"
    lanes_seen="$(jq '.jobs | length' <<<"$lanes_view")"
    [ "$status" != completed ] || break
    # An `A && B && break` list would take `set -e` down with it the
    # moment A is false, which is every poll but the last.
    if [ "$lanes_seen" -gt 0 ] && [ "$lanes_pending" -eq 0 ]; then break; fi
    sleep "$POLL_INTERVAL"
done

# A FAILED LANE IS NOT AUTOMATICALLY A MISSING RENDER, and this is the
# distinction the whole take-CI's-render path rests on: every lane
# uploads its artifact BEFORE the gate step compares it, so the gate
# failing — the one case you most want the frames — leaves them intact.
# So failures are reported and the download decides: no artifact for a
# requested lane is the real error, and it is raised there.
failed="$(jq -r '.jobs[] | select((.conclusion // "") != "" and .conclusion != "success" and .conclusion != "skipped")
                 | "      \(.conclusion | ascii_upcase)  \(.name)"' <<<"$lanes_view")"
if [ -n "$failed" ]; then
    echo >&2
    echo "render-hosted: render lanes that did not succeed on run $RUN_ID:" >&2
    echo "$failed" >&2
    echo >&2
    echo "render-hosted:   A STALE-LANE GATE FAILURE IS EXPECTED HERE when you are" >&2
    echo "render-hosted:   refreshing frames: the artifacts are still this run's" >&2
    echo "render-hosted:   render, and installing them is the fix. A wedged or" >&2
    echo "render-hosted:   crashed lane publishes NOTHING, and shows up below as a" >&2
    echo "render-hosted:   missing artifact." >&2
    echo "render-hosted:   Full log: gh run view $RUN_ID --log-failed" >&2
    echo >&2
else
    say "render lanes on run $RUN_ID succeeded"
fi

# ---------------------------------------------------------------- download

staging="$(mktemp -d)"
# shellcheck disable=SC2064  # $staging must expand now, not at trap time
trap "rm -rf '$staging'" EXIT

have=""
for lane in $(lanes_of "$LANE"); do
    art="$(artifact_for "$lane")"
    if gh run download "$RUN_ID" -n "$art" -D "$staging/$lane" 2>/dev/null; then
        say "pulled $art ($(find "$staging/$lane" -type f | wc -l) files)"
        have="$have $lane"
    else
        echo "render-hosted: no '$art' artifact on run $RUN_ID (lane skipped?)" >&2
    fi
done
[ -n "$have" ] || die "run $RUN_ID produced none of the requested lanes' artifacts"

# ------------------------------------------------------------------ verify

# THE ROUND-TRIP PROOF. Not a claim that hosted pixels match local ones
# — the FreeCAD lanes' do not, and render.yml says so — but that the
# ARTIFACT PATH is lossless: a byte-reproducible lane (wild: matplotlib
# Agg, pinned deps, no GL; uv: stdlib text) that came back through
# upload/zip/download/unzip must be byte-identical to what is committed,
# stamp chunks and all. If that ever stops holding, the provenance guard
# is being fed laundered files and every other lane's pull is suspect.
if [ "$VERIFY" = 1 ]; then
    checked=0
    for lane in $have; do
        case "$lane" in uv|wild) ;; *) continue ;; esac
        dir="$(dir_for "$lane")"
        say "verify: $lane — pulled bytes vs committed $dir/"
        while IFS= read -r rel; do
            src="$staging/$lane/$rel"
            dst="$dir/$rel"
            [ -f "$dst" ] || { echo "    NEW      $rel"; continue; }
            if cmp -s "$src" "$dst"; then
                echo "    identical $rel  ($(sha256sum <"$src" | cut -c1-16))"
                checked=$(( checked + 1 ))
            else
                echo "    DIFFERS  $rel" >&2
                die "byte-exactness broken on $dir/$rel — the artifact path is not lossless, \
or this lane's render is not reproducible. Do not trust a pulled tree until this is explained."
            fi
        done < <(cd "$staging/$lane" && find . -type f -printf '%P\n' | sort)
    done
    [ "$checked" -gt 0 ] || die "--verify had nothing to check (needs the uv or wild lane)"
    say "verify: $checked file(s) byte-identical through upload -> zip -> download"
fi

# ----------------------------------------------------------------- install

if [ "$INSTALL" = 0 ]; then
    keep="${staging}"
    trap - EXIT
    say "--no-install: artifacts left in $keep"
    exit 0
fi

for lane in $have; do
    dir="$(dir_for "$lane")"
    mkdir -p "$dir"
    # Copy, do not sync: a file present locally but absent from the
    # artifact is REPORTED, never deleted. A lane publishes wholesale, so
    # a leftover is a real signal (a retired scene, or a lane that only
    # half-ran) and silently deleting tracked files on the way in is the
    # wrong default for a convenience script.
    while IFS= read -r rel; do
        mkdir -p "$dir/$(dirname "$rel")"
        cp -p "$staging/$lane/$rel" "$dir/$rel"
    done < <(cd "$staging/$lane" && find . -type f -printf '%P\n' | sort)
    while IFS= read -r rel; do
        [ -e "$staging/$lane/$rel" ] \
            || echo "render-hosted: NOTE — $dir/$rel is committed but not in this run's artifact" >&2
    done < <(git ls-files "$dir" | sed "s|^$dir/||")
    say "installed $lane -> $dir/"
done

# The guard the committed tree is held to. Running it here means a pull
# that laundered or mixed provenance fails at the pull, not at review.
if command -v python3 >/dev/null; then
    say "provenance guard over the working tree"
    python3 demos/check_render_provenance.py
fi

echo
say "what moved (review, then commit normally):"
git -c color.status=always status --short -- demos/renders demos/renders-freecad \
    demos/renders-uv demos/renders-wild
