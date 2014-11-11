List of modes and module handling them
======================================

User modes
----------

- `a` : user is away, cannot be changed with MODE, only with AWAY (module: `away`)
- `i` : user is invisible (module: `core`)
- `o` : user is network operator (module: `core`)

Channel modes
-------------

- `m` : channel is moderated (module: `core`)
- `n` : only members can send messages to chan (module: `core`)
- `o <user>` : user is channel operator `@` (module: `core`)
- `s` : channel is secret (module: `core`)
- `t` : only operators can change topic (module: `core`)
- `v <user>` : user is voiced `+` (module: `core`)
