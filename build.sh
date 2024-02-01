#!/bin/bash/env bash

set -ex

GROUP_ID="com.elcon"
ARTIFACT_ID="health-handler"
ARTIFACT_VERSION=""

cargo build -r
echo "Cargo build completed"

echo "Computing artifact version..."
VERSION=$(grep "^version" Cargo.toml | sed -e "s/version = \"\(.*\)\"/\1/g")

#Determine the version based on the branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [[ "${CURRENT_BRANCH}" == "main" ]]; then
  ARTIFACT_VERSION=$VERSION

  git tag $VERSION
  git push origin main --tags

  #Update the version of the GIT
  NEW_VERSION=$(echo $VERSION | awk -F. -v OFS=. 'NF==1{print ++$NF}; NF>1{$NF=sprintf("%0*d", length($NF), ($NF+1)); print}')
  sed -i -e "s/^version =.*/version = \"$NEW_VERSION\"/" Cargo.toml
  git commit - am "Update VERSION"
  git push origin main
else
  CURRENT_COMMIT=$(git rev-parse --short HEAD)
  ARTIFACT_VERSION="${VERSION}-${CURRENT_BRANCH}-${CURRENT_COMMIT}"
fi