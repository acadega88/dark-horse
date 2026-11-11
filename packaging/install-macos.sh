#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
dmg_path="$script_dir/dark-horse-macos-universal.dmg"
mount_path=$(mktemp -d "${TMPDIR:-/tmp}/dark-horse.XXXXXX")
mounted=0

cleanup() {
    if [ "$mounted" -eq 1 ]; then
        hdiutil detach "$mount_path" -quiet || true
    fi
    rmdir "$mount_path" 2>/dev/null || true
}
trap cleanup EXIT

if [ ! -f "$dmg_path" ]; then
    printf 'Missing package: %s\n' "$dmg_path" >&2
    exit 1
fi

hdiutil attach "$dmg_path" -nobrowse -readonly -mountpoint "$mount_path" >/dev/null
mounted=1
mkdir -p "$HOME/Applications"
ditto "$mount_path/Dark Horse Browser.app" "$HOME/Applications/Dark Horse Browser.app"
hdiutil detach "$mount_path" -quiet
mounted=0
printf 'Installed Dark Horse Browser in %s\n' "$HOME/Applications"
