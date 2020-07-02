#!/bin/bash
export LANG='en_US.UTF-8'
export LC_ALL='en_US.UTF-8'
cd /home/bwbot/data/src/galileo_matrix
/home/bwbot/.cargo/bin/cargo run --release
