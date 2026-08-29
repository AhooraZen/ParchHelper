# Parch Linux Command Helper - Fish Shell Integration
# Place in /etc/fish/conf.d/parch-helper.fish

for _foreign_cmd in apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap
    function $_foreign_cmd --wraps=parch-helper --description "Parch Linux command helper wrapper"
        /usr/bin/parch-helper (status current-command) $argv
    end
end

function fish_command_not_found
    set -l cmd $argv[1]
    switch $cmd
        case apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap
            if test -x /usr/bin/parch-helper
                /usr/bin/parch-helper $argv
                return $status
            end
        case '*'
            if type -q __fish_default_command_not_found_handler
                __fish_default_command_not_found_handler $argv
                return $status
            else if type -q pkgfile
                pkgfile -b -- "$cmd"
                return $status
            else
                echo "fish: Unknown command: $cmd" >&2
                return 127
            end
    end
end
