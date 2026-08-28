# Parch Linux Command Helper - Zsh Integration
# Place in /etc/profile.d/parch-helper.zsh or source in ~/.zshrc

command_not_found_handler() {
    local cmd="$1"
    case "$cmd" in
        apt|apt-get|apt-cache|aptitude|dnf|yum|apk|zypper|brew|dpkg|rpm)
            if [[ -x /usr/bin/parch-helper ]]; then
                /usr/bin/parch-helper "$@"
                return $?
            fi
            ;;
        *)
            if (( $+commands[pkgfile] )); then
                pkgfile -b -- "$cmd"
                return $?
            fi
            ;;
    esac
    print -u2 "zsh: command not found: $cmd"
    return 127
}
