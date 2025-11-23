# Firmware

I want to clarify that, while this code is in no way firmware, I would like this project to be funded by [Blueprint](https://blueprint.hackclub.com/), and in order to receive the grant I am following their repository organizational guidelines to the best of my ability.

# Building

For anyone who wishes to build this code, a few things need to be installed for everything to properly build.

`rustup target add aarch64-unknown-linux-gnu`

`cargo install cross`

Then to build, rather than using cargo, you will use cross.

`cross build --release --target aarch64-unknown-linux-gnu`

The binary can then be found in `target/aarch64-unknown-linux-gnu/release/`