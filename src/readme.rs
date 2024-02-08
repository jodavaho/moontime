pub const README:&str="https://api.jodavaho.io/s   

    Some SPICE web services related to my favorite space missions.

§ Endpoints:

    /et - returns the ephemeris time for the current time or a time specified in the t parameter.

        OUPUT example: '553333629.1837274'

        QUERY | BODY PARAMETERS:
        * t = optional time. See Input Parameter Information for more information.
        * f = optional format of the response. See Output Parameter Information for more information.

    /cadre/solartime - returns the solar time at present, given CADRE's location. Currently, the
    location is notional. It'll be updated later.

        OUPUT example: '02:48 AM'

        QUERY | BODY PARAMETERS:
        * t = optional time. See Input Parameter Information for more information.
        * f = optional format of the response. See Output Parameter Information for more information.

    /cadre/sun/* - returns pointing information to the sun, where '*' is a return type. 

        Currently, only 'azel' is supported.

        OUPUT example: 147250710.53859484 1.604846196990565 0.0788116644999063

        QUERY PARAMETERS:
        * t = optional time. See Input Parameter Information for more information.
        * f = optional format of the response. See Output Parameter Information for more information.
        
        BODY PARAMETERS:
        * t = optional time. See Input Parameter Information for more information.
        * f = optional format of the response. See Output Parameter Information for more information.
        * u = optional 'units' specification. See Output Parameter Information for more information.

    /moon/solartime - returns the solar time at present, given a position on the moon's surface.

        OUPUT example: '02:48 AM'

        QUERY | BODY PARAMETERS:
        * pos = required position. See Input Parameter Information for more information.
        * t = optional time. See Input Parameter Information for more information.
        * f = optional format of the response. See Output Parameter Information for more information.

    /moon/sun/* - returns pointing information to the sun, where '*' is a return type.
    
        Currently, only 'azel' is supported.

        OUPUT example: 147250710.53859484 1.604846196990565 0.0788116644999063

        QUERY PARAMETERS:
        * pos = required position. See Input Parameter Information for more information.
        * t = optional time. See Input Parameter Information for more information.
        * f = optional format of the response. See Output Parameter Information for more information.
        
        BODY PARAMETERS:
        * pos = required position. See Input Parameter Information for more information.
        * t = optional time. See Input Parameter Information for more information.
        * f = optional format of the response. See Output Parameter Information for more information.
        * u = optional 'units' specification. See Output Parameter Information for more information.

    NOTE - all endpoints support methods GET and POST. 

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

        * pos = { \"lat\":double, \"lon\":double, \"alt\":double, \"units\": <units specifier> } 
          <units specifier> = [\"degrees\" | \"radians\" ]

§ Output Parameter Information:

        * f = ['json'| None] is the format of the response. json may return extra information. If
          not specified, the response is a string representing just the most important payload.

        * u = ['radians'|'degrees'| None] is the units of the response. If not specified, the
          response is in degrees.

§ SEE ALSO:

    * https://naif.jpl.nasa.gov/naif/webgeocalc.html
    * https://ssd.jpl.nasa.gov/horizons/

    We dedicate these hours to the advancement of understanding. We thank humanity for this
    opportunity. May our children find use of our work.
";
