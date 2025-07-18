%define debug_package %{nil}

Name: pezkuwi
Summary: Implementation of a https://pezkuwi.network node in Rust based on the Substrate framework.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPLv3
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

Requires: systemd, shadow-utils
Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}


%prep
%setup -q


%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%post
config_file="/etc/default/pezkuwi"
getent group pezkuwi >/dev/null || groupadd -r pezkuwi
getent passwd pezkuwi >/dev/null || \
    useradd -r -g pezkuwi -d /home/pezkuwi -m -s /sbin/nologin \
    -c "User account for running pezkuwi as a service" pezkuwi
if [ ! -e "$config_file" ]; then
    echo 'PEZKUWI_CLI_ARGS=""' > /etc/default/pezkuwi
fi
exit 0

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/usr/lib/systemd/system/pezkuwi.service
