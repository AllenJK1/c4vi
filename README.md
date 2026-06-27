# c4vi - Custom Vim/Neovim Environment for C4

c4vi is a portable, lightweight, and customizable editor environment optimized for the C4 language. It includes syntax highlighting, auto-indentation, filetype detection, and a custom Python-based Language Server Protocol (LSP) server providing hover tooltips, diagnostics, and context-aware autocomplete.

---

## Installation Guide

Follow these steps to clone and install c4vi on your system:

### 1. Clone the Repository
Clone the codebase into your local machine:
```bash
git clone https://github.com/AllenJK1/c4vi.git
cd c4vi
```

### 2. Run the Installer
Execute the self-contained installation script:
```bash
./install.sh
```

The installer will:
1. Copy all configurations, colorschemes, syntax rules, and LSP binaries to a portable local folder: `~/.c4vi/`.
2. Clean up any runtime caches.
3. Guide you through interactive setup menus to customize your colorscheme and autocompletion preferences.
4. Detect your active shell profile (`bash`, `zsh`, `fish`, `ksh`, `sh`, `csh`, or `tcsh`) and add the alias:
   `alias c4vi="nvim -u ~/.c4vi/vimconfigs/vimrc"` (formatted automatically for your shell dialect).

### 3. Reload Your Shell
Activate the new alias in your current terminal session:
* For **Zsh**: `source ~/.zshrc`
* For **Bash**: `source ~/.bashrc`
* For **Fish**: `source ~/.config/fish/config.fish`
* For other shells: Restart your terminal.

Launch your new custom editor with:
```bash
c4vi example.c4
```

---

## Editor Options & Themes

During installation, you will configure your preferences. You can adjust these settings later by editing `~/.c4vi/vimconfigs/config.json`.

### Color Themes
c4vi ships with five color palettes designed to fit any aesthetic. All themes support dynamic diagnostics (underlines) and C4 grammar rules:

1. **Default Dark (`c4_theme`)**: A modern dark coding interface based on VS Code aesthetics.
2. **Cyberpunk Neon (`c4_neon`)**: Deep dark workspace with vibrant cyan, hot pink, and lime green accents.
3. **Retro Monochrome (`c4_monochrome`)**: A clean black-and-white theme featuring classic terminal amber highlights.
4. **Solarized Dark (`c4_solarized`)**: A classic dark green-teal palette with low-contrast, easy-on-the-eyes syntax highlights.
5. **Crisp Light (`c4_light`)**: A clean, high-contrast, premium paper-white light theme.

*You can swap themes live inside the editor using the standard command: `:colorscheme c4_<name>`.*

---

## Autocomplete Configuration Combinations

c4vi allows you to fine-tune how and when autocomplete popups appear to match your typing flow. The configurations are saved in `~/.c4vi/vimconfigs/config.json`.

Here are the primary configurations, their impacts, and use cases:

| Combination | `config.json` Settings | Visual & Editing Impact | Best For |
| :--- | :--- | :--- | :--- |
| **1. Full Autocomplete** | `"autocomplete_enabled": true`<br>`"auto_trigger": true`<br>`"trigger_on_methods_only": false` | Autocomplete menu pops up automatically on every keystroke (letters, numbers, underscores, operators). | Users who want maximum speed and real-time guidance as they type. |
| **2. Method-Only Trigger** | `"autocomplete_enabled": true`<br>`"auto_trigger": true`<br>`"trigger_on_methods_only": true` | Autocomplete ONLY triggers automatically when you type a member/method access symbol (`.` or `->`). | Users who want a clean typing flow, but want instant help discovering struct fields and methods. |
| **3. Manual Trigger** | `"autocomplete_enabled": true`<br>`"auto_trigger": false`<br>`"trigger_on_methods_only": false` | The popup never appears automatically. You must explicitly press `<C-Space>` in insert mode to show completions. | Traditional Vim users who find automated menus distracting but still want autocomplete on demand. |
| **4. Completion Disabled** | `"autocomplete_enabled": false`<br>`"auto_trigger": false`<br>`"trigger_on_methods_only": false` | Autocomplete features are fully disabled. No popups or triggers are active. | Users using external autocomplete plugins or who only want LSP hover tooltips and diagnostic highlights. |

### Tab Navigation Settings
Configure how you navigate and confirm items in the autocomplete popup:

* **Standard Vim Mode (`"map_tab_complete": false`)** *(Recommended)*:
  * *Impact*: Preserves standard Vim keymaps. Use **`<C-n>`** to move down, **`<C-p>`** to move up, and **`<C-y>`** to confirm a selection. 
  * *Why*: Standard `<Tab>` remains mapped to insert spacing and formatting, preventing coding layout issues.
* **VS Code Mode (`"map_tab_complete": true`)**:
  * *Impact*: Overrides standard keys. Use **`<Tab>`** to move down, **`<S-Tab>`** to move up, and **`<CR>` (Enter)** to confirm.
