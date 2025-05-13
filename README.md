# SSHY
Tool to create key pairs and modify the user's SSH configuration file (~/.config/.ssh/config).

# Installation
sshy can be installed using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) with `cargo install sshy`

# Usage
To run sshy, simply run `sshy` in a terminal and the TUI should take control.
The controls are listed bellow

# Controls

### General
- `q`: exit sshy
- `left` and `right`: navigate sections
- `up` and `down`: navigate the content of the focused section

### Popups
- `esc`: hide popup

### Public keys section
- `c`: copy to clipboard the content of the focused public key
- `n`: create new key pair

### Ssh client config
- `enter`: enter interactive (vim) mode

### Ssh client config (interactive mode)
- `q`: exit interactive mode
- `ctrl+s`: write buffer to file (~/.config/.ssh/config)
- `i`: insert mode
- `v`: visual mode
- `y`: yank
- `p`: paste
- `u`: undo
- `ctrl+r`: redo
