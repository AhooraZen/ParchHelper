# Parch Linux Command Helper - Bash Integration
# Place in /etc/profile.d/parch-helper.sh or source in ~/.bashrc

alias sudo='sudo '

for _foreign_cmd in apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap; do
    eval "function ${_foreign_cmd}() { /usr/bin/parch-helper ${_foreign_cmd} \"\$@\"; }"
done
unset _foreign_cmd

if declare -f command_not_found_handle >/dev/null 2>&1; then
    eval "_orig_command_not_found_handle() $(declare -f command_not_found_handle | tail -n +2)"
fi

command_not_found_handle() {
    local cmd="$1"
    case "$cmd" in
        apt|apt-get|apt-cache|aptitude|dnf|yum|apk|zypper|brew|dpkg|rpm|flatpak|snap)
            if [ -x /usr/bin/parch-helper ]; then
                /usr/bin/parch-helper "$@"
                return $?
            fi
            ;;
        *)
            if [ -x /usr/bin/pkgfile ]; then
                /usr/bin/pkgfile -b -- "$cmd" && return 0
            fi
            if declare -f _orig_command_not_found_handle >/dev/null 2>&1; then
                _orig_command_not_found_handle "$@"
                return $?
            fi
            ;;
    esac
    echo "bash: $cmd: command not found" >&2
    return 127
}
