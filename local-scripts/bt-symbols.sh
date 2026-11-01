#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge): symbol census of built rlibs.
cd "$(dirname "$0")/.."
printf '%-22s %10s %10s %10s %10s\n' crate rlib_MB text_syms obj_bytes metadata_MB
for f in target/debug/deps/lib*.rlib; do
  base=$(basename "$f")
  case "$base" in
    libeditor_core*|libtopo*|libgeom_brep*|libgeom_core*|libgeom-*|libmesh*|libsweep*|libprofile*|libstep_import*|libstep_export*|libstl*|libpncad*|libbvh*|libquantity*) ;;
    *) continue ;;
  esac
  syms=$(nm --defined-only "$f" 2>/dev/null | grep -cE ' [Tt] ')
  sz=$(stat -c %s "$f")
  meta=$(ar t "$f" 2>/dev/null | grep -c 'rmeta')
  objbytes=$(ar tv "$f" 2>/dev/null | awk '$0 !~ /rmeta/ {s+=$3} END {print s+0}')
  metabytes=$(ar tv "$f" 2>/dev/null | awk '/rmeta/ {s+=$3} END {print s+0}')
  printf '%-22s %10.1f %10d %10d %10.1f\n' "${base%%-*}" "$(echo "scale=1; $sz/1048576" | bc)" "$syms" "$objbytes" "$(echo "scale=1; $metabytes/1048576" | bc)"
done
