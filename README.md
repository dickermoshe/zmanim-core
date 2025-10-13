> [!NOTE]
>  If you are looking for the Python Readme, please see [zmanim-core-bindings](./PYTHON.README.md).  
If you are looking for the Javascript Readme, please see [zmanim-core-bindings](./js/README.md).


# Zmanim Core

[![Crates.io](https://img.shields.io/crates/v/zmanim-core)](https://crates.io/crates/zmanim-core)
[![Documentation](https://docs.rs/zmanim-core/badge.svg)](https://docs.rs/zmanim-core)
[![License](https://img.shields.io/badge/license-LGPL2.1-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

A high-performance, `no_std` Rust library for calculating Jewish religious times (zmanim) and astronomical events. This library provides accurate calculations for sunrise, sunset, prayer times, and Jewish calendar dates based on astronomical algorithms.

## 🌟 Features

- **Astronomical Calculations**: Precise sunrise, sunset, and astronomical event calculations
- **Jewish Religious Times**: Complete zmanim calculations including:
  - Alos Hashachar (dawn)
  - Tzais (nightfall)
  - Chatzos (midday)
  - Prayer times (Shacharis, Mincha, Maariv)
  - Candle lighting times
- **Hebrew Calendar**: Full Jewish calendar support with:
  - Date conversions
  - Holiday calculations
  - Parsha (weekly Torah portion) information
  - Daf Yomi calculations
- **Geolocation Support**: Location-based calculations using coordinates
- **Cross-Platform**: Supports multiple targets including:
  - Windows, Linux, macOS
  - WebAssembly (WASM)
  - Embedded systems (thumbv7em-none-eabihf)
- **High Performance**: Optimized Rust implementation with LTO and stripping
- **No Standard Library**: `no_std` compatible for embedded and WASM use cases

## Documentation

This fork closely follows the original KosherJava api where possible.
See the JavaDoc for documentation: https://kosherjava.com/zmanim/docs/api/index.html?overview-summary.html

## 🚀 Quick Start

### Installation

Run the following
```toml
cargo add zmanim-core
```

### Basic Usage

```rust
use zmanim_core::prelude::*;
use chrono::{DateTime, Utc};

// Create a location (Jerusalem coordinates)
let location = GeoLocation::new(
    31.78,   // latitude
    35.22,   // longitude
    754.0,   // elevation in meters
);

// Create a calendar for a specific date
let timestamp = chrono::Utc::now().timestamp_millis();
let calendar = ZmanimCalendar::new(
    timestamp, 
    location, 
    false,  // Use astronomical chatzos for zmanim
    false,  // Use astronomical chatzos for other zmanim
    18 * 60 * 1000, // Candle lighting offset in milliseconds
);

// Get sunrise and sunset times
if let Some(alos72) = calendar.get_alos72() {
    println!("Alos72: {}", DateTime::from_timestamp_millis(alos72).unwrap());
}

if let Some(sunset) = calendar.get_astronomical_calendar().get_sunset() {
    println!("Sunset: {}", DateTime::from_timestamp_millis(sunset).unwrap());
}

// Get Jewish date information
let jewish_date = JewishDate::from_gregorian(
    timestamp, 
    4 * 60 * 60 * 1000, // Timezone offset in milliseconds (4 Hours)
);
println!("Jewish Date: {} {}, {}", 
    jewish_date.get_day_of_month(),
    jewish_date.get_jewish_month_name(),
    jewish_date.get_jewish_year()
);
```

## 🧪 Testing

This library includes comprehensive testing against the original KosherJava implementation to ensure accuracy and correctness.

### Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test zmanim_calendar_tests
```

### Test Coverage

The test suite includes:
- Unit tests for all modules
- Integration tests against Java reference implementation
- Astronomical calculation accuracy tests
- Jewish calendar correctness tests
- Cross-platform compatibility tests



### Features

The library supports multiple build targets:
- **Native**: Windows, Linux, macOS
- **WebAssembly**: Browser and Node.js compatibility
- **Embedded**: ARM Cortex-M7 and similar microcontrollers

## 📖 Documentation

- **API Reference**: [docs.rs/zmanim-core](https://docs.rs/zmanim-core)
- **Tests**: Comprehensive test suite in `tests/`

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/dickermoshe/zmanim-core.git
cd zmanim-core

# Install dependencies
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

This library is a Rust port of the [KosherJava](https://github.com/KosherJava/KosherJava) library, which provides the reference implementation and testing framework. Special thanks to the KosherJava contributors for their excellent work.


## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/dickermoshe/zmanim-core/issues)
- **Discussions**: [GitHub Discussions](https://github.com/dickermoshe/zmanim-core/discussions)
- **Documentation**: [docs.rs/zmanim-core](https://docs.rs/zmanim-core)

---

**Made with ❤️ for the Jewish community and Rust enthusiasts**
