#!/bin/bash
# Maintainer: Allen
# Description: Portable installer script for c4vi.

set -e

INSTALL_DIR="$HOME/.c4vi"
SRC_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Installing c4vi to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

cp -r "$SRC_DIR/bin" "$SRC_DIR/colors" "$SRC_DIR/ftdetect" "$SRC_DIR/ftplugin" "$SRC_DIR/indent" "$SRC_DIR/syntax" "$SRC_DIR/vimconfigs" "$SRC_DIR/example.c4" "$SRC_DIR/c4.peg" "$INSTALL_DIR/"

rm -rf "$INSTALL_DIR/bin/__pycache__"
chmod +x "$INSTALL_DIR/bin/c4-lsp"

# ─── Configuration Prompts ───────────────────────────────────────────────────
echo ""
echo "Select your preferred color theme:"
echo "1) Default Dark (c4_theme)"
echo "2) Cyberpunk Neon (c4_neon)"
echo "3) Retro Monochrome (c4_monochrome)"
echo "4) Solarized Dark (c4_solarized)"
echo "5) Crisp Light (c4_light)"
read -p "Enter choice [1-5]: " theme_choice

case $theme_choice in
    1) THEME="c4_theme" ;;
    2) THEME="c4_neon" ;;
    3) THEME="c4_monochrome" ;;
    4) THEME="c4_solarized" ;;
    5) THEME="c4_light" ;;
    *) THEME="c4_theme" ;;
esac

echo ""
read -p "Enable autocompletion? [Y/n]: " autocomplete_input
if [[ "$autocomplete_input" =~ ^[Nn] ]]; then
    AUTO_COMP=false
else
    AUTO_COMP=true
fi

if [ "$AUTO_COMP" = true ]; then
    read -p "Enable auto-popup as you type? [Y/n]: " autotrigger_input
    if [[ "$autotrigger_input" =~ ^[Nn] ]]; then
        AUTO_TRIG=false
    else
        AUTO_TRIG=true
    fi

    if [ "$AUTO_TRIG" = true ]; then
        read -p "Only trigger autocomplete after '.' or '->'? [y/N]: " methodsonly_input
        if [[ "$methodsonly_input" =~ ^[Yy] ]]; then
            METHODS_ONLY=true
        else
            METHODS_ONLY=false
        fi
    else
        METHODS_ONLY=false
    fi

    read -p "Use VS Code-style Tab completion navigation? [y/N]: " tabcomplete_input
    if [[ "$tabcomplete_input" =~ ^[Yy] ]]; then
        TAB_COMP=true
    else
        TAB_COMP=false
    fi
else
    AUTO_TRIG=false
    METHODS_ONLY=false
    TAB_COMP=false
fi

# Write updated configuration files
cat <<EOF > "$INSTALL_DIR/vimconfigs/config.json"
{
  "autocomplete_enabled": $AUTO_COMP,
  "auto_trigger": $AUTO_TRIG,
  "trigger_on_methods_only": $METHODS_ONLY,
  "map_tab_complete": $TAB_COMP
}
EOF

# Update colorscheme line in vimrc
sed -i "s/colorscheme \S\+/colorscheme ${THEME}/g" "$INSTALL_DIR/vimconfigs/vimrc"

# ─── Multi-Tiered Shell Detection ────────────────────────────────────────────
# 1. System login database check
LOGIN_SHELL=""
if command -v getent &> /dev/null; then
    LOGIN_SHELL=$(getent passwd "$USER" | cut -d: -f7 | xargs basename)
elif command -v finger &> /dev/null; then
    LOGIN_SHELL=$(finger "$USER" | grep -i 'shell:' | awk '{print $NF}' | xargs basename)
fi

# 2. Parent Process comm check
PARENT_SHELL=""
if command -v ps &> /dev/null; then
    PARENT_SHELL=$(ps -p $PPID -o comm= | tr -d '-')
fi

# 3. Environment Variable check
ENV_SHELL=""
if [ -n "$SHELL" ]; then
    ENV_SHELL=$(basename "$SHELL")
fi

