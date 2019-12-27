const expect = require('chai').expect

const SUCCESS = 0;
const FAILURE = 1;

describe('NAPI addon', () => {
    const test = require('./build/Release/test')
    it('should be able to call c library', () => {
        expect(test.hello()).to.equal(SUCCESS)
    })

    it('should be able to call the get method', () => {
        expect(test.get()).to.equal(SUCCESS)
    })
})

describe('FFI', () => {
    const ffi = require('ffi-napi')
    const test = ffi.Library('./target/debug/libtest.so', {
        'hello': ['int', []],
        'get': ['int', []],
        'post': ['int', []],
        'get_with_requests': ['int', []],
        'openssl_version_info': ['int', []]
    })
    it('should be able to call the c library', () => {
        expect(test.hello()).to.equal(SUCCESS)
    })

    it('should be able to call the get method', () => {
        expect(test.get()).to.equal(SUCCESS)
    })
})

describe('Other failing tests', () => {
    const test = require('./build/Release/test')
    it('should be able to call the post method', () => {
        expect(test.post()).to.equal(SUCCESS)
    })
    it('should be able to call the get_with_requests method', () => {
        expect(test.get_with_requests()).to.equal(SUCCESS)
    })
})