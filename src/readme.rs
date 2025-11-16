pub const README: &str = "https://api.jodavaho.io/s

    Some SPICE web services related to my
    favorite space missions.

§ Endpoints:

    See Input Parameter Information
    and Output Parameter Information
    for details on individual parameter types.

    /et - returns the ephemeris time for the current
        time or a time specified in the t parameter.

        OUPUT example: '553333629.1837274'

        * t = optional time.
        * f = optional format of the response.

    /cadre/solartime - returns the solar time at present,
        given CADRE's (currently hypothesized) location.

        OUPUT example: '02:48 AM'

        * t = optional time.
        * f = optional format of the response.

    /cadre/sun/* - returns pointing information to the sun,
        where '*' is a return type.
        Currently, only 'azel' is supported.

        OUPUT example: 147250710.538 1.6048 0.0788 u=degrees

        * t = optional time.
        * f = optional format of the response.
        * u = optional 'units' specification.

    /moon/solartime - returns the solar time at present,
        given a position on the moon's surface.

        OUPUT example: '02:48 AM'

        * pos = required position.
        * t = optional time.
        * f = optional format of the response.

    /moon/sun/* - returns pointing information to the sun,
        where '*' is a return type.

        Currently, only 'azel' is supported.

        OUPUT example:
        'r: 147250710.538 az: 1.6048 el: 0.0788 u: radians'

        QUERY | BODY PARAMETERS:
        * pos = required position.
        * t = optional time.
        * f = optional format of the response.
        * u = optional 'units' specification.

    /sun/earth - returns Earth's position from Sun's
        rotating reference frame (IAU_SUN). Returns both
        rectangular (x,y,z) and spherical (r,lon,lat) coords.

        OUTPUT example: 'x: 149597870.7 km, y: 0.0 km, z: 0.0 km,
        r: 149597870.7 km, lon: 0.0, lat: 0.0, u: degrees'

        * t = optional time.
        * f = optional format of the response.
        * u = optional 'units' specification for angles.

    /ecliptic/earth - returns Earth's orbital position
        in the solar system's ecliptic plane (ECLIPJ2000).
        Longitude cycles 0-360° over the year.

        OUTPUT example: 'x: 149597870.7 km, y: 0.0 km, z: 0.0 km,
        r: 149597870.7 km, lon: 102.3, lat: 0.0, u: degrees'

        * t = optional time.
        * f = optional format of the response.
        * u = optional 'units' specification for angles.

    /galaxy/earth - returns Earth's position in galactic
        coordinates (GALACTIC frame). Shows orientation
        relative to Milky Way center and north pole.

        OUTPUT example: 'x: 149597870.7 km, y: 0.0 km, z: 0.0 km,
        r: 149597870.7 km, lon: 180.0, lat: -60.0, u: degrees'

        * t = optional time.
        * f = optional format of the response.
        * u = optional 'units' specification for angles.

§ NOTE:

    All endpoints support methods GET and POST, so
    parameters can be specified in the query string
    or in the body

    curl 'https://api.jodavaho.io/s/et?t=2021-10-01T12%3A00%3A00.00%2B00%3A00'

        or

    curl -X POST 'https://api.jodavaho.io/s/et' \\
      -d '{\"t\":\"2021-10-01T12:00:00.00+00:00\"}'\\
      -H 'Content-Type: application/json'

    should both return '686361669.1823467'

§ Input Parameter Information:

    * t = [ <iso8601> | None]
      if t is not specified, the current time is used.
      Please use ISO8601 format e.g. the following are valid:
          * 2021-10-01T12:00:00.00+00:00
          * 2021-10-01T12:00:00.00Z

    * pos = { \"lat\":double, \"lon\":double,
              \"alt\":double, \"units\": <units specifier> }

            <units specifier> = [\"degrees\" | \"radians\" ]

§ Output Parameter Information:

    * f = ['json'| None] is the format of the response.
      json may return extra information. If not specified,
      the response is a string w/ just the most important payload.

    * u = ['radians'|'degrees'| None] is the units of the response.
      If not specified, the response is in degrees.

§ SEE ALSO:

    * https://naif.jpl.nasa.gov/naif/webgeocalc.html
    * https://ssd.jpl.nasa.gov/horizons/

We dedicate these hours to the advancement of understanding.
We thank humanity for this opportunity. May our children find
use of our work.
";
