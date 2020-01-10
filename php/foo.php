<?php

$ffi = \FFI::cdef(
    file_get_contents(__DIR__ . '/../rlib/rlib.h'),
    __DIR__ . '/../clib/lib.dylib'
);

$poll1 = $ffi->poll();
var_dump($poll1);