shelly() {
    local cmd
    cmd=$(command shelly generate "$@")
    if [[ -n "$cmd" ]]; then
        print -z "$cmd"
    fi
}
