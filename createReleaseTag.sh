#!/usr/bin/env bash

TAG=${1:-notag}

cargo install cargo-edit
export CURRENTDATE="$(date)"
export SEMVER=$(echo $TAG | grep -Po "(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$")

if [[ -z "${SEMVER}" ]]; then
  echo "No Semver: $TAG -> \"${SEMVER}\""
  exit 1
else
  echo "Will use version $SEMVER"
  while [ true ]; do
    read -t 3 -n 1
    if [ $? = 0 ]; then
      break
    else
      echo "waiting for the keypress"
    fi
  done
fi

echo "Setting cargo workspace version.."
cargo set-version --workspace "${SEMVER}" || exit 1
echo "Testing build.."
cargo build --workspace
echo "Committing changed files.."
git add -u
git commit -m "🔖 [release] create release tag for v${SEMVER}"
git tag -m "🔖 [release] release tag on ${CURRENTDATE}" v${SEMVER}
echo "You are free to decide to push this tag or drop it. !!!Please DOUBLE-CHECK THE COMMIT and tag!!!"
echo "  git push origin v${SEMVER}"
