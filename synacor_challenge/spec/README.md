Taken from: https://github.com/Aneurysm9/vm_challenge

# Synacor Challenge

This repository holds my copy of the architecture specification and challenge
binary from the Synacor Challenge as presented at OSCON 2012.  It is preserved
for historical and educational purposes.  

## Codes

I have included MD5 hashes of the eight codes produced by this instance of the
challenge for use in testing implementations of the architecture.  These codes
can be validated as follows, replacing the quoted string with the code to test:

```console
$ echo -n "<Code Here>" | md5sum
6fcd818224b42f563e10b91b4f2a5ae8  -
```

- 76ec2408e8fe3f1753c25db51efd8eb3
- 0e6aa7be1f68d930926d72b3741a145c
- 7997a3b2941eab92c1c0345d5747b420
- 186f842951c0dcfe8838af1e7222b7d4
- 2bf84e54b95ce97aefd9fc920451fc45
- e09640936b3ef532b7b8e83ce8f125f4
- 4873cf6b76f62ac7d5a53605b2535a0c
- d0c54d4ed7f943280ce3e19532dbb1a6

Status:
- 76ec2408e8fe3f1753c25db51efd8eb3
  - Got the first one by reading the documentation
- 0e6aa7be1f68d930926d72b3741a145c
  - Got this one by implementing opcodes 0, 19, 21
- 7997a3b2941eab92c1c0345d5747b420
  - Got this one after implementing all opcodes except for 20
- 186f842951c0dcfe8838af1e7222b7d4
  - Got this one after implementing opcode In and starting the game
- 2bf84e54b95ce97aefd9fc920451fc45
  - Got this in the maze
- e09640936b3ef532b7b8e83ce8f125f4
  - After using teleporter
- 4873cf6b76f62ac7d5a53605b2535a0c
  - At instruction_counter 6049 set R[0]=0 and R[1]=5, code on beach
  - Don't forget to reverse engineer the algo from the byte code
- d0c54d4ed7f943280ce3e19532dbb1a6
  - Solve last puzzle. use mirror (don't forget to flip code at the end)
