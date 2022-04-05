# what-to-do
---
## Description
A Rust program that is a type of CLI to-do list for people that have a short attention span and often forget the different tasks that they have left to accomplish. Can give a random idea for something that you can spend your time on, must add the things as you think of them and delete them as you complete them.

---
## Current Features
- Can use 'add', 'delete', and 'print-all' subcommands with clap.
- 'add' asks for the task to append to the list of tasks in the save file.
- 'print-all' reads the save file and outputs the whole list.

---
## To-do
- Must get each line in the file to be listed with a reference number for it.
- Must be able to run the 'delete' subcommand and then choose which index to delete.