# 🖼 nu_plugin_image  

A [Nushell](https://www.nushell.sh/) plugin to convert ANSI strings into PNG images and create ANSI text from images.

---

## ✨ Features  

This plugin allows you to:

- Convert ANSI strings to PNG images with customizable fonts and themes.
- Create ANSI text from an image, enabling you to transform visual data into a textual representation.

---

### **`to png`** – Convert ANSI String to PNG Image  

The `to png` command converts an ANSI string into a PNG image. Customizable font and theme options are available, with custom flags overriding the default settings.

#### 📌 Usage  

```bash
> to png {flags} (output-path)
```

#### ⚙️ Available Flags  

- `-h, --help`               → Display the help message for this command.  
- `-w, --width <int>`        → Output width.  
- `-t, --theme <string>`     → Select the theme of the output. Available themes: ["vscode", "xterm", "ubuntu", "eclipse", "mirc", "putty", "winxp", "terminal", "win10", "win_power-shell", "win_ps"]. Defaults to `vscode`.  
- `--font <string>`          → Select the font from one of ["SourceCodePro", "Ubuntu", "IosevkaTerm", "AnonymousPro"]. Defaults to the first font in the list.  
- `--custom-font-regular <path>` → Path to a custom regular font.  
- `--custom-font-bold <path>`    → Path to a custom bold font.  
- `--custom-font-italic <path>`  → Path to a custom italic font.  
- `--custom-font-bold_italic <path>` → Path to a custom bold italic font.  
- `--custom-theme-fg <string>`   → Custom foreground color in hex format (e.g., `#FFFFFF` for white).  
- `--custom-theme-bg <string>`   → Custom background color in hex format (e.g., `#00000000` for transparent).  
- `--custom-theme-black <string>` → Custom black color in hex format (e.g., `#1C1C1C`).  
- `--custom-theme-red <string>`   → Custom red color in hex format (e.g., `#FF0000`).  
- `--custom-theme-green <string>` → Custom green color in hex format (e.g., `#00FF00`).  
- `--custom-theme-yellow <string>` → Custom yellow color in hex format (e.g., `#FFFF00`).  
- `--custom-theme-blue <string>`  → Custom blue color in hex format (e.g., `#0000FF`).  
- `--custom-theme-magenta <string>` → Custom magenta color in hex format (e.g., `#FF00FF`).  
- `--custom-theme-cyan <string>`  → Custom cyan color in hex format (e.g., `#00FFFF`).  
- `--custom-theme-white <string>` → Custom white color in hex format (e.g., `#FFFFFF`).  
- `--custom-theme-bright_black <string>` → Custom bright black color in hex format (e.g., `#808080`).  
- `--custom-theme-bright_red <string>` → Custom bright red color in hex format (e.g., `#FF5555`).  
- `--custom-theme-bright_green <string>` → Custom bright green color in hex format (e.g., `#55FF55`).  
- `--custom-theme-bright_yellow <string>` → Custom bright yellow color in hex format (e.g., `#FFFF55`).  
- `--custom-theme-bright_blue <string>` → Custom bright blue color in hex format (e.g., `#5555FF`).  
- `--custom-theme-bright_magenta <string>` → Custom bright magenta color in hex format (e.g., `#FF55FF`).  
- `--custom-theme-bright_cyan <string>` → Custom bright cyan color in hex format (e.g., `#55FFFF`).  
- `--custom-theme-bright_white <string>` → Custom bright white color in hex format (e.g., `#FFFFFF`).  
- `--log-level <string>`      → Set log level. Options: `CRITICAL (c)`, `ERROR (e)`, `WARN (w)`, `INFO (i)`, `DEBUG (d)`, `TRACE (t)`. Defaults to `INFO`.  

#### 📊 Example: Convert ANSI String to PNG with Custom Theme  

```bash
> to png --theme "xterm" --custom-theme-fg "#FF00FF" --custom-theme-bg "#00000000" output.png
```

---

### **`from png`** – Create ANSI Text from an Image  

The `from png` command converts an image into its corresponding ANSI text representation.

#### 📌 Usage  

```bash
> from png {flags}
```

#### ⚙️ Available Flags  

- `-h, --help`               → Display the help message for this command.  
- `-x, --width <int>`        → Output width, in characters.  
- `-y, --height <int>`       → Output height, in characters.  
- `--log-level <string>`     → Set log level. Options: `CRITICAL (c)`, `ERROR (e)`, `WARN (w)`, `INFO (i)`, `DEBUG (d)`, `TRACE (t)`. Defaults to `INFO`.  

#### 📊 Example: Convert PNG Image to ANSI Text  

```bash
> from png --width 80 --height 20 image.png
```

---

## 🔧 Installation  

### 🚀 Recommended: Using [nupm](https://github.com/nushell/nupm)  

This method automatically handles dependencies and features.  

```bash
git clone https://github.com/FMotalleb/nu_plugin_image.git  
nupm install --path nu_plugin_image -f  
```  

### 🛠️ Manual Compilation  

```bash
git clone https://github.com/FMotalleb/nu_plugin_image.git  
cd nu_plugin_image  
cargo build -r  
plugin add target/release/nu_plugin_image  
```  

### 📦 Install via Cargo (using git)  

```bash
cargo install --git https://github.com/FMotalleb/nu_plugin_image.git  
plugin add ~/.cargo/bin/nu_plugin_image  
```  

### 📦 Install via Cargo (crates.io) _Not Recommended_  
>
> _Since I live in Iran and crates.io often restricts package updates, the version there might be outdated._  

```bash
cargo install nu_plugin_image  
plugin add ~/.cargo/bin/nu_plugin_image  
```
