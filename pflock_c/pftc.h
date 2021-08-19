/*\

Copyright 2017 The University of North Carolina at Chapel Hill.
All Rights Reserved.

Permission to use, copy, modify and distribute this software and its
documentation for educational, research and non-profit purposes, without
fee, and without a written agreement is hereby granted, provided that the
above copyright notice and the following three paragraphs appear in all
copies.

IN NO EVENT SHALL THE UNIVERSITY OF NORTH CAROLINA AT CHAPEL HILL BE
LIABLE TO ANY PARTY FOR DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR
CONSEQUENTIAL DAMAGES, INCLUDING LOST PROFITS, ARISING OUT OF THE
USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF THE UNIVERSITY
OF NORTH CAROLINA HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH
DAMAGES.

THE UNIVERSITY OF NORTH CAROLINA SPECIFICALLY DISCLAIM ANY
WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE SOFTWARE
PROVIDED HEREUNDER IS ON AN "AS IS" BASIS, AND THE UNIVERSITY OF
NORTH CAROLINA HAS NO OBLIGATIONS TO PROVIDE MAINTENANCE, SUPPORT,
UPDATES, ENHANCEMENTS, OR MODIFICATIONS.

The authors may be contacted via:

US Mail: Real-Time Systems Group at UNC
Department of Computer Science
Sitterson Hall
University of N. Carolina
Chapel Hill, NC 27599-3175

EMail: nemitz@cs.unc.edu; tamert@cs.unc.edu; anderson@cs.unc.edu

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

typedef struct pftc_lock_struct {

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
   /* unsigned int w;

    // Increment the rin count and read the writer bits
    w = __sync_fetch_and_add(&lock->rin, PF_RINC) & PF_WBITS;

    // Spin (wait) if there is a writer present (w != 0), until
    // either PRES and/or PHID flips
    while ((w != 0) && (w == (lock->rin & PF_WBITS)))
    {
        cpu_relax();
    }*/

    	u32 blocked = xadd32(4, &l->rin) & 0x3;
	while (blocked && ((l->rin & 0x3) == blocked))
		cpu_relax();

}

/*
 *  Phase-Fair (ticket) Lock: read unlock.
 */
void pftc_read_unlock(pftc_lock_t *l)
{
    // Increment rout to mark the read-lock returned
    //__sync_fetch_and_add(&lock->rout, PF_RINC);

    xadd32(4, &l->rout);
}

/*
 *  Phase-Fair (ticket) Lock: write lock.
 */
void pftc_write_lock(pftc_lock_t *l)
{
    /*
    unsigned int w, rticket, wticket;

    // Wait until it is my turn to write-lock the resource
    wticket = __sync_fetch_and_add(&lock->win, 1);
    while (wticket != lock->wout)
    {
        cpu_relax();
    }

    // Set the write-bits of rin to indicate this writer is here
    w = PF_PRES | (wticket & PF_PHID);
    rticket = __sync_fetch_and_add(&lock->rin, w);

    // Wait until all current readers have finished (i.e rout
    // catches up)
    while (rticket != lock->rout)
    {
        cpu_relax();
    }*/

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
	/*
    unsigned int andoperand;

    // Clear the least-significant byte of rin
    andoperand = -256;
    __sync_fetch_and_and(&lock->rin, andoperand);

    // Increment wout to indicate this write has released the lock
    lock->wout++; // only one writer should ever be here
*/
    	u32 ticket = l->wout;
	xadd32((u32) -(0x2 | (ticket & 0x1)), &l->rin);
	l->wout++;	
}

#endif // PFTC_H
