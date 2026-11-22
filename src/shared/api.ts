import { invokeCommand } from "./commands";

export type AppStatus = {
  appName: string;
  version: string;
  databaseReady: boolean;
  schemaVersion: number;
  currencyCode: string;
};

export function getAppStatus(): Promise<AppStatus> {
  return invokeCommand<AppStatus>("get_app_status");
}
