# PFlock: Phase-Fair Reader-Writer Lock

This library provides a phase-fair reader-writer lock, as described in the
paper ["Reader-Writer Synchronization for Shared-Memory Multiprocessor
Real-Time Systems"][paper]. by Brandenburg et. al.

> Reader preference, writer preference, and task-fair reader-writer locks are
> shown to cause undue blocking in multiprocessor real-time systems. A new
> phase-fair reader-writer lock is proposed as an alternative that
> significantly reduces worst-case blocking for readers.

## Example

```rust
use pflock::PFLock;

let lock = PFLock::new(5);

// many reader locks can be held at once
{
    let r1 = lock.read();
    let r2 = lock.read();
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
} // read locks are dropped at this point

// only one write lock may be held, however
{
    let mut w = lock.write();
    *w += 1;
    assert_eq!(*w, 6);
} // write lock is dropped here
```

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

## C implementation

A reference implementation in C is provided in the branch [cnord/ffi](https://github.com/cmnord/pflock/tree/cnord/ffi) in the directory[pflock_c/](https://github.com/cmnord/pflock/tree/cnord/ffi/pflock_c). Run tests with the reference implementation using `RUSTFLAGS="--cfg c_reference"`, e.g.

```bash
RUSTFLAGS="--cfg c_reference" cargo test
```

## License

All code is under the MIT license except for the C implementation in
[pflock_c/](https://github.com/cmnord/pflock/tree/cnord/ffi/pflock_c), which has its own license in the file.

[paper]: https://www.cs.unc.edu/~anderson/papers/ecrts09b.pdf
