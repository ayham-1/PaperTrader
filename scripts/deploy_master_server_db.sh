#!/bin/sh

docker-compose -f ./DockerFiles/master_server.yml ${1:-up}
