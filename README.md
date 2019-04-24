# OS110_MediumExercism
This is my take on the second task of OS110 class


 #List of Medium Problem :
 1. Triangle
 2. Clock
 3. Robot Simulator
 -
 
 #List of Beginner Problem :
 1. Leap
 2. Gigasecond
 3. Hello World
 4. Nth Prime
 5. Reverse String
 6. Raindrop
 7. Beer Song
 
 
## Essay of Robot Simulator Task
 This essay's purpose is to study and research the 'Robot Simulator' Task from exercism Rust Track's.
 
 
### Introduction of the Task

Write a robot simulator.

A robot factory's test facility needs a program to verify robot movements.

The robots have three possible movements:

- turn right
- turn left
- advance

Robots are placed on a hypothetical infinite grid, facing a particular
direction (north, east, south, or west) at a set of {x,y} coordinates,
e.g., {3,8}, with coordinates increasing to the north and east.

The robot then receives a number of instructions, at which point the
testing facility verifies the robot's new position, and in which
direction it is pointing.

- The letter-string "RAALAL" means:
  - Turn right
  - Advance twice
  - Turn left
  - Advance once
  - Turn left yet again
- Say a robot starts at {7, 3} facing north. Then running this stream
  of instructions should leave it at {9, 4} facing west.
