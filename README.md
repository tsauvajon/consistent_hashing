[![codecov](https://codecov.io/gh/tsauvajon/consistent_hashing/branch/master/graph/badge.svg?token=zpMTxerH5W)](https://codecov.io/gh/tsauvajon/consistent_hashing)

# Consistent hashing

Consistent hashing is an optimised way to shard data on multiple servers.

Let's say for example we want to store 3 values ("hello", "world", "thomas")
on three servers (A, B, C).

#### Simple sharding

![Simple sharding](/docs/simple_sharding.png)

A simple sharding strategy could be to %3 that data. If `"hello"%3 == 0` it goes on server A,
`"world"%3 == 1` so it goes on server B and `"thomas"%3 == 2` so it goes on server C.

Problems arise when we want to add or remove a server: rebalancing will move almost all the data from server to server.
If we add a server, we now have to %4 instead of %3, so the server for most already-existing keys will be recalculated.

#### Sharding ring

![Sharding ring](/docs/ring.png)

We can partially solve that by adding a "Ring" of servers.

A ring is a virtual range of numbers, for example 0..255 in our implementation.
Servers can get a position on that ring, for example A = 0, B = 64, C = 128.

To know which server has the key we want, we simply hash our key (for example `hash("world") == 120`),
and walk the ring clockwise until we find a server. In our case, the first server we would find
is server C, at position 128: C is the server that contains our value.

This approach makes rebalancing (adding or removing a server) easier: if we had a server D at position 192,
and we remove it, we simply have to move all keys with a hash between 128 and 192 to A instead.

When adding a new server, we can find the optimal position easily: take the server that has the biggest
range and split it in half. In the example picture, we would add a server at position 192, and move all
keys between 128 and 192 to the new server.

Though, this is still sub-optimal: it is very easy to get imbalanced load between servers.
Also, when rebalancing, the most busy server will also be the one that has to send most of the data to the new server!

#### Consistent hashing

![Consistent hashing](/docs/consistent_hashing.png)

To make the hashing more consistent, we can think of having multiple "virtual nodes" for each server on
the ring, instead of just one.

In the example picture, each server (A, B, C) has 3 virtual nodes in the ring. In this repo's implementation, each server has 5 virtual nodes instead.
We follow the same rules as above: we hash our data, and find the next virtual node on the ring to know
which server has our data.

The virtual nodes are randomly placed (there can only be one virtual node per position), but since there are
many nodes for each server, the repartition between servers is much more evened out. Adding or removing
a node doesn't negatively impact the repartition. Also, rebalancing is spread out between several nodes:
for example, on the picture above, if we added server D at positions 10, 115 and 203, we would move just
a few keys from servers A, B and C.


# TODO

Find a way to build a visualisation for the ring.
