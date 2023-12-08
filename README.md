# Mastodon Admin Directory tools

experimental code for finding all the profile info




## How to build and run

with rust installed...
```
cargo run
```

## What works so far
shows the id and username for local directory result
along with count (may be just first page, not sure yet)

sample output:
```
[
    Account {
        id: "109492876944531521",
        username: "Jonah",
    },
    Account {
        id: "109558956120383361",
        username: "olaf_radicke",
    },
    Account {
        id: "109691471888992464",
        username: "johanpdx",
    },
    :
    :
]
count: 27

```

see complete profile format in [result.json](result.json) -- just public data