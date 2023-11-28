# nu_plugin_image

A [nushell](https://www.nushell.sh/) plugin to create qr code in terminal

## Examples

```bash
~> "https://google.com" | to qr
```

![image](https://github.com/FMotalleb/nu_plugin_image/assets/30149519/1771961a-b06b-4310-81ed-63865e8d2f8e)

## Installing

* using [nupm](https://github.com/nushell/nupm)

```bash
git clone https://github.com/FMotalleb/nu_plugin_image.git
nupm install --path nu_plugin_image -f
```

* or compile manually

```bash
git clone https://github.com/FMotalleb/nu_plugin_image.git
cd nu_plugin_image
cargo build
register target/debug/nu_plugin_image
```

* or using cargo

```bash
cargo install nu_plugin_image
register  ~/.cargo/bin/nu_plugin_image
```
