[![Build status][travis-image]][travis-url]
[![codecov][codecov-image]][codecov-url]
<br>
[![npm][npm-image]][npm-url]
[![npm-downloads][npm-downloads-image]][npm-url]
<br>
[![semantic-release][semantic-release-image]][semantic-release-url]
[![code-style-prettier][code-style-prettier-image]][code-style-prettier-url]

[travis-image]: https://api.travis-ci.org/fair-exchange/safecoin-web3.js.svg?branch=master
[travis-url]: https://travis-ci.org/fair-exchange/safecoin-web3.js
[codecov-image]: https://codecov.io/gh/fair-exchange/safecoin-web3.js/branch/master/graph/badge.svg
[codecov-url]: https://codecov.io/gh/fair-exchange/safecoin-web3.js
[npm-image]: https://img.shields.io/npm/v/@safecoin/web3.js.svg?style=flat
[npm-downloads-image]: https://img.shields.io/npm/dm/@safecoin/web3.js.svg?style=flat
[npm-url]: https://www.npmjs.com/package/@safecoin/web3.js
[semantic-release-image]: https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg
[semantic-release-url]: https://github.com/semantic-release/semantic-release
[code-style-prettier-image]: https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square
[code-style-prettier-url]: https://github.com/prettier/prettier

# Safecoin JavaScript API

This is the Safecoin Javascript API built on the Safecoin [JSON RPC API](https://docs.solana.com/apps/jsonrpc-api)

[Latest API Documentation](https://solana-labs.github.io/solana-web3.js/)


## Installation

### Yarn
```
$ yarn add @safecoin/web3.js
```

### npm
```
$ npm install --save @safecoin/web3.js
```

### Browser bundle
```html
<!-- Development (un-minified) -->
<script src="https://unpkg.com/@safecoin/web3.js@0.92.0/lib/index.iife.js"></script>

<!-- Production (un-minified) -->
<script src="https://unpkg.com/@safecoin/web3.js@0.92.0/lib/index.iife.min.js"></script>
```

## Development Environment Setup

Install the latest Safecoin release from https://docs.solana.com/cli/install-solana-cli-tools

### Run test validator

**Use `safecoin-test-validator` from the latest Safecoin release**

### BPF program development

**Use `cargo build-bpf` from the latest Safecoin release**

## Usage

### Javascript
```js
const solanaWeb3 = require('@safecoin/web3.js');
console.log(solanaWeb3);
```

### ES6
```js
import * as solanaWeb3 from '@safecoin/web3.js';
console.log(solanaWeb3);
```

### Browser bundle
```js
// `solanaWeb3` is provided in the global namespace by the `solanaWeb3.min.js` script bundle.
console.log(solanaWeb3);
```

## Flow

A [Flow library definition](https://flow.org/en/docs/libdefs/) is provided at
[module.flow.js](https://github.com/fair-exchange/safecoin-web3.js/tree/master/module.flow.js).
Add the following line under the [libs] section of your project's .flowconfig to
activate it:
```ini
[libs]
node_modules/@safecoin/web3.js/module.flow.js
```

## Releases
Releases are available on [Github](https://github.com/fair-exchange/safecoin-web3.js/releases)
and [npmjs.com](https://www.npmjs.com/package/@safecoin/web3.js)

Each Github release features a tarball containing API documentation and a
minified version of the module suitable for direct use in a browser environment
(&lt;script&gt; tag)
