# Dancing Links Sudoku

A Rust implementation of the most efficent sudoku solving algorithm: Dancing
Links.

### Performance


| Algorithm           |  Time per Board     |
|---------------------|---------------------|
| Dancing Links       | **0.107 ms**        |
| Depth First Search  | **0.374 ms**        |
| Algorithm X         | **253 ms**          |

### The Algorithm

Dancing Links requires reducing the problem to and from an [exact cover problem](https://en.wikipedia.org/wiki/Exact_cover#Incidence_matrix).

This article was instrumental in teaching me how to perform the reduction: 
[https://web.archive.org/web/20230426084731/https://garethrees.org/2007/06/10/zendoku-generation/](https://web.archive.org/web/20230426084731/https://garethrees.org/2007/06/10/zendoku-generation/).

Once you have the exact cover matrix, you need to turn it into a doubly linked
table. Each `1` in the matrix contains a pointer to the nearest 
`left`/`right`/`up`/`down` `1` value. The pointers may wrap around to the other
side of the table.

For example:

![Dancing links doubly linked circular matrix](https://web.archive.org/web/20230426084731im_/https://garethrees.org/2007/06/10/zendoku-generation/dancing-links-1.png)

Dancing links efficiently finds a solution by rewiring the pointers to hide
columns and cells that could not contribute to our solution, shrinking the
problem space as it runs. 

Pointers after covering the first column:

![Dancing links table after the first column is hidden](https://web.archive.org/web/20230426084731im_/https://garethrees.org/2007/06/10/zendoku-generation/dancing-links-2.png)

Just as importantly, dancing links is able to backtrack from dead ends by
revealing previously hidden columns and cells using only a few pointer updates.

To understand the low level mechanics of this algorithm you will need to read
the afformentioned amazing article: 

[https://web.archive.org/web/20230426084731/https://garethrees.org/2007/06/10/zendoku-generation/](https://web.archive.org/web/20230426084731/https://garethrees.org/2007/06/10/zendoku-generation/).

and also Donald Knuths Paper to support it (Particularly the very first section
as well as the section with images showing dancing link table pointer updates):
[https://www.ocf.berkeley.edu/~jchu/publicportal/sudoku/0011047.pdf](https://www.ocf.berkeley.edu/~jchu/publicportal/sudoku/0011047.pdf)


