# P0

- Tests that ensure that LightClone is failing compile time for bad types
- Use in a real project to measure ergonomics
- Change name to LightClone with main entry being light_clone, lc still okay for shorthand if ppl want it.
- ~~Add compatibility for additional immutable crates like imbl and rpds~~ DONE - im, imbl, and rpds all supported with feature flags

# P1

- ~~Improve benchmarking to cover more cases~~ DONE - Added collection/map clone and clone_then_mutate benchmarks comparing std, im, imbl, rpds

# P2
