# Parch Linux Command Helper - Zsh Integration
# Place in /etc/profile.d/parch-helper.zsh or source in ~/.zshrc

alias sudo='sudo '

for _foreign_cmd in apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap; do
    eval "function ${_foreign_cmd}() { /usr/bin/parch-helper ${_foreign_cmd} \"\$@\"; }"
done
unset _foreign_cmd

if (( $+functions[command_not_found_handler] )); then
    functions[_orig_command_not_found_handler]=$functions[command_not_found_handler]
fi

command_not_found_handler() {
    local cmd="$1"
    case "$cmd" in
        apt|apt-get|apt-cache|aptitude|dnf|yum|apk|zypper|brew|dpkg|rpm|flatpak|snap)
            if [[ -x /usr/bin/parch-helper ]]; then
                /usr/bin/parch-helper "$@"
                return $?
            fi
            ;;
        *)
            if (( $+commands[pkgfile] )); then
                pkgfile -b -- "$cmd" && return 0
            fi
            if (( $+functions[_orig_command_not_found_handler] )); then
                _orig_command_not_found_handler "$@"
                return $?
            fi
            ;;
    esac
    print -u2 "zsh: command not found: $cmd"
    return 127
}
