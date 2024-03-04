%global orig_src_dir %{getenv:SRC_DIR}
%global rust_target_arch %{getenv:CARGO_BUILD_TARGET}
%global tgt_src_dir %{getenv:CARGO_TARGET_DIR}
%global tgt_profile %{getenv:RUST_PROFILE}

Name:           nachtwacht-metrics-server
Version:        %{getenv:RPM_VERSION}
Release:        %{?getenv:RPM_RELEASE}%{?dist}
Group: Applications/System
Summary:        A metrics server to offer some openmetrics data to calling entities (like Prometheus)

License:        GPLv3
URL:            https://gitlab.com/ds_2/n8w8-rust-fe
# Source: https://gitlab.com/ds_2/n8w8-rust-fe/-/archive/develop/n8w8-rust-fe-develop.tar.gz
# Source0:

Requires: systemd
Requires: filesystem
Requires(preun): systemd-units
Requires(postun): systemd-units
Requires(post): systemd-units

%description
A metrics server to offer some openmetrics data to calling entities (like Prometheus)


%install
rm -rf $RPM_BUILD_ROOT
%{__install} -m 0755 -D %{tgt_src_dir}/%{rust_target_arch}/%{tgt_profile}/n8w8-openmetrics-server $RPM_BUILD_ROOT/%{_bindir}/nachtwacht-metrics-server
%{__install} -m 644 -D %{orig_src_dir}/os-packaging/linux/files/metrics-server.service $RPM_BUILD_ROOT/%{_unitdir}/nachtwacht-metrics-server.service

%files
%{_bindir}/nachtwacht-metrics-server
%{_unitdir}/nachtwacht-metrics-server.service



%changelog
* Thu Oct  6 2022 root
- initial release
