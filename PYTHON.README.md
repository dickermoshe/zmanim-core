# Zmanim Core Python Bindings

[![PyPI Version](https://img.shields.io/pypi/v/zmanim-core-bindings)](https://pypi.org/project/zmanim-core-bindings/)
[![Python Versions](https://img.shields.io/pypi/pyversions/zmanim-core-bindings)](https://pypi.org/project/zmanim-core-bindings/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

High-performance Python bindings for the Zmanim Core library - a comprehensive solution for calculating Jewish religious times (zmanim) and astronomical events.

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
- **High Performance**: Optimized Rust implementation with minimal overhead
- **Cross-Platform**: Supports Windows, Linux, macOS

## 🚀 Installation

### From PyPI (Recommended)

```bash
pip install zmanim-core-bindings
```

## 📖 Quick Start

### Basic Usage

```python
import zmanim_py.zmanim_core as zmanim
from datetime import datetime
import time

# Get current timestamp (milliseconds since epoch)
now = int(time.time() * 1000)

# Create a location (Jerusalem coordinates)
location = zmanim.new_geolocation(
    latitude=31.78,    # Jerusalem latitude
    longitude=35.22,   # Jerusalem longitude
    elevation=754.0    # Elevation in meters
)

# Create a zmanim calendar
calendar = zmanim.new_zmanim_calendar(
    timestamp=now,
    geo_location=location,
    use_astronomical_chatzos=True,
    use_astronomical_chatzos_for_other_zmanim=True,
    candle_lighting_offset=18 * 60 * 1000  # 18 minutes before sunset
)

# Get Alos Hashachar (dawn)
alos_timestamp = calendar.get_alos_hashachar()
if alos_timestamp:
    alos_datetime = datetime.fromtimestamp(alos_timestamp / 1000)
    print(f"Alos Hashachar: {alos_datetime}")
    # Note: This is in UTC, convert to local timezone as needed
```

For more information, see the [Rust API Docs](https://docs.rs/zmanim-core) and the original KosherJava [docs](https://github.com/KosherJava/KosherJava/blob/master/README.md) which this library is based on.

## 📄 License

This project is licensed under the Lesser General Public License v2.1 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

This library is a Rust port of the [KosherJava](https://github.com/KosherJava/KosherJava) library, which provides the reference implementation and testing framework. Special thanks to the KosherJava contributors for their excellent work.


## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/dickermoshe/zmanim-core/issues)
- **Discussions**: [GitHub Discussions](https://github.com/dickermoshe/zmanim-core/discussions)
- **Documentation**: [Rust API Docs](https://docs.rs/zmanim-core)

---

**Made with ❤️ for the Jewish community and Python developers**
