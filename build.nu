use std log


# TODO add licenses
let fonts = [
    [name, feature];
    [
        "AnonymousPro Font",
        font-anonymous_pro 
    ],
   [
    
   "IosevkaTerm Font",
   font-iosevka_term 
   ],
    [
        "Ubuntu Font",
        font-ubuntu 
    ],
   [ 
    "Debug log level (only used for debuging)",
   with-debug
   ],
    [
         "Trace log level (only used for advanced debuging)",
         with-trace 
    ],
]


def main [package_file: path] {
    let repo_root = $package_file | path dirname
    let install_root = $env.NUPM_HOME | path join "plugins"
    let selected_fonts = $fonts 
        | input list -d name -m "select features to install"
        | get feature
    
    let name = open ($repo_root | path join "Cargo.toml") | get package.name
    let ext = if ($nu.os-info.name == 'windows') { '.exe' } else { '' }
    let command = $"cargo install --path ($repo_root) --root ($install_root) --features=\"($selected_fonts | str join ',')\""
    log info $"building using `($command)`" 
    nu --commands $"($command)"
    plugin add $"($install_root | path join "bin" $name)($ext)"
    log info "do not forget to restart Nushell for the plugin to be fully available!"
    nu ($repo_root | path join scripts theme_exporter.nu)
}
