set -e

cargo build
cp target/debug/liblovr_rs.so lovr_rs.so
../../build/bin/lovr .
