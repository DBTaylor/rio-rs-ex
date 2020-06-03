#! /bin/bash

. scripts/config
. scripts/password

url=sftp://$ip/$remote_dir/

cargo build
. scripts/kill-app.sh
curl --insecure --user admin:$password -T target/armv7-unknown-linux-gnueabi/debug/$app $url
curl --insecure --user admin:$password -T src/fpga/$bitfile_name $url
plink admin@$ip -pw "$password" -batch "chmod +x $remote_dir/$app"