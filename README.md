# PFlock: Phase-Fair Reader-Writer Lock

From ["Reader-Writer Synchronization for Shared-Memory Multiprocessor Real-Time
Systems"][paper] by Brandenburg et. al.

```latex
@inproceedings{brandenburg2009reader,
  title={Reader-writer synchronization for shared-memory multiprocessor real-time systems},
  author={Brandenburg, Bj{\"o}rn B and Anderson, James H},
  booktitle={2009 21st Euromicro Conference on Real-Time Systems},
  pages={184--193},
  year={2009},
  organization={IEEE}
}
```

## License

All code is under the MIT license except for the C implementation in
[pflock_c](pflock_c/), which has its own license in the file.

[paper]: https://www.cs.unc.edu/~anderson/papers/ecrts09b.pdf