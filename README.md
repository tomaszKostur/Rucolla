# Rucolla - Rust Collabotation Tool
## This supposed to be my personal project for rust research
### What it should be?
 It should be collaboration tool, made in Rust, to share a virtual blackboard with other, remote co-workers.
 It's inspired by miro board, and any software for managing tasks mgmnt like jira.
 The idea is to have that both functionalities in one tool.
 Advantages to aim:
 * better sharing, with live view of what other participants are doing right now (unlike jira)
 * better performance, cause miro gets very slow and unresponsive not even to have so much on it
 * Better visually tracking of differences appeared on boards
    - To consider is to adapt drawing schemas from kicad, or maybe some other test->drawing language, to be possible to use text diff tool to see the differences
 * Versioning of boards (and backlog) by git
  - Cause commits rules, and documentation rules. However I experienced a lot of resistance to people for review and versioning of documentation.
  - My experience of versioning documents in M$ Word/jira/azure that's again really bad.
 * Many renderers
  - **It have to be some browser renderer**. I don't know yet if its possible to adapt react, or js at all to make it. Maybe the only way to do it is webassebly.
  - It should be some native renderer. Because of performance, and inefficiency or present frontend technologies. I'd start from piston.

## Stack to research
* Piston, as a initial renderer
* Tokio, and all web libraries inherited
* React (browser renderer)
* Webassembly (if React would not be helpful, or maybe another separate renderer?)
* Redis, becouse the boards should be stored someware

### For later
* Kubernetes, and all of its mess with scallability
* MariaDB - If I really need relational DB for sth

## License
All rights reserved, with compatibility for all licenses that this project will inherit from.
At least until time, when I find out how to make it free and open source, but in the way I'd knew I want it to be.
Who knows. Maybe someday if I made it work, someone will donate me, at least for a coffee.
