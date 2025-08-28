# Zmanim Core - Go Bindings

Go bindings for the Zmanim Core library, providing Jewish calendar calculations and zmanim (halachic times) functionality.

## Prerequisites

- Go 1.16 or later
- CGO enabled (default)

## Installation

### Option 1: Dynamic Linking (Recommended)

1. **Install the Go package**:
   ```bash
   go get github.com/dickermoshe/zmanim-core/zmanim_core
   ```

2. **Download the dynamic libraries** from the [GitHub releases page](https://github.com/dickermoshe/zmanim-core/releases)

3. **Place the library files** in your Go project directory:
   - **Windows**: Copy `zmanim_core.dll` to the same directory as your Go executable
   - **Linux**: Copy `libzmanim_core.so` to your system library path (e.g., `/usr/local/lib/`) or the same directory as your executable
   - **macOS**: Copy `libzmanim_core.dylib` to your system library path or the same directory as your executable

4. **Import and use** the Go bindings:

```go
package main

import (
	"fmt"
	"time"

	zmanim_core "github.com/dickermoshe/zmanim-core/zmanim_core"
)

func main() {
	msSinceEpoch := time.Now().UnixMilli()
	location := zmanim_core.NewGeolocation(31.768319, 35.213711, 0)
	calendar := zmanim_core.NewAstronomicalCalendar(msSinceEpoch, *location)
	sunrise := calendar.GetSunrise()
	// Parse the sunrise time to a time.Time ms since epoch
	sunriseTime := time.UnixMilli(*sunrise)
	fmt.Println("Sunrise time:", sunriseTime)
}
```

4. **Build and run**:
```bash
go build -o your_app main.go
./your_app  # On Windows: your_app.exe
```

### Option 2: Static Linking

If you prefer to statically link the Rust library into your Go binary:

1. **Download the static libraries** from the [GitHub releases page](https://github.com/your-repo/zmanim-core/releases)
   - **Windows**: `zmanim_core.lib`
   - **Linux/macOS**: `libzmanim_core.a`

2. **Place the static library** in your project directory or system library path

3. **Build with static linking flags**:

```bash
# Windows
set CGO_ENABLED=1
set CGO_LDFLAGS=-L. -lzmanim_core
go build -ldflags "-linkmode external -extldflags -static" -o your_app.exe main.go

# Linux/macOS
export CGO_ENABLED=1
export CGO_LDFLAGS="-L. -lzmanim_core"
go build -ldflags "-linkmode external -extldflags -static" -o your_app main.go
```

## Troubleshooting

### Common Issues

1. **"cannot find library"** error:
   - Ensure the dynamic library is in the same directory as your executable
   - On Linux/macOS, you can also add the library path to `LD_LIBRARY_PATH`

2. **CGO compilation errors**:
   - Make sure CGO is enabled: `export CGO_ENABLED=1` (Linux/macOS) or `set CGO_ENABLED=1` (Windows)
   - Verify that you have a C compiler installed (gcc, clang, or MSVC)

3. **Static linking issues**:
   - Ensure you have the static library file (`.lib`, `.a`) not the dynamic one
   - Some systems may require additional system libraries to be linked

## License

This project is licensed under the same license as the main Zmanim Core project.
