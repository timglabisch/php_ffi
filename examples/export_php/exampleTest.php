<?php

use PHPUnit\Framework\TestCase;

class exampleTest extends TestCase
{

    public static function assertObSame(string $expected, callable $cb)
    {
        ob_start();
        $cb();
        $res = ob_get_contents();
        ob_end_clean();

        static::assertSame($expected, $res);
    }

    public function testReturnU64()
    {

        $zend = FFI::cdef("
    typedef int (*zend_write_func_t)(const char *str, size_t str_length);
    extern zend_write_func_t zend_write;
        ");

        static::assertObSame('aaa', function() use ($zend) { $a = $zend->zend_write; $a("aaa", 3); });
        static::assertObSame('aaa', function() use ($zend) { ($zend->zend_write)("aaa", 3); });
        static::assertObSame('aaa', function() use ($zend) { (clone $zend->zend_write)("aaa", 3); });
        static::assertObSame('aaa', function() use ($zend) { $a = [$zend, 'zend_write']; $a("aaa", 3); });

        // does not work
        // static::assertObSame('aaa', function() use ($zend) { $zend->zend_write("aaa", 3); });
    }

}