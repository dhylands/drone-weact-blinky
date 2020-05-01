export DRONE_RUSTFLAGS := '--cfg cortex_m_core="cortex_m4f_r0p1" --cfg stm32_mcu="stm32f411"'
target := 'thumbv7em-none-eabihf'
features := ''
name := `basename $(pwd)`
release_bin := "target/" + target + "/release/" + name

# Install dependencies
deps:
	rustup target add {{target}}
	rustup component add rust-src
	rustup component add rustfmt
	rustup component add clippy
	rustup component add llvm-tools-preview
	type cargo-objdump >/dev/null || cargo +stable install cargo-binutils
	type itmsink >/dev/null || cargo install itmsink
	type drone >/dev/null || cargo install drone

# Reformat the source code
fmt:
	cargo fmt

# Check for mistakes
lint:
	drone env {{target}} -- cargo clippy --features "{{features}}"

# Build the firmware
build:
	drone env {{target}} -- cargo build --features "{{features}}" --release

# Generate the docs
doc:
	drone env {{target}} -- cargo doc --features "{{features}}"

# Open the docs in a browser
doc-open: doc
	drone env {{target}} -- cargo doc --features "{{features}}" --open

# Run the tests
test:
	drone env -- cargo test --features "std {{features}}"

# Display information from the compiled firmware
dump: build
	drone env {{target}} -- cargo objdump --target {{target}} \
		--features "{{features}}" --release --bin {{name}} -- \
		--disassemble --demangle --full-contents -all-headers --syms | pager

# Display section sizes inside the compiled firmware
size +args='': build
	drone env {{target}} -- cargo size --target {{target}} \
		--features "{{features}}" --release --bin {{name}} -- {{args}}

# Display result of macro expansion
expand:
	drone env {{target}} -- cargo rustc --target {{target}} \
		--features "{{features}}" --lib -- -Z unstable-options --pretty=expanded

# Force a pulse on the reset line of the attached device
reset:
	drone probe reset

# Flash the firmware to the attached device
flash: build
	drone probe flash {{release_bin}}

# Run a GDB session for the attached device
gdb:
	drone probe gdb {{release_bin}} --reset

# Display ITM output from the attached device
itm:
	drone probe itm --reset 0,1 -- 0,1

# Record `heaptrace` file from the attached device (should be compiled with `heaptrace` feature)
heaptrace:
	truncate -s0 heaptrace
	drone probe itm --reset 0,1,31 -- 0,1 31:heaptrace
