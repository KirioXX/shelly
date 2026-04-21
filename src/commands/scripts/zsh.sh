shelly() {
    # If first arg is a non-generate subcommand, pass through directly
    if [[ "$1" =~ ^(setup|help|-h|--help|-V|--version|generate)$ ]]; then
        command shelly "$@"
        return $?
    fi
    
    # Otherwise, treat as generate (for backwards compat with bare prompts)
    local cmd
    cmd=$(command shelly generate "$@")
    if [[ -n "$cmd" ]]; then
        print -z "$cmd"
    fi
}
