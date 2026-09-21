# Echora Hardware

## Initial BOM

Minimum development setup:
- 1× ESP32-S3-DevKitC-1 class board;
- data-capable USB cable;
- existing 2.4 GHz Wi-Fi AP/router;
- development PC.

Spatial phase:
- 3–6 sensing nodes;
- fixed mounts/known positions;
- optional Kinect/RGB-D/camera reference instrument.

## Topology

Single-link work is for acquisition, presence, motion and respiration experiments. Localization and pose research require multiple spatially distinct links and explicit geometry.

## Installation record

Each physical experiment should record:
- node IDs;
- board/SoC/revision;
- antenna type/orientation;
- XYZ pose in room frame;
- AP identity/position/channel;
- distance to sensing region;
- firmware hash/version;
- power source;
- environmental notes.

Moving a node can invalidate calibration even when the code does not change.

## Reference instrumentation

Kinect/RGB-D/camera is used only in controlled capture/validation. It should have its own:
- serial/device identity;
- calibration;
- frame rate/resolution;
- position/orientation;
- synchronization record;
- reference-model version if keypoints are algorithmically generated.
