#!/bin/bash

# This script is used to build the project and push the pkgbuilds to the AUR
CASAOS=./build/casaos
CASAOS_APP_MANAGEMENT=./build/casaos-app-management
CASAOS_CLI=./build/casaos-cli
CASAOS_GATEWAY=./build/casaos-gateway
CASAOS_MESSAG_=./build/casaos-message-bus
CASAOS_LOCAL_STORAGE=./build/casaos-local-storage
CASAOS_UI=./build/casaos-ui
CASAIS_USER_SERVICE=./build/casais-user-service

PATHS=($CASAOS $CASAOS_APP_MANAGEMENT $CASAOS_CLI $CASAOS_GATEWAY $CASAOS_MESSAG_ $CASAOS_LOCAL_STORAGE $CASAOS_UI $CASAIS_USER_SERVICE)

build() {
    for path in "${PATHS[@]}"; do
        cd "$path"
        makepkg -si --clean
    done
}
