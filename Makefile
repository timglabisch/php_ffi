example_async:
	cd examples/async && cargo build --release
	php ./vendor/bin/phpunit --bootstrap vendor/autoload.php ./examples/async

example_call_rust_func:
	cd examples/call_rust_func && cargo build
	php ./vendor/bin/phpunit --bootstrap vendor/autoload.php ./examples/call_rust_func

run: example_call_rust_func
