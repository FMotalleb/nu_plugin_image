use std log

let default_font = {
        name: IoSevkaTerm,
        feature: font-iosevka_term 
    };
# TODO add licenses
let fonts = [
    # {
    #     name: IoSevkaTerm,
    #     feature: font-iosevka_term 
    # },
    {
        name: AnonymousPro,
        feature: font-anonymous_pro 
    },
    {
        name: SourceCodePro,
        feature: font-source_code_pro 
    },
    {
        name: Ubuntu,
        feature: font-ubuntu 
    },
]

def main [package_file: path] {
    let repo_root = $package_file | path dirname
    let install_root = $env.NUPM_HOME | path join "plugins"
    let selected_fonts = $fonts 
        | input list -m "select other fonts to install"
        | append $default_font
        | get feature
    
    let name = open ($repo_root | path join "Cargo.toml") | get package.name
    let ext = if ($nu.os-info.name == 'windows') { '.exe' } else { '' }
    let command = $"cargo install --path ($repo_root) --root ($install_root) --features=\"($selected_fonts | str join ',')\""
    log info $"building using `($command)`" 
    nu --commands $"($command)"
    nu --commands $"register ($install_root | path join "bin" $name)($ext)"
    log info "do not forget to restart Nushell for the plugin to be fully available!"
}
