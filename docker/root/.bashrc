if [ -f "$HOME/.cargo/env" ]; then
source "$HOME/.cargo/env"
fi

export BUN_INSTALL="$HOME/.bun"
export PATH=$BUN_INSTALL/bin:$PATH

command -v direnv &>/dev/null && eval "$(direnv hook bash)"
command -v rtx &>/dev/null && eval $(rtx env)

[ -z "$PS1" ]  && return

command -v rtx &>/dev/null && eval "$(rtx activate --quiet bash)"
