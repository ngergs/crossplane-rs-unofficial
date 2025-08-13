#!/bin/bash

set -euxo pipefail

function retry {
  until eval "$*"; do
    echo "Retrying in 5 seconds"
    sleep 5
  done
}

executables="crossplane docker envsubst k3d kubectl kustomize helm"

# Loop through each executable
for cmd in $executables; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "$cmd is required but not in the PATH"
        exit 1
    fi
done


docker build -t crossplane-rust-config-fn -f ../Dockerfile ../..

k3d cluster create --config k3d.yaml
export REGISTRY="k3d-crossplane.localhost:5000"
CONTEXT="k3d-crossplane"

export FN_IMAGE="crossplane-rust-config-fn"
export FN_TAG="v0.1.0"
export CONF_IMAGE="crossplane-rust-config"
export CONF_TAG="v0.1.0"
crossplane xpkg build --package-root=function --embed-runtime-image=${FN_IMAGE} --package-file=fn.xpkg
# registry might not be ready yet
retry crossplane xpkg push --package-files=fn.xpkg ${REGISTRY}/${FN_IMAGE}:${FN_TAG}
envsubst < crossplane-config-template.yaml > configuration/crossplane.yaml
crossplane xpkg build --package-root=configuration --package-file=conf.xpkg
crossplane xpkg push --package-files=conf.xpkg ${REGISTRY}/${CONF_IMAGE}:${CONF_TAG}
kustomize build k3d/crossplane --enable-helm | kubectl apply --context ${CONTEXT} -f -
# make sure crossplane is ready
retry kubectl get CompositeResourceDefinition --context ${CONTEXT}
retry kubectl get Provider --context ${CONTEXT}
envsubst < k3d/crossplane-providers/configuration-template.yaml >  k3d/crossplane-providers/configuration.yaml
kustomize build k3d/crossplane-providers --enable-helm | kubectl apply --context ${CONTEXT} -f -
# make sure our function Configuration is ready
retry kubectl get Configs
kustomize build k3d --enable-helm | kubectl apply --context ${CONTEXT} -f -
