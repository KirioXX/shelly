function shelly
    set -l cmd (command shelly $argv)
    if test -n "$cmd"
        commandline -r "$cmd"
    end
end