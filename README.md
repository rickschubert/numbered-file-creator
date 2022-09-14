scene-creator
=============

This is my first Rust project ever🥳🦀. This tool can be used to create a new scene in a directory. In my case, there is a directory like `scenes/` that contains numbered files, like `01_something.fountain`, `02_something.fountain`, `03_something.fountain` etc. Let's say we want to create a new scene that should come after `01` and thus push the other scenes to the back: We can do so by running `scene-creator ./scenes/02_my_new_file.fountain`. This will create a new file under the specified location and rename the files that need to be "pushed away" in order to make space for the new scene.

## Build
- Run `cargo build --release` on your specific environment - it will then be in the `target/release/` directory ready for you to use!
