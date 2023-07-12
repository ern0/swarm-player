#!/bin/bash
clear

nginx -s quit 2> /dev/null
set -e

export host=`ifconfig | grep 192 | cut -d' ' -f2`
export pwd=`pwd`
export cert_crt=`pwd`/cert.crt
export cert_key=`pwd`/cert.key

if [ ! -f "$cert_crt" ]; then
	openssl req \
		-subj "/CN=$host" \
		-nodes -x509 -newkey rsa:4096 \
		-keyout $cert_key \
		-out $cert_crt \
		-sha256 -days 365
fi

envsubst <nginx.conf >/tmp/nginx.conf

nginx -c /tmp/nginx.conf
ps -e | grep nginx | grep -v grep
read
nginx -s quit