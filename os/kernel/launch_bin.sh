#!/bin/bash

../../1-shell/ttywrite/target/release/ttywrite -i build/kernel.bin /dev/ttyUSB0
screen /dev/ttyUSB0 115200