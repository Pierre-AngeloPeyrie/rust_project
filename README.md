# Granular simulation
Student project for a Rust course.
Simple granular simulation using the [ggez](https://ggez.rs/) library.
It simulates 2D colliding particles subject to gravity using a simple verlet integration. Inspired by [pezzza's work](https://www.youtube.com/watch?v=lS_qeBy3aQI).

## Goal
The goal of the project is to leverage rust's speed and fearless concurrency to display as much colliding particles as possible.

## Progress
- [x] Setup the project with the dependencies
- [x] Display particle
- [x] Apply gravity
- [x] Add border constraints
- [x] Spawn multiple particles
- [x] Add simple collisions to particles
- [x] Implement FPS and particles counter
- [x] Implement spatial partition with an homogenous grid
- [ ] Implement multi-threading on collision solving
- [ ] Solve instability problem
- [ ] Add more control on particle volume spawn
  
  
