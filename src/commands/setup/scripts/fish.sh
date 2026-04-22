function shelly
    # Check if first arg is a valid subcommand or help/version flag
    set -l valid_subcommand (command shelly cmds | grep -qx -- "$argv[1]" 2>/dev/null; echo $status)
    set -l is_help (string match -r "^(-h|--help|-V|--version)$" "$argv[1]" 2>/dev/null; echo $status)
    if test -n "$argv[1]" -a "$valid_subcommand" -eq 0 -o "$is_help" -eq 0
        command shelly $argv
        return $status
    end
    
    # Otherwise, treat as a prompt for command generation
    set -l cmd (command shelly generate $argv)
    if test -n "$cmd"
        commandline -r "$cmd"
    end
end