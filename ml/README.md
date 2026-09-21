# Echora ML

## Status

**Research design only. No production model is currently claimed.**

## Responsibilities

This area will contain:
- dataset construction;
- feature/embedding research;
- supervised/self-supervised training;
- evaluation;
- confidence calibration;
- model export;
- model cards.

## Initial approach

Do not begin with pose. Build capability incrementally:
1. empty/occupied;
2. motion;
3. respiration;
4. occupancy/localization;
5. pose.

A simple statistical baseline is required before a complex neural model is accepted.

## Dataset rules

Splits should isolate the strongest applicable dimensions:
- person;
- room;
- session;
- hardware;
- firmware;
- topology/layout.

Adjacent overlapping windows from one recording may not appear on both train and test sides unless that weaker evaluation is explicitly named.

## Model output

Models must return uncertainty/confidence and support abstention. A model file alone is not a capability.

## Export

Preferred portable path is PyTorch → ONNX when numerical parity is proven. Runtime selection remains an ADR when inference code is implemented.

## Required model card fields

- purpose/non-goals;
- input feature contract;
- training data;
- split method;
- metrics and definitions;
- per-class/per-condition results;
- confidence/rejection behavior;
- limitations;
- supported hardware/runtime;
- artifact hash/license.
