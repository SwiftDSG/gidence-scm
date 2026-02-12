export type Camera = {
  id: string;
  name: string;
  address: CameraAddress;
};

export type CameraAddress = {
  host: [number, number, number, number];
  port: number;
  path?: string;
  authentication?: [string, string];
};