# Heuristic shell resolution
DETECTED_SHELL=""
for s in "$PARENT_SHELL" "$ENV_SHELL" "$LOGIN_SHELL"; do
    if [[ "$s" =~ ^(bash|zsh|fish|ksh|sh|csh|tcsh)$ ]]; then
        DETECTED_SHELL="$s"
        break
    fi
done

# 4. File existence fallback check
if [ -z "$DETECTED_SHELL" ]; then
    if [ -f "$HOME/.zshrc" ]; then
        DETECTED_SHELL="zsh"
    elif [ -f "$HOME/.bashrc" ]; then
        DETECTED_SHELL="bash"
    elif [ -f "$HOME/.config/fish/config.fish" ]; then
        DETECTED_SHELL="fish"
    else
        DETECTED_SHELL="bash"
    fi
fi

# ─── Alias Setup ─────────────────────────────────────────────────────────────
ALIAS_INJECT="alias c4vi=\"nvim -u $INSTALL_DIR/vimconfigs/vimrc\""

setup_alias() {
    local rc_file="$1"
    if [ -f "$rc_file" ]; then
        if ! grep -q "alias c4vi=" "$rc_file"; then
            echo "" >> "$rc_file"
            echo "# c4vi shortcut alias" >> "$rc_file"
            echo "$ALIAS_INJECT" >> "$rc_file"
            echo "Added alias to $rc_file"
        else
            sed -i "s|alias c4vi=.*|$ALIAS_INJECT|g" "$rc_file"
            echo "Updated existing alias in $rc_file"
        fi
    fi
}

echo ""
echo "Detected Shell: $DETECTED_SHELL"

case "$DETECTED_SHELL" in
    zsh)
        setup_alias "$HOME/.zshrc"
        ;;
    bash)
        setup_alias "$HOME/.bashrc"
        setup_alias "$HOME/.bash_profile"
        ;;
    fish)
        FISH_CONFIG_DIR="$HOME/.config/fish"
        mkdir -p "$FISH_CONFIG_DIR"
        FISH_CONFIG="$FISH_CONFIG_DIR/config.fish"
        touch "$FISH_CONFIG"
        if ! grep -q "alias c4vi=" "$FISH_CONFIG"; then
            echo "" >> "$FISH_CONFIG"
            echo "# c4vi shortcut alias" >> "$FISH_CONFIG"
            echo "alias c4vi 'nvim -u $INSTALL_DIR/vimconfigs/vimrc'" >> "$FISH_CONFIG"
            echo "Added alias to $FISH_CONFIG"
        else
            sed -i "s|alias c4vi.*|alias c4vi 'nvim -u $INSTALL_DIR/vimconfigs/vimrc'|g" "$FISH_CONFIG"
            echo "Updated existing alias in $FISH_CONFIG"
        fi
        ;;
    ksh)
        setup_alias "$HOME/.kshrc"
        setup_alias "$HOME/.profile"
        ;;
    csh|tcsh)
        CSH_CONFIG="$HOME/.cshrc"
        if [ -f "$HOME/.tcshrc" ]; then
            CSH_CONFIG="$HOME/.tcshrc"
        fi
        touch "$CSH_CONFIG"
        if ! grep -q "alias c4vi " "$CSH_CONFIG"; then
            echo "" >> "$CSH_CONFIG"
            echo "# c4vi shortcut alias" >> "$CSH_CONFIG"
            echo "alias c4vi nvim -u $INSTALL_DIR/vimconfigs/vimrc" >> "$CSH_CONFIG"
            echo "Added alias to $CSH_CONFIG"
        else
            sed -i "s|alias c4vi .*|alias c4vi nvim -u $INSTALL_DIR/vimconfigs/vimrc|g" "$CSH_CONFIG"
            echo "Updated existing alias in $CSH_CONFIG"
        fi
        ;;
    *)
        setup_alias "$HOME/.bashrc"
        setup_alias "$HOME/.profile"
        ;;
esac

# ─── Environment Verification ────────────────────────────────────────────────
if ! command -v nvim &> /dev/null; then
    echo ""
    echo "Warning: 'nvim' (Neovim) was not found in your PATH."
    echo "Please install Neovim for full language server (LSP) functionality."
fi

echo ""
echo "Installation complete!"
echo "Please reload your shell or run: source ~/.${DETECTED_SHELL}rc (or equivalent) to start using 'c4vi'."
