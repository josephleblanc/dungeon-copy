# Attack System
## Overview
The attack system is responsible for whether an attack hits or misses the target.
## Background: The d20 Pathfinder Attack System
In Dnd and Pathfinder games, an attack is made by rolling a d20 die, adding any
attacker modifiers from the attacker, and checking if this value is equal to or
more than the defender's armor class (AC) value with its own bonuses. Sounds
relatively simple to implement, right? Well, it would be simple to implement,
if that is all there were to it. However, there are a number of complicating
factors to consider.
1.) Critical threat range.
2.) Critical multiplier.
3.) Defender Armor Class
4.) Highly specific modifiers to critical threat range, critical multiplier,
attack bonus, and armor class.
### 1. Critical Threat Range
Critical threat range is a range from a lower bound up to 20, and defines the
set of values for which an attack *may* be a critical. If the d20 attack roll
is within the critical threat range, and the roll results in a hit, then the
attack is a critical threat. To become a critical, a second attack roll called
a "critical confirmation roll" is made, with all relevant bonuses included,
and if the critical confirmation roll is a success, then the threat becomes a
critical. More information is available on d20pfsrd.
Critical threat range can be altered by some choices in the player's character
build.
### 2. Critical Multiplier
The critical multiplier is the multiple applied to some of the damage on a
critical hit. The base critical multiplier is dependant upon the weapon being
used, but can be altered by some choices in the player's character build.
### 3. Defender Armor Class
The attacker's roll and bonus is compared to the armor class of the defender in
determining whether the attack was a hit or not. As such, it is necessary to
apply an bonuses the defender may have to their armor class before determining
the outcome of the attack.
### 4. Highly Specific Modifiers
The Pathfinder system has many highly specific modifiers. For example, if you
are a Dwarf, and have chosen to take an optional character trait, and are
attacking an orc, you get a +1 to attack. Deciding how to incorporate all of
these highly specific modifiers into a system without it becoming bloated was
the most difficult design decision in building the attack system.
## Designing for Parallel Computing
I wanted to prioritize parallel processes in designing the implementation of the
attack system. Because there are a large number of possible modifiers, each of
which may require different information to know whether or not it is applicable,
it could become expensive to have each possible modifier run sequentially. By
having each possible modifier run in parallel, the time required to run all
modifier checks could remain low as more modifiers are added.
## Implementing the attack using Events, Query, and explicit system ordering
Bevy provides three great tools to allow the attack system to run with as many
parallel processes as possible. 
1.) `Event`s can be used to send the output of one system to another.
2.) `Query` can be used to request the data required for each modifier, and
because each modifier only requires immutable references, multiple modifier
systems can access the same data at the same time.
3.) Explicit system ordering ensures that any system which requires the events
from another system will run after that system, but can also run in parallel
with other systems in the same `SystemSet`.

The mind map below shows how the attack is handled:
[Mindmap of the attack plugin](/readme_mindmaps/attack.pdf)
## Extensible Design
As you can see in the above mindmap, there can be many modifier systems running
in parallel. Because none of the systems blocks the access of another to
immutable data, the cost of adding modifier systems is low. While it could
become expensive to run all the systems at once, each is designed with an `if`
clause to only run its logic if the conditions for it to run are present.
Because there will only ever be a few characters in combat at a time, and each
of these can only be made to trigger a low number of modifiers, the majority of
the modifier systems will only check whether it needs to run, then stop.
Because the `sum_modifier` systems each iterate over the events which are
produced by a triggered modifier, the cost of consolidating the output from the
modifier systems will remain low. Finally, because there is explicit system
ordering to prevent an `Event` from arriving out of order, each system will only
trigger once for each attack.
