%define _name icdofflinedb
%define _version 2.0.2
%define _release 23
%define debug_package %{nil}

Name: %{_name}
Version: %{_version}
Release: %{_release}
Summary: ICD Offline Database
License: MIT
Group: Applications/Utilities
URL: https://github.com/XRayAdams/icdofflinedb-rs
BugURL: https://github.com/XRayAdams/icdofflinedb-rs/issues
Vendor: Konstantin Adamov

Source0: %{_name}-%{_version}.tar.gz
Source1: app.rayadams.icdofflinedb.desktop
Source2: app.rayadams.icdofflinedb.png
Source3: app.rayadams.icdofflinedb.metainfo.xml

Requires: gtk3, libstdc++, sqlite-libs, sqlite-devel

%description
ICD Offline Database is a free and open-source application that allows users to search through the entire ICD-10 and ICD-9 databases of codes.

%prep
%setup -q -n release

%build
# This section is intentionally left blank as we are packaging a pre-compiled Flutter application.

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/applications
mkdir -p %{buildroot}/usr/share/icons/hicolor/256x256/apps
mkdir -p %{buildroot}/opt/%{_name}
mkdir -p %{buildroot}%{_datadir}/metainfo

# Copy the application files
cp -r ./* %{buildroot}/opt/%{_name}/

# Create a symlink in /usr/bin
ln -s /opt/%{_name}/%{_name} %{buildroot}/usr/bin/%{_name}

# Copy the desktop file
install -m 644 %{SOURCE1} %{buildroot}/usr/share/applications/%{_name}.desktop

# Copy the application icon
install -m 644 %{SOURCE2} %{buildroot}/usr/share/icons/hicolor/256x256/apps/%{_name}.png

# Copy meta info
install -m 644 %{SOURCE3} %{buildroot}%{_datadir}/metainfo/%{name}.metainfo.xml
%files
/usr/bin/%{_name}
/opt/%{_name}
/usr/share/applications/%{_name}.desktop
/usr/share/icons/hicolor/256x256/apps/%{_name}.png
%{_datadir}/metainfo/%{name}.metainfo.xml

%changelog
*loghere
- Initial RPM release
