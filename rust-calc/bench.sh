#!/usr/bin/env bash

export RUSTFLAGS="-Ctarget-cpu=native"

cargo bench
