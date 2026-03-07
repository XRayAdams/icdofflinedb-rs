#!/bin/bash
set -e

# --- Configuration ---
MACHINE_ARCH=$(uname -m)
DEBIAN_ARCH="amd64"
APP_ID="app.rayadams.icdofflinedb"
CARGO_FILE="Cargo.toml"

if [ "$MACHINE_ARCH" == "aarch64" ]; then
    MACHINE_ARCH="arm64"
    DEBIAN_ARCH="arm64"
elif [ "$MACHINE_ARCH" == "x86_64" ]; then
    MACHINE_ARCH="x64"
    DEBIAN_ARCH="amd64"
fi



# Clean and build app in release mode
cargo clean
cargo build --release

APP_NAME=$(grep -E '^\s*name = ' "$CARGO_FILE" | head -n1 | cut -d ' ' -f 3 | tr -d '"')
APP_VERSION_LONG=$(grep -E '^\s*version = ' "$CARGO_FILE" | head -n1 | cut -d ' ' -f 3 | tr -d '"')
APP_VERSION=$(echo "$APP_VERSION_LONG" | cut -d'+' -f1)
APP_BUILD=$(echo "$APP_VERSION_LONG" | cut -d'+' -f2)


# Set app versions to all files for packaging
packaging/set_app_versions.sh
rm -rf dist
mkdir -p dist

PACKAGE_DIR="$APP_NAME-$APP_VERSION+$APP_BUILD-$DEBIAN_ARCH"
printf "Creating DEB package in %s.deb\n" "$PACKAGE_DIR"
# Create the package directory
rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR/usr/bin"
mkdir -p "$PACKAGE_DIR/usr/share/applications"
mkdir -p "$PACKAGE_DIR/usr/share/icons/hicolor/512x512/apps"
mkdir -p "$PACKAGE_DIR/usr/share/metainfo"
mkdir -p "$PACKAGE_DIR/usr/share/$APP_NAME"

# Copy the built binary
cp "target/release/$APP_NAME" "$PACKAGE_DIR/usr/bin/$APP_NAME"
cp packaging/gui/$APP_ID.desktop "$PACKAGE_DIR/usr/share/applications/"
cp packaging/gui/$APP_ID.png "$PACKAGE_DIR/usr/share/icons/hicolor/512x512/apps/"
cp packaging/$APP_ID.metainfo.xml "$PACKAGE_DIR/usr/share/metainfo/"
cp assets/icddb.db "$PACKAGE_DIR/usr/share/$APP_NAME/"
# Copy control file
mkdir -p "$PACKAGE_DIR/DEBIAN"
cp packaging/control "$PACKAGE_DIR/DEBIAN/control"

# Build the .deb package
dpkg-deb --root-owner-group --build "$PACKAGE_DIR"

# Clean up
rm -rf "$PACKAGE_DIR"

cp "$PACKAGE_DIR.deb" dist/
rm "$PACKAGE_DIR.deb"

echo "DEB package created in dist/"
echo "___________________________________________________________"
# Package RPM
echo "Preparing RPM package"

# Create RPM build directories
RPM_BUILD_ROOT="$(pwd)/rpmbuild"
rm -rf "$RPM_BUILD_ROOT"

mkdir -p "$RPM_BUILD_ROOT/BUILD"
mkdir -p "$RPM_BUILD_ROOT/RPMS"
mkdir -p "$RPM_BUILD_ROOT/SOURCES"
mkdir -p "$RPM_BUILD_ROOT/SPECS"
mkdir -p "$RPM_BUILD_ROOT/SRPMS"

CHANGE_DATE=$(date +"%a %b %d %Y")
CHANGE_DATE="$CHANGE_DATE Konstantin Adamov (xrayadamo@gmail.com) - $APP_VERSION-$APP_BUILD"
sed "s/^*loghere$/* $CHANGE_DATE/" "packaging/$APP_NAME.spec" > "$RPM_BUILD_ROOT/SPECS/$APP_NAME.spec"

# Copy desktop and icon files
cp "packaging/gui/$APP_ID.desktop" "$RPM_BUILD_ROOT/SOURCES/"
cp packaging/gui/"$APP_ID".png "$RPM_BUILD_ROOT/SOURCES/"
cp packaging/"$APP_ID".metainfo.xml "$RPM_BUILD_ROOT/SOURCES/"

# Package the application files into a tarball
TMP_TAR_DIR="$RPM_BUILD_ROOT/SOURCES/$APP_NAME-$APP_VERSION"
mkdir -p "$TMP_TAR_DIR/assets"
cp "target/release/$APP_NAME" "$TMP_TAR_DIR/"
cp "assets/icddb.db" "$TMP_TAR_DIR/assets/"

pushd "$RPM_BUILD_ROOT/SOURCES" || exit
tar -czvf "$APP_NAME-$APP_VERSION.tar.gz" "$APP_NAME-$APP_VERSION"
rm -rf "$APP_NAME-$APP_VERSION"
popd || exit

# Build the RPM
rpmbuild -bb \
    --define "_topdir $RPM_BUILD_ROOT" \
    --define "_name $APP_NAME" \
    --define "_version $APP_VERSION" \
    --define "_release $APP_BUILD" \
    "$RPM_BUILD_ROOT/SPECS/$APP_NAME.spec"

# Move the RPM to the dist directory
find "$RPM_BUILD_ROOT/RPMS" -name "*.rpm" -exec mv {} dist/ \;

# Clean up
rm -rf "$RPM_BUILD_ROOT"
echo "RPM package created in dist/"

echo "___________________________________________________________"

#Packaging AUR
cp packaging/PKGBUILD .
PACKAGER="Konstantin Adamov <xrayadamo@gmail.com>" PKGEXT='.pkg.tar.zst' COMPRESSZST=(zstd -c -T0 --auto-threads=logical -) env makepkg --nodeps -f
mv *.pkg.tar.zst dist/
rm -rf pkg/
rm PKGBUILD

echo "AUR package created in dist/"
echo "___________________________________________________________"

# Package TAR
echo "Preparing TAR archive"

ARCHIVE_NAME="${APP_NAME}-${APP_VERSION}+${APP_BUILD}-${MACHINE_ARCH}.tar.gz"
FULL_ARCHIVE_PATH="dist/${ARCHIVE_NAME}"

# Only include the executable from target/release and the assets folder from the root directory
tar -czvf "$FULL_ARCHIVE_PATH" -C "target/release" "$APP_NAME" -C "$(pwd)" "assets" > /dev/null
echo "TAR archive created in dist/"
echo "___________________________________________________________"
