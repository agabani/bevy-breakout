set shell := ["bash", "-uc"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

GIT_COMMIT := `git rev-parse --verify --short HEAD`

# help
help:
  @just --list

# lint
lint:
  @cargo clippy

# run
run:
  @cargo run

# run dynamic
run-dynamic:
  @cargo run --features bevy/dynamic_linking
