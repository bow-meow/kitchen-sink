set shell := ["nu", "-c"]

build appname:
    just "{{appname}}/build"
run appname:
    just "{{appname}}/build"
    cargo run
test appname:
    just "{{appname}}/test"