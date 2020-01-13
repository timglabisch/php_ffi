example_async:
	cd examples/async && cargo build --release
	php ./vendor/bin/phpunit --bootstrap vendor/autoload.php ./examples/async

run: example_async
