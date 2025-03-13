#!/bin/bash

if [ -n "${IN_DOCKER}" ];then
	cargo tauri android build --target aarch64
else
	echo "Not in docker, building container..."
	PARENT=$(dirname $0)
	CARGO=/home/$USER/.cargo
	ARGS=(--rm -it -v $CARGO/registry:$CARGO/registry -v $CARGO/git:$CARGO/git -v $PARENT:/home/$USER/scout)

	if hash podman 2>/dev/null; then
		podman build -t frc900/scout-builder:latest --build-arg USER=$USER $PARENT
		podman run ${ARGS[@]} --userns=keep-id frc900/scout-builder:latest scout/build.sh
	else
		docker build -t frc900/scout-builder:latest --build-arg USER=$USER $PARENT
		docker run ${ARGS[@]} docker.io/frc900/scout-builder:latest scout/build.sh
	fi
fi
