
# world

## tile
1 Ground
0/1 structure
any number of dynamic entities

# Things in the game

## Dynamic

### Players
can move
can interact


## Static

### Ground
Players can stand on some ground, but not all

### Plants
can be planted
require certain ground for planting
require certain action for state update?
Grow to other stage
can be harvested in certain stage
after harvesting either destroyed or different stage

### Rocks etc
generated rom start
can not be passed
can not be destroyed

### Quarry/wild plants?
generated from start
can be mined from
either indestructable or regenerating

## Items

items do not have state

### Resources
can be acquired
can be lost
quantity matters

### Tools/catalysts
can be acquired
once acquired never lost
at most one per player


## Actions

### move around
depends on floor and structure whether possible

### use an item on current/adjacent square
action firstly depending on item; secondly on square

#### Plant
costs used seed
only when no structure and correct ground

#### Water/boost
costs booster
plant changes state/type

#### Craft ingredient
costs used item
used on crafting station
item gets placed on crafting station

#### Craft finish
using tool on crafting station with items
crafting station loses ingredient items
player gains result item

#### Remove structure
possible to remove structure built by player
player may or may not get item back

### interact without item on current/adjacent square

#### harvest
player gains item
plant changes state

#### grab
player gains item
station loses item

### timed ticks
#### grow plant
change state
can set timer




# Action structure

## Condition
### Hand
### Alignment
### Structure
#### id
#### classes
#### ownstate
### Ground

## Result
### Cost item removed
### Structure changes
### Structure changes state
### Timer set on structure
