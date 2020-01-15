example_async:
	cd examples/async && cargo build --release
	php ./vendor/bin/phpunit --bootstrap vendor/autoload.php ./examples/async

example_call_rust_func:
	cd examples/call_rust_func && cargo build
	php ./vendor/bin/phpunit --bootstrap vendor/autoload.php ./examples/call_rust_func

example_call_c_function:
	cd examples/call_c_func && gcc main.c -shared -o lib.so
	php ./vendor/bin/phpunit --bootstrap vendor/autoload.php ./examples/call_c_func


run: example_call_c_function
