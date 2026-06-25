#!/bin/bash

# optional:      --envelope envelope.yaml \

urml translate "Drive around the four corners of the room and come back home." \
    --manifest classroom.manifest.yaml \
    --profile educational \
    --no-policy \
    --provider openai \
    --model "qwen2.5:1.5b" \
    --out local.qwen2.5_1.5b.hello-square.program.urml.yaml
    --save-rejected local.fail.qwen2.5_1.5b.hello-square.out
