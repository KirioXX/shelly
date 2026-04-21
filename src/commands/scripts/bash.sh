shelly() {
    local cmd
    cmd=$(command shelly generate "$@")
    if [[ -n "$cmd" ]]; then
        bind "\" \e[0n\": \"$cmd\""
        printf '\e[5n'
    fi
}
