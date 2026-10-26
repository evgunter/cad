#!/bin/bash
# kill editor-core test processes older than 400 s unless union-related
while true; do
  ps -eo pid,etimes,args | awk '$2>400 && $3 ~ /t-(head|base)\/debug\/deps\/all-/ {print}' | while read pid et rest; do
    if ! echo "$rest" | grep -qE "union|intersect|docm5_subject|coincidence|fold|member|pinch|flush"; then
      echo "$(date +%T) reap $et $rest" >> /home/user/reaped.log; kill -9 $pid
    fi
  done
  sleep 20
done
