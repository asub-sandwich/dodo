# `dodo`: A minimal to-do app

This document contains the help content for the `dodo` command-line program.

**Command Overview:**

* [`dodo`↴](#dodo)
* [`dodo add`↴](#dodo-add)
* [`dodo done`↴](#dodo-done)
* [`dodo urge`↴](#dodo-urge)
* [`dodo norm`↴](#dodo-norm)
* [`dodo remove`↴](#dodo-remove)
* [`dodo up`↴](#dodo-up)
* [`dodo down`↴](#dodo-down)

## `dodo`

**Usage:** `dodo [COMMAND]`

###### **Subcommands:**

* `add` — Add a task
* `done` — Mark a task as completed
* `urge` — Mark a task as urgent
* `norm` — Mark a task as normal
* `remove` — Delete an item from the list (with ID)
* `up` — Make a task seem more important
* `down` — Make a task seem less important



## `dodo add`

Add a task

**Usage:** `dodo add [TASK]...`

###### **Arguments:**

* `<TASK>`



## `dodo done`

Mark a task as completed

**Usage:** `dodo done [ID]...`

###### **Arguments:**

* `<ID>`



## `dodo urge`

Mark a task as urgent

**Usage:** `dodo urge [ID]...`

###### **Arguments:**

* `<ID>`



## `dodo norm`

Mark a task as normal

**Usage:** `dodo norm [ID]...`

###### **Arguments:**

* `<ID>`



## `dodo remove`

Delete an item from the list (with ID)

**Usage:** `dodo remove [ID]...`

###### **Arguments:**

* `<ID>`



## `dodo up`

Make a task seem more important

**Usage:** `dodo up [ID] [COUNT]`

###### **Arguments:**

* `<ID>`
* `<COUNT>`



## `dodo down`

Make a task seem less important

**Usage:** `dodo down [ID] [COUNT]`

###### **Arguments:**

* `<ID>`
* `<COUNT>`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

