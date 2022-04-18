/*!
## How to deploy to aws

Prerequisites:
- [aws account](https://aws.amazon.com)
- [aws cli](https://aws.amazon.com/cli/)
- [musl gcc](https://www.musl-libc.org/how.html)

Installing musl-gcc:
- Darwin: `brew install filosottile/musl-cross/musl-cross`
- Linux: `sudo apt get install musl-tools`

**1. Add build target to rustup**

```no_run
rustup target add x86_64-unknown-linux-musl
```

**2. Update cargo config with target & linker**

```no_run
mkdir .cargo
echo -e '[target.x86_64-unknown-linux-musl]\nlinker = "x86_64-linux-musl-gcc"' > .cargo/config
```

You can either create the .cargo folder inside you project or install it in you home directory if you prefer to use the same settings across all your projects.
You might also have to create a symbolic link to the installd linker.
```no_run
ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
```

**3. Build your program**
```no_run
cargo build --release --target x86_64-unknown-linux-musl
```

The aws lambda expects the uploaded binary to be named `bootstrap`. So either make sure to rename your binary add boostrap to your Config.toml and build with: `--bin bootstrap`.
```no_run
[[ bin ]]
name = "boostrap"
path = "src/main.rs"
```

**4. Zip the binary**

-j *stores just the name of the saved file, 'junk the path'*
```no_run
zip -j lambda.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
```

**5. Create the lambda.**

*/
