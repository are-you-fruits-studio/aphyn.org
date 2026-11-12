#!/usr/bin/env bash
# Gives every page of every language its own HTML file in the build output, so
# GitHub Pages answers deep links (/examples, /uk/roadmap) with 200 instead of
# falling back to 404.html. The app itself decides what to render from the URL.
#
# Languages come from locales/*.txt; the default one lives at the site root.
# Usage: scripts/static-routes.sh [dist-dir]
set -euo pipefail

dist="${1:-dist}"
default_locale="en"          # keep in sync with the first entry of LOCALES in src/i18n.rs
pages=(docs examples roadmap releases)  # every route except home; keep in sync with src/route.rs

index="$dist/index.html"
[ -f "$index" ] || { echo "no $index — run trunk build first" >&2; exit 1; }

cp "$index" "$dist/404.html"
for page in "${pages[@]}"; do
  cp "$index" "$dist/$page.html"
done

for file in locales/*.txt; do
  code="$(basename "$file" .txt)"
  [ "$code" = "$default_locale" ] && continue
  mkdir -p "$dist/$code"
  cp "$index" "$dist/$code/index.html"
  for page in "${pages[@]}"; do
    cp "$index" "$dist/$code/$page.html"
  done
done

echo "static routes written to $dist"
