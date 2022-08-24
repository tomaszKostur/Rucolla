# Rucolla - Rust Collaboration Tool
## This supposed to be my personal project for rust research
### What it should be?
 It should be a collaboration tool, made in Rust. Implementation of a virtual blackboard, possible to share with other, remote co-workers.
 It's inspired by Miro board, and any software for managing tasks like Jira.
 The idea is to have that both functionalities in one tool.
 Advantages to aim:
 * better sharing, with live view of what other participants are doing right now (unlike Jira)
 * better performance, because Miro gets very slow and unresponsive quickly, if amount of content grows.
 * Better visually tracking of differences appeared on boards
    - To consider is to adapt drawing schemas from KiCad, or maybe some other tekst->drawing language, to be possible to use text diff tool to see the differences
 * Versioning of boards (and backlog) by git
    - There is already idea "Infrastructure as a code", why not "task management as a code"
    - Commits rules, and documentation rules. However, I experienced a lot of resistance of people to review and version a documentation.
    - My experience of versioning documents in M$ Word/jira/azure that's again terrible.
 * Many renderers
    - **It has to be some browser renderer**. I don't know yet if it's possible to adapt react, or JS at all, to make it. Maybe the only way to do it is WebAssembly.
    - It should be some native renderer. Because of performance, and inefficiency of present frontend technologies (electron for eg. ). I'd start from piston.

## Stack to research
* Piston, as an initial renderer
* Tokio, and all web libraries inherited
* React (browser renderer)
* WebAssembly (if React would not be helpful, or maybe another separate renderer?)
* Redis, because the boards should be stored someway

### For later
* Kubernetes, and all of its mess with scalability
* MariaDB - If I really need relational DB for sth

## License
All rights reserved, with compatibility for all licenses that this project will inherit from.
At least until time, when I find out how to make it free and open source, but in the way I'd know I want it to be.
Who knows. Maybe someday, if I made it work, someone will donate to me, at least for a coffee.
