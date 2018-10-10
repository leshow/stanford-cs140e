#!/bin/bash

sudo ../../1-shell/ttywrite/target/release/ttywrite -i build/kernel.bin /dev/ttyUSB0
sudo screen /dev/ttyUSB0 115200