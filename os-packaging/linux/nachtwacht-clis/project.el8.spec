%global orig_src_dir %{getenv:SRC_DIR}
%global rust_target_arch %{getenv:CARGO_BUILD_TARGET}
%global tgt_src_dir %{getenv:CARGO_TARGET_DIR}
%global tgt_profile %{getenv:RUST_PROFILE}

Name:           nachtwacht-clis
Version:        %{getenv:RPM_VERSION}
Release:        %{?getenv:RPM_RELEASE}%{?dist}
Summary:        A set of command line interfaces for dealing with local health data.

License:        GPLv3
URL:            https://gitlab.com/ds_2/n8w8-rust-fe
# Source: https://gitlab.com/ds_2/n8w8-rust-fe/-/archive/develop/n8w8-rust-fe-develop.tar.gz
# Source0:

%description
A set of command line interfaces for dealing with local health data.


%install
rm -rf $RPM_BUILD_ROOT
install -m 0755 -D %{tgt_src_dir}/%{rust_target_arch}/%{tgt_profile}/n8w8-simple-val $RPM_BUILD_ROOT/%{_bindir}/n8w8-simple-val
install -m 0755 -D %{tgt_src_dir}/%{rust_target_arch}/%{tgt_profile}/n8w8-http-check $RPM_BUILD_ROOT/%{_bindir}/n8w8-http-check


%files
%{_bindir}/n8w8-simple-val
%{_bindir}/n8w8-http-check



%changelog
* Thu Oct  6 2022 root
- initial release
