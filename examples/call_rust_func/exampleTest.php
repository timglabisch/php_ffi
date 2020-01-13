<?php

use PHPUnit\Framework\TestCase;

class exampleTest extends TestCase
{
    public function ffi()
    {
        return \FFI::cdef(
            file_get_contents(__DIR__ . '/example.h'),
            __DIR__ . '/target/release/libexample.dylib'
        );
    }

    public function testBasic()
    {
        static::assertSame(42, $this->ffi()->example_return_u64());
    }
}