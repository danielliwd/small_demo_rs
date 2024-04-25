#!/bin/sh
set -ex

export app_name=$(awk -F' = ' '$1 ~ /^name/ { gsub(/"/, "", $2); printf("%s",$2) }' Cargo.toml)
remove_local_deps_after_build() {
	echo $app_name >target/local_deps.txt
	awk '$0 ~ /path = "/ {print $1}' Cargo.toml >>target/local_deps.txt
	awk '{print "find target -name \047"$1".d\047 -or -name \047"$1"-*\047"}' target/local_deps.txt | sh >target/local_deps_path.txt
	cat target/local_deps_path.txt | xargs rm -rf
}

rename_artifact_to_app() {
	cd ./target/release/
	mv $app_name app
}

test() {
	echo "test"
}

"$@"
