function shelly
    # Check if first arg is a valid subcommand (not a prompt)
    if test -n "$argv[1]" && command shelly commands | grep -qx "$argv[1]"
        command shelly $argv
        return $status
    end
    
    # Otherwise, treat as a prompt for command generation
    set -l cmd (command shelly generate $argv)
    if test -n "$cmd"
        commandline -r "$cmd"
    end
end