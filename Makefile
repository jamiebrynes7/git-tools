MAJOR_VERSION := 0
MINOR_VERSION := 1
PATCH_VERSION := 0

VERSION := v$(MAJOR_VERSION).$(MINOR_VERSION).$(PATCH_VERSION)

release: build package

build: install-dependencies
	TARGET="x86_64-unknown-linux-gnu" sh ci/build.sh
	TARGET="x86_64-pc-windows-gnu" sh ci/build.sh

package: build
	zip -j git-tools-$(VERSION)-x86_64-linux-gnu.zip target/x86_64-unknown-linux-gnu/release/branch_selector target/x86_64-unknown-linux-gnu/release/clean_branches
	zip -j git-tools-$(VERSION)-x86_64_windows.zip target/x86_64-pc-windows-gnu/release/branch_selector.exe target/x86_64-pc-windows-gnu/release/clean_branches.exe

install-dependencies:
	sh ci/install.sh

clean-all: clean-zips clean-build-artifacts

clean-build-artifacts:
	rm -rf target/
	rm -rf cross/

clean-zips:
	rm -f *.zip
