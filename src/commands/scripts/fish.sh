function shelly
    # If first arg is a non-generate subcommand, pass through directly
    if contains -- "$argv[1]" setup help -h --help -V --version generate
        command shelly $argv
        return $status
    end
    
    # Otherwise, treat as generate (for backwards compat with bare prompts)
    set -l cmd (command shelly generate $argv)
    if test -n "$cmd"
        commandline -r "$cmd"
    end
end