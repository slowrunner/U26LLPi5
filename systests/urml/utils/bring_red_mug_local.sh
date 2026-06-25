#!/bin/bash

urml translate "Bring me the red mug from the kitchen." \
    --manifest manifest.yaml \
    --envelope envelope.yaml \
    --profile home \
    --provider openai \
    --model "qwen2.5:0.5b" \
    --out local.my-program.urml.yaml
