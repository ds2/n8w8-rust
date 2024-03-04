%global orig_src_dir %{getenv:SRC_DIR}
%global rust_target_arch %{getenv:CARGO_BUILD_TARGET}
%global tgt_src_dir %{getenv:CARGO_TARGET_DIR}
%global tgt_profile %{getenv:RUST_PROFILE}

Name:           nachtwacht-agent
Version:        %{getenv:RPM_VERSION}
Release:        %{?getenv:RPM_RELEASE}%{?dist}
Summary:        A host service to retrieve local health data and send it to N8w8

License:        GPLv3
URL:            https://gitlab.com/ds_2/n8w8-rust-fe
# Source: https://gitlab.com/ds_2/n8w8-rust-fe/-/archive/develop/n8w8-rust-fe-develop.tar.gz
# Source0:

Requires: systemd

%description
A host service to retrieve local health data and send it to N8w8 api server.


%install
rm -rf $RPM_BUILD_ROOT
install -m 0755 -D %{tgt_src_dir}/%{rust_target_arch}/%{tgt_profile}/n8w8d $RPM_BUILD_ROOT/%{_bindir}/n8w8d


%files
# %license %{orig_src_dir}/LICENSE
%{_bindir}/n8w8d
# %doc %{orig_src_dir}/agent/README.md



%changelog
* Thu Oct  6 2022 root
- initial release
