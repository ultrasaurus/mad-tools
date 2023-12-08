# Mastodon Admin Directory tools

experimental code for finding all the profile info


## Directory API

Directory API (seems separate from ActivityPub)
https://docs.joinmastodon.org/methods/directory/
- seems to be for just public data?

## How to build and run

with rust installed...
```
cargo run
```

## What works so far
shows some info for local directory listing of [active][] users
followed by result count

sample output:
```
[
    :
    :
   Account {
        id: "109492876944531521",
        username: "Jonah",
        display_name: "Jonah",
        created_at: 2022-12-11T00:00:00Z,
    },
    Account {
        id: "109485699733442779",
        username: "david",
        display_name: "David Coletta",
        created_at: 2022-12-09T00:00:00Z,
    },
    Account {
        id: "109480407171500507",
        username: "ultrasaurus",
        display_name: "Sarah Allen",
        created_at: 2022-12-08T00:00:00Z,
    },
]
count: 27

```

see complete profile format in [result.json](result.json) 


[active]: "Active" users [definition](https://github.com/mastodon/mastodon/discussions/18920) - people who have been logged in at one point during a period of time (7 days [default](https://github.com/mastodon/mastodon/blob/main/app/models/user.rb) )
