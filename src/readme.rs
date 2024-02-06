
pub const README:&str="

https://api.jodavaho.io/s

    Some SPICE web services related to my favorite space missions.

    -----------------------------------------------------------------------------------------------

    Endpoints:

    /et - returns the ephemeris time for the current time or a time specified in the t parameter.

        OUPUT example: '553333629.1837274'

        QUERY | BODY PARAMETERS:
        * t = optional time. See Input Parameter Information for more information.
        * f = format of the response. See Output Parameter Information for more information.

    /cadre/solartime - returns the solar time at present, given CADRE's location. Currently, the
    location is notional. It'll be updated later.

        OUPUT example: '02:48 AM'

        QUERY | BODY PARAMETERS:
        * t = optional time. See Input Parameter Information for more information.
        * f = format of the response. See Output Parameter Information for more information.

    /cadre/sun/* - returns pointing information to the sun, where '*' is a return type. 

        Currently, only 'azel' is supported.

        OUPUT example: 147250710.53859484 1.604846196990565 0.0788116644999063

        QUERY PARAMETERS:
        * t = optional time. See Input Parameter Information for more information.
        * f = format of the response. See Output Parameter Information for more information.
        
        BODY PARAMETERS:
        * t = optional time. See Input Parameter Information for more information.
        * f = format of the response. See Output Parameter Information for more information.
        * pos = optional position. See Input Parameter Information for more information.

    -----------------------------------------------------------------------------------------------

    NOTE - all parameters are optional, and all endpoints support methods GET and POST. 

        curl 'https://api.jodavaho.io/s/et?t=2021-10-01T12%3A00%3A00.00%2B00%3A00'
        curl -X POST 'https://api.jodavaho.io/s/et' -d '{{\"t\":\"2021-10-01T12:00:00.00+00:00\"}}'
        both return '686361669.1823467'

    -----------------------------------------------------------------------------------------------

    Input Parameter Information:

        * t = [ <rfc3339> | None] 
          if t is not specified, the current time is used.
          Please use RFC3339 format e.g., '2021-10-01T12:00:00.00+00:00' is valid. 

        * pos = [ { \"lat\":double, \"lon\":double, \"alt\":double } | None]
          if pos is not specified, the position of the CADRE spacecraft is used. 

    -----------------------------------------------------------------------------------------------

    Output Parameter Information:

        * f = ['json'| None] is the format of the response. json may return extra information. If
          not specified, the response is a string representing just the most important payload.

    ----------------------------------------------------------------------------------------------

    We dedicate these hours to the advancement of understanding. We thank humanity for this
    opportunity. May our children find use of our work.
";
