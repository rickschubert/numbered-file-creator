✍️ numbered-file-creator
=======================

I created this tool for my local writing workflow. I have a directory with a lot of scenes which I number, i.e. 01, 02, 03 etc. This tool helps me achieve that!

Let's say I want to create a new scene that should be on position `02` and thus push all scenes to the back that are already there, i.e. `02`, `03`, and `04` which now need to become `03`, `04` and `05`: We can do so by running `numbered-file-creator ./scenes/02_my_new_file.fountain`. This will create a new file under the specified location and rename the files that need to be "pushed away" in order to make space for the new scene.

## Build
- Run `cargo build --release` on your specific environment - it will then be in the `target/release/` directory ready for you to use!
