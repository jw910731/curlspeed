# curlspeed
Use libcurl to test download speed of a specific URL

# Usage
Basic usage is as follow:

```bash
./curlspeed <Connection target> [<Min Speed> <Max Speed>]
```

For example:

```bash
./curlspeed https://github.com/ppy/osu/releases/download/2025.316.0/osu.app.Apple.Silicon.zip'
```

## Speed Limit
You can set min speed to early terminates the speed test if download speed per second is lower then this value for over 10 seconds.

The max speed need to be set as well, which force the download speed to be less then this value.

```bash
./curlspeed https://github.com/ppy/osu/releases/download/2025.316.0/osu.app.Apple.Silicon.zip' 100KB 33MB
```

## Timeout
The program has a hard-coded 30 seconds timeout. 

If the download do not complete within 30 seconds, the test will force to end.

# Output
The output of the program is like:
```
<min download speed> <max download speed> <average download speed> <force termination> 
```

Download speed ar in unit of [G|M|K]iB (Byte) per second.

Note that min download speed refers to the minimum non-zero download speed during the test.

Force termination refers whether the download file really successfully downloaded or not. 

`true` means the test early terminate, this may indicate that the downlaod speed is too slow for 10 seconds, or the test timeout is reached.


# Build
> [!IMPORTANT]
> The project depends on native `libcurl`, install it before project build or development

> [!NOTE]
> Since this project is inteded to run on OpenWRT, `libcurl` is statically linked to the executable.

Use `cargo build` to build native executable

## Cross Compile to Linux
You can leverage the nix flake in the project to cross compile to linux target.

The corresponding output is `#curlspeed.<target>-linux`, which target can be `aarch64` or `x86_64`.

For example, the following command will build executable target to aarch64-linux at `result/bin/curlspeed`.

```bash
nix build '.#curlspeed.aarch64-linux'
```

# Development
The fastest way to do this is to use nix flake + devenv + direnv.

If everything setup correctly, after cd into the project directory, use command `direnv allow` will automatically setup everything and you are ready to go. (The command might need some time to install environment, it usually takes about 10+ minute at the first time)

