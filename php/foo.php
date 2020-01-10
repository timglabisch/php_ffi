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
echo "read two\n";
echo $ffi->queue_read('/tmp/foo2.txt');
echo "start polling\n";

$poll1 = $ffi->poll();
#echo 'read ' . $poll1->data . ', token' . $poll1->token . "\n";

while(1);

#$poll2 = $ffi->poll();