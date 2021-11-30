# @wasm-image/bilinear

> Fast bilinear image resampling, made with Rust + WASM 🦀⚡️

## Installation

```sh
npm i @wasm-image/bilinear
```

or

```sh
yarn add @wasm-image/bilinear
```

## Usage

Make sure that the WASM Module is not included in initial chunk, always load WASM Modules asynchronously.

### Usage with Next.js 12 or application using Webpack 5

Prepare your configuration with

````js
// next.config.js
module.exports = {
  reactStrictMode: true,
  webpack(config) {
    config.experiments.asyncWebAssembly = true;
    return config;
  },
};
```
````

or

```js
// webpack.config.js

const config = {
  // your config
  experiments: {
  	asyncWebAssembly: true
	}
}

module.exports = config
```

and then

```typescript
import mod from "a-module";

async function main() {
  try {
    ...
    const { bilinear } = await import("@wasm-image/bilinear")
    
    // example of usage
    const { width, height } = canvas; // make sure that you have a canvas with content
    const imageData = context.getImageData(0, 0, videoWidth, videoHeight); // and a 2d context
    const resized = bilinear(imageData, width / 10,  height / 10);
    
    console.log(resized);
  } catch (error) {
    // handle import error
  }
}
```



### Usage as native ESM module.

```js
// TODO: implement example
```

