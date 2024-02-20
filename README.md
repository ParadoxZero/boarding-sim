# Boarding Simulator

The project consists of a boarding simulator written in Rust. It simulates the process of people boarding a plane based on various parameters such as the number of rows, columns, and seats per column. The simulation tracks the movement of people within the plane and measures the total ticks required for the boarding process.

```bash
Run a simulation of people boarding plane based on the given parameters

Usage: boarding-sim [OPTIONS]

Options:
  -r, --rows <ROWS>    Number of rows in the plane [default: 10]
  -c, --cols <COLS>    Number of columns in the plane [default: 1]
  -s, --seats <SEATS>  Number of seats per column [default: 3]
  -h, --help           Print help
  -V, --version        Print version
```

