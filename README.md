# Physics Plotter
Originally created to plot physics two-variable observation data with best-fit lines, max,min-gradient lines, and error bars.
But you can plot any such dataset as you like.

## Prerequisite
Install `gnuplot` (will be removed soon).

## Usage
1. Create a data file with your data in this format:
 - Each line is a pair of data
 - Write two columns like this: `10.20 anything -1.00`.  
   And the x value will be `10.20` while y is `-1.00`.
 - Three columns means an explicit value of y uncertainty (the third column).  
   e.g. `10.20, -1.00 0.01` means this y value has an uncertainty of `0.01`.
 - Four columns means both an explicit x uncertainty (the second column) and y uncertainty (the fourth column)  
   e.g. `10.20 0.02, -1.00 0.01` means this pair of x, y value has uncertainties of `0.02, 0.01`, respectively.
 - Anything other than numbers, like labels, units, commas, or even `±` signs are ignored.
 
2. Run `phys_plotter -t <title> -x <x_label> -y <y_label> <your_data_file>`
3. Enjoy the graph!

## Contributing
Propose anything with Issues and Pull Requests!
