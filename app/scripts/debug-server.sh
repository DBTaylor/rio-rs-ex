#! /bin/bash

. scripts/config
. scripts/password

#kills the app if it's running and runs it with gdbserver
start sh -c "plink admin@$ip -pw \"$password\" -batch \"gdbserver host:$debug_port $remote_dir/$app\"; sleep 10; exit; exec sh"