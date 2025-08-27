let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}
/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_clone_astronomicalcalculator(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_clone_astronomicalcalculator(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 */
export function ubrn_uniffi_zmanim_core_fn_free_astronomicalcalculator(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    wasm.ubrn_uniffi_zmanim_core_fn_free_astronomicalcalculator(ptr, f_status_.__wbg_ptr);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_clone_astronomicalcalendar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_clone_astronomicalcalendar(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 */
export function ubrn_uniffi_zmanim_core_fn_free_astronomicalcalendar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    wasm.ubrn_uniffi_zmanim_core_fn_free_astronomicalcalendar(ptr, f_status_.__wbg_ptr);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_begin_astronomical_twilight(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_begin_astronomical_twilight(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_begin_civil_twilight(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_begin_civil_twilight(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_begin_nautical_twilight(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_begin_nautical_twilight(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_end_astronomical_twilight(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_end_astronomical_twilight(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_end_civil_twilight(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_end_civil_twilight(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_end_nautical_twilight(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_end_nautical_twilight(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_geo_location(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_geo_location(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_noaa_calculator(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_noaa_calculator(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sea_level_sunrise(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sea_level_sunrise(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sea_level_sunset(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sea_level_sunset(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_solar_midnight(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_solar_midnight(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sun_transit(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sun_transit(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_time
 * @param {bigint} end_time
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sun_transit_with_start_and_end_times(ptr, start_time, end_time, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sun_transit_with_start_and_end_times(ptr, start_time, end_time, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sunrise(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sunrise(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {number} degrees
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sunrise_offset_by_degrees(ptr, degrees, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sunrise_offset_by_degrees(ptr, degrees, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sunset(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sunset(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {number} degrees
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sunset_offset_by_degrees(ptr, degrees, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_sunset_offset_by_degrees(ptr, degrees, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_temporal_hour(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_temporal_hour(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_time
 * @param {bigint} end_time
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_temporal_hour_with_start_and_end_times(ptr, start_time, end_time, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_temporal_hour_with_start_and_end_times(ptr, start_time, end_time, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_timestamp(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_timestamp(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {number} zenith
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_utc_sea_level_sunrise(ptr, zenith, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_utc_sea_level_sunrise(ptr, zenith, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {number} zenith
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_utc_sea_level_sunset(ptr, zenith, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_utc_sea_level_sunset(ptr, zenith, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {number} zenith
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_utc_sunrise(ptr, zenith, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_utc_sunrise(ptr, zenith, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {number} zenith
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_utc_sunset(ptr, zenith, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_astronomicalcalendar_get_utc_sunset(ptr, zenith, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_clone_bavlidaf(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_clone_bavlidaf(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 */
export function ubrn_uniffi_zmanim_core_fn_free_bavlidaf(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    wasm.ubrn_uniffi_zmanim_core_fn_free_bavlidaf(ptr, f_status_.__wbg_ptr);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_clone_complexzmanimcalendar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_clone_complexzmanimcalendar(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 */
export function ubrn_uniffi_zmanim_core_fn_free_complexzmanimcalendar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    wasm.ubrn_uniffi_zmanim_core_fn_free_complexzmanimcalendar(ptr, f_status_.__wbg_ptr);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_120(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_120(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_120_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_120_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_18_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_18_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_19_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_19_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_19_point_8_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_19_point_8_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_26_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_26_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_60(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_60(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_72_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_72_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_90(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_90(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_90_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_90_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_96(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_96(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_96_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_96_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_baal_hatanya(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_alos_baal_hatanya(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_astronomical_calendar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_astronomical_calendar(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_ateret_torah_sunset_offset(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_ateret_torah_sunset_offset(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_rt_13_point_24_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_rt_13_point_24_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_rt_13_point_5_minutes_before_7_point_083_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_rt_13_point_5_minutes_before_7_point_083_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_rt_2_stars(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_rt_2_stars(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_rt_58_point_5_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_rt_58_point_5_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_13_point_5_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_13_point_5_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_16_point_875_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_16_point_875_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_18_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_18_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_2_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_2_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_2_point_8_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_2_point_8_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_3_point_05_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hashmashos_yereim_3_point_05_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosrt_13_point_24_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosrt_13_point_24_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosrt_13_point_5_minutes_before_7_point_083_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosrt_13_point_5_minutes_before_7_point_083_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosrt_2_stars(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosrt_2_stars(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosrt_58_point_5_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosrt_58_point_5_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_13_point_5_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_13_point_5_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_16_point_875_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_16_point_875_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_18_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_18_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_2_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_2_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_2_point_8_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_2_point_8_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_3_point_05_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_bain_hasmashosyereim_3_point_05_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_candle_lighting_offset(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_candle_lighting_offset(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_fixed_local_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_fixed_local_chatzos(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_half_day
 * @param {bigint} end_of_half_day
 * @param {number} hours
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_fixed_local_chatzos_based_zmanim(ptr, start_of_half_day, end_of_half_day, hours, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_fixed_local_chatzos_based_zmanim(ptr, start_of_half_day, end_of_half_day, hours, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_30_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_30_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_72_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_72_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_ahavat_shalom(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_ahavat_shalom(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_ateret_torah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_ateret_torah(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_baal_hatanya(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_baal_hatanya(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_baal_hatanya_greater_than_30(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_baal_hatanya_greater_than_30(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_gra_fixed_local_chatzos_30_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_gra_fixed_local_chatzos_30_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_greater_than_30(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_gedola_greater_than_30(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_72_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_72_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_ahavat_shalom(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_ahavat_shalom(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_ateret_torah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_ateret_torah(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_baal_hatanya(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_baal_hatanya(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_gra_fixed_local_chatzos_to_sunset(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_mincha_ketana_gra_fixed_local_chatzos_to_sunset(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_misheyakir_10_point_2_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_misheyakir_10_point_2_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_misheyakir_11_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_misheyakir_11_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_misheyakir_11_point_5_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_misheyakir_11_point_5_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_misheyakir_7_point_65_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_misheyakir_7_point_65_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_misheyakir_9_point_5_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_misheyakir_9_point_5_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_ahavat_shalom(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_ahavat_shalom(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_alos_to_sunset(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_alos_to_sunset(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_120_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_120_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_120_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_120_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_18_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_18_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_19_point_8_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_19_point_8_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_26_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_26_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_60_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_60_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_72_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_72_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_72_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_72_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_90_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_90_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_90_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_90_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_96_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_96_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_96_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_96_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_ateret_torah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_ateret_torah(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_baal_hatanya(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_baal_hatanya(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_gra_fixed_local_chatzos_to_sunset(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_plag_hamincha_gra_fixed_local_chatzos_to_sunset(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_72_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_72_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_gra(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_gra(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_120_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_120_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_120_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_120_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_18_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_18_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_19_point_8_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_19_point_8_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_26_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_26_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_60_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_60_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_72_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_72_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_72_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_72_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_90_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_90_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_90_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_90_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_96_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_96_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_96_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_96_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_7(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_7(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_8(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_8(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_ateret_torah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_ateret_torah(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_baal_hatanya(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_shaah_zmanis_baal_hatanya(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_baal_hatanya(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_baal_hatanya(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_gra(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_gra(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_72_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_72_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_72_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_72_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_biur_chametz_baal_hatanya(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_biur_chametz_baal_hatanya(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_biur_chametz_gra(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_biur_chametz_gra(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_72_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_72_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_72_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_72_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
 * @param {bigint} ptr
 * @param {Uint8Array} _alos
 * @param {Uint8Array} _tzais
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_kidush_levana_15_days(ptr, _alos, _tzais, f_status_) {
    const ptr0 = passArray8ToWasm0(_alos, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(_tzais, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_kidush_levana_15_days(ptr, ptr0, len0, ptr1, len1, f_status_.__wbg_ptr);
    var v3 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v3;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_kidush_levana_15_days_default(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_kidush_levana_15_days_default(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {Uint8Array} _alos
 * @param {Uint8Array} _tzais
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_kidush_levana_between_moldos(ptr, _alos, _tzais, f_status_) {
    const ptr0 = passArray8ToWasm0(_alos, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(_tzais, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_kidush_levana_between_moldos(ptr, ptr0, len0, ptr1, len1, f_status_.__wbg_ptr);
    var v3 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v3;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_kidush_levana_between_moldos_default(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_kidush_levana_between_moldos_default(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_3_hours_before_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_3_hours_before_chatzos(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_alos_16_point_1_to_sunset(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_alos_16_point_1_to_sunset(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_ateret_torah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_ateret_torah(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_baal_hatanya(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_baal_hatanya(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_fixed_local(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_fixed_local(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_gra_sunrise_to_fixed_local_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_gra_sunrise_to_fixed_local_chatzos(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_kol_eliyahu(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_kol_eliyahu(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_120_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_120_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_16_point_1_degrees_to_fixed_local_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_16_point_1_degrees_to_fixed_local_chatzos(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_18_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_18_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_18_degrees_to_fixed_local_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_18_degrees_to_fixed_local_chatzos(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_19_point_8_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_19_point_8_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes_to_fixed_local_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes_to_fixed_local_chatzos(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes_to_fixed_local_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes_to_fixed_local_chatzos(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_96_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_96_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_96_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_shma_mga_96_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_2_hours_before_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_2_hours_before_chatzos(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_ateret_torah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_ateret_torah(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_baal_hatanya(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_baal_hatanya(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_fixed_local(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_fixed_local(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_gra_sunrise_to_fixed_local_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_gra_sunrise_to_fixed_local_chatzos(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_120_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_120_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_18_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_18_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_19_point_8_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_19_point_8_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_72_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_72_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_72_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_72_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_90_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_90_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_90_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_90_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_96_minutes(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_96_minutes(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_96_minutes_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfila_mga_96_minutes_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfilah_ateret_torah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_sof_zman_tfilah_ateret_torah(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_3_days(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_3_days(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {Uint8Array} _alos
 * @param {Uint8Array} _tzais
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_3_days_with_times(ptr, _alos, _tzais, f_status_) {
    const ptr0 = passArray8ToWasm0(_alos, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(_tzais, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_3_days_with_times(ptr, ptr0, len0, ptr1, len1, f_status_.__wbg_ptr);
    var v3 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v3;
}

/**
 * @param {bigint} ptr
 * @param {Uint8Array} _alos
 * @param {Uint8Array} _tzais
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_7_days(ptr, _alos, _tzais, f_status_) {
    const ptr0 = passArray8ToWasm0(_alos, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(_tzais, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_7_days(ptr, ptr0, len0, ptr1, len1, f_status_.__wbg_ptr);
    var v3 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v3;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_7_days_default(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_7_days_default(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_120(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_120(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_120_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_120_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_16_point_1_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_16_point_1_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_18_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_18_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_19_point_8_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_19_point_8_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_26_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_26_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_50(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_50(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_60(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_60(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_72_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_72_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_90(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_90(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_90_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_90_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_96(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_96(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_96_zmanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_96_zmanis(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_ateret_torah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_ateret_torah(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_baal_hatanya(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_baal_hatanya(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_3_point_65_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_3_point_65_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_3_point_676_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_3_point_676_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_3_point_7_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_3_point_7_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_3_point_8_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_3_point_8_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_4_point_37_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_4_point_37_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_4_point_61_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_4_point_61_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_4_point_8_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_4_point_8_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_5_point_88_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_5_point_88_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_5_point_95_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_5_point_95_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_6_point_45_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_6_point_45_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_7_point_083_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_7_point_083_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_7_point_67_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_7_point_67_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_8_point_5_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_8_point_5_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_9_point_3_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_9_point_3_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_9_point_75_degrees(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_tzais_geonim_9_point_75_degrees(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_use_astronomical_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_use_astronomical_chatzos(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_use_astronomical_chatzos_for_other_zmanim(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_use_astronomical_chatzos_for_other_zmanim(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_zman_molad(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_complexzmanimcalendar_get_zman_molad(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_clone_geolocation(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_clone_geolocation(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 */
export function ubrn_uniffi_zmanim_core_fn_free_geolocation(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    wasm.ubrn_uniffi_zmanim_core_fn_free_geolocation(ptr, f_status_.__wbg_ptr);
}

/**
 * @param {bigint} ptr
 * @param {bigint} location
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_geolocation_geodesic_distance(ptr, location, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_geolocation_geodesic_distance(ptr, location, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} location
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_geolocation_geodesic_final_bearing(ptr, location, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_geolocation_geodesic_final_bearing(ptr, location, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} location
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_geolocation_geodesic_initial_bearing(ptr, location, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_geolocation_geodesic_initial_bearing(ptr, location, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_geolocation_get_elevation(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_geolocation_get_elevation(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_geolocation_get_latitude(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_geolocation_get_latitude(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_geolocation_get_longitude(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_geolocation_get_longitude(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {bigint} location
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_geolocation_rhumb_line_bearing(ptr, location, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_geolocation_rhumb_line_bearing(ptr, location, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {bigint} location
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_geolocation_rhumb_line_distance(ptr, location, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_geolocation_rhumb_line_distance(ptr, location, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {bigint} location
 * @param {Uint8Array} formula
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_geolocation_vincenty_inverse_formula(ptr, location, formula, f_status_) {
    const ptr0 = passArray8ToWasm0(formula, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_geolocation_vincenty_inverse_formula(ptr, location, ptr0, len0, f_status_.__wbg_ptr);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_clone_jewishcalendar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_clone_jewishcalendar(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 */
export function ubrn_uniffi_zmanim_core_fn_free_jewishcalendar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    wasm.ubrn_uniffi_zmanim_core_fn_free_jewishcalendar(ptr, f_status_.__wbg_ptr);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_bavli_daf_yomi(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_bavli_daf_yomi(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_day_of_chanukah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_day_of_chanukah(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_day_of_omer(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_day_of_omer(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_in_israel(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_in_israel(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_jewish_date(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_jewish_date(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_parshah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_parshah(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_use_modern_holidays(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_use_modern_holidays(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_yom_tov_index(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_get_yom_tov_index(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_has_candle_lighting(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_has_candle_lighting(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_aseres_yemei_teshuva(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_aseres_yemei_teshuva(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_assur_bemelacha(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_assur_bemelacha(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_chanukah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_chanukah(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_chol_hamoed(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_chol_hamoed(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_chol_hamoed_pesach(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_chol_hamoed_pesach(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_chol_hamoed_succos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_chol_hamoed_succos(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_erev_yom_tov(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_erev_yom_tov(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_erev_yom_tov_sheni(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_erev_yom_tov_sheni(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_hoshana_rabba(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_hoshana_rabba(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_isru_chag(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_isru_chag(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_pesach(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_pesach(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_purim(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_purim(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_rosh_chodesh(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_rosh_chodesh(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_rosh_hashana(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_rosh_hashana(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_shavuos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_shavuos(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_shemini_atzeres(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_shemini_atzeres(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_simchas_torah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_simchas_torah(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_succos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_succos(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_taanis(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_taanis(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_taanis_bechoros(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_taanis_bechoros(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_tisha_beav(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_tisha_beav(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_tomorrow_shabbos_or_yom_tov(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_tomorrow_shabbos_or_yom_tov(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_yom_kippur(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_yom_kippur(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_yom_tov(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_yom_tov(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_yom_tov_assur_bemelacha(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishcalendar_is_yom_tov_assur_bemelacha(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_clone_jewishdate(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_clone_jewishdate(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 */
export function ubrn_uniffi_zmanim_core_fn_free_jewishdate(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    wasm.ubrn_uniffi_zmanim_core_fn_free_jewishdate(ptr, f_status_.__wbg_ptr);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_chalakim_since_molad_tohu(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_chalakim_since_molad_tohu(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_cheshvan_kislev_kviah(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_cheshvan_kislev_kviah(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_day_of_week(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_day_of_week(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_days_in_jewish_month(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_days_in_jewish_month(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_days_in_jewish_year(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_days_in_jewish_year(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_days_since_start_of_jewish_year(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_days_since_start_of_jewish_year(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_gregorian_day_of_month(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_gregorian_day_of_month(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_gregorian_month(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_gregorian_month(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_gregorian_year(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_gregorian_year(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_jewish_day_of_month(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_jewish_day_of_month(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_jewish_month(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_jewish_month(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_jewish_year(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_jewish_year(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_molad_data(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_molad_data(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_molad_date(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_get_molad_date(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_is_cheshvan_long(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_is_cheshvan_long(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_is_jewish_leap_year(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_is_jewish_leap_year(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_jewishdate_is_kislev_short(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_jewishdate_is_kislev_short(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_clone_moladdata(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_clone_moladdata(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 */
export function ubrn_uniffi_zmanim_core_fn_free_moladdata(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    wasm.ubrn_uniffi_zmanim_core_fn_free_moladdata(ptr, f_status_.__wbg_ptr);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_clone_noaacalculator(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_clone_noaacalculator(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 */
export function ubrn_uniffi_zmanim_core_fn_free_noaacalculator(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    wasm.ubrn_uniffi_zmanim_core_fn_free_noaacalculator(ptr, f_status_.__wbg_ptr);
}

/**
 * @param {bigint} ptr
 * @param {bigint} timestamp
 * @param {bigint} geo_location
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_solar_azimuth(ptr, timestamp, geo_location, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_solar_azimuth(ptr, timestamp, geo_location, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} timestamp
 * @param {bigint} geo_location
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_solar_elevation(ptr, timestamp, geo_location, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_solar_elevation(ptr, timestamp, geo_location, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} timestamp
 * @param {bigint} geo_location
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_utc_midnight(ptr, timestamp, geo_location, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_utc_midnight(ptr, timestamp, geo_location, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} timestamp
 * @param {bigint} geo_location
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_utc_noon(ptr, timestamp, geo_location, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_utc_noon(ptr, timestamp, geo_location, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} timestamp
 * @param {bigint} geo_location
 * @param {number} zenith
 * @param {number} adjust_for_elevation
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_utc_sunrise(ptr, timestamp, geo_location, zenith, adjust_for_elevation, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_utc_sunrise(ptr, timestamp, geo_location, zenith, adjust_for_elevation, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} timestamp
 * @param {bigint} geo_location
 * @param {number} zenith
 * @param {number} adjust_for_elevation
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_utc_sunset(ptr, timestamp, geo_location, zenith, adjust_for_elevation, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_noaacalculator_get_utc_sunset(ptr, timestamp, geo_location, zenith, adjust_for_elevation, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_clone_zmanimcalendar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_clone_zmanimcalendar(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 */
export function ubrn_uniffi_zmanim_core_fn_free_zmanimcalendar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    wasm.ubrn_uniffi_zmanim_core_fn_free_zmanimcalendar(ptr, f_status_.__wbg_ptr);
}

/**
 * @param {bigint} ptr
 * @param {Uint8Array} start_of_day
 * @param {bigint} end_of_day
 * @param {number} synchronous
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_mincha_gedola(ptr, start_of_day, end_of_day, synchronous, f_status_) {
    const ptr0 = passArray8ToWasm0(start_of_day, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_mincha_gedola(ptr, ptr0, len0, end_of_day, synchronous, f_status_.__wbg_ptr);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {bigint} ptr
 * @param {Uint8Array} start_of_day
 * @param {bigint} end_of_day
 * @param {number} synchronous
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_mincha_ketana(ptr, start_of_day, end_of_day, synchronous, f_status_) {
    const ptr0 = passArray8ToWasm0(start_of_day, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_mincha_ketana(ptr, ptr0, len0, end_of_day, synchronous, f_status_.__wbg_ptr);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {bigint} ptr
 * @param {Uint8Array} start_of_day
 * @param {bigint} end_of_day
 * @param {number} synchronous
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_plag_hamincha(ptr, start_of_day, end_of_day, synchronous, f_status_) {
    const ptr0 = passArray8ToWasm0(start_of_day, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_plag_hamincha(ptr, ptr0, len0, end_of_day, synchronous, f_status_.__wbg_ptr);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {bigint} ptr
 * @param {Uint8Array} start_of_day
 * @param {bigint} end_of_day
 * @param {number} synchronous
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_samuch_le_mincha_ketana(ptr, start_of_day, end_of_day, synchronous, f_status_) {
    const ptr0 = passArray8ToWasm0(start_of_day, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_samuch_le_mincha_ketana(ptr, ptr0, len0, end_of_day, synchronous, f_status_.__wbg_ptr);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_day
 * @param {Uint8Array} end_of_day
 * @param {number} synchronous
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_sof_zman_shma(ptr, start_of_day, end_of_day, synchronous, f_status_) {
    const ptr0 = passArray8ToWasm0(end_of_day, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_sof_zman_shma(ptr, start_of_day, ptr0, len0, synchronous, f_status_.__wbg_ptr);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_day
 * @param {Uint8Array} end_of_day
 * @param {number} synchronous
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_sof_zman_tfila(ptr, start_of_day, end_of_day, synchronous, f_status_) {
    const ptr0 = passArray8ToWasm0(end_of_day, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar__get_sof_zman_tfila(ptr, start_of_day, ptr0, len0, synchronous, f_status_.__wbg_ptr);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_alos72(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_alos72(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_alos_hashachar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_alos_hashachar(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_astronomical_calendar(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_astronomical_calendar(ptr, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_candle_lighting(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_candle_lighting(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_candle_lighting_offset(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_candle_lighting_offset(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_chatzos(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_chatzos_as_half_day(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_chatzos_as_half_day(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_half_day
 * @param {bigint} end_of_half_day
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_half_day_based_shaah_zmanis(ptr, start_of_half_day, end_of_half_day, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_half_day_based_shaah_zmanis(ptr, start_of_half_day, end_of_half_day, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_half_day
 * @param {bigint} end_of_half_day
 * @param {number} hours
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_half_day_based_zman(ptr, start_of_half_day, end_of_half_day, hours, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_half_day_based_zman(ptr, start_of_half_day, end_of_half_day, hours, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_mincha_gedola_default(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_mincha_gedola_default(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_day
 * @param {bigint} end_of_day
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_mincha_gedola_simple(ptr, start_of_day, end_of_day, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_mincha_gedola_simple(ptr, start_of_day, end_of_day, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_mincha_ketana_default(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_mincha_ketana_default(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_day
 * @param {bigint} end_of_day
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_mincha_ketana_simple(ptr, start_of_day, end_of_day, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_mincha_ketana_simple(ptr, start_of_day, end_of_day, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {number} degrees
 * @param {number} sunset
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_percent_of_shaah_zmanis_from_degrees(ptr, degrees, sunset, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_percent_of_shaah_zmanis_from_degrees(ptr, degrees, sunset, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_plag_hamincha_default(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_plag_hamincha_default(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_day
 * @param {bigint} end_of_day
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_plag_hamincha_simple(ptr, start_of_day, end_of_day, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_plag_hamincha_simple(ptr, start_of_day, end_of_day, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_day
 * @param {bigint} end_of_day
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_samuch_le_mincha_ketana_simple(ptr, start_of_day, end_of_day, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_samuch_le_mincha_ketana_simple(ptr, start_of_day, end_of_day, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_day
 * @param {bigint} end_of_day
 * @param {number} hours
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_shaah_zmanis_based_zman(ptr, start_of_day, end_of_day, hours, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_shaah_zmanis_based_zman(ptr, start_of_day, end_of_day, hours, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_shaah_zmanis_gra(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_shaah_zmanis_gra(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_shaah_zmanis_mga(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_shaah_zmanis_mga(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_shma_gra(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_shma_gra(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_shma_mga(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_shma_mga(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_day
 * @param {bigint} end_of_day
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_shma_simple(ptr, start_of_day, end_of_day, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_shma_simple(ptr, start_of_day, end_of_day, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_tfila_gra(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_tfila_gra(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_tfila_mga(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_tfila_mga(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {bigint} start_of_day
 * @param {bigint} end_of_day
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_tfila_simple(ptr, start_of_day, end_of_day, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_sof_zman_tfila_simple(ptr, start_of_day, end_of_day, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_tzais(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_tzais(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_tzais72(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_tzais72(ptr, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_use_astronomical_chatzos(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_use_astronomical_chatzos(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} ptr
 * @param {RustCallStatus} f_status_
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_use_astronomical_chatzos_for_other_zmanim(ptr, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_method_zmanimcalendar_get_use_astronomical_chatzos_for_other_zmanim(ptr, f_status_.__wbg_ptr);
    return ret;
}

/**
 * @param {bigint} timestamp
 * @param {bigint} geo_location
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_func_new_astronomical_calendar(timestamp, geo_location, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_func_new_astronomical_calendar(timestamp, geo_location, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} timestamp
 * @param {bigint} geo_location
 * @param {number} use_astronomical_chatzos
 * @param {number} use_astronomical_chatzos_for_other_zmanim
 * @param {bigint} candle_lighting_offset
 * @param {bigint} ateret_torah_sunset_offset
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_func_new_complex_zmanim_calendar(timestamp, geo_location, use_astronomical_chatzos, use_astronomical_chatzos_for_other_zmanim, candle_lighting_offset, ateret_torah_sunset_offset, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_func_new_complex_zmanim_calendar(timestamp, geo_location, use_astronomical_chatzos, use_astronomical_chatzos_for_other_zmanim, candle_lighting_offset, ateret_torah_sunset_offset, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {number} latitude
 * @param {number} longitude
 * @param {number} elevation
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_func_new_geolocation(latitude, longitude, elevation, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_func_new_geolocation(latitude, longitude, elevation, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {bigint} timestamp
 * @param {bigint} tz_offset
 * @param {RustCallStatus} f_status_
 * @returns {Uint8Array}
 */
export function ubrn_uniffi_zmanim_core_fn_func_new_jewish_date(timestamp, tz_offset, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_func_new_jewish_date(timestamp, tz_offset, f_status_.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_func_new_noaa_calculator(f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_func_new_noaa_calculator(f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} timestamp
 * @param {bigint} geo_location
 * @param {number} use_astronomical_chatzos
 * @param {number} use_astronomical_chatzos_for_other_zmanim
 * @param {bigint} candle_lighting_offset
 * @param {RustCallStatus} f_status_
 * @returns {bigint}
 */
export function ubrn_uniffi_zmanim_core_fn_func_new_zmanim_calendar(timestamp, geo_location, use_astronomical_chatzos, use_astronomical_chatzos_for_other_zmanim, candle_lighting_offset, f_status_) {
    _assertClass(f_status_, RustCallStatus);
    const ret = wasm.ubrn_uniffi_zmanim_core_fn_func_new_zmanim_calendar(timestamp, geo_location, use_astronomical_chatzos, use_astronomical_chatzos_for_other_zmanim, candle_lighting_offset, f_status_.__wbg_ptr);
    return BigInt.asUintN(64, ret);
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_func_new_astronomical_calendar() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_func_new_astronomical_calendar();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_func_new_complex_zmanim_calendar() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_func_new_complex_zmanim_calendar();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_func_new_geolocation() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_func_new_geolocation();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_func_new_jewish_date() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_func_new_jewish_date();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_func_new_noaa_calculator() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_func_new_noaa_calculator();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_func_new_zmanim_calendar() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_func_new_zmanim_calendar();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_begin_astronomical_twilight() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_begin_astronomical_twilight();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_begin_civil_twilight() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_begin_civil_twilight();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_begin_nautical_twilight() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_begin_nautical_twilight();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_end_astronomical_twilight() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_end_astronomical_twilight();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_end_civil_twilight() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_end_civil_twilight();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_end_nautical_twilight() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_end_nautical_twilight();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_geo_location() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_geo_location();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_noaa_calculator() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_noaa_calculator();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sea_level_sunrise() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sea_level_sunrise();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sea_level_sunset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sea_level_sunset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_solar_midnight() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_solar_midnight();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sun_transit() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sun_transit();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sun_transit_with_start_and_end_times() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sun_transit_with_start_and_end_times();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sunrise() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sunrise();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sunrise_offset_by_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sunrise_offset_by_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sunset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sunset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sunset_offset_by_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_sunset_offset_by_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_temporal_hour() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_temporal_hour();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_temporal_hour_with_start_and_end_times() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_temporal_hour_with_start_and_end_times();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_timestamp() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_timestamp();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_utc_sea_level_sunrise() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_utc_sea_level_sunrise();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_utc_sea_level_sunset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_utc_sea_level_sunset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_utc_sunrise() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_utc_sunrise();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_utc_sunset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_astronomicalcalendar_get_utc_sunset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_120() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_120();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_120_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_120_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_18_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_18_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_19_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_19_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_19_point_8_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_19_point_8_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_26_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_26_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_60() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_60();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_72_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_72_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_90() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_90();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_90_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_90_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_96() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_96();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_96_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_96_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_baal_hatanya() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_alos_baal_hatanya();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_astronomical_calendar() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_astronomical_calendar();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_ateret_torah_sunset_offset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_ateret_torah_sunset_offset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_rt_13_point_24_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_rt_13_point_24_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_rt_13_point_5_minutes_before_7_point_083_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_rt_13_point_5_minutes_before_7_point_083_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_rt_2_stars() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_rt_2_stars();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_rt_58_point_5_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_rt_58_point_5_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_13_point_5_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_13_point_5_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_16_point_875_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_16_point_875_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_18_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_18_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_2_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_2_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_2_point_8_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_2_point_8_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_3_point_05_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hashmashos_yereim_3_point_05_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosrt_13_point_24_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosrt_13_point_24_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosrt_13_point_5_minutes_before_7_point_083_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosrt_13_point_5_minutes_before_7_point_083_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosrt_2_stars() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosrt_2_stars();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosrt_58_point_5_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosrt_58_point_5_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_13_point_5_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_13_point_5_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_16_point_875_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_16_point_875_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_18_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_18_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_2_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_2_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_2_point_8_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_2_point_8_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_3_point_05_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_bain_hasmashosyereim_3_point_05_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_candle_lighting_offset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_candle_lighting_offset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_fixed_local_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_fixed_local_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_fixed_local_chatzos_based_zmanim() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_fixed_local_chatzos_based_zmanim();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_30_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_30_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_72_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_72_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_ahavat_shalom() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_ahavat_shalom();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_ateret_torah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_ateret_torah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_baal_hatanya() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_baal_hatanya();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_baal_hatanya_greater_than_30() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_baal_hatanya_greater_than_30();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_gra_fixed_local_chatzos_30_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_gra_fixed_local_chatzos_30_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_greater_than_30() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_gedola_greater_than_30();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_72_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_72_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_ahavat_shalom() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_ahavat_shalom();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_ateret_torah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_ateret_torah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_baal_hatanya() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_baal_hatanya();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_gra_fixed_local_chatzos_to_sunset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_mincha_ketana_gra_fixed_local_chatzos_to_sunset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_misheyakir_10_point_2_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_misheyakir_10_point_2_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_misheyakir_11_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_misheyakir_11_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_misheyakir_11_point_5_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_misheyakir_11_point_5_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_misheyakir_7_point_65_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_misheyakir_7_point_65_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_misheyakir_9_point_5_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_misheyakir_9_point_5_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_ahavat_shalom() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_ahavat_shalom();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_alos_16_point_1_to_tzais_geonim_7_point_083_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_alos_16_point_1_to_tzais_geonim_7_point_083_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_alos_to_sunset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_alos_to_sunset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_120_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_120_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_120_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_120_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_18_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_18_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_19_point_8_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_19_point_8_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_26_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_26_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_60_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_60_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_72_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_72_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_72_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_72_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_90_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_90_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_90_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_90_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_96_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_96_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_96_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_96_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_ateret_torah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_ateret_torah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_baal_hatanya() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_baal_hatanya();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_gra_fixed_local_chatzos_to_sunset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_plag_hamincha_gra_fixed_local_chatzos_to_sunset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_72_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_72_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_gra() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_samuch_le_mincha_ketana_gra();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_120_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_120_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_120_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_120_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_18_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_18_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_19_point_8_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_19_point_8_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_26_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_26_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_60_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_60_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_72_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_72_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_72_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_72_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_90_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_90_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_90_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_90_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_96_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_96_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_96_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_96_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_7() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_7();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_8() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_8();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_ateret_torah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_ateret_torah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_baal_hatanya() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_shaah_zmanis_baal_hatanya();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_baal_hatanya() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_baal_hatanya();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_gra() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_gra();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_72_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_72_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_72_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_achilas_chametz_mga_72_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_biur_chametz_baal_hatanya() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_biur_chametz_baal_hatanya();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_biur_chametz_gra() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_biur_chametz_gra();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_72_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_72_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_72_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_biur_chametz_mga_72_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_kidush_levana_15_days() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_kidush_levana_15_days();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_kidush_levana_15_days_default() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_kidush_levana_15_days_default();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_kidush_levana_between_moldos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_kidush_levana_between_moldos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_kidush_levana_between_moldos_default() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_kidush_levana_between_moldos_default();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_3_hours_before_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_3_hours_before_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_alos_16_point_1_to_sunset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_alos_16_point_1_to_sunset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_alos_16_point_1_to_tzais_geonim_7_point_083_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_alos_16_point_1_to_tzais_geonim_7_point_083_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_ateret_torah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_ateret_torah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_baal_hatanya() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_baal_hatanya();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_fixed_local() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_fixed_local();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_gra_sunrise_to_fixed_local_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_gra_sunrise_to_fixed_local_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_kol_eliyahu() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_kol_eliyahu();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_120_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_120_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_16_point_1_degrees_to_fixed_local_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_16_point_1_degrees_to_fixed_local_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_18_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_18_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_18_degrees_to_fixed_local_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_18_degrees_to_fixed_local_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_19_point_8_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_19_point_8_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes_to_fixed_local_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes_to_fixed_local_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_72_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes_to_fixed_local_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes_to_fixed_local_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_90_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_96_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_96_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_96_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_shma_mga_96_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_2_hours_before_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_2_hours_before_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_ateret_torah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_ateret_torah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_baal_hatanya() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_baal_hatanya();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_fixed_local() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_fixed_local();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_gra_sunrise_to_fixed_local_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_gra_sunrise_to_fixed_local_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_120_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_120_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_18_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_18_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_19_point_8_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_19_point_8_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_72_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_72_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_72_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_72_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_90_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_90_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_90_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_90_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_96_minutes() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_96_minutes();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_96_minutes_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfila_mga_96_minutes_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfilah_ateret_torah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_sof_zman_tfilah_ateret_torah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_3_days() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_3_days();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_3_days_with_times() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_3_days_with_times();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_7_days() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_7_days();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_7_days_default() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tchilas_zman_kidush_levana_7_days_default();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_120() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_120();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_120_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_120_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_16_point_1_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_16_point_1_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_18_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_18_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_19_point_8_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_19_point_8_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_26_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_26_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_50() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_50();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_60() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_60();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_72_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_72_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_90() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_90();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_90_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_90_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_96() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_96();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_96_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_96_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_ateret_torah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_ateret_torah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_baal_hatanya() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_baal_hatanya();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_3_point_65_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_3_point_65_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_3_point_676_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_3_point_676_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_3_point_7_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_3_point_7_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_3_point_8_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_3_point_8_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_4_point_37_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_4_point_37_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_4_point_61_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_4_point_61_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_4_point_8_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_4_point_8_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_5_point_88_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_5_point_88_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_5_point_95_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_5_point_95_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_6_point_45_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_6_point_45_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_7_point_083_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_7_point_083_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_7_point_67_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_7_point_67_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_8_point_5_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_8_point_5_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_9_point_3_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_9_point_3_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_9_point_75_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_tzais_geonim_9_point_75_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_use_astronomical_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_use_astronomical_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_use_astronomical_chatzos_for_other_zmanim() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_use_astronomical_chatzos_for_other_zmanim();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_zman_molad() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_complexzmanimcalendar_get_zman_molad();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_geolocation_geodesic_distance() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_geolocation_geodesic_distance();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_geolocation_geodesic_final_bearing() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_geolocation_geodesic_final_bearing();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_geolocation_geodesic_initial_bearing() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_geolocation_geodesic_initial_bearing();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_geolocation_get_elevation() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_geolocation_get_elevation();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_geolocation_get_latitude() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_geolocation_get_latitude();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_geolocation_get_longitude() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_geolocation_get_longitude();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_geolocation_rhumb_line_bearing() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_geolocation_rhumb_line_bearing();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_geolocation_rhumb_line_distance() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_geolocation_rhumb_line_distance();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_geolocation_vincenty_inverse_formula() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_geolocation_vincenty_inverse_formula();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_bavli_daf_yomi() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_bavli_daf_yomi();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_day_of_chanukah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_day_of_chanukah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_day_of_omer() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_day_of_omer();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_in_israel() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_in_israel();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_jewish_date() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_jewish_date();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_parshah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_parshah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_use_modern_holidays() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_use_modern_holidays();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_yom_tov_index() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_get_yom_tov_index();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_has_candle_lighting() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_has_candle_lighting();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_aseres_yemei_teshuva() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_aseres_yemei_teshuva();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_assur_bemelacha() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_assur_bemelacha();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_chanukah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_chanukah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_chol_hamoed() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_chol_hamoed();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_chol_hamoed_pesach() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_chol_hamoed_pesach();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_chol_hamoed_succos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_chol_hamoed_succos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_erev_yom_tov() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_erev_yom_tov();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_erev_yom_tov_sheni() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_erev_yom_tov_sheni();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_hoshana_rabba() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_hoshana_rabba();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_isru_chag() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_isru_chag();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_pesach() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_pesach();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_purim() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_purim();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_rosh_chodesh() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_rosh_chodesh();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_rosh_hashana() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_rosh_hashana();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_shavuos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_shavuos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_shemini_atzeres() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_shemini_atzeres();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_simchas_torah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_simchas_torah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_succos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_succos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_taanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_taanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_taanis_bechoros() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_taanis_bechoros();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_tisha_beav() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_tisha_beav();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_tomorrow_shabbos_or_yom_tov() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_tomorrow_shabbos_or_yom_tov();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_yom_kippur() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_yom_kippur();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_yom_tov() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_yom_tov();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_yom_tov_assur_bemelacha() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishcalendar_is_yom_tov_assur_bemelacha();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_chalakim_since_molad_tohu() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_chalakim_since_molad_tohu();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_cheshvan_kislev_kviah() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_cheshvan_kislev_kviah();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_day_of_week() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_day_of_week();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_days_in_jewish_month() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_days_in_jewish_month();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_days_in_jewish_year() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_days_in_jewish_year();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_days_since_start_of_jewish_year() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_days_since_start_of_jewish_year();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_gregorian_day_of_month() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_gregorian_day_of_month();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_gregorian_month() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_gregorian_month();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_gregorian_year() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_gregorian_year();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_jewish_day_of_month() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_jewish_day_of_month();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_jewish_month() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_jewish_month();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_jewish_year() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_jewish_year();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_molad_data() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_molad_data();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_molad_date() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_get_molad_date();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_is_cheshvan_long() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_is_cheshvan_long();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_is_jewish_leap_year() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_is_jewish_leap_year();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_jewishdate_is_kislev_short() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_jewishdate_is_kislev_short();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_solar_azimuth() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_solar_azimuth();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_solar_elevation() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_solar_elevation();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_utc_midnight() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_utc_midnight();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_utc_noon() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_utc_noon();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_utc_sunrise() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_utc_sunrise();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_utc_sunset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_noaacalculator_get_utc_sunset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_mincha_gedola() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_mincha_gedola();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_mincha_ketana() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_mincha_ketana();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_plag_hamincha() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_plag_hamincha();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_samuch_le_mincha_ketana() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_samuch_le_mincha_ketana();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_sof_zman_shma() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_sof_zman_shma();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_sof_zman_tfila() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar__get_sof_zman_tfila();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_alos72() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_alos72();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_alos_hashachar() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_alos_hashachar();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_astronomical_calendar() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_astronomical_calendar();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_candle_lighting() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_candle_lighting();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_candle_lighting_offset() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_candle_lighting_offset();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_chatzos_as_half_day() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_chatzos_as_half_day();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_half_day_based_shaah_zmanis() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_half_day_based_shaah_zmanis();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_half_day_based_zman() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_half_day_based_zman();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_mincha_gedola_default() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_mincha_gedola_default();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_mincha_gedola_simple() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_mincha_gedola_simple();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_mincha_ketana_default() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_mincha_ketana_default();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_mincha_ketana_simple() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_mincha_ketana_simple();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_percent_of_shaah_zmanis_from_degrees() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_percent_of_shaah_zmanis_from_degrees();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_plag_hamincha_default() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_plag_hamincha_default();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_plag_hamincha_simple() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_plag_hamincha_simple();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_samuch_le_mincha_ketana_simple() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_samuch_le_mincha_ketana_simple();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_shaah_zmanis_based_zman() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_shaah_zmanis_based_zman();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_shaah_zmanis_gra() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_shaah_zmanis_gra();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_shaah_zmanis_mga() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_shaah_zmanis_mga();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_shma_gra() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_shma_gra();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_shma_mga() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_shma_mga();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_shma_simple() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_shma_simple();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_tfila_gra() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_tfila_gra();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_tfila_mga() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_tfila_mga();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_tfila_simple() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_sof_zman_tfila_simple();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_tzais() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_tzais();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_tzais72() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_tzais72();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_use_astronomical_chatzos() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_use_astronomical_chatzos();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_use_astronomical_chatzos_for_other_zmanim() {
    const ret = wasm.ubrn_uniffi_zmanim_core_checksum_method_zmanimcalendar_get_use_astronomical_chatzos_for_other_zmanim();
    return ret;
}

/**
 * @returns {number}
 */
export function ubrn_ffi_zmanim_core_uniffi_contract_version() {
    const ret = wasm.ubrn_ffi_zmanim_core_uniffi_contract_version();
    return ret >>> 0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

const RustCallStatusFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rustcallstatus_free(ptr >>> 0, 1));

export class RustCallStatus {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RustCallStatusFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rustcallstatus_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get code() {
        const ret = wasm.__wbg_get_rustcallstatus_code(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set code(arg0) {
        wasm.__wbg_set_rustcallstatus_code(this.__wbg_ptr, arg0);
    }
    constructor() {
        const ret = wasm.rustcallstatus_new();
        this.__wbg_ptr = ret >>> 0;
        RustCallStatusFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Uint8Array | undefined}
     */
    get errorBuf() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.rustcallstatus_error_buf(ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @param {Uint8Array | null} [bytes]
     */
    set errorBuf(bytes) {
        var ptr0 = isLikeNone(bytes) ? 0 : passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.rustcallstatus_set_error_buf(this.__wbg_ptr, ptr0, len0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_0;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }


    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
