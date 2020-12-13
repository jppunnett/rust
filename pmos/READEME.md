# pmos (Photomosaic)

A Rust program for creating a photomosaic.

Given an original input image, `pmos` creates a new image using other, smaller
images.

Insipred by [Robert Heaton's Advanced Beginners](https://robertheaton.com). 

## Examples

The following example creates a photomosaic of `original.png` and stores it in
a file named original_pmos.png in the directory where you ran `pmos`.

	pmos original.png
