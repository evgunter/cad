#!/usr/bin/env bash
# RENDER YOUR PUSHED BRANCH ON HOSTED CI, THEN INSTALL THE FRAMES.
#
#   local-scripts/render-hosted.sh                      # every lane
#   local-scripts/render-hosted.sh --lane uv            # one lane
#   local-scripts/render-hosted.sh --run 12345678       # pull a finished run
#
# PRs do not render and the nightly re-baselines main, so a change you
# expect to move frames is rendered on demand: this dispatches render.yml
# on your branch, polls it, and installs each lane's artifact at its
# committed path. The dispatched run also commits the re-baselined cells
# to the branch itself, so `git pull` after it lands the same bytes.
#
# Renders are hosted (`.github/workflows/render.yml`); the local entry
# points refuse without an explicit override (demos/hosted-render-guard.sh).
#
# IT RENDERS THE PUSHED TREE, SO IT REFUSES AN UNPUSHED HEAD: a runner
# checks out the remote ref and cannot see local commits.
#
# BYTE-EXACTNESS IS THE CONTRACT: an artifact is the file, not a
# rendering of it (upload-artifact zips and `gh run download` unzips,
# both lossless), and `--verify` round-trips the lanes `VERIFY_LANES`
# names and diffs them against what is committed.
set -euo pipefail

die() { echo "render-hosted: $*" >&2; exit 1; }
say() { echo "==> $*"; }

# THE LANE TABLE: one row per lane `.github/workflows/render.yml`
# declares — the lane's name, the artifact it uploads, and the committed
# directory that artifact installs into. Everything below reads it: the
# `--lane` choices, what `all` means, the download, the verify, the
# install, the closing `git status` and the usage text. A lane is one
# row, and there is no second list for a new lane to fall behind.
#
# A ROSTER, NOT A COUNT. A count goes stale the next time a lane is
# added. Nothing checks this table against render.yml.
LANE_TABLE="
kernel  renders-kernel  demos/renders
freecad renders-freecad demos/renders-freecad
uv      renders-uv      demos/renders-uv
mc      renders-mc      demos/renders-mc
wild    renders-wild    demos/renders-wild
gui     renders-gui     demos/renders-gui
"

# THE LANES `--verify` CAN ROUND-TRIP, and this one IS a hand-written
# list: it is not the roster but a PROPERTY of a lane — byte-reproducible
# off-box, so a pulled file may be compared to the committed one at all.
# render.yml states that property in prose, per lane, and declares it
# nowhere a reader can key on. What it
# costs if it goes stale is bounded and visible: a lane missing here is a
# lane `--verify` silently does not prove, and `--verify` says how many
# files it checked. A lane wrongly added reds on the first GL-stack
# difference, loudly and in the right direction.
VERIFY_LANES="uv mc wild"

lane_names() {
    local l _a _d
    while read -r l _a _d; do [ -z "$l" ] || echo "$l"; done <<<"$LANE_TABLE"
}
# Lane -> (artifact name, committed directory), read by the download, the
# install and the verify. An unknown lane is a caller bug rather than a
# lane with empty fields, so it dies instead of echoing nothing.
lane_field() {
    local want="$1" which="$2" l a d
    while read -r l a d; do
        [ "$l" = "$want" ] || continue
        case "$which" in artifact) echo "$a" ;; dir) echo "$d" ;; esac
        return 0
    done <<<"$LANE_TABLE"
    die "no lane '$want' in the lane table"
}
artifact_for() { lane_field "$1" artifact; }
dir_for() { lane_field "$1" dir; }
lanes_of() {
    if [ "$1" = all ]; then lane_names | tr '\n' ' '; else echo "$1"; fi
}
lane_dirs() { local l; for l in $(lane_names); do dir_for "$l"; done; }
# THE ONE PREDICATE `--lane` IS VALIDATED AGAINST. Everything a lane
# argument may be: a row of the table, or `all`.
lane_accepted() {
    local l
    for l in $(lane_names) all; do [ "$l" != "$1" ] || return 0; done
    return 1
}
lane_choices() {  # "kernel|…|all", the spelling the usage and the refusal share
    local sep="$1" out="" l
    for l in $(lane_names) all; do out="${out:+$out$sep}$l"; done
    echo "$out"
}
lane_install_roster() {
    local l _a d
    while read -r l _a d; do
        [ -z "$l" ] || printf '  %-7s -> %s/\n' "$l" "$d"
    done <<<"$LANE_TABLE"
}

