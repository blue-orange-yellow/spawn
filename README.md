# Overview
This tool provide generating uuid command.
The existing `uuidgen` command does not provide an option to copy the result directly to the clipboard,  
so I created this tool with that functionality.

# Demo
![Demo](./demo.gif)

# Install
```bash
git clone <this repository>
cd spawn
cargo install --path .
```

# Usage
```bash
# basically usage
spawn uuid

# copy to clipboard
spawn uuid -c
```