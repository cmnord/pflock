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

#ifndef I386_MEM_H
#define I386_MEM_H

static inline void mb(void)
{
    __asm__ __volatile__("mfence" : : : "memory");
}

static inline void cpu_relax(void)
{
    __asm__ __volatile__("pause");
}

#endif
