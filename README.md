Solution:

The first thing i can think of is the use of a balanced Binary search tree for inserting and finding out collisions in the full packet
just like operating system kernels use for VMA tracking

the second thing is the allocation for the full packet, we dont know the full size beforehand.
do i iterate over all fragments once and sum their lengths and then allocate a buffer?

after allocating the full message buffer start reading fragments and inserting them in order to the tree-map

or actually instead, just start reading fragments and insert into the tree-map if there is a collision/gap it will be found before any allocation is even made
if there is no conflict/gap then we can start building the original message: allocate and then memcpy the data from each fragment

