<?php

$ffi = \FFI::cdef(
    file_get_contents(__DIR__ . '/../rlib/rlib.h'),
    __DIR__ . '/../rlib/target/release/librlib.dylib'
);

/*
$buffer = FFI::new("char[1024]");
foreach (range(0, 1023) as $i) {
    $buffer[$i] = 'a';
}
*/

file_put_contents('/tmp/foo1.txt', 'one');
file_put_contents('/tmp/foo2.txt', 'two');


$ffi->start();
echo "read one\n";
echo $ffi->queue_read('/tmp/foo1.txt');
echo $ffi->queue_read('/tmp/foo2.txt');

$buffer = $ffi->poll();
var_dump(FFI::string($buffer[0]->data, $buffer[0]->len));
$buffer = $ffi->poll();
// var_dump(FFI::string($buffer[0]->data, $buffer[0]->len));

