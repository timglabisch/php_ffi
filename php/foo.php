<?php

$ffi = \FFI::cdef(
    file_get_contents(__DIR__ . '/../rlib/rlib.h'),
    __DIR__ . '/../osx_librlib.dylib' // __DIR__ . '/../rlib/target/release/librlib.dylib'
);

file_put_contents('/tmp/foo1.txt', 'one');
file_put_contents('/tmp/foo2.txt', 'two');

$ffi->start();

while (true) {
    $ffi->queue_read('/tmp/foo1.txt');
    $ffi->queue_read('/tmp/foo2.txt');

    $buffer = $ffi->poll();
    var_dump(FFI::string($buffer[0]->data, $buffer[0]->len));
    $buffer = $ffi->poll();
    var_dump(FFI::string($buffer[0]->data, $buffer[0]->len));
    echo "\n";
}

