
release: build package

build:
	cargo build --release
	cargo build --target "x86_64-pc-windows-gnu" --release

package:
	zip -j git_tools_linux_x86_64_binaries.zip target/release/branch_selector target/release/clean_branches
	zip -j git_tools_win_x86_64_binaries.zip target/x86_64-pc-windows-gnu/release/branch_selector.exe target/x86_64-pc-windows-gnu/release/clean_branches.exe

clean:
	rm -rf target/
	rm git_tools_linux_x86_64_binaries.zip git_tools_win_x86_64_binaries.zip
