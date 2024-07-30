# Rust Metrohash Node bindings

![Build Status](https://github.com/braquino/metrohash-rs-n/actions/workflows/CI.yml/badge.svg)
<span class="badge-npmversion"><a href="https://www.npmjs.com/package/metrohash-rs-n" title="View this project on NPM"><img src="https://img.shields.io/npm/v/metrohash-rs-n.svg" alt="NPM version" /></a></span>

This lib is a very simple Node API for Rust MetroHash. All credits for: https://github.com/arthurprs/metrohash-rs

It was created because the C implementation of MetroHash cannot be compiled for ARM.

## Usage

Install:
```bash
npm install metrohash-rs-n
```

Use:
```js
import { metrohash128, metrohash64 } from 'metrohash-rs-n';

const data = "foo";
const seed = 12345;
console.log(metrohash128(data, seed));
console.log(metrohash64(data, seed));
```
