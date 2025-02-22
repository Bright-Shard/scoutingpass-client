#!/bin/sh

PARENT=$(dirname $0)

adb install $PARENT/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk
