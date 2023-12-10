# The Dark Matter Problem <!-- omit in toc -->

- [Setup](#setup)

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [VSCode](https://code.visualstudio.com/download)
3. Type `cargo run` in the command line

## Git

Get current project version

1. git pull

Upload your changes to the World Wide Web

1. git add .
2. git commit -m "commit message here"
3. git push

To update existing image (background / sprite animation / etc.)

1. Drag your new image from your file explorer to the assets folder
2. Look for system.rs in the specific file you are updating, eg mushroom or player
3. Use control+f for asset_server.load or your eyes to find something like one of these -
   let texture_handle = asset_server.load("file_name.png");
   texture: asset_server.load("other_file_name.png"),
4. Then, replace the file name with your new file and save
5. Do git shenangins above
