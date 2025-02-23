This toy, mini ECS is heavilly inspired by:
https://austinmorlan.com/posts/entity_component_system/

But it is not direct translation from C++ to Rust.
Differences include:
- there is not an attempt to pack POD/entieis in order to avoid cache mises, at least at the moment
- signatures concept from morlant is dropped, and HashSet of types is are used for identification of
  systems interested in particular entites

