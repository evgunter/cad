#!/usr/bin/env bash
# Per-account Claude usage watchdog (arm via Monitor; persistent).
#
# WHY THIS EXISTS
# ---------------
# Hitting the subscription session limit is not a soft failure: Claude
# Code puts up the usage-credits dialog and that KILLS the session for
# the rest of the day — the session does not come back when the 5h
# window resets. (Contrast the fable-specific API-error rate limit,
# which is a transient error the agent retries through.) So the only
# safe play is to stop an account's agents BEFORE the window fills, and
# resume them after it resets.
#
# `mngr usage` cannot be used for this: its reader takes the freshest
# rate_limits payload across ALL agents ("freshest-wins", see
# mngr_usage/api.py::_combine_agent_walks) and Ev runs several Claude
# accounts, one per agent state dir. So it reports whichever account
# rendered a statusline most recently, unlabeled, and hides every other
# account's limits. This monitor does the per-account join mngr omits.
#
# DATA SOURCE
# -----------
# The same statusline log mngr reads:
#   $MNGR_AGENTS_ROOT/agent-*/events/claude/usage/events.jsonl
# one JSON object per statusline render, carrying session_id, cost and
# rate_limits{five_hour,seven_day,overage}{used_percentage,resets_at}.
# Those events carry NO account identity — it is recovered by reading
# the agent's own Claude config dir:
#   agent-*/plugin/claude/anthropic/.claude.json -> oauthAccount.emailAddress
#
# PROTOCOL (one line per event; thresholds are CROSSINGS, not levels)
#   USAGE WARN       >=90%  — wind down; finish the current unit of work
#   USAGE STOP-SOON  >=95%  — land what's in flight, stop starting work
#   USAGE STOP-NOW   >=99%  — pause that account's agents immediately
#   USAGE RESET             — window rolled over (usage dropped or the
#                             tracked resets_at passed); paused agents on
#                             that account may resume
#   USAGE CACHE-PING due (55m cadence) — while any account sits in
#                             [90,99), every 55 minutes: ping the paused
#                             agents so their prompt caches stay warm.
#                             Suppressed at >=99% (pinging there spends
#                             the last of the window).
# Each alert names the ACCOUNT, the governing window (5h/7d/overage),
# the current %, and that window's reset time.
#
# Thresholds are env-overridable (CAD_USAGE_WARN / _STOP_SOON / _STOP_NOW)
# so the script can be smoke-tested against live data without waiting for
# a real 90% event.
#
# State is in-memory: a restart re-arms every band, so a restart while an
# account is hot re-emits its current band once. That is deliberate — a
# missed STOP is far worse than a duplicated one.
set -u

AGENTS_ROOT="${MNGR_AGENTS_ROOT:-$HOME/.mngr/agents}"
WARN="${CAD_USAGE_WARN:-90}"
STOP_SOON="${CAD_USAGE_STOP_SOON:-95}"
STOP_NOW="${CAD_USAGE_STOP_NOW:-99}"
POLL="${CAD_USAGE_POLL:-60}"        # seconds between polls
STALE="${CAD_USAGE_STALE:-1800}"    # ignore accounts whose newest render is older than this
PING_EVERY="${CAD_USAGE_PING_EVERY:-3300}"  # 55m
ONCE="${CAD_USAGE_ONCE:-0}"         # 1 = single poll then exit (testing)

declare -A BAND        # account -> 0 | $WARN | $STOP_SOON | $STOP_NOW
declare -A RESETS      # account -> resets_at epoch of the governing window
declare -A LASTPCT     # account -> last seen governing percentage
LAST_PING=0            # epoch of last CACHE-PING; 0 = not currently in band

# Emit "account<TAB>window<TAB>pct<TAB>resets_at<TAB>event_epoch" for the
# freshest rate_limits event of each account, governing window = the one
# with the highest used_percentage. Best-effort: any unreadable agent dir
# or malformed line is skipped, never fatal.
scan() {
  AGENTS_ROOT="$AGENTS_ROOT" STALE="$STALE" python3 - <<'PY' 2>/dev/null
import calendar, glob, json, os, time
root = os.environ["AGENTS_ROOT"]; stale = int(os.environ["STALE"]); now = time.time()
best = {}
for path in glob.glob(os.path.join(root, "agent-*", "events", "claude", "usage", "events.jsonl")):
    agent_dir = path.split("/events/")[0]
    try:
        cfg = json.load(open(os.path.join(agent_dir, "plugin", "claude", "anthropic", ".claude.json")))
        acct = (cfg.get("oauthAccount") or {}).get("emailAddress")
    except Exception:
        acct = None
    if not acct:
        continue
    if now - os.path.getmtime(path) > stale:   # cheap reject before reading
        continue
    ev = None
    try:                                       # tail: only the last chunk matters
        with open(path, "rb") as fh:
            fh.seek(0, 2); size = fh.tell(); fh.seek(max(0, size - 262144))
            for line in fh.read().split(b"\n")[1 if size > 262144 else 0:]:
                if not line.strip():
                    continue
                try:
                    e = json.loads(line)
                except Exception:
                    continue
                if isinstance(e.get("rate_limits"), dict) and e["rate_limits"]:
                    ev = e
    except Exception:
        continue
    if ev is None:
        continue
    try:
        # writer stamps UTC ("...Z"); timegm, not mktime -- mktime would
        # apply the local DST offset and silently age the event by an hour
        ts = calendar.timegm(time.strptime(ev["timestamp"][:19], "%Y-%m-%dT%H:%M:%S"))
    except Exception:
        ts = os.path.getmtime(path)
    if now - ts > stale:
        continue
    win = pct = res = None
    for name, w in ev["rate_limits"].items():
        if not isinstance(w, dict):
            continue
        p = w.get("used_percentage")
        if p is None:
            continue
        if pct is None or p > pct:
            win, pct, res = w.get("label") or name, p, w.get("resets_at") or 0
    if pct is None:
        continue
    if acct not in best or ts > best[acct][4]:
        best[acct] = (acct, win, pct, res, ts)
for acct, (a, w, p, r, ts) in sorted(best.items()):
    print("%s\t%s\t%d\t%d\t%d" % (a, w, int(p), int(r), int(ts)))
PY
}

