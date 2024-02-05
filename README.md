Moon Time

What time is it where CADRE will land?

Uses - [rust-spice](https://github.com/GregoireHENRY/rust-spice)
And - [NAIF giant shoulders](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/req/spk.html#If%20you're%20in%20a%20hurry)

API:

- `api.jodavaho.io/moontime/` , get the current time, as hours from midnight
- `api.jodavaho.io/moontime/sunrise` , get the time until sunrise in `earth-days hours:minutes:seconds`
- `api.jodavaho.io/moontime/sunset` , get the time until sunset in `earth-days hours:minutes:seconds`
- `api.jodavaho.io/moontime/X/json`, get whatever `X` woudl send, but in `json` format

TODO:

- [ ] implement "see sky" instrument kernel
- [ ] implement "where is cadre" e.g., what crater / location
- [ ] AWS / jodavaho endpoint
- [ ] include kernels in binary using `include_dir` crate
- [ ] get the heck off `lambda_warp` and just use `lambda_runtime`
