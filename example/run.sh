#!/bin/bash

set -euxo pipefail

function retry {
  until eval "$*"; do
    echo "Retying in 5 seconds"
    sleep 5
  done
}

minikube start
minikube addons enable registry
docker build -t crossplane-rust-config-fn ..
crossplane xpkg build --package-root=function --embed-runtime-image=crossplane-rust-config-fn --package-file=fn.xpkg
# registry might not be ready yet
retry crossplane xpkg push --package-files=fn.xpkg $(minikube ip):5000/crossplane-rust-config-fn:v0.1.0
minikube image load $(minikube ip):5000/crossplane-rust-config-fn:v0.1.0
crossplane xpkg build --package-root=configuration --package-file=conf.xpkg
crossplane xpkg push --package-files=conf.xpkg $(minikube ip):5000/crossplane-rust-config:latest
minikube image load $(minikube ip):5000/crossplane-rust-config:latest
kustomize build minikube/crossplane --enable-helm | kubectl apply --context minikube -f -
# make sure crossplane is ready
retry kubectl get CompositeResourceDefinition --context minikube
kustomize build minikube/crossplane-providers --enable-helm | kubectl apply --context minikube -f -
# make sure kubernetes provider is ready
retry kubectl get providerconfigs.kubernetes
# make sure our function Configuration is ready
retry kubectl get Configs
kustomize build minikube --enable-helm | kubectl apply --context minikube -f -
