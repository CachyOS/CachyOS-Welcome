#!/usr/bin/env bash

set -euo pipefail

repo_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
helper="$repo_dir/src/scripts/terminal-helper"
rootshell="$repo_dir/src/scripts/rootshell.sh"
test_dir="$(mktemp -d)"
fake_bin="$test_dir/bin"
mkdir -p "$fake_bin"

cleanup() {
    rm -rf -- "$test_dir"
}
trap cleanup EXIT

cat > "$fake_bin/mktemp" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

file="$(/usr/bin/mktemp "$@")"
printf '%s\n' "$file" > "$TEST_DIR/mktemp-path"
printf '%s\n' "$file"
EOF

cat > "$fake_bin/kgx" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

printf '%s\n' "${@: -1}" > "$TEST_DIR/script-path"
if [[ -f "$TEST_DIR/kgx-fails" ]]; then
    exit 1
fi
exit 0
EOF

cat > "$fake_bin/pkexec" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

printf '%s\n' "$@" > "$TEST_DIR/pkexec-args"
exec "$@"
EOF

chmod 755 "$fake_bin/mktemp" "$fake_bin/kgx" "$fake_bin/pkexec"
ln -s /usr/bin/bash "$fake_bin/bash"
ln -s /usr/bin/rm "$fake_bin/rm"

run_with_fakes() {
    TEST_DIR="$test_dir" PATH="$fake_bin" "$@"
}

assert_script_exists() {
    local script
    script="$(<"$test_dir/script-path")"
    [[ -f "$script" ]]
    printf '%s\n' "$script"
}

marker="$test_dir/marker"
run_with_fakes "$helper" "printf '%s\\n' direct-marker > '$marker'"
script="$(assert_script_exists)"
bash "$script"
[[ "$(<"$marker")" == direct-marker ]]
[[ ! -e "$script" ]]

run_with_fakes "$helper" -s "pkexec $rootshell" "printf '%s\\n' pkexec-marker > '$marker'"
script="$(assert_script_exists)"
run_with_fakes pkexec "$rootshell" "$script"
[[ "$(<"$marker")" == pkexec-marker ]]
[[ ! -e "$script" ]]
[[ "$(sed -n '1p' "$test_dir/pkexec-args")" == "$rootshell" ]]

run_with_fakes "$helper" "false; exit 7"
script="$(assert_script_exists)"
if bash "$script"; then
    echo "expected child command failure" >&2
    exit 1
fi
[[ ! -e "$script" ]]

launch_marker="$test_dir/launch-marker"
touch "$test_dir/kgx-fails"
if run_with_fakes "$helper" "printf '%s\\n' should-not-run > '$launch_marker'"; then
    echo "expected terminal launch failure" >&2
    exit 1
fi
script="$(<"$test_dir/script-path")"
[[ ! -e "$script" ]]
[[ ! -e "$launch_marker" ]]

echo "terminal-helper regression tests passed"
