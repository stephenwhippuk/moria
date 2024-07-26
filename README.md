# moria

This is a full sample text adventure game, written in Rust. 

Moria presents a dungeon that consists of locations from which a player may move n,s,e,w
not all locations offer the ability to move in all directions, others may have doors which may be open or closed and may be locked. Where locked a trigger must be used to unlock them
this trigger may be a lever etc within a location, or it may be attached to an item such as a key

Each Player has a character which is loosely based around a simpliar concept to ADnD though no
real relationship exists, beyond surface level details. 

The level may contain monsters, in the initial version, as its a terminal system, we will avoid real time aspects where typing could be problematic, so combat will be a round based system and monsters remain at fixed locations. 
in combat a player may

attack
use spells and other abilities
flee 

The primary aim of the game is to explore, the levels of the dungeon, level up and loot valuables. However there will also be special levels with big bads appropriate for the level of the character. More specifically each level of the dundeon will have a recommended minimum level that the player will be warned about before entering if its beyond their level. 

Outside the dungeon is the City. However there are shortcuts from deeper levels back to this so the player hasn;t got to backtrack. They may also get gate spells/potions to retrun home. Only when they reach safety is their progress saved. There will also be precious safe points within levels that allow a character to save their progress, often for a cost. 