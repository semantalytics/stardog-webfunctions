#!/usr/bin/env bash

ssh -N -L 5002:localhost:5001 -L 5821:localhost:5820 wf.semantalytics.com &
