#!/bin/bash


# SECURITY: SAFE to execute
#           It only creates/overwrites pre-commit hook file 
#           path: <PWD>/.git/hooks/pre-commit


echo "
#!/bin/bash

# exit on error
set -e

# verify all the PIPELINE CHECKS LOCALLY before commiting

# run cargo test suite 
cargo test --all-features --workspace

# perform format check
cargo fmt --check --all

# clippy check
cargo clippy --all-targets --all-features --workspace -- -D warnings

" > .git/hooks/pre-commit

# give newly created pre-commit hook executable permission
chmod +x .git/hooks/pre-commit