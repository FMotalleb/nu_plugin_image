# nu_plugin_image

A [nushell](https://www.nushell.sh/) plugin to create qr code in terminal

## Examples

```bash
~> ls | table -c | to png output.png
```

<img src="https://github.com/FMotalleb/nu_plugin_image/assets/30149519/faab9f4f-8935-4c0e-afd7-9fd7c5c6eccc" alt="drawing" width="200"/>

```bash
~> open test.png -r | from png --width 150 | to png output.png
```

from: <img src="https://github.com/FMotalleb/nu_plugin_image/assets/30149519/73e20721-dec0-4604-8f10-c5b36fbad389" alt="drawing" width="200"/>
to:  <img src="https://github.com/FMotalleb/nu_plugin_image/assets/30149519/c3258e71-e42f-483a-b8c8-6fcb3af82e78" alt="drawing" width="200"/>

## Installing

Cargo Features:

* fonts:
* log level:

* using [nupm](https://github.com/nushell/nupm) (preferred)

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
