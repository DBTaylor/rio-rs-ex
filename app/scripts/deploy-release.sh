#! /bin/bash

. scripts/config
. scripts/password

url=sftp://$ip/$remote_dir/

cargo build --release
. scripts/kill-app.sh
curl --insecure --user admin:$password -T target/armv7-unknown-linux-gnueabi/release/$app $url
curl --insecure --user admin:$password -T src/fpga/$bitfile_name $url
curl --insecure --user admin:$password -T scripts/app.sh sftp://$ip//etc/init.d/
plink admin@$ip -pw "$password" -batch "chmod +x $remote_dir/$app; chmod +x /etc/init.d/app.sh; /usr/sbin/update-rc.d -f app.sh defaults; reboot"
echo rebooting