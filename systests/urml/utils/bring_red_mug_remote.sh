#!/bin/bash

# optional:      --envelope envelope.yaml \

urml translate "Bring me the red mug from the kitchen." \
    --manifest manifest.yaml \
    --profile home \
    --provider openai \
    --model "qwen2.5:3b" \
    --out remote.qwen2.5_3b.red_mug.program.urml.yaml
