# Testing of loading of dynamic libraries 

The relations between libraries: `testkit_binding` depends on the `testkit` lib (statically linked). `testkit` depends on `exonum`(dynamic lib). `exonum_binding` depends on `exonum` (dynamic). Java app depends on `testlit_binding` and `exonum_binding`.
 
 Use the `build.sh` script in order to build and run everything.