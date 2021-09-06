#!/bin/bash
diesel database setup && diesel migration run && ./target/release/api