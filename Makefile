example_async:
	cd examples/async && cargo build --release
	php ./vendor/bin/phpunit --bootstrap vendor/autoload.php ./examples/async

example_call_rust_func:
	cd examples/call_rust_func && cargo build --release
	php ./vendor/bin/phpunit --bootstrap vendor/autoload.php ./examples/call_rust_func

export_php:
	php ./vendor/bin/phpunit --bootstrap vendor/autoload.php ./examples/export_php

run: export_php
