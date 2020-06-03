#! /bin/bash

. scripts/config
. scripts/password

plink admin@$ip -pw "$password" -batch "kill \$(ps | grep -v grep | grep $remote_dir/$app | awk '{print \$1}')"