# curlspeed
Use libcurl to test download speed of a specific URL

# Build
> [!IMPORTANT]
> The project depends on native `libcurl`, install it before project build or development

> [!NOTE]
> Since this project is inteded to run on OpenWRT, `libcurl` is statically linked to the executable.

Use `cargo build` to build native executable

## Cross compile to linux
You can leverage the nix flake in the project to cross compile to linux target.

The corresponding output is `#curlspeed.<target>-linux`, which target can be `aarch64` or `x86_64`.

For example, the following command will build executable target to aarch64-linux at `result/bin/curlspeed`.

```bash
nix build '.#curlspeed.aarch64-linux'
```

# Development
The fastest way to do this is to use nix flake + devenv + direnv.

If everything setup correctly, after cd into the project directory, use command `direnv allow` will automatically setup everything and you are ready to go. (The command might need some time to install environment, it usually takes about 10+ minute at the first time)

