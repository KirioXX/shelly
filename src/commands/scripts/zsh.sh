shelly() {
    # Check if first arg is a valid subcommand (not a prompt)
    if [[ -n "$1" ]] && command shelly cmds | grep -qx "$1"; then
        command shelly "$@"
        return $?
    fi
    
    # Otherwise, treat as a prompt for command generation
    local cmd
    cmd=$(command shelly generate "$@")
    if [[ -n "$cmd" ]]; then
        print -z "$cmd"
    fi
}
