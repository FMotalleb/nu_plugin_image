use std log
def get-terminal-colors [] {
    let color_00 = (input $"hold enter 1 of 16\n(ansi -o '4;0;?')(ansi st)" --bytes-until-any "\e\\" -s) 
    let color_01 = (input $"hold enter 2 of 16\n(ansi -o '4;1;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_02 = (input $"hold enter 3 of 16\n(ansi -o '4;2;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_03 = (input $"hold enter 4 of 16\n(ansi -o '4;3;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_04 = (input $"hold enter 5 of 16\n(ansi -o '4;4;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_05 = (input $"hold enter 6 of 16\n(ansi -o '4;5;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_06 = (input $"hold enter 7 of 16\n(ansi -o '4;6;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_07 = (input $"hold enter 8 of 16\n(ansi -o '4;7;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_08 = (input $"hold enter 9 of 16\n(ansi -o '4;8;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_09 = (input $"hold enter 10 of 16\n(ansi -o '4;9;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_10 = (input $"hold enter 11 of 16\n(ansi -o '4;10;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_11 = (input $"hold enter 12 of 16\n(ansi -o '4;11;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_12 = (input $"hold enter 13 of 16\n(ansi -o '4;12;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_13 = (input $"hold enter 14 of 16\n(ansi -o '4;13;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_14 = (input $"hold enter 15 of 16\n(ansi -o '4;14;?')(ansi st)" --bytes-until-any "\e\\" -s)
    let color_15 = (input $"hold enter 16 of 16\n(ansi -o '4;15;?')(ansi st)" --bytes-until-any "\e\\" -s)
    print ("Found the colors of your shell, in the next step I will store them in your env var" | ansi gradient --fgstart 0xee33ff)
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
        let red = ($rgb | get 0 | str substring 0..2)
        let green = ($rgb | get 1 | str substring 0..2)
        let blue = ($rgb | get 2 | str substring 0..2)
        $"$env.($col | first) = 0x($red)($green)($blue)"
    }

    if ((["yes","no"] | input list "write config to the env file?") == "yes") {
        let config_path = $nu.env-path | path dirname | path join nu_image_plugin_conf.nu
        if (not ( $config_path | path exists)) {
            $"source ($config_path)" | save $nu.env-path --append
        } 
        "# Auto generated code" | save $config_path -f
        for i in $env_vars {
            $"($i)\n" | save $config_path --append
        }
    } else {
        for i in $env_vars {
            print $"($i)\n"
        }
        print "add thse values to environment variables using `config env`"
    }
}
if ((["yes","no" ] | input list "do you want to save your current shell's theme as default for `to png`?") == "yes" ) {
   print (get-terminal-colors)
}