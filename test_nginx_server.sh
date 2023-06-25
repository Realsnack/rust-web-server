#!/bin/bash

apt install -y nginx

# Prepare and run test
PORT=80
HOST=localhost
URL=http://$HOST:$PORT

# Check if nginx is running
if [ "$(systemctl is-active nginx)" = "active" ]; then
    echo "Nginx is running"
else
    echo "Nginx is not running"
    exit 1
fi

# Check if nginx is listening on port 80
if [ "$(ss -tulpn | grep nginx | grep 80)" ]; then
    echo "Nginx is listening on port 80"
else
    echo "Nginx is not listening on port 80"
    exit 1
fi

export PATH=$PATH:/root/k6

# Run k6 test
./k6 run /root/test.js -e URL=http://localhost:80 -e MAX_VU=1000
