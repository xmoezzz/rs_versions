# rs_versions
Python (PyO3) wrapper of Rust's [versions](https://docs.rs/versions/3.0.3/versions/index.html) crate.

# API
## `rs_versions.parse_version(str) -> RsVersion`
Parses a given version string if possible, returns None if it could not be parsed.

## `RsVersion`
All comparison and equality operations are implemented.

# Build
```bash
python3 -m venv .venv
. .venv/bin/activate
pip install maturin
maturin build --release
# or
maturin develop
```