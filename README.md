# Node.JS 10 and Node.JS 12 OpenSSL Issue

## Explanation

There seems to be an issue with calling OpenSSL from a C library called in an extension or called using the `ffi-napi` Node.js library. This issue is not present with Node.JS v8, but is present with Node.JS v10 and v12.  I know that some versions of Node.js are statically linked to OpenSSL, but I've tried to follow [these outdated instructions](https://github.com/nodejs/node-gyp/wiki/Linking-to-OpenSSL#windows) to call the version of OpenSSL linked to Node.js and saw the same issue.

Any help would be greatly appreciated.

## This issue is only present on Linux. To duplicate the problem:

1. Install n to easily switch between Node.JS Versions

    ```sh
    sudo npm install n
    ```

2. Install Rust build tools at https://www.rust-lang.org/tools/install

3. Build the C library

    ```sh
    cargo build
    ```

4. Show all tests pass with Node.JS v8

    ```sh
    sudo n 8
    npm install
    npm test
    ```

5. Show all tests fail with Node.JS v10 or v12

    ```sh
    sudo n 12 # Or 10
    npm install
    npm test
    ```