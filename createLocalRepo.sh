#!/usr/bin/env bash
set -euo pipefail #e=exitonfail u=unboundvarsdisallow o=pipefail x=debug

# env vars
export SRC_DIR=${SRC_DIR:-$(pwd)}
export REPO_DIR=${REPO_DIR:-$SRC_DIR/repository/rpm}
export ARTIFACTS_DIR=${ARTIFACTS_DIR:-/tmp/artifacts}

mkdir -p $REPO_DIR/el/{8,9}/{x86_64,src,noarch} || true

while getopts ':cbs' OPTION; do
    case "$OPTION" in
    c)
        rm -rf $REPO_DIR/el
        mkdir -p $REPO_DIR/el/{8,9}/{x86_64,src,noarch} || true
        ;;
    b)
        cp $ARTIFACTS_DIR/*.rpm $REPO_DIR/el/8/x86_64/
        createrepo --update $REPO_DIR/el/8/x86_64/
        ;;
    s)
        if [[ -z "$FPR" ]]; then
            echo "No Fingerprint set for GPG signing! FPR is set??"
            exit 1
        fi
        if [[ ! -f "$HOME/.rpmmacros" ]]; then
            echo "%_gpg_name $FPR" >$HOME/.rpmmacros
            echo "%_gpg_path $GNUPGHOME" >>$HOME/.rpmmacros
        fi
        rpmsign --addsign $SRC_DIR/repository/rpm/el/8/x86_64/*.rpm
        createrepo --update $SRC_DIR/repository/rpm/el/8/x86_64/
        gpg --armor --detach-sign --default-key $FPR $SRC_DIR/repository/rpm/el/8/x86_64/repodata/repomd.xml
        gpg --armor --export $FPR >$SRC_DIR/repository/rpm/gpg-repository-key.asc
        ;;

    *)
        echo "Unbekannter Parameter"
        echo " -c = clean all RPMs from the repository directory"
        ;;
    esac
done
