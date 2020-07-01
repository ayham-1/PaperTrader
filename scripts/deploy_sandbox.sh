#!/bin/sh

docker-compose -f ../DockerFiles/sandbox.yml ${1:-up}
