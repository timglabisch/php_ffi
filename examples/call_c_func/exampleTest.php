<?php

function ffi()
{
    return \FFI::cdef(
        file_get_contents(__DIR__ . '/example.h'),
        __DIR__ . '/lib.so'
    );
}

$res = ffi()->bug79096();

var_dump($res);
