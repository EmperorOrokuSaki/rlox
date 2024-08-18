#!/bin/bash

# Define color codes
MAGENTA='\033[1;37m'    # Magenta for headers
YELLOW='\033[1;33m'     # Yellow for action descriptions
CYAN='\033[0;36m'       # Cyan for decorative borders
NC='\033[0m'            # No Color / Reset

# Print the header with fancy formatting
echo -e "${MAGENTA}=============================="
echo -e "   rLox Interpreter Benchmark"
echo -e "==============================${NC}"

# Print the message for printing the input file
echo -e "${YELLOW}Printing the input test file${NC}"

# Use cat to print the contents of ./test/input.lox
echo -e "${CYAN}--------------------------------"
cat ./test/input.lox
echo -e "--------------------------------${NC}"

# Print the message for running the rLox interpreter
echo -e "${YELLOW}Running the rLox interpreter${NC}"

# Run the rLox interpreter using cargo and time the execution
echo -e "${CYAN}--------------------------------"
time cargo run -- --path ./test/input.lox
echo -e "--------------------------------${NC}"

# Print benchmark results
echo -e "${YELLOW}Benchmark results above (execution time of rLox interpreter)${NC}"
