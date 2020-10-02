Temperature
===========

Temperature monitoring system
-----------------------------
### Hardware
- Particle Photon
- Adafruit display
- temperature sensor

### Software
- Rust web server
    - Rocket for serving API/`index.html`
    - currently only in-memory (need to add dumps to disk)
- simple `uPlot` frontend
