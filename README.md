# WISCANet GUI

This interface handles configuring nodes for program execution (handles node YAML and iplist)

### Ideas/ToDo
- perhaps use: https://github.com/isaacHagoel/svelte-dnd-action to drag and drop pieces together, or add/sub from list, and drop downs
- add delete button to node/app pair

## Models (Database)

Each one of these is plural, because there will be a list in the database

### Edge Node

- ipaddress
- name
- radio_type
- address/serial
- available parameters

### Node Parameters

- logic_id
- op_mode
- macmode
- matDir
- bbMatlab
- LogMatlab - always NULL
- nsamps - always 50000 right now
- rate (sample)
- subdev
- freq
- txgain
- rxgain
- bw
- radio_type link to edge node being assigned to
- channels
- antennas

# Legal
Copyright 2020, WISCANet Contributors
