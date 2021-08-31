/*\
 Lock was taken from Bjorn Brandenburg's disseration for which the 
 code is available here: https://www.cs.unc.edu/~bbb/diss/
 with adjustment for lock structure declaration starting line 36.
**/

#ifndef PFTC_H
#define PFTC_H

#include "mem.h"
#include <linux/types.h>

/*
typedef __u32 u32;
static __inline__ u32 xadd32(u32 i, volatile u32* mem)
{
	u32 inc = i;
	__asm__ __volatile__(
		"lock; xaddl %0, %1"
		:"+r" (i), "+m" (*mem)
		: : "memory");
	return i + inc;
}*/


// Phase-Fair (ticket) Lock
type
def struct pftc_lock_struct {

    volatile u32 win;
    volatile u32 wout;
    volatile u32 rin;
    volatile u32 rout;
    unsigned int _buf1[14];

} __attribute ((aligned (16) )) pftc_lock_t;


/*
 *  Phase-Fair (ticket) Lock: initialize.
 */
void pftc_lock_init(pftc_lock_t *lock)
{
    lock->rin = 0;
    lock->rout = 0;

    lock->win = 0;
    lock->wout = 0;
}

/*
 *  Phase-Fair (ticket) Lock: read lock.
 */
void pftc_read_lock(pftc_lock_t *l)
{
    u32 blocked = xadd32(4, &l->rin) & 0x3;
	while (blocked && ((l->rin & 0x3) == blocked))
		cpu_relax();

}

/*
 *  Phase-Fair (ticket) Lock: read unlock.
 */
void pftc_read_unlock(pftc_lock_t *l)
{
    xadd32(4, &l->rout);
}

/*
 *  Phase-Fair (ticket) Lock: write lock.
 */
void pftc_write_lock(pftc_lock_t *l)
{
    u32 ticket;
	ticket = xadd32(1, &l->win) - 1;
	while (ticket != l->wout)  {
		cpu_relax();
	}
	ticket = xadd32(0x2 | (ticket & 0x1), &l->rin) & (~(u32)3);
	while (ticket != l->rout)
		cpu_relax();
}

/*
 *  Phase-Fair (ticket) Lock: write unlock.
 */
void pftc_write_unlock(pftc_lock_t *l)
{
    u32 ticket = l->wout;
	xadd32((u32) -(0x2 | (ticket & 0x1)), &l->rin);
	l->wout++;	
}

#endif // PFTC_H
