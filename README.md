# Apprentice Roguelike
[Play Online](https://www.apprentice-game.com)

## Dev

### Dev for local
`cargo run --release`

### Dev for wasm
`npm run serve`

## Build

### Build for local
`cargo build`

### Build for wasm
`npm run build`


## Feature Roadmap
This roguelike is being developed using [this tutorial](http://bfnightly.bracketproductions.com/rustbook/chapter_0.html) as a starting point.

[] Add Doors
[] Remove Orcs
[] Furnish Rooms
[] update Goblin AI - Utility based AI system
[] enemies can follow around corners, search for target
[] Goblins can spawn in parties of 1 to 4
[] player can hide in crates, barrels, armoires
[] add sneaking state
[] add prone state
[] add duergar enemies
    - fight goblins on sight
    - may fight player on sight
    - spawn in groups of 1 - 4
[] player can speak with duergar or goblin entities
    - perhaps resulting in the entities not attacking the player
    - perhaps resulting in the entities attacking the player
    - perhaps resulting in the entities helping the player
        - with items
        - with information
        - with protection
        - by following the player
        