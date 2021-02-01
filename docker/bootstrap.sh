#!/bin/sh

echo root:$PASSWD | chpasswd

/usr/sbin/sshd -D