#!/bin/sh

PARENT=$(dirname $0)
docker build -t frc900/scout-builder:latest --build-arg USER=$USER .
docker run "$@" --rm -it -v $PARENT:/home/$USER/scout --userns=keep-id frc900/scout-builder:latest /bin/bash
