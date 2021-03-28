# Physics Plotter
Originally created to plot physics two-variable observation data with best-fit lines, max,min-gradient lines, and error bars.
But you can plot any such dataset as you like.

## Prerequisite
Optional: 
 - Install `gnuplot`. (Only required if you plan to use this backend)
 - Install GTK. (Only required if you want to build the GUI)
 - Install `cargo-bundle`. (Only required if you want to build app bundles or installers)

## CLI Usage
1. Create a data file with your data in this format:
 - Each line is a pair of data.
 - Write two columns like this: `10.20 anything -1.00`.  
   And the x value will be `10.20` while y is `-1.00`.
 - Three columns means an explicit value of y uncertainty (the third column).  
   e.g. `10.20, -1.00 0.01` means this y value has an uncertainty of `0.01`.
 - Four columns means both an explicit x uncertainty (the second column) and y uncertainty (the fourth column)  
   e.g. `10.20 0.02, -1.00 0.01` means this pair of x, y value has uncertainties of `0.02, 0.01`, respectively.
 - Anything other than numbers, like labels, units, commas, or even `±` signs are ignored.
 
2. Run `phys_plotter -t <title> -x <x_label> -y <y_label> <your_data_file>`.
3. Enjoy the graph!
4. For more options, please run `phys_plotter --help`.

## GUI Usage
1. Input your data in the same format as above.
2. Change the titles and labels on the left.
3. Click `Ctrl+G` (or `⌘+G` on a Mac).

## Building
- To build everything, run `cargo build --release`.
- To build only the GUI, run `cargo build --release --bin phys_plotter_gui`.
- To build only the CLI, run `cargo build --release --bin phys_plotter`.
- To create a macOS app or other installers, run `cargo bundle --release --bin phys_plotter_gui`.

## Contributing
Propose anything with Issues and Pull Requests!
