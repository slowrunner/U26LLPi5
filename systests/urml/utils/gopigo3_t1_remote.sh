#!/bin/bash

# optional:      --envelope envelope.yaml \

urml translate "Announce 'Driving forward 1 meter' and then execute the action" \
    --manifest gopigo3.manifest.yaml \
    --profile educational \
    --no-policy \
    --provider openai \
    --model "qwen3.5:4b" \
    --out remote.qwen3.5_4b.gopigo3_t1.program.urml.yaml
