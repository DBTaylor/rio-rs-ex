#!/bin/bash

PIDFILE=/var/run/app.pid

do_start(){
	/sbin/start-stop-daemon --start --pidfile $PIDFILE --make-pidfile --background --chuid admin --exec /home/lvuser/app
}
do_stop(){
	/sbin/start-stop-daemon --stop --pidfile $PIDFILE
	kill $(ps | grep http2mb | grep -v grep | awk '{print $1}')
}

case "$1" in
	start)
		do_start
		;;
	stop)
		do_stop
		;;
	restart)
		do_stop
		do_start
		;;
esac

exit 0