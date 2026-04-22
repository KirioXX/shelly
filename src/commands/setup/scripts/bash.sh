shelly() {
    # Check if first arg is a valid subcommand or help/version flag
    if [[ -n "$1" ]] && { command shelly cmds | grep -qx -- "$1" || [[ "$1" =~ ^(-h|--help|-V|--version)$ ]]; }; then
        command shelly "$@"
        return $?
    fi
    
    # Otherwise, treat as a prompt for command generation
    local cmd
    cmd=$(command shelly generate "$@")
    if [[ -n "$cmd" ]]; then
        bind "\" \e[0n\": \"$cmd\""
        printf '\e[5n'
    fi
}