fmt_reset() {  # epoch -> "15:40 local (in 2h13m)"
  local at="$1" now delta h m
  now=$(date +%s)
  if [ "$at" -le 0 ] 2>/dev/null; then echo "unknown"; return; fi
  delta=$((at - now)); [ "$delta" -lt 0 ] && delta=0
  h=$((delta / 3600)); m=$(((delta % 3600) / 60))
  echo "$(date -d "@$at" +%H:%M 2>/dev/null || echo "@$at") local (in ${h}h${m}m)"
}

band_of() {  # pct -> band number
  local p="$1"
  if   [ "$p" -ge "$STOP_NOW" ];  then echo "$STOP_NOW"
  elif [ "$p" -ge "$STOP_SOON" ]; then echo "$STOP_SOON"
  elif [ "$p" -ge "$WARN" ];      then echo "$WARN"
  else echo 0; fi
}

while true; do
  now=$(date +%s)
  seen=""
  in_band=0
  while IFS=$'\t' read -r acct win pct res ts; do
    [ -n "${acct:-}" ] || continue
    seen="$seen $acct"
    prev_band="${BAND[$acct]:-0}"
    prev_res="${RESETS[$acct]:-0}"

    # Window rollover: the reset timestamp moved, or usage fell back below
    # WARN. Either way the account is out of the danger band -> resume.
    if [ "$prev_band" -gt 0 ] && { [ "$res" -ne "$prev_res" ] || [ "$pct" -lt "$WARN" ]; }; then
      echo "USAGE RESET: $acct — $win window rolled over, now ${pct}% (was ${LASTPCT[$acct]:-?}%); next reset $(fmt_reset "$res"). Paused agents on this account may resume."
      prev_band=0
    fi

    new_band=$(band_of "$pct")
    if [ "$new_band" -gt "$prev_band" ]; then
      case "$new_band" in
        "$STOP_NOW")  echo "USAGE STOP-NOW: $acct at ${pct}% of its $win limit — PAUSE this account's agents now; the session-limit dialog kills a session for the day. Resets $(fmt_reset "$res")." ;;
        "$STOP_SOON") echo "USAGE STOP-SOON: $acct at ${pct}% of its $win limit — land in-flight work, start nothing new. Resets $(fmt_reset "$res")." ;;
        *)            echo "USAGE WARN: $acct at ${pct}% of its $win limit — wind down this account's lanes. Resets $(fmt_reset "$res")." ;;
      esac
    fi

    BAND[$acct]="$new_band"; RESETS[$acct]="$res"; LASTPCT[$acct]="$pct"
    if [ "$new_band" -ge "$WARN" ] && [ "$new_band" -lt "$STOP_NOW" ]; then
      in_band=1; ping_acct="$acct"; ping_pct="$pct"; ping_win="$win"
    fi
  done < <(scan)

  # Accounts that stopped reporting while hot: if their tracked reset has
  # passed, the window is up even though no agent is rendering. Announce
  # it so paused lanes get resumed, then forget the account.
  for acct in "${!BAND[@]}"; do
    case " $seen " in *" $acct "*) continue ;; esac
    if [ "${BAND[$acct]}" -gt 0 ] && [ "${RESETS[$acct]:-0}" -gt 0 ] && [ "$now" -ge "${RESETS[$acct]}" ]; then
      echo "USAGE RESET: $acct — tracked reset time passed with no further renders (last seen ${LASTPCT[$acct]:-?}%). Paused agents on this account may resume."
      unset "BAND[$acct]" "RESETS[$acct]" "LASTPCT[$acct]"
    fi
  done

  if [ "$in_band" -eq 1 ]; then
    if [ "$LAST_PING" -eq 0 ]; then
      LAST_PING="$now"                                   # entered the band; first ping is 55m out
    elif [ $((now - LAST_PING)) -ge "$PING_EVERY" ]; then
      echo "USAGE CACHE-PING due (55m cadence) — $ping_acct at ${ping_pct}% ($ping_win); ping paused agents to keep their prompt caches warm."
      LAST_PING="$now"
    fi
  else
    LAST_PING=0
  fi

  [ "$ONCE" = "1" ] && break
  sleep "$POLL"
done
