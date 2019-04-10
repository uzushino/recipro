export PATH += $(shell pwd)/depot_tools/

depot_tools:
	git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git

build: depot_tools
	python build.py

clean:
	cargo clean
	-rm -rf depot_tools
	-rm -rf _gclient*
	-rm -rf .gclient*
	-rm -rf .cipd
	-rm -rf out
	-rm -rf v8
	-rm v8.pc