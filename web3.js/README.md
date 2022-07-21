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
<script src="https://unpkg.com/@safecoin/web3.js@latest/lib/index.iife.js"></script>

<!-- Production (minified) -->
<script src="https://unpkg.com/@safecoin/web3.js@latest/lib/index.iife.min.js"></script>
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

## Examples

Example scripts for the web3.js repo and native programs:

- [Web3 Examples](https://github.com/fair-exchange/safecoin/tree/master/web3.js/examples)

Example scripts for the Safecoin Program Library:

- [Token Program Examples](https://github.com/fair-exchange/safecoin-program-library/tree/master/token/js/examples)

## Flow

A [Flow library definition](https://flow.org/en/docs/libdefs/) is provided at
https://unpkg.com/@safecoin/web3.js@latest/module.flow.js.
Download the file and add the following line under the [libs] section of your project's `.flowconfig` to
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
(`<script>` tag)

## Disclaimer

All claims, content, designs, algorithms, estimates, roadmaps,
specifications, and performance measurements described in this project
are done with the Solana Foundation's ("SF") best efforts. It is up to
the reader to check and validate their accuracy and truthfulness.
Furthermore nothing in this project constitutes a solicitation for
investment.

Any content produced by SF or developer resources that SF provides, are
for educational and inspiration purposes only. SF does not encourage,
induce or sanction the deployment, integration or use of any such
applications (including the code comprising the Safecoin blockchain
protocol) in violation of applicable laws or regulations and hereby
prohibits any such deployment, integration or use. This includes use of
any such applications by the reader (a) in violation of export control
or sanctions laws of the United States or any other applicable
jurisdiction, (b) if the reader is located in or ordinarily resident in
a country or territory subject to comprehensive sanctions administered
by the U.S. Office of Foreign Assets Control (OFAC), or (c) if the
reader is or is working on behalf of a Specially Designated National
(SDN) or a person subject to similar blocking or denied party
prohibitions.

The reader should be aware that U.S. export control and sanctions laws
prohibit U.S. persons (and other persons that are subject to such laws)
from transacting with persons in certain countries and territories or
that are on the SDN list. As a project based primarily on open-source
software, it is possible that such sanctioned persons may nevertheless
bypass prohibitions, obtain the code comprising the Safecoin blockchain
protocol (or other project code or applications) and deploy, integrate,
or otherwise use it. Accordingly, there is a risk to individuals that
other persons using the Safecoin blockchain protocol may be sanctioned
persons and that transactions with such persons would be a violation of
U.S. export controls and sanctions law. This risk applies to
individuals, organizations, and other ecosystem participants that
deploy, integrate, or use the Safecoin blockchain protocol code directly
(e.g., as a node operator), and individuals that transact on the Safecoin
blockchain through light clients, third party interfaces, and/or wallet
software.
