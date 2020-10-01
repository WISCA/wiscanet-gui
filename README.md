
# WISCANET GUI

This interface handles configuring nodes for program execution (handles YAML and iplist)

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
