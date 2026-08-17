%define _name icdofflinedb
%define _version 2.1.4
%define _release 26
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
Source4: README.txt

Requires: gtk4

%description
ICD Offline Database is a free and open-source application that allows
users to search through the entire ICD-10 and ICD-9 databases of codes.

%prep
%setup -q

%build
# This section is intentionally left blank as we are packaging a pre-compiled application.

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/applications
mkdir -p %{buildroot}/usr/share/icons/hicolor/512x512/apps
mkdir -p %{buildroot}/usr/share/%{_name}
mkdir -p %{buildroot}%{_datadir}/metainfo

# Copy the application binary
install -m 755 %{_name} %{buildroot}/usr/bin/%{_name}

# Copy the database
install -m 644 assets/icddb.db %{buildroot}/usr/share/%{_name}/icddb.db

# Copy the desktop file
install -m 644 %{SOURCE1} %{buildroot}/usr/share/applications/app.rayadams.icdofflinedb.desktop

# Copy the application icon
install -m 644 %{SOURCE2} %{buildroot}/usr/share/icons/hicolor/512x512/apps/app.rayadams.icdofflinedb.png

# Copy meta info
install -m 644 %{SOURCE3} %{buildroot}%{_datadir}/metainfo/app.rayadams.icdofflinedb.metainfo.xml

# Copy documentation
install -Dm 644 %{SOURCE4} %{buildroot}%{_docdir}/%{_name}/README.txt

%files
/usr/bin/%{_name}
%dir /usr/share/%{_name}
/usr/share/%{_name}/icddb.db
/usr/share/applications/app.rayadams.icdofflinedb.desktop
/usr/share/icons/hicolor/512x512/apps/app.rayadams.icdofflinedb.png
%{_datadir}/metainfo/app.rayadams.icdofflinedb.metainfo.xml
%doc %{_docdir}/%{_name}/README.txt

%changelog
*loghere
- Updating location of DB file
