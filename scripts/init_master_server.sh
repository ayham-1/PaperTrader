#!/bin/sh

# Set up postgresql config files
su
echo "listen_addresses = 'localhost'" > /var/lib/postgres/data/postgresql.conf
	# TYPE		DBs		ADDRESS			AUTH
echo "local		pt_db	127.0.0.1/32	md5" > /var/lib/postgres/data/pg_hba.conf

# Start postgresql
systemctl start postgresql.service

# Set up data 'cluster'
su -l postgres
initdb --locale=en_US.UTF-8 -E UTF8 -D /var/lib/postgres/data

# Set up pt_usr
su
useradd -ms /bin/bash pt_usr
su -l postgres
createuser pt_usr
createdb pt_db
