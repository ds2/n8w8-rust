#!/usr/bin/env bash

TAG=${1:-notag}

cargo install cargo-edit
export CURRENTDATE="$(date)"
export SEMVER=$(echo $TAG | grep -Eo "(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$")

if [[ -z "${SEMVER}" ]]; then
  echo "No Semver: $TAG"
  exit 1
fi

cargo set-version --workspace "${SEMVER}"
git commit -m "🔖 [release] create release tag for v${SEMVER}"
git tag -m "🔖 [release] release tag on ${CURRENTDATE}" v${SEMVER}
echo "You are free to decide to push this tag or drop it. !!!Please DOUBLE-CHECK THE COMMIT and tag!!!"
echo "  git push origin v${SEMVER}"
