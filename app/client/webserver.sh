#!/bin/bash

host=`ifconfig | grep 192 | cut -d' ' -f2`

caddy file-server --domain $host &
caddy file-server --listen :8000 &
read
pkill caddy
