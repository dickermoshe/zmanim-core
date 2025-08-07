/**************************************************************************/
/*!
  @file Adafruit_GPS.cpp

  @mainpage Adafruit Ultimate GPS Breakout

  @section intro Introduction

  This is the Adafruit GPS library - the ultimate GPS library
  for the ultimate GPS module!

  Tested and works great with the Adafruit Ultimate GPS module
  using MTK33x9 chipset
  ------> http://www.adafruit.com/products/746

  Adafruit invests time and resources providing this open source code,
  please support Adafruit and open-source hardware by purchasing
  products from Adafruit!

  @section author Author

  Written by Limor Fried/Ladyada for Adafruit Industries.

  @section license License

  BSD license, check license.txt for more information
  All text above must be included in any redistribution
*/
/**************************************************************************/

#include <Adafruit_GPS.h>

static bool strStartsWith(const char *str, const char *prefix);

/**************************************************************************/
/*!
    @brief Start the HW or SW serial port
    @param baud_or_i2caddr Baud rate if using serial, I2C address if using I2C
    @returns True on successful hardware init, False on failure
*/
/**************************************************************************/
bool Adafruit_GPS::begin(uint32_t baud_or_i2caddr)
{
    gpsHwSerial->begin(baud_or_i2caddr);
    delay(10);
    return true;
}

/**************************************************************************/
/*!
    @brief Constructor when using HardwareSerial
    @param ser Pointer to a HardwareSerial object
*/
/**************************************************************************/
Adafruit_GPS::Adafruit_GPS(HardwareSerial *ser)
{
    common_init();     // Set everything to common state, then...
    gpsHwSerial = ser; // ...override gpsHwSerial with value passed.
}

/**************************************************************************/
/*!
    @brief Initialization code used by all constructor types
*/
/**************************************************************************/
void Adafruit_GPS::common_init(void)
{
    gpsHwSerial = NULL; // port pointer in corresponding constructor
    recvdflag = false;
    paused = false;
    lineidx = 0;
    currentline = line1;
    lastline = line2;

    hour = minute = seconds = year = month = day = fixquality = fixquality_3d =
        satellites = antenna = 0; // uint8_t
    lat = lon = mag = 0;          // char
    fix = false;                  // bool
    milliseconds = 0;             // uint16_t
    latitude = longitude = geoidheight = altitude = speed = angle = magvariation =
        HDOP = VDOP = PDOP = 0.0; // nmea_float_t
}

/**************************************************************************/
/*!
    @brief How many bytes are available to read - part of 'Print'-class
   functionality
    @return Bytes available, 0 if none
*/
/**************************************************************************/
size_t Adafruit_GPS::available(void)
{
    if (paused)
        return 0;

    return gpsHwSerial->available();
}

/**************************************************************************/
/*!
    @brief Write a byte to the underlying transport - part of 'Print'-class
   functionality
    @param c A single byte to send
    @return Bytes written - 1 on success, 0 on failure
*/
/**************************************************************************/
size_t Adafruit_GPS::write(uint8_t c)
{

    return gpsHwSerial->write(c);
}

/**************************************************************************/
/*!
    @brief Read one character from the GPS device.

    Call very frequently and multiple times per opportunity or the buffer
    may overflow if there are frequent NMEA sentences. An 82 character NMEA
    sentence 10 times per second will require 820 calls per second, and
    once a loop() may not be enough. Check for newNMEAreceived() after at
    least every 10 calls, or you may miss some short sentences.
    @return The character that we received, or 0 if nothing was available
*/
/**************************************************************************/
char Adafruit_GPS::read(void)
{
    static uint32_t firstChar = 0; // first character received in current sentence
    uint32_t tStart = millis();    // as close as we can get to time char was sent
    char c = 0;

    if (paused || noComms)
        return c;

    if (!gpsHwSerial->available())
        return c;
    c = gpsHwSerial->read();

    // Serial.print(c);

    currentline[lineidx] = c;
    lineidx = lineidx + 1;
    if (lineidx >= MAXLINELENGTH)
        lineidx = MAXLINELENGTH -
                  1; // ensure there is someplace to put the next received character

    if (c == '\n')
    {
        currentline[lineidx] = 0;

        if (currentline == line1)
        {
            currentline = line2;
            lastline = line1;
        }
        else
        {
            currentline = line1;
            lastline = line2;
        }

        // Serial.println("----");
        // Serial.println((char *)lastline);
        // Serial.println("----");
        lineidx = 0;
        recvdflag = true;
        recvdTime = millis(); // time we got the end of the string
        sentTime = firstChar;
        firstChar = 0; // there are no characters yet
        return c;      // wait until next character to set time
    }

    if (firstChar == 0)
        firstChar = tStart;
    return c;
}

