docs: src
	rm -rf docs
	cargo doc --no-deps
	mkdir docs
	cp -rf target/doc/* docs/
	mv -f docs/rust_action_heroes/* docs/
	rmdir docs/rust_action_heroes

clean:
	cargo clean
