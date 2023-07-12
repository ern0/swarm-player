#!/bin/bash

dir="$( dirname -- "${BASH_SOURCE[0]}" )"
dir="$( realpath -- "$dir" )"
server=$dir/app/server


#alias p='clear; cargo build 2> /tmp/err.txt ; less /tmp/err.txt'
alias p='clear; cd $server; cargo build'

alias s='clear; curl -Is "http://localhost:8000/" | grep Started; target/debug/swarm-player-server'

#alias t='clear; cargo test -- --nocapture'
alias t='clear; cd $server; cargo test'
