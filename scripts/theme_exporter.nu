use std log
def confirm [message: string] : -> bool {
    ["yes","no"] | input list $message | $in == "yes"
}
def get-terminal-colors [] {
    let color_00 = (input $"hold enter 1 of 16(ansi -o '4;0;?')(ansi st)" --bytes-until-any "\e\\" -s) 
    if ($color_00 | is-empty) {
        log error "failed to read colors from terminal, you have to use one of predefined themes or set colors manually"
        return
    }
    let color_01 = (input $"\rhold enter 2 of 16(ansi -o '4;1;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_02 = (input $"\rhold enter 3 of 16(ansi -o '4;2;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_03 = (input $"\rhold enter 4 of 16(ansi -o '4;3;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_04 = (input $"\rhold enter 5 of 16(ansi -o '4;4;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_05 = (input $"\rhold enter 6 of 16(ansi -o '4;5;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_06 = (input $"\rhold enter 7 of 16(ansi -o '4;6;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_07 = (input $"\rhold enter 8 of 16(ansi -o '4;7;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_08 = (input $"\rhold enter 9 of 16(ansi -o '4;8;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_09 = (input $"\rhold enter 10 of 16(ansi -o '4;9;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_10 = (input $"\rhold enter 11 of 16(ansi -o '4;10;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_11 = (input $"\rhold enter 12 of 16(ansi -o '4;11;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_12 = (input $"\rhold enter 13 of 16(ansi -o '4;12;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_13 = (input $"\rhold enter 14 of 16(ansi -o '4;13;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_14 = (input $"\rhold enter 15 of 16(ansi -o '4;14;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_15 = (input $"\rhold enter 16 of 16(ansi -o '4;15;?')(ansi st)" --bytes-until-any "\e\\" -s)
    print ("\nFound the colors of your shell, in the next step I will store them in your env var" | ansi gradient --fgstart 0xee33ff)
    sleep 1sec

    # goes through 4;255;?
    let env_vars = [
      [NU_PLUGIN_IMAGE_FG,  $color_07]
      [NU_PLUGIN_IMAGE_BG ,  $color_00]
      [NU_PLUGIN_IMAGE_BLACK ,  $color_00]
      [NU_PLUGIN_IMAGE_RED, $color_01]
      [NU_PLUGIN_IMAGE_GREEN, $color_02]
      [NU_PLUGIN_IMAGE_YELLOW,  $color_03]
      [NU_PLUGIN_IMAGE_BLUE, $color_04]
      [NU_PLUGIN_IMAGE_MAGENTA,  $color_05]
      [NU_PLUGIN_IMAGE_CYAN,  $color_06]
      [NU_PLUGIN_IMAGE_WHITE,  $color_07]
      [NU_PLUGIN_IMAGE_BRIGHT_BLACK, $color_08]
      [NU_PLUGIN_IMAGE_BRIGHT_RED, $color_09]
      [NU_PLUGIN_IMAGE_BRIGHT_GREEN, $color_10]
      [NU_PLUGIN_IMAGE_BRIGHT_YELLOW,  $color_11]
      [NU_PLUGIN_IMAGE_BRIGHT_BLUE,  $color_12]
      [NU_PLUGIN_IMAGE_BRIGHT_MAGENTA, $color_13]
      [NU_PLUGIN_IMAGE_BRIGHT_CYAN,  $color_14]
      [NU_PLUGIN_IMAGE_BRIGHT_WHITE, $color_15]
     ] | each {|col|
        let rgb = $col | get 1 | split row : | get 1 | split row /
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