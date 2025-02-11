#!/bin/bash

sudo apt update -y && sudo apt install -y gcc sysstat linux-tools-common linux-tools-5.4.0-1045-aws

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env