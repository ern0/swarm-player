#!/bin/bash
clear

if [ `uname -s` = "Linux" ]; then
	LINUX_SUDO=sudo
else
	LINUX_SUDO=
fi
$LINUX_SUDO nginx -s quit 2> /dev/null

set -e

export host=`ifconfig | grep 192 | cut -d' ' -f2`
if [ -z $host ]; then
	export host=`ifconfig | grep "inet 10" | cut -d't' -f2 | cut -d' ' -f2`
fi
export started=`date -Iminutes | cut -d"T" -f2 | cut -d"+" -f1`
export pwd=`pwd`
export cert_crt=`pwd`/cert.crt
export cert_key=`pwd`/cert.key

if [ ! -f "$cert_crt" ]; then

	if [ -z $host ]; then
		echo variable \$host is not set
		exit
	fi

	openssl req \
		-subj "/CN=$host" \
		-nodes -x509 -newkey rsa:4096 \
		-keyout $cert_key \
		-out $cert_crt \
		-sha256 -days 365
fi

export http_upgrade="\$http_upgrade"
envsubst <nginx.conf >/tmp/nginx.conf


$LINUX_SUDO nginx -c /tmp/nginx.conf
ps -e | grep nginx | grep -v grep
read
$LINUX_SUDO nginx -s quit
