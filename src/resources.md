# Summary

## Raw Materials
These are what is most commonly found. Some exception exist, like stars, and Jovians which have large quantities of Fusibles instead of water, or some asteroids which have Structural and Technological Metal already refined. 1 Unit used here is equal to 100 kg of material.

- Water
- Biomass
- Structural Ore
- Technological Ore
- Radioactive Ore

## Refined Material
While some of these can be found naturally, most require some amount of refining or purification to get them out of their source 'ores'.

- Fusibles - Matter which has little use beyond being fused for energy.
- Organic Matter - Light materials and non-metal elements.
- Structural Metal - Iron and similar Metals.
- Technological Metal - Gold, Silver, and similar highly conductive metals which can be used for particular ends.
- Fissiles - Refined Radioactive material which can be used as reaction material and produce energy.

## Intermediate Products
Various secondary goods which are created and used to produce other things. These have no direct use in and of themselves.

- ## Intermediate Parts
  - Structural Components
    - Strucutral Parts, designed to shield from the environment or bear a load.
    - 1 Structural Metal -> 1 Structural Component
  - Mechanical Components
    - Gears, Screws, Driveshafts, and similar parts which are used to transfer mechanical power.
    - 1 Structural Metal -> 1 Mechanical Components
  - Oil
    - Liquified and highly energetic organic matter.
    - 1 Organic Matter -> 1 Oil
  - Plastic
    - Material made out of various carbon chains and the like.
    - 1 Oil -> 1 Plastic
  - Lubricants
    - Slippery substances used to keep machines cool while working.
    - 1 Oil -> 1 Lubricant
  - Carbon Block
    - Refined Carbon, used for carbon based materials.
    - 1 Organic Matter -> 1 Carbon Block
  - Diamonds
    - Compressed and Annealed Carbon. Very strong.
    - 1 Carbon Block -> 1 Diamond
  - Nanotubes
    - An advanced form of Carbon, forming molecular threads of high strength.
    - 1 Carbon Block -> 1 Nanotubes
  - Wires
    - Common Electrical wires, typically made of copper and other materials.
    - 1 Technological Metal -> 1 Wires
  - Motor
    - An Electric Motor, capable of moving things around through electrical power.
    - 1 Wires + 1 Mechanical Component + 1 Lubricant -> 3 Electric Motor
  - Technical Components
    - Various parts and components used in electrical and computational applications.
    - 1 Technological Metal -> 1 Technical Components
  - Basic Processor
    - A simple General Purpose Computation Unit, turing completed, but otherwise entirely inert.
    - 1 Wires + 1 Plastic + 1 Technical Components -> 3 Basic Processors
  - Personality Core Components
    - A highly advanced, parallel processing core, capable of being animated by your personality, or 
    - 1 Basic Processors + 5 Wires + 4 Techincal Components -> 10 Personality Core Components
  - Electromagnets
    - Electromagnetic Components which have various purposes.
    - Structural Metal + Technical Metal -> Electromagnets
  - Fusion Reactor Components (10000 kg for 1 reactor)
    - A complex and powerful source of energy which can be fed by Fusibles, which are among the most abundant resource in the Galaxy.
    - Electromagnets + Wires + Structural Components + Basic Processor + Technical Components -> Fusion Reactor Components
  - Steam Turbine Parts
    - Parts for power generator which absorbs energy from hot steam to produce electricity.
  -  Mechanical Components + Structural Component + Motor -> Steam Turbine Parts
  - Fission Reactor Parts
    - Parts for a Fission Reactor.
    - Basic Processors + Structural Components + Technical Components + Wires + Mechanical Components -> Fission Reactor Parts
  - Photovoltaic Cell
    - A component which absorbs photons and converts it into electricity for use.
    - Wires + Structural Components + Technical Components -> Photovoltaic Cells
  - Mirror Panelling
    - Refined to a mirror finish, capable of reflecting most light.
    - Structural Components -> Mirror Panelling
  - Electrical Heater
    - Powerful Resistive Heaters capable of melting metals.
    - Technical components + Wires + Structural Components -> Electrical Heater
