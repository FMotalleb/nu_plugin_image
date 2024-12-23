use std log
def confirm [message: string] : any -> bool {
    ["yes","no"] | input list $message | $in == "yes"
}
def "from binary" [] : binary -> string {
    $in | encode base64 | base64 -d
}

def get-terminal-colors [] {
    
    let colors = (0..15 | each {|i| $"4;($i);?" 
        | (term query $"(ansi osc)(ansi -o $in)(ansi st)" --terminator "\e\\" 
            | from binary
            | split row :
            | get 1
            | split row /)
        })
    
    let env_vars = [
      [NU_PLUGIN_IMAGE_FG,  ($colors | get 07)]
      [NU_PLUGIN_IMAGE_BG ,  ($colors | get 00)]
      [NU_PLUGIN_IMAGE_BLACK ,  ($colors | get 00)]
      [NU_PLUGIN_IMAGE_RED, ($colors | get 01)]
      [NU_PLUGIN_IMAGE_GREEN, ($colors | get 02)]
      [NU_PLUGIN_IMAGE_YELLOW,  ($colors | get 03)]
      [NU_PLUGIN_IMAGE_BLUE, ($colors | get 04)]
      [NU_PLUGIN_IMAGE_MAGENTA,  ($colors | get 05)]
      [NU_PLUGIN_IMAGE_CYAN,  ($colors | get 06)]
      [NU_PLUGIN_IMAGE_WHITE,  ($colors | get 07)]
      [NU_PLUGIN_IMAGE_BRIGHT_BLACK, ($colors | get 08)]
      [NU_PLUGIN_IMAGE_BRIGHT_RED, ($colors | get 09)]
      [NU_PLUGIN_IMAGE_BRIGHT_GREEN, ($colors | get 10)]
      [NU_PLUGIN_IMAGE_BRIGHT_YELLOW,  ($colors | get 11)]
      [NU_PLUGIN_IMAGE_BRIGHT_BLUE,  ($colors | get 12)]
      [NU_PLUGIN_IMAGE_BRIGHT_MAGENTA, ($colors | get 13)]
      [NU_PLUGIN_IMAGE_BRIGHT_CYAN,  ($colors | get 14)]
      [NU_PLUGIN_IMAGE_BRIGHT_WHITE, ($colors | get 15)]
     ] | each {|col|
        let rgb = $col | get 1
        # 16bit rgb to 8bit =  0xe7e7 | bits and 0x00ff
        let red = ($"0x($rgb | get 0)" | into int | bits and 0x00ff)
        let green = ($"0x($rgb | get 1)" | into int | bits and 0x00ff)
        let blue = ($"0x($rgb | get 2)" | into int | bits and 0x00ff)
        let red_hx = ($red | fmt).lowerhex | str substring 2..
        let green_hx = ($green | fmt).lowerhex | str substring 2..
        let blue_hx = ($blue | fmt).lowerhex | str substring 2..
        $"$env.($col | first) = 0x($red_hx)($green_hx)($blue_hx)"
    }

    if (confirm "write config to the env file?") {
        
        let default = ($nu.env-path | path dirname | path join nu_image_plugin_conf.nu)
        let config_path = input $"where should i save the env file? \(default: ($default)\)\n~> "
            | if (not ($in | is-empty)) {
                $in
              } else {
                ($default)
            } 

              
        if (not ( $config_path | path exists)) {
            $"source ($config_path)" | save $nu.env-path --append
        } 
        
        $"# Auto generated code\n($env_vars | str join "\n")" | save $config_path -f
  
        log info "Please restart the shell"
    } else {
        for i in $env_vars {
            print $"($i)\n"
        }
        print "add thse values to environment variables using `config env`"
    }
}

if (confirm "do you want to save your current shell's theme as default for `to png`?") {
   print (get-terminal-colors)
}