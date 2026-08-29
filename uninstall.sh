#!/usr/bin/env bash
set -e

# --- ANSI Neon Color Palette ---
C_RESET="\033[0m"
C_BOLD="\033[1m"
C_CYAN="\033[38;5;51m"
C_BLUE="\033[38;5;39m"
C_GREEN="\033[38;5;48m"
C_YELLOW="\033[38;5;220m"
C_MAGENTA="\033[38;5;201m"
C_RED="\033[38;5;196m"
C_GRAY="\033[38;5;244m"

print_banner() {
    echo -e "${C_BLUE}╭─────────────────────────────────────────────────────────────╮${C_RESET}"
    echo -e "${C_BLUE}│${C_RESET}  ${C_BOLD}${C_RED}❬⚡❭ Parch Linux Command Helper Uninstaller${C_RESET}              ${C_BLUE}│${C_RESET}"
    echo -e "${C_BLUE}│${C_RESET}      ${C_MAGENTA}Clean removal of binaries, wrappers, & shell hooks${C_RESET}     ${C_BLUE}│${C_RESET}"
    echo -e "${C_BLUE}╰─────────────────────────────────────────────────────────────╯${C_RESET}"
    echo ""
}

print_banner

AUTO_CONFIRM=false
REMOVE_CONFIG=true

# Parse command line flags
for arg in "$@"; do
    case "$arg" in
        -y|--yes)
            AUTO_CONFIRM=true
            ;;
        --keep-config)
            REMOVE_CONFIG=false
            ;;
        -h|--help)
            echo -e "${C_BOLD}Usage:${C_RESET} $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -y, --yes          Auto-confirm uninstallation without interactive prompt"
            echo "  --keep-config      Keep configuration file in /etc/parch/helper.toml"
            echo "  -h, --help         Show this help message"
            exit 0
            ;;
    esac
done

# Interactive confirmation
if [ "$AUTO_CONFIRM" = false ] && [ -t 0 ]; then
    echo -ne "${C_CYAN}╭─▶ ${C_BOLD}Are you sure you want to uninstall Parch Command Helper? ${C_YELLOW}[y/N]${C_CYAN} ❯ ${C_RESET}"
    read -r user_choice
    clean_choice=$(echo "$user_choice" | tr '[:upper:]' '[:lower:]' | xargs)
    if [[ "$clean_choice" != "y" && "$clean_choice" != "yes" ]]; then
        echo -e "${C_YELLOW}⚠ Uninstallation aborted by user.${C_RESET}"
        exit 0
    fi
    echo ""
fi

# Privilege elevation check
SUDO=""
if [ "$(id -u)" -ne 0 ]; then
    SUDO="sudo"
    echo -e "${C_MAGENTA}==>${C_RESET} ${C_BOLD}Elevating privileges with sudo for system cleanup...${C_RESET}"
fi

# 1. Remove foreign package manager emulation wrappers
echo -e "${C_CYAN}==>${C_RESET} Removing package manager emulation symlinks from /usr/bin..."
for cmd in apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap; do
    if [ -L "/usr/bin/$cmd" ]; then
        target=$($SUDO readlink "/usr/bin/$cmd" 2>/dev/null || true)
        if [[ "$target" == *"parch-helper"* || "$target" == *"parch-translate"* ]]; then
            $SUDO rm -f "/usr/bin/$cmd"
            echo -e "  ${C_GRAY}Removed link:${C_RESET} /usr/bin/$cmd"
        fi
    fi
done

# 2. Remove primary binaries and aliases
echo -e "${C_CYAN}==>${C_RESET} Removing primary binary and alias..."
if [ -f "/usr/bin/parch-helper" ] || [ -L "/usr/bin/parch-helper" ]; then
    $SUDO rm -f "/usr/bin/parch-helper"
    echo -e "  ${C_GRAY}Removed:${C_RESET} /usr/bin/parch-helper"
fi

if [ -f "/usr/bin/parch-translate" ] || [ -L "/usr/bin/parch-translate" ]; then
    $SUDO rm -f "/usr/bin/parch-translate"
    echo -e "  ${C_GRAY}Removed:${C_RESET} /usr/bin/parch-translate"
fi

# 3. Remove shell integration hooks
echo -e "${C_CYAN}==>${C_RESET} Removing shell hooks (Bash, Zsh, Fish)..."
if [ -f "/etc/profile.d/parch-helper.sh" ]; then
    $SUDO rm -f "/etc/profile.d/parch-helper.sh"
    echo -e "  ${C_GRAY}Removed:${C_RESET} /etc/profile.d/parch-helper.sh"
fi

if [ -f "/usr/share/zsh/site-functions/parch-helper.zsh" ]; then
    $SUDO rm -f "/usr/share/zsh/site-functions/parch-helper.zsh"
    echo -e "  ${C_GRAY}Removed:${C_RESET} /usr/share/zsh/site-functions/parch-helper.zsh"
fi

if [ -f "/usr/share/fish/vendor_conf.d/parch-helper.fish" ]; then
    $SUDO rm -f "/usr/share/fish/vendor_conf.d/parch-helper.fish"
    echo -e "  ${C_GRAY}Removed:${C_RESET} /usr/share/fish/vendor_conf.d/parch-helper.fish"
fi

# 4. Remove configuration if requested
if [ "$REMOVE_CONFIG" = true ]; then
    if [ -f "/etc/parch/helper.toml" ]; then
        echo -e "${C_CYAN}==>${C_RESET} Removing configuration file..."
        $SUDO rm -f "/etc/parch/helper.toml"
        echo -e "  ${C_GRAY}Removed:${C_RESET} /etc/parch/helper.toml"
        $SUDO rmdir "/etc/parch" 2>/dev/null || true
    fi
else
    echo -e "${C_YELLOW}ℹ Preserved configuration at /etc/parch/helper.toml${C_RESET}"
fi

echo ""
echo -e "${C_GREEN}${C_BOLD}✔ Parch Linux Command Helper uninstalled successfully!${C_RESET}"