- ## End Products and Buildings
  - Building Prefab
    - Generic building, used for everything else.
    - 10 Structural Components -> 1 Building Prefab
  - Warehouse (Component)
    - Turns a Building Prefab into an empty warehouse, can hold N * 100 * 1.1^N goods for each Warehouse component attached to the building.
    - 1 Building Prefab -> 1 Warehouse
  - Fusion Reactior (Component, Generator)
    - High efficiency, with cheap and plentiful fuel. It's cost comes in it's inherestly large size, low scaling efficiency, and highly complex construction.
    - 10 Building Prefab + X Fusion Reactor Components -> Fusion Reactor
  - Fission Reactor (Component, Generator)
    - Moderate Efficiency, fuel is relatively expensive and rare, but highly efficient. It's construction is also fairly simple, a containing structure and small control mechanism.
    - Steam Turbine Parts + Structural Components + Wires + Fission Reactor Parts + 1 Building  -> Fission Reactor
  - Steam Turbine (Component, Generator)
    - Low Efficiency, but capable of burning Organic Matter and Biomatter for energy.
    - Steam Turbine + Structural Components + Wires + Building Prefab -> Steam Engine
  - Light Collectors (Component, Solo, Surface Only)
    - Light which hits it is absorbed. A portion dissipates into heat, the rest is converted into Energy. It also imparts a change in momentum to the object.
    - Photovoltaic Cells + Structural Components + Basic Processor + Technical Component -> Light Collectors
  - Mirror (Component, solo, Surface Only, Minimal Power)
    - Designed to recieve and reflect light. Gets double the momentum change of the Photoelectric Cell, and has a lower heat loss coefficient.
    - Mirror Panelling + Structural Components + Motors -> Mirror
  - Heat Sink (Component, Solo, Surface Only)
    - A part of a building or ship which increases heat dissapation for the body.
    - Wires + Structural Components => Heat Sink
  - Cooling Tower (Component, Solo, Surface Only, Power Required)
    - A more proactive heat sink, it expends some energy (creating more thermal energy) to move an even larger quantity of thermal energy into it's heat sink, allowing it to dissapate energy more efficiently.
    - Wires + Lubricant + Basic Motor + Strucutral Components -> Cooling Tower
  - 3-D Printer (Component, Factory Component, Exclusive with Other Factory Campnoents, )
    - The most flexible form of assembly, it uses lots of energy and raw resource to produce final products directly. Always more expensive in terms of time and energy, but highly flexible.
    - Wires + Basic Motors + Mechanical Components + Technical Components + Basic Processor -> 3D printer
  - Assembler (Compnoent, Factory Component)
    - Capable of putting together various components and pieces, it is generally used for final assembly.
    - Wires + Basic Motors + Mechanical Components + Basic Processor -> Assembler
  - Factory (Component, Factory Component)
    - Takes in material components and reforms them into new forms.
    - Electric Heater + Structural Components + Wires + Basic Motors + Mechanical Components + Basic Processor + Building Prefab -> Factory
  - Oil Refinery (Component, Factory Component)
    - Specialized in oil and organic products. It is capable fo breaking them down and reforming complex organic molecules into more useful forms.
    - Electric Heater + Wires + Structural Components + Building Prefab + Basic Processor + Mechanical Components + Technical Components -> Oil Refinery
  - Electrolyser (Component, Factory Component)
    - Refines water into Fusible material.
    - Wires + Structural Components + Technical Components + Mechanical Components + Building Prefab -> Electrolyser
  - Macerator
  - 
  - Mill
  - Extractor
  - Enricher
  - Worker Hub
  - Communication Hub
  - Research Hub
  - Mass Driver
  - Star Port
- ## Megastructures
  - Space Elevator
  - Lofstram Loop
  - Orbital Ring
  - Superdrivers
  - 


Components marked Solo cannot be combined with other buildings.


```mermaid
flowchart TD;
%% Raw materials
Water{{Water}}
RadioOre{{Radioactive Ores}}
Biomass{{Biomass}}
StructOre{{Structural Ores}}
TechOre{{Technological Ores}}

%% Refined Material
Water -- Electrolysis --> Fusibles(Fusibles)
RadioOre -- Enrichment --> Fissiles(Fissiles)

%% Intermediate Products
Fusibles --> Fusion{Fusion}
Fusion --> OrgMat
Fusion --> Energy

StructOre -- Refining --> StructMet(Structural Metals)
TechOre -- Extraction --> TechMet(Technological Metals)
Biomass -- Maceration --> OrgMat(Organic Material)

StructMet -- Casting --> Structure([Structural Components])
StructMet -- Casting --> MechComp([Mechanical Component])
StructMet -- Casting --> Plating([Structural Plating])
StructMet -- Extrusion --> Cabling([Structural Cables])

OrgMat -- Plastification --> Plastic([Plastics])
OrgMat -- Liquifaction --> Lubricants([Lubricants])
OrgMat -- Plastification --> Adhesives([Adhesives])

TechMet -- Extrusion --> Wires([Electrical Wires])
TechMet -- Casting --> TechComponents([Technical Components])
TechMet -- Casting --> Magnets([Magnets])

Plastic -- Forming --> Boards([Electrical Boards])
Plastic -- Conditioning --> Rubber([Rubber])

Structure --> Construction{Construction}
Plating --> Construction
Construction --> Building([Building Prefab])

MechComp --> Fabrication{Fabrication}
Structure --> Fabrication
Fabrication --> Machines([Machines])

%% End Products
Energy[Energy]

Building --> Storage[Storage - 10 units]

%% Processes

Fissiles --> Fission{Fission}

StructMet --> Composite{Composite}
Plastic --> Composite
Composite --> lightalloys([Light Alloys])

StructMet --> ResAlloy{Alloy}
TechMet --> ResAlloy
ResAlloy --> ResMet([Resistant Alloys])

StructMet --> StrAlloy{Alloying}
OrgMat --> StrAlloy
StrAlloy --> StrMet([Reinforced Alloys])


Fission --> TechOre
Fission --> Energy
```