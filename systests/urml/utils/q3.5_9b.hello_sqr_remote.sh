#!/bin/bash

# optional:      --envelope envelope.yaml \

urml translate "Drive around the four corners of the room and come back home." \
    --manifest classroom.manifest.yaml \
    --profile educational \
    --provider openai \
    --model "qwen3.5:9b" \
    --out remote.qwen3.5_9b.hello-square.program.urml.yaml
