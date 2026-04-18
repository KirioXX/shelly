shelly() {
    local cmd
    cmd=$(command shelly "$@")
    if [[ -n "$cmd" ]]; then
        print -z "$cmd"
    fi
}
