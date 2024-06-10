# Identifiers

Identifiers between the various parts are designed to intentionally overlap. This should explain the overlaps quickly here.

## Orbital ID

The Orbital Id is used by all of those things which interact with the physics engine. 

Included are:

- Bodies
- Fleets
- Packets?

## Construct ID

Construct IDs are used to consolidate the possibly millions of constructs into like groups. 

Constructs share their IDs with their blueprints. If the ID does not lead to a blueprint, then it's a unique construct.

Because of this, fleets store the 