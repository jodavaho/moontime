# Moon Time

What time is it where CADRE will land?

Uses - [rust-spice](https://github.com/GregoireHENRY/rust-spice)
And - [NAIF giant shoulders](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/req/spk.html#If%20you're%20in%20a%20hurry)
See - [https://api.jodavaho.io/s/readme](https://api.jodavaho.io/s/readme)

Try it out: 

```
curl https://api.jodavaho.io/s/cadre/solartime
```

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


