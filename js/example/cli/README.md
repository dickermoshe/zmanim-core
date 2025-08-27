# Sunset CLI Example

This is a command-line interface example that demonstrates how to use the zmanim-core library to calculate sunset times for any location.

## Features

- Calculate standard sunset times for any geographic location
- Display sea-level sunset times
- Show additional sunset calculations with degree offsets
- Support for custom latitude, longitude, and elevation
- Defaults to New York City if no location is provided
- Beautiful formatted output with emojis and timezone information

## Installation

1. **Navigate to the zmanim-core-bindings directory:**
   ```bash
   cd js
   ```

2. **Install zmanim-core-bindings dependencies:**
   ```bash
   npm install
   ```
  
3. **Build the zmanim-core-bindings library:**
   ```bash
   npm run build
   ```
  
4. **Navigate to the cli directory:**
   ```bash
   cd example/cli
   ```

5. **Install cli dependencies:**
   ```bash
   npm install
   ```

6. **Run the cli:**
   ```bash
   npm start
   ```



## Usage

### Basic Usage (uses New York City as default)
```bash
npm start
# or
node sunset.js
```

### With Custom Location
```bash
node sunset.js <latitude> <longitude> <elevation>
```

### Examples

**New York City:**
```bash
node sunset.js 40.7128 -74.0060 10
```

**Los Angeles:**
```bash
node sunset.js 34.0522 -118.2437 71
```

**Jerusalem:**
```bash
node sunset.js 31.7683 35.2137 754
```

**London:**
```bash
node sunset.js 51.5074 -0.1278 11
```

## Parameters

- **latitude**: Latitude in decimal degrees (-90 to 90)
- **longitude**: Longitude in decimal degrees (-180 to 180)  
- **elevation**: Elevation in meters above sea level


## Sample Output

```
🌅 Sunset Time Calculator
========================

Initializing zmanim-core library...
📍 Location: New York City, NY (default)
   Latitude: 40.7128°
   Longitude: -74.006°
   Elevation: 10m above sea level

📅 Date: Monday, December 16, 2024

🌇 Sunset Times:
================
Standard Sunset:     4:28:42 PM EST
Sea Level Sunset:    4:28:35 PM EST
Sunset + 15°:        5:11:23 PM EST
Sunset + 18°:        5:23:15 PM EST

✨ Calculation complete!
```

