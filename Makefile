all:
	echo Nothing to do...

ctags:
	ctags --recurse --options=ctags.rust --languages=Rust

run:
	cargo run

push:
	git push origin master
	git push github master