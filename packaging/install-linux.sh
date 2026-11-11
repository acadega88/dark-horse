#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
bundle_path=${1:-"$script_dir/dark-horse-linux-x86_64.flatpak"}

if ! command -v flatpak >/dev/null 2>&1; then
    printf 'Flatpak is required. Install Flatpak for your distribution and retry.\n' >&2
    exit 1
fi
if [ ! -f "$bundle_path" ]; then
    printf 'Missing Flatpak bundle: %s\n' "$bundle_path" >&2
    exit 1
fi

flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install --user --noninteractive --bundle "$bundle_path"
printf 'Installed Dark Horse Browser. Launch it from your application menu or with:\n'
printf 'flatpak run com.acadega88.darkhorse\n'
