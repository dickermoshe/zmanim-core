import { useState, useEffect } from 'react';
import { uniffiInitAsync, newGeolocation, newAstronomicalCalendar } from 'zmanim-core-bindings';
import './SunsetCalculator.css';

interface LocationData {
    latitude: number;
    longitude: number;
    elevation: number;
    name: string;
}

interface SunsetData {
    standardSunset: string;
    seaLevelSunset: string;
    sunset15Degrees?: string;
    sunset18Degrees?: string;
    location: LocationData;
    date: string;
}

const PRESET_LOCATIONS: LocationData[] = [
    { latitude: 40.7128, longitude: -74.0060, elevation: 10, name: 'New York City, NY' },
    { latitude: 34.0522, longitude: -118.2437, elevation: 71, name: 'Los Angeles, CA' },
    { latitude: 31.7683, longitude: 35.2137, elevation: 754, name: 'Jerusalem, Israel' },
    { latitude: 51.5074, longitude: -0.1278, elevation: 11, name: 'London, UK' },
    { latitude: 48.8566, longitude: 2.3522, elevation: 35, name: 'Paris, France' },
    { latitude: 35.6762, longitude: 139.6503, elevation: 40, name: 'Tokyo, Japan' },
];

export default function SunsetCalculator() {
    const [isInitialized, setIsInitialized] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [sunsetData, setSunsetData] = useState<SunsetData | null>(null);

    // Form state
    const [selectedPreset, setSelectedPreset] = useState<string>('0');
    const [customLocation, setCustomLocation] = useState<{
        latitude: string;
        longitude: string;
        elevation: string;
        name: string;
    }>({
        latitude: '',
        longitude: '',
        elevation: '',
        name: ''
    });
    const [useCustomLocation, setUseCustomLocation] = useState(false);

    // Initialize the WASM module
    useEffect(() => {
        const initializeLibrary = async () => {
            try {
                setIsLoading(true);
                await uniffiInitAsync();
                setIsInitialized(true);
                // Calculate sunset for default location (NYC)
                await calculateSunset(PRESET_LOCATIONS[0]);
            } catch (err) {
                setError(`Failed to initialize library: ${err instanceof Error ? err.message : 'Unknown error'}`);
            } finally {
                setIsLoading(false);
            }
        };

        initializeLibrary();
    }, []);

    const formatTime = (timestamp: bigint | undefined): string => {
        if (!timestamp) return 'Not available';

        try {
            const date = new Date(Number(timestamp));
            return date.toLocaleTimeString('en-US', {
                hour: '2-digit',
                minute: '2-digit',
                second: '2-digit',
                timeZoneName: 'short'
            });
        } catch {
            return 'Invalid time';
        }
    };

    const calculateSunset = async (location: LocationData) => {
        if (!isInitialized) {
            setError('Library not initialized');
            return;
        }

        setIsLoading(true);
        setError(null);

        try {
            // Create geolocation
            const geoLocation = newGeolocation(location.latitude, location.longitude, location.elevation);
            if (!geoLocation) {
                throw new Error('Failed to create geolocation');
            }

            // Get current timestamp
            const now = Date.now();
            const timestamp = BigInt(now);

            // Create astronomical calendar
            const calendar = newAstronomicalCalendar(timestamp, geoLocation);

            // Get sunset times
            const standardSunset = calendar.getSunset();
            const seaLevelSunset = calendar.getSeaLevelSunset();

            let sunset15Degrees: bigint | undefined;
            let sunset18Degrees: bigint | undefined;

            try {
                sunset15Degrees = calendar.getSunsetOffsetByDegrees(15.0);
                sunset18Degrees = calendar.getSunsetOffsetByDegrees(18.0);
            } catch {
                // These calculations might not be available for all locations
            }

            const currentDate = new Date(now);

            setSunsetData({
                standardSunset: formatTime(standardSunset),
                seaLevelSunset: formatTime(seaLevelSunset),
                sunset15Degrees: formatTime(sunset15Degrees),
                sunset18Degrees: formatTime(sunset18Degrees),
                location,
                date: currentDate.toLocaleDateString('en-US', {
                    weekday: 'long',
                    year: 'numeric',
                    month: 'long',
                    day: 'numeric'
                })
            });

        } catch (err) {
            setError(`Calculation failed: ${err instanceof Error ? err.message : 'Unknown error'}`);
        } finally {
            setIsLoading(false);
        }
    };

    const handlePresetChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        const index = parseInt(event.target.value);
        setSelectedPreset(event.target.value);
        setUseCustomLocation(false);
        calculateSunset(PRESET_LOCATIONS[index]);
    };

    const handleCustomLocationSubmit = (event: React.FormEvent) => {
        event.preventDefault();

        const lat = parseFloat(customLocation.latitude);
        const lon = parseFloat(customLocation.longitude);
        const elev = parseFloat(customLocation.elevation);

        if (isNaN(lat) || isNaN(lon) || isNaN(elev)) {
            setError('Please enter valid numbers for all location fields');
            return;
        }

        if (lat < -90 || lat > 90) {
            setError('Latitude must be between -90 and 90 degrees');
            return;
        }

        if (lon < -180 || lon > 180) {
            setError('Longitude must be between -180 and 180 degrees');
            return;
        }

        const location: LocationData = {
            latitude: lat,
            longitude: lon,
            elevation: elev,
            name: customLocation.name || `${lat.toFixed(4)}°, ${lon.toFixed(4)}°`
        };

        calculateSunset(location);
    };

    const toggleCustomLocation = () => {
        setUseCustomLocation(!useCustomLocation);
        setError(null);
    };

    if (!isInitialized && isLoading) {
        return (
            <div className="sunset-calculator">
                <div className="loading">
                    <div className="spinner"></div>
                    <p>Initializing Zmanim Core library...</p>
                </div>
            </div>
        );
    }

    return (
        <div className="sunset-calculator">
            <header className="header">
                <h1>🌅 Sunset Time Calculator</h1>
                <p>Calculate precise sunset times for any location using astronomical algorithms</p>
            </header>

            <div className="location-selector">
                <div className="preset-selector">
                    <h2>Choose Location</h2>
                    <div className="location-options">
                        <label className="radio-option">
                            <input
                                type="radio"
                                name="location-type"
                                checked={!useCustomLocation}
                                onChange={() => setUseCustomLocation(false)}
                            />
                            <span>Preset Locations</span>
                        </label>
                        <label className="radio-option">
                            <input
                                type="radio"
                                name="location-type"
                                checked={useCustomLocation}
                                onChange={toggleCustomLocation}
                            />
                            <span>Custom Location</span>
                        </label>
                    </div>

                    {!useCustomLocation && (
                        <select
                            value={selectedPreset}
                            onChange={handlePresetChange}
                            className="preset-dropdown"
                        >
                            {PRESET_LOCATIONS.map((location, index) => (
                                <option key={index} value={index}>
                                    {location.name}
                                </option>
                            ))}
                        </select>
                    )}

                    {useCustomLocation && (
                        <form onSubmit={handleCustomLocationSubmit} className="custom-form">
                            <div className="form-row">
                                <div className="form-group">
                                    <label>Location Name (optional)</label>
                                    <input
                                        type="text"
                                        value={customLocation.name}
                                        onChange={(e) => setCustomLocation(prev => ({ ...prev, name: e.target.value }))}
                                        placeholder="e.g., My City"
                                    />
                                </div>
                            </div>
                            <div className="form-row">
                                <div className="form-group">
                                    <label>Latitude *</label>
                                    <input
                                        type="number"
                                        step="any"
                                        value={customLocation.latitude}
                                        onChange={(e) => setCustomLocation(prev => ({ ...prev, latitude: e.target.value }))}
                                        placeholder="e.g., 40.7128"
                                        required
                                    />
                                </div>
                                <div className="form-group">
                                    <label>Longitude *</label>
                                    <input
                                        type="number"
                                        step="any"
                                        value={customLocation.longitude}
                                        onChange={(e) => setCustomLocation(prev => ({ ...prev, longitude: e.target.value }))}
                                        placeholder="e.g., -74.0060"
                                        required
                                    />
                                </div>
                                <div className="form-group">
                                    <label>Elevation (m) *</label>
                                    <input
                                        type="number"
                                        step="any"
                                        value={customLocation.elevation}
                                        onChange={(e) => setCustomLocation(prev => ({ ...prev, elevation: e.target.value }))}
                                        placeholder="e.g., 10"
                                        required
                                    />
                                </div>
                            </div>
                            <button type="submit" className="calculate-btn" disabled={isLoading}>
                                {isLoading ? 'Calculating...' : 'Calculate Sunset'}
                            </button>
                        </form>
                    )}
                </div>
            </div>

            {error && (
                <div className="error">
                    <span className="error-icon">⚠️</span>
                    {error}
                </div>
            )}

            {sunsetData && !error && (
                <div className="results">
                    <div className="location-info">
                        <h2>📍 {sunsetData.location.name}</h2>
                        <div className="coordinates">
                            <span>Latitude: {sunsetData.location.latitude}°</span>
                            <span>Longitude: {sunsetData.location.longitude}°</span>
                            <span>Elevation: {sunsetData.location.elevation}m</span>
                        </div>
                        <div className="date">📅 {sunsetData.date}</div>
                    </div>

                    <div className="sunset-times">
                        <h2>🌇 Sunset Times</h2>
                        <div className="time-grid">
                            <div className="time-item primary">
                                <span className="time-label">Standard Sunset</span>
                                <span className="time-value">{sunsetData.standardSunset}</span>
                            </div>
                            <div className="time-item">
                                <span className="time-label">Sea Level Sunset</span>
                                <span className="time-value">{sunsetData.seaLevelSunset}</span>
                            </div>
                            {sunsetData.sunset15Degrees && sunsetData.sunset15Degrees !== 'Not available' && (
                                <div className="time-item">
                                    <span className="time-label">Sunset + 15°</span>
                                    <span className="time-value">{sunsetData.sunset15Degrees}</span>
                                </div>
                            )}
                            {sunsetData.sunset18Degrees && sunsetData.sunset18Degrees !== 'Not available' && (
                                <div className="time-item">
                                    <span className="time-label">Sunset + 18°</span>
                                    <span className="time-value">{sunsetData.sunset18Degrees}</span>
                                </div>
                            )}
                        </div>
                    </div>
                </div>
            )}

            {isLoading && sunsetData && (
                <div className="loading-overlay">
                    <div className="spinner"></div>
                    <p>Calculating...</p>
                </div>
            )}
        </div>
    );
}
