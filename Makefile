run:
	#cd rlib && cargo build --release
	cd clib && gcc main.c -shared -o lib.dylib
	cd php && php foo.php
