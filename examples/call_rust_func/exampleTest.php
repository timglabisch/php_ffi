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
        $ffi = $this->ffi();
        static::assertSame(42, $ffi->return_u64());
    }

    public function testReturnPointer()
    {
        $ffi = $this->ffi();
        $res = $ffi->return_pointer();

        static::assertSame(1, $res[0]->_1);
        static::assertSame(2, $res[0]->_2);
        static::assertSame(3, $res[0]->_3);
        static::assertSame(4, $res[0]->_4);
        static::assertSame(5, $res[0]->_5);
        static::assertSame(6, $res[0]->_6);
        static::assertSame(7, $res[0]->_7);
        static::assertSame(8, $res[0]->_8);

        $ffi->free_pointer($res);
    }

    public function testReturnString() {
        $ffi = $this->ffi();
        $res = $ffi->return_string();

        static::assertSame(
            \FFI::string($res),
            "some string ...😀"
        );

        $ffi->free_string($res);
    }

    public function testReturnStructWithPointer() {
        $ffi = $this->ffi();
        $res = $ffi->return_struct_with_pointer();

        static::assertSame(\FFI::string($res->_1), "foo1");
        static::assertSame(\FFI::string($res->_2->_1), "ifoo1");
        static::assertSame(\FFI::string($res->_2->_2), "ifoo2");
        static::assertSame(\FFI::string($res->_3), "foo3");

    }
}