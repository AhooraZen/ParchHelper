# Parch Linux Command Helper - Bash Integration
# Place in /etc/profile.d/parch-helper.sh or source in ~/.bashrc

command_not_found_handle() {
    local cmd="$1"
    case "$cmd" in
        apt|apt-get|apt-cache|aptitude|dnf|yum|apk|zypper|brew|dpkg|rpm)
            if [ -x /usr/bin/parch-helper ]; then
                /usr/bin/parch-helper "$@"
                return $?
            fi
            ;;
        *)
            if [ -x /usr/bin/pkgfile ]; then
                /usr/bin/pkgfile -b -- "$cmd"
                return $?
            fi
            ;;
    esac
    echo "bash: $cmd: command not found" >&2
    return 127
}
