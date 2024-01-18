# Method

This calculates sun and earth position in a topocentric frame from the center of Gale crater, Luna. 

It uses CSPICE (rust binding), and current time from NTP, converted to ephemeris time using, again, CSPICE.

The sun/moon orientations and positions are specified in *aberation-corrected* observer reference frames from a position at the center of gale crater, Luna. Calculations use binary kernels provided by NASA JPL (which are pulled monthly). The comments provided by JPL can be fetched using the API below. 

Geometric events are calculating using .. and include:

- time to sun/earth center visibility from topocentric frame
- time to sun/earth perimeter visibility from topocentric frame
- az/el to earth/sun center from topocentric frame


# BPC files

come from https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/

using high-precision where possible. 

Comment files are available as well. 
