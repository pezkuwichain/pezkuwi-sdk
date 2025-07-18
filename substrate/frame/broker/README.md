# Pallet Broker

Brokerage tool for managing Pezkuwi Core scheduling.

Properly described in [RFC-0001 Agile Coretime](https://github.com/pezkuwi-fellows/RFCs/blob/main/text/0001-agile-coretime.md).

## Implementation Specifics

### Core Mask Bits

This is 1/80th of a Pezkuwi Core per timeslice. Assuming timeslices are 80 blocks, then this
indicates usage of a single core one time over a timeslice.

### The Sale

```nocompile
					1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7
--------------------------------------------------------
< interlude  >
			  <                   sale                 >
							... of which ...
			  <  descending-price   ><   fixed-price   >
														| <-------\
price fixed, unsold assigned to instapool, system cores reserved -/
```
