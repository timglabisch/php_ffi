<?php

function ffi()
{
    return \FFI::cdef(
        file_get_contents(__DIR__ . '/example.h'),
        __DIR__ . '/lib.so'
    );
}

$res = ffi()->return_pointer();

var_dump($res);
