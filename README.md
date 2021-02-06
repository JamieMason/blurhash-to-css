# blurhash-to-css

> Converts a [BlurHash](https://blurha.sh) to a
> [CSS Object](https://developer.mozilla.org/en-US/docs/Web/API/ElementCSSInlineStyle/style) using
> [TypeScript](https://www.typescriptlang.org/), [Rust](https://www.rust-lang.org/), and
> [WebAssembly](https://github.com/rustwasm/wasm-pack).

[![NPM version](http://img.shields.io/npm/v/blurhash-to-css.svg?style=flat-square)](https://www.npmjs.com/package/blurhash-to-css)
[![NPM downloads](http://img.shields.io/npm/dm/blurhash-to-css.svg?style=flat-square)](https://www.npmjs.com/package/blurhash-to-css)
[![Build Status](http://img.shields.io/travis/JamieMason/blurhash-to-css/master.svg?style=flat-square)](https://travis-ci.org/JamieMason/blurhash-to-css)
[![Follow JamieMason on GitHub](https://img.shields.io/github/followers/JamieMason.svg?style=social&label=Follow)](https://github.com/JamieMason)
[![Follow fold_left on Twitter](https://img.shields.io/twitter/follow/fold_left.svg?style=social&label=Follow)](https://twitter.com/fold_left)

## 🌩 Installation

```
npm install blurhash-to-css
```

## 📣 Summary

[BlurHash](https://blurha.sh) is a compact representation of a placeholder for an image, it is a
great option to
[improve perceived load times and offer a better user experience when a connection or site is slow to load images](https://blog.imgix.com/2021/01/26/blurhash).
But BlurHash is rendered using a `<canvas>` element, which means some client-side JavaScript is
needed to render it. The goal is to have our images feel like they're loading as fast as we possibly
can, and we will need to run this JS on the critical path when loading our application.

This tool was created based on the premise that achieving this same visual effect via CSS rendered
on the Server will be easier to implement and should also be more efficient due to not having any
dependency on client-side JavaScript.

With this tool in place, we can try both approaches and measure whether rendering a CSS BlurHash is
in fact faster than using a `<canvas>`. This approach uses CSS transforms and filters, which _may_
be expensive in other ways?

Let's find out.

## 🤝 Credit

- The image buffer to CSS conversion is a port to Rust of
  [this Script](https://github.com/joe-bell/plaiceholder/blob/d09df807df5d11c5d91a7971c2f90e2faa845843/packages/css/src/index.ts#L18-L51)
  originally by [**@joe-bell**](https://github.com/joe-bell) for
  [plaiceholder: Beautiful image placeholders, without the hassle](https://plaiceholder.co/).
- A lot of help came from the source code in [wasm-image](https://github.com/peerigon/wasm-image) by
  [**@acidicX**](https://github.com/acidicX).

## 🕹 Usage

```ts
import { blurhashToCss } from 'blurhash-to-css';

const css = await blurhashToCss({
  blurhash: 'eCF6B#-:0JInxr?@s;nmIoWUIko1%NocRk.8xbIUaxR*^+s;RiWAWU',
  height: 400,
  width: 600,
});
```

↓↓↓↓↓

```json
{
  "backgroundImage": "linear-gradient(90deg, rgb(139,153,96) 10%,rgb(153,160,118) 10% 20%,rgb(170,172,142) 20% 30%,rgb(177,180,151) 30% 40%,rgb(174,180,146) 40% 50%,rgb(162,172,130) 50% 60%,rgb(148,162,114) 60% 70%,rgb(136,150,104) 70% 80%,rgb(131,145,95) 80% 90%,rgb(130,144,91) 90% 100%),linear-gradient(90deg, rgb(109,129,65) 10%,rgb(117,131,80) 10% 20%,rgb(130,138,101) 20% 30%,rgb(142,148,113) 30% 40%,rgb(146,150,114) 40% 50%,rgb(136,144,103) 50% 60%,rgb(116,130,88) 60% 70%,rgb(102,119,75) 70% 80%,rgb(104,118,71) 80% 90%,rgb(112,123,72) 90% 100%),linear-gradient(90deg, rgb(110,122,70) 10%,rgb(114,122,85) 10% 20%,rgb(126,129,106) 20% 30%,rgb(143,140,122) 30% 40%,rgb(151,147,128) 40% 50%,rgb(144,142,122) 50% 60%,rgb(123,126,107) 60% 70%,rgb(106,111,90) 70% 80%,rgb(108,109,83) 80% 90%,rgb(118,117,83) 90% 100%),linear-gradient(90deg, rgb(134,133,102) 10%,rgb(141,138,120) 10% 20%,rgb(155,150,144) 20% 30%,rgb(172,164,161) 30% 40%,rgb(179,173,168) 40% 50%,rgb(173,168,162) 50% 60%,rgb(156,151,146) 60% 70%,rgb(137,132,127) 70% 80%,rgb(127,124,110) 80% 90%,rgb(128,124,101) 90% 100%),linear-gradient(90deg, rgb(116,124,95) 10%,rgb(125,129,112) 10% 20%,rgb(143,141,135) 20% 30%,rgb(160,156,153) 30% 40%,rgb(169,164,160) 40% 50%,rgb(165,161,155) 50% 60%,rgb(148,145,141) 60% 70%,rgb(124,125,118) 70% 80%,rgb(107,110,97) 80% 90%,rgb(99,106,82) 90% 100%),linear-gradient(90deg, rgb(95,105,51) 10%,rgb(99,105,66) 10% 20%,rgb(110,111,88) 20% 30%,rgb(125,122,107) 30% 40%,rgb(136,131,115) 40% 50%,rgb(134,129,111) 50% 60%,rgb(120,116,98) 60% 70%,rgb(97,97,77) 70% 80%,rgb(78,84,51) 80% 90%,rgb(70,78,30) 90% 100%),linear-gradient(90deg, rgb(109,106,21) 10%,rgb(109,104,44) 10% 20%,rgb(112,104,67) 20% 30%,rgb(122,109,83) 30% 40%,rgb(131,114,88) 40% 50%,rgb(130,112,83) 50% 60%,rgb(119,103,70) 60% 70%,rgb(104,91,51) 70% 80%,rgb(93,84,34) 80% 90%,rgb(92,84,24) 90% 100%)",
  "backgroundPosition": "0 0 ,0 16.666666666666664%,0 33.33333333333333%,0 50%,0 66.66666666666666%,0 83.33333333333334%,0 100%",
  "backgroundSize": "100% 14.285714285714286%",
  "backgroundRepeat": "no-repeat",
  "filter": "blur(24px)",
  "transform": "scale(1.2)"
}
```

To use this a CommonJS project, you should destructure it like so:

```ts
const { blurhashToCss } = require('blurhash-to-css');
```

## 🛠 Options

### `blurhash`

A small string generated to represent a blurry version of an image which will download sometime
soon. Have a play around on [BlurHash](https://blurha.sh),
[plaiceholder.co](https://plaiceholder.co/), and have a read of
[BlurHash for imgix: An Alternative to Generic Image Placeholders](https://blog.imgix.com/2021/01/26/blurhash)
to try them out and find out more.

### `width` and `height`

The purpose of the `height` and `width` properties are to provide the physical dimensions of the
original image which was encoded into your blurhash so it can be parsed for processing – it is not
related to the size you want to display the image in your app.
