<?php

use PHPUnit\Framework\TestCase;

class exampleTest extends TestCase
{
    public function testBasic()
    {

        $ffi = \FFI::cdef(
            file_get_contents(__DIR__ . '/example.h'),
            __DIR__ . '/target/release/libexample.dylib'
        );

        file_put_contents($file1 = sys_get_temp_dir().'/foo1.txt', $content1 = 'one');
        file_put_contents($file2 = sys_get_temp_dir().'/foo1.txt', $content2 = 'two');

        $ffi->start();

        $ffi->queue_read('/tmp/foo1.txt');
        $ffi->queue_read('/tmp/foo2.txt');

        $contents = [];
        $buffer = $ffi->poll();
        $contents[] = FFI::string($buffer[0]->data, $buffer[0]->len);
        $buffer = $ffi->poll();
        $contents[] = FFI::string($buffer[0]->data, $buffer[0]->len);

        static::assertContains($content1, $contents);
        static::assertContains($content2, $contents);
    }
}