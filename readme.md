# Gsshaskpass

gsshaskpass is a gui dialog for ssh-agent.

To use it set SSH_ASKPASS to the gsshaskpass bin.

```shell
export SSH_ASKPASS=~/bin/gsshaskpass
```

## Dependencies

gtk v3.10 or greater.

## Building from source

To build this project you will need gtk as mentioned above and also the rust compiler and cargo tooling.
To install rust and cargo refer to [https://rustup.rs/](https://rustup.rs/)

```shell
cargo build --release
```

License MIT
Author: Adilson Schmitt Junior
