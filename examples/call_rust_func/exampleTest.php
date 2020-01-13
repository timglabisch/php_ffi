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

    public function testReturnU64()
    {
        static::assertSame(42, $this->ffi()->return_u64());
    }

    public function testReturnPointer()
    {
        $res = $this->ffi()->return_pointer();

        var_dump($res);

        // static::assertSame(1, $res->_1);
    }
}