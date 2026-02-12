import type { Camera } from "./camera";
import type { Evidence } from "./evidence";
import type { Processor } from "./processor";

export type View = "large" | "medium" | "small";
export type Theme = "light" | "dark";
export type State = "idle" | "changing";

export type Device = {
  processor: Processor;
  camera: Camera[];
};

export type Reading = {
  camera: {
    [id: string]: {
      evidence: Evidence | null;
      timestamp: number;
    };
  };
};

export type Menu = {
  processor?: {},
  processorInformation?: {},
  camera?: { camera_id: string },
  cameraDelete?: { camera_id: string },
  cameraInformation?: { camera_id?: string },
}