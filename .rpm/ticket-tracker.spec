%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: ticket-tracker
Summary: A desktop application for easy JIRA ticket tracking with visual feedback
Version: 0.1.0
Release: 1%{?dist}
License: MIT
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://github.com/munnotubbel/ticket-tracker

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root
BuildRequires: cargo gcc make

%description
A desktop application that helps track JIRA tickets with visual feedback.
Features include:
* Visual feedback with reactive faces
* Excel export
* Sound feedback
* Autostart capability

%prep
%setup -q

%build
cargo build --release

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/man/man1
install -m 755 target/release/%{name} %{buildroot}/usr/bin/%{name}
install -m 644 ticket-tracker.1 %{buildroot}/usr/share/man/man1/%{name}.1

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
/usr/bin/%{name}
/usr/share/man/man1/%{name}.1*

%changelog
* Fri Feb 23 2024 Marcus Weissohn Eede <marcus.weissohn@gmail.com> - 0.1.0
- First RPM release 