WORKFLOW=render.yml
LANE=all
RUN_ID=""
# The lane jobs, by the name each ends with. A dispatched render.yml run
# names them exactly; called from ci.yml they arrive prefixed ("render
# lanes / freecad montages (kernel + freecad)"), so the match is on the
# SUFFIX and one list serves both. This is what lets the poll wait for the
# render rather than for a whole CI run — the lanes settle in ~3 minutes,
# CI in ~12. Those two are approximate hosted-runner readings, unguarded
# and left that way on purpose: nothing here computes with them (the outer
# stop below is derived from the workflow's own job caps, not from these),
# so they only tell a reader why waiting on the whole run is the wrong
# wait, and that stays true at any pair of numbers with this ordering.
#
# A LANE IS NOT A JOB, so this is its own roster: render.yml groups the
# renderer-free lanes into `scene inputs + uv sheet + wild montage` and
# the two FreeCAD ones into `freecad montages (kernel + freecad)` — one
# runner setup, one 821 MB FreeCAD cache restore and one apt install for
# work that shares all of it — while the viewer's window is photographed
# on a runner of its own. Every lane still uploads its own artifact under
# its own name, which is what the download path below keys on; this
# regex only decides which jobs the PROGRESS DISPLAY waits for.
#
# Getting it wrong is not cosmetic: an unlisted lane job is one the poll
# neither waits for nor reports, so a run can be declared settled while
# that lane is still drawing and its artifact is not there yet.
RENDER_JOBS_RE='(scene inputs \+ uv sheet \+ wild montage|freecad montages \(kernel \+ freecad\)|viewer gui montage)$'
REF=""
SCENE_TIMEOUT=""
INSTALL=1
VERIFY=0
# Outer stop. The merged FreeCAD job is capped at 150 min by the workflow
# (two lanes on one runner) and the scene-inputs job at 75; queueing on top
# of that is the slack. Both caps are COPIES of `render.yml`'s own
# `timeout-minutes:` and nothing checks that they still agree — they are
# quoted to show the arithmetic, and if they drift it is this comment that
# is wrong, never the workflow. The default below is not a copy of
# anything, and the usage text prints it rather than restating it: the one
# drift this file had was exactly there, a heredoc claiming 150 while this
# line said 200.
POLL_BUDGET_MIN=200
POLL_INTERVAL=20

cd "$(git rev-parse --show-toplevel)"

usage() {
    cat <<EOF
usage: local-scripts/render-hosted.sh [options]

  (default)                             dispatch render.yml on the branch,
                                        wait, and install the frames
  --on-demand                           the default; accepted for old callers
  --lane <$(lane_choices '|')>
                                        which lane(s) (default: all)
  --ref <branch|tag|sha>                which branch (default: current)
  --run <id>                            take a specific run; no new render
  --scene-timeout <seconds>             FreeCAD per-scene budget (unset: the
                                        render.yml input's own default)
  --no-install                          download to a temp dir, do not touch the tree
  --verify                              round-trip proof: assert the pulled bytes
                                        equal the committed ones (the lanes
                                        VERIFY_LANES names: $VERIFY_LANES)
  --budget-min <n>                      give up polling after n minutes
                                        (default: $POLL_BUDGET_MIN, printed from
                                        the variable so it cannot drift)
  -h, --help

Artifacts land at their committed paths:
$(lane_install_roster)
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        --lane) LANE="${2:?--lane needs a value}"; shift 2 ;;
        --ref) REF="${2:?--ref needs a value}"; shift 2 ;;
        --run) RUN_ID="${2:?--run needs a value}"; shift 2 ;;
        --on-demand) shift ;;
        --scene-timeout) SCENE_TIMEOUT="${2:?--scene-timeout needs a value}"; shift 2 ;;
        --budget-min) POLL_BUDGET_MIN="${2:?--budget-min needs a value}"; shift 2 ;;
        --no-install) INSTALL=0; shift ;;
        --verify) VERIFY=1; shift ;;
        -h|--help) usage; exit 0 ;;
        *) usage >&2; die "unknown argument: $1" ;;
    esac
done

lane_accepted "$LANE" \
    || die "--lane must be one of $(lane_choices ', ') (got '$LANE')"
command -v gh >/dev/null || die "gh is not installed (https://cli.github.com)"
gh auth status >/dev/null 2>&1 || die "gh is not authenticated — run: gh auth login"


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
# ARTIFACT PATH is lossless: a lane in `VERIFY_LANES` above (matplotlib
# Agg with pinned deps and no GL, or stdlib text) that came back through
# upload/zip/download/unzip must be byte-identical to what is committed,
# stamp chunks and all. If that ever stops holding, the provenance guard
# is being fed laundered files and every other lane's pull is suspect.
if [ "$VERIFY" = 1 ]; then
    checked=0
    for lane in $have; do
        case " $VERIFY_LANES " in *" $lane "*) ;; *) continue ;; esac
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
    [ "$checked" -gt 0 ] \
        || die "--verify had nothing to check (needs one of: $VERIFY_LANES)"
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
# shellcheck disable=SC2046  # the lane dirs are the table's, one word each
git -c color.status=always status --short -- $(lane_dirs)
