[![justforfunnoreally.dev badge](https://img.shields.io/badge/justforfunnoreally-dev-9ff)](https://justforfunnoreally.dev)

# Moon Time

What time is it where CADRE will land?

Uses - [rust-spice](https://github.com/GregoireHENRY/rust-spice)
And - [NAIF giant shoulders](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/req/spk.html#If%20you're%20in%20a%20hurry)
See - [https://api.jodavaho.io/s/readme](https://api.jodavaho.io/s/readme)

Try it out:

```
curl https://api.jodavaho.io/s/cadre/solartime
```

New: Earth's position in different reference frames:

```
curl https://api.jodavaho.io/s/sun/earth            # Full: x,y,z,r,lon,lat
curl https://api.jodavaho.io/s/sun/earth/xyz        # Cartesian only
curl https://api.jodavaho.io/s/sun/earth/spherical  # Spherical only: r,lon,lat

curl https://api.jodavaho.io/s/ecliptic/earth           # Full: x,y,z,r,lon,lat
curl https://api.jodavaho.io/s/ecliptic/earth/xyz       # Cartesian only
curl https://api.jodavaho.io/s/ecliptic/earth/spherical # Spherical only: r,lon,lat
```

All endpoints support query parameters: t (time), f (format: json/txt), u (units: degrees/radians)

And for a complete list:

```
curl https://api.jodavaho.io/s/readme
```

# Building

The endpoint won't build with just Cargo, due to the legendary `openssl` problem on aws lambda.

But there's a quick fix.

Download [spice](https://naif.jpl.nasa.gov/naif/toolkit.html) C toolkit.

build as

`CSPICE_DIR=~/blah cargo build`

# Running

This being built on SPICE, you'll need the most up to date datasets on solar system emphemeris. The list of kernels to find is in `src/main.rs`, and you can get them via the naif website above. They should be placed in `data/`, and are not included here because they are *massive*.
