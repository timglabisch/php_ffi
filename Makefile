run:
	cd rlib && cargo build --release
	cp rlib/target/release/librlib.dylib osx_librlib.dylib
	cd php && php foo.php
