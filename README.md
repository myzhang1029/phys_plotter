# Physics Plotter

***Archived: I am no longer willing to update the codebase to work with newer packages.***

Originally created to plot physics two-variable observation data with best-fit lines, max,min-gradient lines, and error bars.
But you can plot any such dataset as you like.

## Prerequisite
Optional: 
 - Install `gnuplot`. (Only required if you plan to use this backend)
 - Install GTK. (Only required if you want to build the GTK GUI)
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
2. Change the titles and labels.
3. Click "Plot".

## Building
- To build everything, run `cargo build --release --features=ui_gtk,ui_egui`.
- To build only the GTK GUI, run `cargo build --release --bin phys_plotter_gtk --features=ui_gtk`.
- To build only the egui GUI, run `cargo build --release --bin phys_plotter_egui --features=ui_egui`.
- To build only the CLI, run `cargo build --release --bin phys_plotter`.
- To create a macOS app or other installers, run `cargo bundle --release --bin phys_plotter_gui`.

## Contributing
Propose anything with Issues and Pull Requests!
