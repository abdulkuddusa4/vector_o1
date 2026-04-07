# O_1_Vector
O_1_Vector is a Vector data structure implementation with constant time read write update
remove without sacrificing data order in exchange for some space complexity.

the data structure consists of 3 vector Vec<Data>, Vec<Id>, Vec<Index> of the same length.- 

## the only downside of this data structure is 
  - 2 extra parallel Vector- one for index -> data mapping and one for data -> index maping.
  - its auxilary Vectors Vec<Id> Vec<Index> only grows in sizes.
      for example,
         if it stores 5 element and then removes 2, index and id array length will be unchanged.
         But if we add more element again we can use the existing one.
