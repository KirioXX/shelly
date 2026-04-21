function shelly
    set -l cmd (command shelly generate $argv)
    if test -n "$cmd"
        commandline -r "$cmd"
    end
end