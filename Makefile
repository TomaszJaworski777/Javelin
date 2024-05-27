EXE = javelin

ifeq ($(OS),Windows_NT)
	DEV_NAME := target/builds/$(EXE)-dev.exe
	RELEASE_NAME := target/builds/$(EXE).exe
	AVX2_NAME := target/builds/$(EXE)-avx2.exe
else
	DEV_NAME := target/builds/$(EXE)-dev
	RELEASE_NAME := target/builds/$(EXE)
	AVX2_NAME := target/builds/$(EXE)-avx2
endif

rule:
	cargo rustc --release --package javelin --bin javelin -- -C target-cpu=native --emit link=$(DEV_NAME)

release:
	cargo rustc --release --bin javelin -- --emit link=$(RELEASE_NAME)
	cargo rustc --release --bin javelin -- -C target-cpu=x86-64-v2 -C target-feature=+avx2 --emit link=$(AVX2_NAME)