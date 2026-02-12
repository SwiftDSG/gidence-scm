export type Processor = {
  id: string;
  name: string;
  model: string;
  address: ProcessorAddress;
  webhook?: ProcessorWebhook;
  version: number;
};

export type ProcessorAddress = {
  host: [number, number, number, number];
  port: number;
};

export type ProcessorWebhook = {
  host: ProcessorWebhookHost;
  port?: number;
  path: {
    evidence: string;
    update: string;
  };
  secure: boolean;
};

export type ProcessorWebhookHost = string | [number, number, number, number];
