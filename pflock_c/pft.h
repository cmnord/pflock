/*\
 Lock was taken from Bjorn Brandenburg's disseration for which the 
 code is available here: https://www.cs.unc.edu/~bbb/diss/
 with adjustment for lock structure declaration starting line 36.
**/

#ifndef PFT_H
#define PFT_H

#include "mem.h"
#include <linux/types.h>

typedef __u32 u32;
static __inline__ u32 xadd32(u32 i, volatile u32* mem)
{
	u32 inc = i;
	__asm__ __volatile__(
		"lock; xaddl %0, %1"
		:"+r" (i), "+m" (*mem)
		: : "memory");
	return i + inc;
}


// Phase-Fair (ticket) Lock
#define PF_RINC 0x100 // reader increment
#define PF_WBITS 0x3  // writer bits in rin
#define PF_PRES 0x2   // writer present bit
#define PF_PHID 0x1   // writer phase ID bit

typedef struct pft_lock_struct {

    /* 
     * Modified lock struct to cache align struct attributes based on machine.
     */

    volatile u32 win;
    u32 _b1;
    unsigned int _buf1[15];

    volatile u32 wout;
    u32 _b2;
    unsigned int _buf2[15];

    volatile u32 rin;
    u32 _b3;
    unsigned int _buf3[15];

    volatile u32  rout;
     u32 _b4;
     unsigned int _buf4[15];
} __attribute ((aligned (16) )) pft_lock_t;


/*
 *  Phase-Fair (ticket) Lock: initialize.
 */
void pft_lock_init(pft_lock_t *lock)
{
    lock->rin = 0;
    lock->rout = 0;

    lock->win = 0;
    lock->wout = 0;
}

/*
 *  Phase-Fair (ticket) Lock: read lock.
 */
void pft_read_lock(pft_lock_t *l)
{

    u32 blocked = xadd32(4, &l->rin) & 0x3;
	while (blocked && ((l->rin & 0x3) == blocked))
		cpu_relax();

}

/*
 *  Phase-Fair (ticket) Lock: read unlock.
 */
void pft_read_unlock(pft_lock_t *l)
{
    xadd32(4, &l->rout);
}

/*
 *  Phase-Fair (ticket) Lock: write lock.
 */
void pft_write_lock(pft_lock_t *l)
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
void pft_write_unlock(pft_lock_t *l)
{
    u32 ticket = l->wout;
	xadd32((u32) -(0x2 | (ticket & 0x1)), &l->rin);
	l->wout++;	
}

#endif // PFT_H
