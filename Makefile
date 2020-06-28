docs:
	rm -rf html
	cargo doc --no-deps
	mkdir html
	cp -rf target/doc/* html/
	mv -f html/rust_action_heroes/* html/
	rmdir html/rust_action_heroes

clean:
	cargo clean
