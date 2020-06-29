docs: src
	rm -rf docs
	cargo doc --no-deps
	mkdir -p docs/
	cp -rf target/doc/* docs/
	mv -f docs/rust_action_heroes/* docs/
	rmdir docs/rust_action_heroes
	cd docs ;\
		find . -type f | xargs -n1 sed -i -e 's/href="\.\.\//href="/g' ;\
		find . -type f | xargs -n1 sed -i -e 's/url("\.\.\//url("/g'   ;\
		find . -type f | xargs -n1 sed -i -e "s/src='\.\.\//src='/g"   ;\
		find . -type f | xargs -n1 sed -i -e 's/src="\.\.\//src="/g'   ;

clean:
	cargo clean