/**************************************************************************/
/*!
    @brief Send a command to the GPS device
    @param str Pointer to a string holding the command to send
*/
/**************************************************************************/
void Adafruit_GPS::sendCommand(const char *str) { println(str); }

/**************************************************************************/
/*!
    @brief Check to see if a new NMEA line has been received
    @return True if received, false if not
*/
/**************************************************************************/
bool Adafruit_GPS::newNMEAreceived(void) { return recvdflag; }

/**************************************************************************/
/*!
    @brief Pause/unpause receiving new data
    @param p True = pause, false = unpause
*/
/**************************************************************************/
void Adafruit_GPS::pause(bool p) { paused = p; }

/**************************************************************************/
/*!
    @brief Returns the last NMEA line received and unsets the received flag
    @return Pointer to the last line string
*/
/**************************************************************************/
char *Adafruit_GPS::lastNMEA(void)
{
    recvdflag = false;
    return (char *)lastline;
}

/**************************************************************************/
/*!
    @brief Wait for a specified sentence from the device
    @param wait4me Pointer to a string holding the desired response
    @param max How long to wait, default is MAXWAITSENTENCE
    @param usingInterrupts True if using interrupts to read from the GPS
   (default is false)
    @return True if we got what we wanted, false otherwise
*/
/**************************************************************************/
bool Adafruit_GPS::waitForSentence(const char *wait4me, uint8_t max,
                                   bool usingInterrupts)
{
    uint8_t i = 0;
    while (i < max)
    {
        if (!usingInterrupts)
            read();

        if (newNMEAreceived())
        {
            char *nmea = lastNMEA();
            i++;

            if (strStartsWith(nmea, wait4me))
                return true;
        }
    }

    return false;
}

/**************************************************************************/
/*!
    @brief Standby Mode Switches
    @return False if already in standby, true if it entered standby
*/
/**************************************************************************/
bool Adafruit_GPS::standby(void)
{
    if (inStandbyMode)
    {
        return false; // Returns false if already in standby mode, so that you do
                      // not wake it up by sending commands to GPS
    }
    else
    {
        inStandbyMode = true;
        sendCommand(PMTK_STANDBY);
        // return waitForSentence(PMTK_STANDBY_SUCCESS);  // don't seem to be fast
        // enough to catch the message, or something else just is not working
        return true;
    }
}

/**************************************************************************/
/*!
    @brief Wake the sensor up
    @return True if woken up, false if not in standby or failed to wake
*/
/**************************************************************************/
bool Adafruit_GPS::wakeup(void)
{
    if (inStandbyMode)
    {
        inStandbyMode = false;
        sendCommand(""); // send byte to wake it up
        return waitForSentence(PMTK_AWAKE);
    }
    else
    {
        return false; // Returns false if not in standby mode, nothing to wakeup
    }
}

/**************************************************************************/
/*!
    @brief Time in seconds since the last position fix was obtained. The
    time returned is limited to 2^32 milliseconds, which is about 49.7 days.
    It will wrap around to zero if no position fix is received
    for this long.
    @return nmea_float_t value in seconds since last fix.
*/
/**************************************************************************/
nmea_float_t Adafruit_GPS::secondsSinceFix()
{
    return (millis() - lastFix) / 1000.;
}

/**************************************************************************/
/*!
    @brief Time in seconds since the last GPS time was obtained. The time
    returned is limited to 2^32 milliseconds, which is about 49.7 days. It
    will wrap around to zero if no GPS time is received for this long.
    @return nmea_float_t value in seconds since last GPS time.
*/
/**************************************************************************/
nmea_float_t Adafruit_GPS::secondsSinceTime()
{
    return (millis() - lastTime) / 1000.;
}

/**************************************************************************/
/*!
    @brief Time in seconds since the last GPS date was obtained. The time
    returned is limited to 2^32 milliseconds, which is about 49.7 days. It
    will wrap around to zero if no GPS date is received for this long.
    @return nmea_float_t value in seconds since last GPS date.
*/
/**************************************************************************/
nmea_float_t Adafruit_GPS::secondsSinceDate()
{
    return (millis() - lastDate) / 1000.;
}

/**************************************************************************/
/*!
    @brief Fakes time of receipt of a sentence. Use between build() and parse()
    to make the timing look like the sentence arrived from the GPS.
*/
/**************************************************************************/
void Adafruit_GPS::resetSentTime() { sentTime = millis(); }

/**************************************************************************/
/*!
    @brief Checks whether a string starts with a specified prefix
    @param str Pointer to a string
    @param prefix Pointer to the prefix
    @return True if str starts with prefix, false otherwise
*/
/**************************************************************************/
static bool strStartsWith(const char *str, const char *prefix)
{
    while (*prefix)
    {
        if (*prefix++ != *str++)
            return false;
    }
    return true;
}