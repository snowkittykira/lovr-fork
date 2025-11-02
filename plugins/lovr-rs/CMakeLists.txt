cmake_minimum_required(VERSION 3.5...4.0)
project(lovr-rs)

add_subdirectory(corrosion)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)
set(LOVR_PLUGIN_TARGETS lovr_rs)
