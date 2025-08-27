#!/usr/bin/env node

/**
 * CLI example showing sunset times using zmanim-core
 * Usage: node sunset.js [latitude] [longitude] [elevation]
 * Example: node sunset.js 40.7128 -74.0060 10
 */

import {
  uniffiInitAsync,
  newGeolocation,
  newAstronomicalCalendar,
} from "zmanim-core-bindings";

// Default location: New York City
const DEFAULT_LATITUDE = 40.7128;
const DEFAULT_LONGITUDE = -74.006;
const DEFAULT_ELEVATION = 10; // meters above sea level

function parseArgs() {
  const args = process.argv.slice(2);

  if (args.length === 0) {
    return {
      latitude: DEFAULT_LATITUDE,
      longitude: DEFAULT_LONGITUDE,
      elevation: DEFAULT_ELEVATION,
      isDefault: true,
    };
  }

  if (args.length !== 3) {
    console.error("Usage: node sunset.js [latitude] [longitude] [elevation]");
    console.error("Example: node sunset.js 40.7128 -74.0060 10");
    process.exit(1);
  }

  const latitude = parseFloat(args[0]);
  const longitude = parseFloat(args[1]);
  const elevation = parseFloat(args[2]);

  if (isNaN(latitude) || isNaN(longitude) || isNaN(elevation)) {
    console.error("Error: All arguments must be valid numbers");
    process.exit(1);
  }

  if (latitude < -90 || latitude > 90) {
    console.error("Error: Latitude must be between -90 and 90 degrees");
    process.exit(1);
  }

  if (longitude < -180 || longitude > 180) {
    console.error("Error: Longitude must be between -180 and 180 degrees");
    process.exit(1);
  }

  return { latitude, longitude, elevation, isDefault: false };
}

function formatTime(timestamp) {
  if (!timestamp) return "Not available";

  // Convert from milliseconds to Date
  const date = new Date(Number(timestamp));
  return date.toLocaleTimeString("en-US", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    timeZoneName: "short",
  });
}

function getLocationName(latitude, longitude, isDefault) {
  if (isDefault) {
    return "New York City, NY (default)";
  }

  // Simple location description based on coordinates
  const latDir = latitude >= 0 ? "N" : "S";
  const lonDir = longitude >= 0 ? "E" : "W";

  return `${Math.abs(latitude).toFixed(4)}°${latDir}, ${Math.abs(
    longitude
  ).toFixed(4)}°${lonDir}`;
}

async function main() {
  try {
    console.log("🌅 Sunset Time Calculator");
    console.log("========================\n");

    // Parse command line arguments
    const { latitude, longitude, elevation, isDefault } = parseArgs();

    // Initialize the WASM module
    console.log("Initializing zmanim-core library...");
    await uniffiInitAsync();

    // Create geolocation
    const geoLocation = newGeolocation(latitude, longitude, elevation);
    if (!geoLocation) {
      console.error("Error: Failed to create geolocation");
      process.exit(1);
    }

    // Get current timestamp in milliseconds
    const now = Date.now();
    const timestamp = BigInt(now);

    // Create astronomical calendar
    const calendar = newAstronomicalCalendar(timestamp, geoLocation);

    // Display location information
    const locationName = getLocationName(latitude, longitude, isDefault);
    console.log(`📍 Location: ${locationName}`);
    console.log(`   Latitude: ${latitude}°`);
    console.log(`   Longitude: ${longitude}°`);
    console.log(`   Elevation: ${elevation}m above sea level\n`);

    // Display current date
    const currentDate = new Date(now);
    console.log(
      `📅 Date: ${currentDate.toLocaleDateString("en-US", {
        weekday: "long",
        year: "numeric",
        month: "long",
        day: "numeric",
      })}\n`
    );

    // Get sunset times
    console.log("🌇 Sunset Times:");
    console.log("================");

    // Get various sunset calculations
    const sunset = calendar.getSunset();
    const seaLevelSunset = calendar.getSeaLevelSunset();

    console.log(`Standard Sunset:     ${formatTime(sunset)}`);
    console.log(`Sea Level Sunset:    ${formatTime(seaLevelSunset)}`);

    // Try some offset calculations
    try {
      const sunsetPlus15 = calendar.getSunsetOffsetByDegrees(15.0);
      const sunsetPlus18 = calendar.getSunsetOffsetByDegrees(18.0);

      console.log(`Sunset + 15°:        ${formatTime(sunsetPlus15)}`);
      console.log(`Sunset + 18°:        ${formatTime(sunsetPlus18)}`);
    } catch (error) {
      console.log("Additional sunset calculations not available");
    }

    console.log("\n✨ Calculation complete!");

    if (isDefault) {
      console.log("\n💡 Tip: Specify your location for accurate times:");
      console.log("   node sunset.js <latitude> <longitude> <elevation>");
      console.log(
        "   Example: node sunset.js 34.0522 -118.2437 71  # Los Angeles"
      );
    }
  } catch (error) {
    console.error("❌ Error calculating sunset times:", error.message);
    process.exit(1);
  }
}

// Run the main function
main().catch((error) => {
  console.error("❌ Unexpected error:", error);
  process.exit(1);
});
