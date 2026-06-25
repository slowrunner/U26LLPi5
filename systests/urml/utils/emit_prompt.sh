#!/bin/bash

# optional?     --envelope envelope.yaml \

urml emit-prompt  \
    --manifest classroom.manifest.yaml \
    --profile educational \
    --no-few-shots \
    --out classroom.prompt.txt
