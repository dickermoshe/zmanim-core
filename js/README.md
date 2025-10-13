# Zmanim Core JavaScript Bindings

[![npm version](https://img.shields.io/npm/v/zmanim-core-bindings)](https://www.npmjs.com/package/zmanim-core-bindings)
[![License](https://img.shields.io/badge/license-LGPL2.1-blue.svg)](LICENSE)
[![Node.js](https://img.shields.io/badge/node.js-18+-green.svg)](https://nodejs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.8+-blue.svg)](https://www.typescriptlang.org/)

High-performance JavaScript/TypeScript bindings for the Zmanim Core library - a comprehensive solution for calculating Jewish religious times (zmanim) and astronomical events. Built with WebAssembly for optimal performance in both Node.js and browser environments.

## 🌟 Features

- **Astronomical Calculations**: Precise sunrise, sunset, and astronomical event calculations
- **Jewish Religious Times**: Complete zmanim calculations including:
  - Alos Hashachar (dawn)
  - Tzais (nightfall)
  - Chatzos (midday)
  - Prayer times (Shacharis, Mincha, Maariv)
  - Candle lighting times
- **Hebrew Calendar**: Full Jewish calendar support with:
  - Date conversions between Gregorian and Jewish dates
  - Holiday calculations
  - Parsha (weekly Torah portion) information
  - Daf Yomi calculations
- **Geolocation Support**: Location-based calculations using coordinates
- **High Performance**: WebAssembly implementation with minimal overhead
- **Cross-Platform**: Supports Node.js, browsers, and React Native
- **TypeScript Support**: Full type definitions included

## 🚀 Installation

### From npm (Recommended)

```bash
npm install zmanim-core-bindings
```

### From yarn

```bash
yarn add zmanim-core-bindings
```

### From pnpm

```bash
pnpm add zmanim-core-bindings
```

## 📖 Quick Start

### Basic Usage

```javascript
import { uniffiInitAsync, newGeolocation, newAstronomicalCalendar } from 'zmanim-core-bindings';

async function calculateSunset() {
  // Initialize the WASM module (required before use)
  await uniffiInitAsync();

  // Create a location (Jerusalem coordinates)
  const location = newGeolocation(
    31.78,    // latitude
    35.22,    // longitude
    754.0     // elevation in meters
  );

  // Get current timestamp in milliseconds
  const now = Date.now();
  const timestamp = BigInt(now);

  // Create an astronomical calendar
  const calendar = newAstronomicalCalendar(timestamp, location);

  // Get sunset time
  const sunset = calendar.getSunset();
  if (sunset) {
    const sunsetDate = new Date(Number(sunset));
    console.log(`Sunset: ${sunsetDate.toLocaleTimeString()}`);
  }
}

calculateSunset().catch(console.error);
```

## 📄 License

This project is licensed under the GNU Lesser General Public License v2.1 - see the [LICENSE](../LICENSE) file for details.

## 🙏 Acknowledgments

This library is a JavaScript port of the [Zmanim Core](https://github.com/dickermoshe/zmanim-core) Rust library, which is itself a port of the [KosherJava](https://github.com/KosherJava/KosherJava) library. Special thanks to the KosherJava contributors for their excellent work.

**Testing against KosherJava git hash**: `0ce858258bff15c11235b1f1063d2eb0ef22b994`

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/dickermoshe/zmanim-core/issues)
- **Discussions**: [GitHub Discussions](https://github.com/dickermoshe/zmanim-core/discussions)
- **Documentation**: [Rust API Docs](https://docs.rs/zmanim-core)

## 🔗 Related Projects

- [Zmanim Core (Rust)](https://github.com/dickermoshe/zmanim-core) - Core Rust library
- [Zmanim Core Python](https://github.com/dickermoshe/zmanim-core) - Python bindings
- [KosherJava](https://github.com/KosherJava/KosherJava) - Original Java implementation

## Building

To build this project, you must have rust installed and be on a linux machine.

Install the wasm target with:
```
rustup target add wasm32-unknown-unknown
```
You must also have the wasm-bindgen-cli installed.

```bash
cargo install wasm-bindgen-cli
```

Then, from the root of the project, run:
```bash
npm run build:rust
```
Then replace this line in `index.ts` with the following:
```typescript
import wasmPath from './generated/wasm-bindgen/index_bg.wasm';
```
with the following:
```typescript
import wasmPath from './generated/wasm-bindgen/index_bg.wasm?url';
```
Then build the project with:
```bash
npm run build
```

---

**Made with ❤️ for the Jewish community and JavaScript developers**
